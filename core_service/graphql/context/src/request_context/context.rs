use crate::{CookieConfig, EnvironmentConfig, Features};
use account_service_server::AccountService;
use actor_auth::Session;
use core_service_graphql_loader::{
    CacheConfig, CustomerLoaders, HandymanLoaders, SyncSessionContext,
};
use db_utils::PgConnectionPool;
use error::{
    Error, Result,
    error_details::{BadRequest, bad_request::FieldViolation},
};
use moka::future::Cache;
use random_util::Random;
use sms_sender::SmsSender;
use std::{net::SocketAddr, ops::Deref, sync::Arc};
use tokio::sync::RwLock;

/// OTP code for account registration having TTL is 15 mins
pub const OTP_CODE_TTL_SECONDS: u64 = 60 * 15;

pub struct ContextInternal {
    /// User session extracted from graphql request
    /// Can be mutable when user login, logout, reset password, etc.
    pub session_context: Arc<RwLock<Option<Arc<Session>>>>,
    /// Connection pool to database
    pub db_connection_pool: PgConnectionPool,
    /// Server feature flags
    pub features: Features,
    /// Configs specific to each deployment environment
    pub environment_config: Arc<EnvironmentConfig>,
    pub cookie_config: Arc<CookieConfig>,
    pub remote_addr: SocketAddr,
    pub account_service_client: AccountService,
    pub sms_sender: Arc<dyn SmsSender>,
    /// Cache [e164_phone_number_str - 6 digits verification code]
    phone_pending_registration_cache: Arc<Cache<String, String>>,
    pub random: Random,
    pub customer_loaders: CustomerLoaders,
    pub handyman_loaders: HandymanLoaders,
}

pub struct NewContextParams {
    pub session_context: Option<Session>,
    pub db_connection_pool: PgConnectionPool,
    pub features: Features,
    pub environment_config: Arc<EnvironmentConfig>,
    pub cookie_config: Arc<CookieConfig>,
    pub remote_addr: SocketAddr,
    pub account_service_client: AccountService,
    pub sms_sender: Arc<dyn SmsSender>,
    pub phone_pending_registration_cache: Arc<Cache<String, String>>,
    pub loader_cache_config: CacheConfig,
}

impl ContextInternal {
    pub fn new(
        NewContextParams {
            session_context,
            db_connection_pool,
            features,
            environment_config,
            cookie_config,
            remote_addr,
            account_service_client,
            sms_sender,
            phone_pending_registration_cache,
            loader_cache_config,
        }: NewContextParams,
    ) -> Self {
        let session_context = Arc::new(RwLock::new(session_context.map(Arc::new)));
        Self {
            features,
            environment_config,
            cookie_config,
            remote_addr,
            customer_loaders: CustomerLoaders::new(
                account_service_client.clone(),
                SyncSessionContext::new(session_context.clone()),
                loader_cache_config,
            ),
            handyman_loaders: HandymanLoaders::new(
                account_service_client.clone(),
                SyncSessionContext::new(session_context.clone()),
                loader_cache_config,
            ),
            session_context,
            db_connection_pool,
            account_service_client,
            phone_pending_registration_cache: phone_pending_registration_cache.clone(),
            sms_sender,
            random: Random::default(),
        }
    }
}

#[derive(Clone)]
/// GraphQL execution per-request context.
pub struct RequestContext(Arc<ContextInternal>);

impl Deref for RequestContext {
    type Target = ContextInternal;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RequestContext {
    pub fn new(internal: ContextInternal) -> Self {
        Self(Arc::new(internal))
    }

    pub async fn try_session_context(&self) -> Result<Arc<Session>> {
        self.session_context
            .read()
            .await
            .as_ref()
            .map(Arc::clone)
            .ok_or_else(|| Error::unauthenticated("User is not authenticated"))
    }

    /// Generate OTP code, insert to cache and return the OTP code
    pub async fn pending_registration_phone_cache(
        &self,
        e164_phone_number_str: String,
    ) -> Result<OtpCode> {
        static OTP_CODE_LENGTH: u8 = 6;

        let code = self.random.gen_numeric_string(OTP_CODE_LENGTH).await?;
        self.phone_pending_registration_cache
            .insert(e164_phone_number_str, code.clone())
            .await;

        Ok(OtpCode {
            code,
            digits: OTP_CODE_LENGTH,
            ttl_seconds: OTP_CODE_TTL_SECONDS,
        })
    }

    /// Generate OTP code, insert to cache and return the OTP code
    pub async fn pending_registration_phone_validate(
        &self,
        e164_phone_number_str: &str,
        otp_code: &str,
    ) -> Result<()> {
        let code = self
            .phone_pending_registration_cache
            .get(e164_phone_number_str)
            .await;

        if Some(otp_code) == code.as_deref() {
            return Ok(());
        }

        Err(Error::invalid_argument_with(
            "OTP verification failed",
            Some(BadRequest {
                field_violations: vec![FieldViolation {
                    field: "OTP_VERIFICATION".into(),
                    description: "FAILED".into(),
                }],
            }),
        ))
    }
}

#[derive(Debug)]
pub struct OtpCode {
    pub code: String,
    pub digits: u8,
    pub ttl_seconds: u64,
}
