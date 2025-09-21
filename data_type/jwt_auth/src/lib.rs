use chrono::{Duration, NaiveDateTime, Utc};
use error::Result;
use jwt_signer::{JwtClaims, JwtSigner};
use serde::{Serialize, de::DeserializeOwned};
use std::sync::Arc;

pub struct JwtAuth {
    signer: Arc<JwtSigner>,
}

impl JwtAuth {
    pub fn new(secret_str: &str) -> Self {
        JwtAuth {
            signer: Arc::new(JwtSigner::new(secret_str)),
        }
    }

    pub fn sign_token<T: DeserializeOwned + Serialize>(
        &self,
        jti: u128,
        inner: T,
        ttl: Duration,
    ) -> Result<SignTokenPayload> {
        let now = Utc::now().naive_utc();
        let exp = now + ttl;
        let claims = JwtClaims::new_with_required_fields(jti, now, exp, inner);
        let token_str = self.signer.sign(&claims)?;
        Ok(SignTokenPayload {
            token_str,
            jti: claims.jti,
            iat: now,
            exp,
        })
    }

    /// Verify token without validating expiry.
    /// If you want to validate expiry at call site, use [`JwtClaims::validate_expiry`].
    pub fn verify_token<T: DeserializeOwned + Serialize>(
        &self,
        token_str: &str,
    ) -> Result<JwtClaims<T>> {
        self.signer
            .verify_token::<JwtClaims<T>>(token_str)
            .map(|data| data.claims)
    }
}

pub struct SignTokenPayload {
    pub token_str: String,
    pub jti: u128,
    pub iat: NaiveDateTime,
    pub exp: NaiveDateTime,
}
