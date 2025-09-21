use actor_auth::{ActorType, Session};
use chrono::{Duration, Utc};
use db_utils::PgConnectionPool;
use error::Result;
use jwt_signer::{JwtClaims, JwtSigner};
use random_util::Random;
use std::sync::Arc;

use crate::{InitiateOrRenewSession, SessionAndCsrfToken};

const SESSION_TOKEN_TTL_DAYS: u8 = 2;

pub struct AccountServiceContext {
    pub(crate) db_connection_pool: PgConnectionPool,
    pub(crate) jwt_signer: Arc<JwtSigner>,
    pub(crate) random: Random,
}

impl AccountServiceContext {
    pub(crate) async fn new_session_token_claims(
        &self,
        actor_type: ActorType,
    ) -> JwtClaims<ActorType> {
        let jti = self.random.gen_u128().await;
        let iat = Utc::now().naive_utc();
        let exp = iat + Duration::days(SESSION_TOKEN_TTL_DAYS as i64);
        JwtClaims::new_with_required_fields(jti, iat, exp, actor_type)
    }

    /// Initate session with token string for setting user cookies
    pub(crate) async fn initiate_user_session(
        &self,
        actor_type: ActorType,
    ) -> Result<InitiateOrRenewSession> {
        let claims = self.new_session_token_claims(actor_type).await;
        let session_token = self.jwt_signer.sign(&claims)?;

        Ok(InitiateOrRenewSession {
            session_token: SessionAndCsrfToken {
                session_token,
                csrf_token: claims.jti.to_string(),
            },
            session: Session {
                iat: claims.iat,
                exp: claims.exp,
                actor_type,
            },
        })
    }
}

pub struct AccountService {
    pub(crate) context: AccountServiceContext,
}

impl AccountService {
    pub fn new(context: AccountServiceContext) -> Self {
        Self { context }
    }
}
