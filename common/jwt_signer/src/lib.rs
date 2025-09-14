//! Implementing JWT signer and verifier

use chrono::{
    NaiveDateTime, Utc,
    naive::serde::{ts_seconds, ts_seconds_option},
};
use error::{Error, Result, error_details::ErrorInfo};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

/// JWT signer / verifier using HMAC SHA256 algorithm
pub struct JwtSigner {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    default_validation: Validation,
}

impl JwtSigner {
    /// Creates a [`JwtSigner`] instance with a secret string
    pub fn new(secret_str: &str) -> Self {
        let encoding_key = EncodingKey::from_secret(secret_str.as_bytes());
        let decoding_key = DecodingKey::from_secret(secret_str.as_bytes());
        let mut default_validation = Validation::default();
        // Set exp as required claim, but disable expiration validation, we will handle expiration
        // in our own logic
        default_validation.set_required_spec_claims(&["exp"]);
        default_validation.validate_exp = false;

        Self {
            encoding_key,
            decoding_key,
            default_validation,
        }
    }

    /// Sign a payload. Returns a JWT token string
    pub fn sign<T>(&self, input: &T) -> Result<String>
    where
        T: Serialize,
    {
        // TODO(huy): encrypt `input`
        Ok(encode(&Header::default(), input, &self.encoding_key)?)
    }

    /// Verify a JWT token string. Returns a [TokenData] if token string is valid
    pub fn verify_token<T>(&self, token_str: &str) -> Result<TokenData<T>>
    where
        T: DeserializeOwned,
    {
        Ok(decode::<T>(
            token_str,
            &self.decoding_key,
            &self.default_validation,
        )?)
    }
}

#[derive(Debug, Deserialize, Serialize)]
/// Standard JWT claim structure according to RFC7519.
/// See <https://datatracker.ietf.org/doc/html/rfc7519#section-4>
pub struct JwtClaims<T> {
    /// The party who issues the JWT. Should be `example.com`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iss: Option<String>,
    /// Claim identifies the principal that is the subject of the JWT.
    /// The subject value MUST either be scoped to be locally unique
    /// in the context of the issuer or be globally unique.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub: Option<String>,
    /// The party that will receive and validate this JWT. Should be `example.com`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<String>,
    /// UTC timestamp where this JWT lose effect
    #[serde(with = "ts_seconds")]
    pub exp: NaiveDateTime,
    /// UTC timestamp where this JWT start being effective
    #[serde(with = "ts_seconds_option")]
    pub nbf: Option<NaiveDateTime>,
    /// UTC timestamp when JWT is issued
    #[serde(with = "ts_seconds")]
    pub iat: NaiveDateTime,
    /// Unique ID among JWTs
    pub jti: u128,
    /// Custom content of the JWT
    pub inner: T,
}

impl<T> JwtClaims<T> {
    /// Create a claim with required fields.
    pub fn new_with_required_fields(
        jti: u128,
        iat: NaiveDateTime,
        exp: NaiveDateTime,
        inner: T,
    ) -> Self {
        Self {
            jti,
            exp,
            inner,
            iat,
            iss: None,
            sub: None,
            aud: None,
            nbf: None,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.exp < Utc::now().naive_utc()
    }

    pub fn validate_expiry(self) -> Result<Self> {
        if self.is_expired() {
            return Err(Error::permission_denied_with(
                "Token expired",
                Some(ErrorInfo {
                    reason: "EXPIRED".into(),
                    domain: "JWT".into(),
                    ..Default::default()
                }),
            ));
        }

        Ok(self)
    }
}

#[cfg(test)]
mod test {
    use chrono::{Duration, Utc};

    use super::*;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct TestClaims {
        dummy: bool,
    }

    #[test]
    fn test_verify_token_ignore_expiration() {
        let now = Utc::now().naive_utc();

        let jti = 123;
        let claims = JwtClaims::new_with_required_fields(
            jti,
            now - Duration::minutes(15),
            now - Duration::seconds(1),
            TestClaims { dummy: true },
        );

        let jwt_signer = JwtSigner::new("secret");
        let token_str = jwt_signer.sign(&claims).unwrap();

        let token = jwt_signer
            .verify_token::<JwtClaims<TestClaims>>(&token_str)
            .unwrap();

        assert_eq!(token.claims.inner.dummy, true);
        assert!(token.claims.exp < now);
    }
}
