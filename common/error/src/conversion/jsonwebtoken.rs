use crate::{Error, error_details::ErrorInfo};
use jsonwebtoken::errors::{Error as JwtError, ErrorKind};

impl From<JwtError> for Error {
    fn from(error: JwtError) -> Self {
        match error.kind() {
            ErrorKind::InvalidToken
            | ErrorKind::ExpiredSignature
            | ErrorKind::MissingRequiredClaim(_)
            | ErrorKind::InvalidIssuer
            | ErrorKind::InvalidAudience
            | ErrorKind::InvalidSubject
            | ErrorKind::ImmatureSignature
            | ErrorKind::InvalidAlgorithm
            | ErrorKind::MissingAlgorithm => Error::unauthenticated_with(
                format!("Invalid JWT {error:?}"),
                Some(ErrorInfo {
                    reason: "INVALID".into(),
                    domain: "JWT".into(),
                    metadata: [(String::from("message"), format!("{:?}", error.kind()))]
                        .into_iter()
                        .collect(),
                }),
            ),

            ErrorKind::InvalidRsaKey(_)
            | ErrorKind::InvalidKeyFormat
            | ErrorKind::InvalidEcdsaKey
            | ErrorKind::RsaFailedSigning
            | ErrorKind::InvalidSignature
            | ErrorKind::InvalidAlgorithmName
            | ErrorKind::Base64(_)
            | ErrorKind::Json(_)
            | ErrorKind::Utf8(_)
            | ErrorKind::Crypto(_) => {
                Error::internal(format!("JWT internal server error {error:?}"))
            }

            _ => Error::internal(format!("JWT internal server unknown error {error:?}")),
        }
    }
}
