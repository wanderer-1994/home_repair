use super::AccountService;
use account_service_db as db;
use actor_auth::{ActorAuth, ActorType, CustomerActor, HandymanActor, Session};
use db_utils::{with_mutable_db, with_readonly_db};
use entity_type::{AccountType, CustomerId, HandymanId};
use error::{Error, Result};
use hex_converter::HexConverter;
use jwt_signer::JwtClaims;
use scoped_futures::ScopedFutureExt;

impl AccountService {
    #[tracing::instrument(skip_all, fields(phone_number=request.e164_phone_number_str))]
    pub async fn account_exists(
        &self,
        request: AccountExistsRequest,
    ) -> Result<AccountExistsResponse> {
        let AccountExistsRequest {
            e164_phone_number_str,
            account_type,
        } = request;
        let e164_phone_number_str = typesafe::normalize_phone_number_str(&e164_phone_number_str)?;
        let exists = with_mutable_db(&self.context.db_connection_pool, |conn| {
            async {
                match account_type {
                    AccountType::Customer => {
                        db::CustomerAccount::phone_exist(&e164_phone_number_str, conn).await
                    }
                    AccountType::Handyman => {
                        db::CustomerAccount::phone_exist(&e164_phone_number_str, conn).await
                    }
                }
            }
            .scope_boxed()
        })
        .await?;

        Ok(AccountExistsResponse { exists })
    }

    #[tracing::instrument(skip_all, fields(phone_number=request.e164_phone_number_str))]
    pub async fn customer_register(
        &self,
        request: CustomerRegisterRequest,
    ) -> Result<CustomerRegisterResponse> {
        let CustomerRegisterRequest {
            e164_phone_number_str,
            password,
        } = request;

        let phone_number = typesafe::phone_number_from_str(&e164_phone_number_str)?;
        let account = with_mutable_db(&self.context.db_connection_pool, |conn| {
            db::CustomerAccount::create(
                &ActorAuth::God,
                db::NewCustomerAccount {
                    phone_number: &phone_number,
                    password: &password,
                },
                conn,
            )
            .scope_boxed()
        })
        .await?;
        let initiate_session = self
            .context
            .initiate_user_session(ActorType::Customer(CustomerActor {
                customer_id: account.id,
            }))
            .await?;

        Ok(CustomerRegisterResponse { initiate_session })
    }

    #[tracing::instrument(skip_all, fields(phone_number=request.e164_phone_number_str))]
    pub async fn handyman_register(
        &self,
        request: HandymanRegisterRequest,
    ) -> Result<HandymanRegisterResponse> {
        let HandymanRegisterRequest {
            e164_phone_number_str,
            password,
        } = request;

        let phone_number = typesafe::phone_number_from_str(&e164_phone_number_str)?;
        let account = with_mutable_db(&self.context.db_connection_pool, |conn| {
            db::HandymanAccount::create(
                &ActorAuth::God,
                db::NewHandymanAccount {
                    phone_number: &phone_number,
                    password: &password,
                },
                conn,
            )
            .scope_boxed()
        })
        .await?;
        let initiate_session = self
            .context
            .initiate_user_session(ActorType::Handyman(HandymanActor {
                handyman_id: account.id,
            }))
            .await?;

        Ok(HandymanRegisterResponse { initiate_session })
    }

    #[tracing::instrument(skip(self))]
    pub async fn customer_create_profile(
        &self,
        request: CustomerCreateProfileRequest,
    ) -> Result<CustomerCreateProfileResponse> {
        let CustomerCreateProfileRequest {
            actor_auth,
            customer_id,
            nick_name,
        } = request;

        let profile = with_mutable_db(&self.context.db_connection_pool, |conn| {
            db::CustomerProfile::create(
                &actor_auth,
                customer_id,
                db::NewCustomerProfile {
                    nick_name: &nick_name,
                },
                conn,
            )
            .scope_boxed()
        })
        .await?;

        Ok(CustomerCreateProfileResponse { profile })
    }

    #[tracing::instrument(skip(self))]
    pub async fn handyman_create_profile(
        &self,
        request: HandymanCreateProfileRequest,
    ) -> Result<HandymanCreateProfileResponse> {
        let HandymanCreateProfileRequest {
            actor_auth,
            handyman_id,
            first_name,
            last_name,
        } = request;

        let profile = with_mutable_db(&self.context.db_connection_pool, |conn| {
            db::HandymanProfile::create(
                &actor_auth,
                handyman_id,
                db::NewHandymanProfile {
                    first_name: &first_name,
                    last_name: &last_name,
                },
                conn,
            )
            .scope_boxed()
        })
        .await?;

        Ok(HandymanCreateProfileResponse { profile })
    }

    #[tracing::instrument(skip_all, fields(phone_number=request.e164_phone_number_str))]
    pub async fn customer_sign_in_with_password(
        &self,
        request: CustomerSignInWithPasswordRequest,
    ) -> Result<CustomerSignInWithPasswordResponse> {
        let CustomerSignInWithPasswordRequest {
            e164_phone_number_str,
            password,
        } = request;

        let account = with_readonly_db(&self.context.db_connection_pool, |conn| {
            db::CustomerAccount::find_by_phone_number(&e164_phone_number_str, conn).scope_boxed()
        })
        .await?;

        let Some(account) = account.and_then(|a| a.verify_password(&password).ok()) else {
            return Err(Error::unauthenticated("Creadentials not found"));
        };

        let initiate_session = self
            .context
            .initiate_user_session(ActorType::Customer(CustomerActor {
                customer_id: account.id,
            }))
            .await?;

        Ok(CustomerSignInWithPasswordResponse { initiate_session })
    }

    #[tracing::instrument(skip_all, fields(phone_number=request.e164_phone_number_str))]
    pub async fn handyman_sign_in_with_password(
        &self,
        request: HandymanSigninWithPasswordRequest,
    ) -> Result<HandymanSigninWithPasswordResponse> {
        let HandymanSigninWithPasswordRequest {
            e164_phone_number_str,
            password,
        } = request;

        let account = with_readonly_db(&self.context.db_connection_pool, |conn| {
            db::HandymanAccount::find_by_phone_number(&e164_phone_number_str, conn).scope_boxed()
        })
        .await?;

        let Some(account) = account.and_then(|a| a.verify_password(&password).ok()) else {
            return Err(Error::unauthenticated("Creadentials not found"));
        };

        let initiate_session = self
            .context
            .initiate_user_session(ActorType::Handyman(HandymanActor {
                handyman_id: account.id,
            }))
            .await?;

        Ok(HandymanSigninWithPasswordResponse { initiate_session })
    }

    #[tracing::instrument(skip(self))]
    pub async fn validate_session_token(
        &self,
        request: ValidateSessionTokenRequest,
    ) -> Result<ValidateSessionTokenResponse> {
        let ValidateSessionTokenRequest { strategy } = request;

        let claims = self
            .context
            .jwt_signer
            .verify_token::<JwtClaims<ActorType>>(strategy.session_token())?
            .claims;

        if let Some(csrf) = strategy.csrf_token() {
            verify_csrf_and_access_token_pairing(&claims, csrf)?;
        }

        Ok(ValidateSessionTokenResponse {
            session: Some(Session {
                iat: claims.iat,
                exp: claims.exp,
                actor_type: claims.inner,
            }),
        })
    }
}

#[derive(Debug)]
pub struct AccountExistsRequest {
    pub e164_phone_number_str: String,
    pub account_type: AccountType,
}

#[derive(Debug)]
pub struct AccountExistsResponse {
    pub exists: bool,
}

#[derive(Debug)]
pub struct CustomerRegisterRequest {
    pub e164_phone_number_str: String,
    pub password: String,
}

#[derive(Debug)]
pub struct CustomerRegisterResponse {
    pub initiate_session: InitiateOrRenewSession,
}

#[derive(Debug)]
pub struct HandymanRegisterRequest {
    pub e164_phone_number_str: String,
    pub password: String,
}

#[derive(Debug)]
pub struct HandymanRegisterResponse {
    pub initiate_session: InitiateOrRenewSession,
}

#[derive(Debug)]
pub struct CustomerCreateProfileRequest {
    pub actor_auth: ActorAuth,
    pub customer_id: CustomerId,
    pub nick_name: String,
}

#[derive(Debug)]
pub struct CustomerCreateProfileResponse {
    pub profile: db::CustomerProfile,
}

#[derive(Debug)]
pub struct HandymanCreateProfileRequest {
    pub actor_auth: ActorAuth,
    pub handyman_id: HandymanId,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug)]
pub struct HandymanCreateProfileResponse {
    pub profile: db::HandymanProfile,
}

#[derive(Debug)]
pub struct CustomerSignInWithPasswordRequest {
    pub e164_phone_number_str: String,
    pub password: String,
}

#[derive(Debug)]
pub struct CustomerSignInWithPasswordResponse {
    pub initiate_session: InitiateOrRenewSession,
}

#[derive(Debug)]
pub struct HandymanSigninWithPasswordRequest {
    pub e164_phone_number_str: String,
    pub password: String,
}

#[derive(Debug)]
pub struct HandymanSigninWithPasswordResponse {
    pub initiate_session: InitiateOrRenewSession,
}

#[derive(Debug)]
pub struct ValidateSessionTokenRequest {
    pub strategy: ValidateSessionTokenStrategy,
}

#[derive(Debug)]
pub enum ValidateSessionTokenStrategy {
    /// Requires csrf token for xhr requests
    CsrfProtectionRequired(SessionAndCsrfToken),
    /// Not requiring csrf token for static asset requests
    CsrfProtectionNotRequired(SessionToken),
}

impl ValidateSessionTokenStrategy {
    fn session_token(&self) -> &str {
        match self {
            ValidateSessionTokenStrategy::CsrfProtectionRequired(t) => &t.session_token,
            ValidateSessionTokenStrategy::CsrfProtectionNotRequired(t) => &t.session_token,
        }
    }

    fn csrf_token(&self) -> Option<&str> {
        match self {
            ValidateSessionTokenStrategy::CsrfProtectionRequired(t) => Some(&t.csrf_token),
            ValidateSessionTokenStrategy::CsrfProtectionNotRequired(_) => None,
        }
    }
}

#[derive(Debug)]
pub struct SessionAndCsrfToken {
    pub session_token: String,
    pub csrf_token: String,
}

#[derive(Debug)]
pub struct SessionToken {
    pub session_token: String,
}

#[derive(Debug)]
/// Struct being returned whenever new session is initiated
pub struct InitiateOrRenewSession {
    pub session_token: SessionAndCsrfToken,
    pub session: Session,
}

#[derive(Debug)]
pub struct ValidateSessionTokenResponse {
    pub session: Option<Session>,
}

/// Validate if csrf & access token is a valid pair
fn verify_csrf_and_access_token_pairing<T>(claims: &JwtClaims<T>, csrf: &str) -> Result<()> {
    let jti = HexConverter::u128_from_hex(csrf)?;
    if claims.jti != jti {
        return Err(Error::unauthenticated("Invalid access token"));
    }
    Ok(())
}
