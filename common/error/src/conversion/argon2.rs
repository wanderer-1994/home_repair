use crate::Error;
use argon2::{Error as Argon2Error, password_hash::Error as PasswordHashError};

impl From<Argon2Error> for Error {
    fn from(error: Argon2Error) -> Self {
        match error {
            Argon2Error::AdTooLong
            | Argon2Error::AlgorithmInvalid
            | Argon2Error::KeyIdTooLong
            | Argon2Error::MemoryTooLittle
            | Argon2Error::MemoryTooMuch
            | Argon2Error::OutputTooShort
            | Argon2Error::OutputTooLong
            | Argon2Error::SaltTooShort
            | Argon2Error::SaltTooLong
            | Argon2Error::SecretTooLong
            | Argon2Error::ThreadsTooFew
            | Argon2Error::ThreadsTooMany
            | Argon2Error::TimeTooSmall
            | Argon2Error::VersionInvalid => Error::internal(format!("Argon2Error: {error}")),
            Argon2Error::B64Encoding(error) => {
                Error::internal(format!("Argon2Error: B64Encoding - {error}"))
            }
            Argon2Error::PwdTooLong => Error::invalid_argument("Password too long"),
        }
    }
}

impl From<PasswordHashError> for Error {
    fn from(error: PasswordHashError) -> Self {
        match error {
            PasswordHashError::Algorithm
            | PasswordHashError::Crypto
            | PasswordHashError::OutputSize { .. }
            | PasswordHashError::ParamNameDuplicated
            | PasswordHashError::ParamNameInvalid
            | PasswordHashError::ParamValueInvalid(_)
            | PasswordHashError::ParamsMaxExceeded
            | PasswordHashError::SaltInvalid(_)
            | PasswordHashError::PhcStringField
            | PasswordHashError::PhcStringTrailingData
            | PasswordHashError::Version => Error::internal(format!("PasswordHashError: {error}")),
            PasswordHashError::B64Encoding(error) => {
                Error::internal(format!("PasswordHashError: B64Encoding - {error}"))
            }
            PasswordHashError::Password => {
                Error::invalid_argument("PasswordHashError: Invalid credential")
            }
            _ => Error::internal(format!("PasswordHashError: {error}")),
        }
    }
}
