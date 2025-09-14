//! Convert [std::io::Error] to yearnings error space.
//! Initially introduced to make ease of error handling for
//! `dataloader` crate which uses [std::io::Error] as error space.

use crate::Error;
use std::io::{Error as IoError, ErrorKind};

impl From<IoError> for Error {
    fn from(error: IoError) -> Self {
        match error.kind() {
            ErrorKind::NotFound => Error::not_found(format!("Resource not found {}", error)),
            ErrorKind::PermissionDenied => {
                Error::permission_denied(format!("Permission denied {}", error))
            }
            ErrorKind::AlreadyExists => {
                Error::already_exists(format!("Resource already exists {}", error))
            }
            ErrorKind::InvalidInput | ErrorKind::InvalidData => {
                Error::invalid_argument(format!("Invalid input {}", error))
            }
            ErrorKind::ConnectionRefused
            | ErrorKind::ConnectionReset
            | ErrorKind::ConnectionAborted
            | ErrorKind::NotConnected
            | ErrorKind::AddrInUse
            | ErrorKind::AddrNotAvailable
            | ErrorKind::BrokenPipe
            | ErrorKind::WouldBlock
            | ErrorKind::TimedOut
            | ErrorKind::WriteZero
            | ErrorKind::Interrupted
            | ErrorKind::Unsupported
            | ErrorKind::UnexpectedEof
            | ErrorKind::OutOfMemory
            | ErrorKind::Other => Error::internal(format!("Internal io error {}", error)),
            _ => Error::internal(format!("Internal unknown error {}", error)),
        }
    }
}
