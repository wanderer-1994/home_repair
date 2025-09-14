use crate::{
    Error,
    error_details::{DebugInfo, ErrorInfo, PreconditionFailure, RetryInfo, precondition_failure},
};
use diesel::result::{DatabaseErrorInformation, DatabaseErrorKind, Error as DieselError};
use diesel_async::pooled_connection::deadpool::PoolError;
use prost_types::Duration;

impl From<PoolError> for Error {
    fn from(error: PoolError) -> Self {
        Error::internal(format!("DB connection pool error: {error:?}"))
    }
}

impl From<DieselError> for Error {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::InvalidCString(err) => {
                Error::internal(format!("Diesel-InvalidCString {err:?}"))
            }
            DieselError::NotFound => Error::not_found(String::from("Data not found in DB")),
            DieselError::QueryBuilderError(err) => {
                Error::internal(format!("Diesel-QueryBuilderError {err:?}"))
            }
            DieselError::DeserializationError(err) => {
                Error::internal(format!("Diesel-DeserializationError {err:?}"))
            }
            DieselError::SerializationError(err) => {
                Error::internal(format!("Diesel-SerializationError {err:?}"))
            }
            DieselError::RollbackErrorOnCommit {
                rollback_error,
                commit_error,
            } => Error::internal(format!(
                "Diesel-SerializationError {rollback_error:?} {commit_error:?}"
            )),
            DieselError::RollbackTransaction => Error::aborted_with(
                "Rolling back transaction",
                Some(ErrorInfo {
                    reason: String::from("ROLLBACK_REQUESTED"),
                    ..Default::default()
                }),
                None,
            ),
            DieselError::AlreadyInTransaction => Error::internal("Diesel-AlreadyInTransaction"),
            DieselError::NotInTransaction => Error::internal("Diesel-NotInTransaction"),
            DieselError::BrokenTransactionManager => {
                Error::internal("Diesel-BrokenTransactionManager")
            }
            DieselError::DatabaseError(kind, info) => map_db_error_kind(kind, info.as_ref()),
            _ => Error::unknown(format!("Database Error: {error:?}")),
        }
    }
}

fn map_db_error_kind(kind: DatabaseErrorKind, info: &dyn DatabaseErrorInformation) -> Error {
    match kind {
        DatabaseErrorKind::UniqueViolation => {
            let violation = precondition_failure::Violation {
                r#type: String::from("UNIQUE_KEY"),
                subject: associated_subject(info).unwrap_or_default(),
                description: info_description(info),
            };

            let payload = PreconditionFailure {
                violations: vec![violation],
            };
            Error::failed_precondition_with("DB uniqueness violation", Some(payload))
        }
        DatabaseErrorKind::ForeignKeyViolation => {
            let violation = precondition_failure::Violation {
                r#type: String::from("FOREIGN_KEY"),
                subject: associated_subject(info).unwrap_or_default(),
                description: info_description(info),
            };

            let payload = PreconditionFailure {
                violations: vec![violation],
            };
            Error::failed_precondition_with("DB foreign key violationation", Some(payload))
        }
        DatabaseErrorKind::UnableToSendCommand => Error::internal(format!(
            "Diesel-UnableToSendCommand {:?}",
            info_description(info)
        )),
        DatabaseErrorKind::SerializationFailure => {
            let retry_info = RetryInfo {
                retry_delay: Some(Duration {
                    seconds: 0,
                    nanos: 100_000_000,
                }),
            };
            Error::aborted_with(
                format!(
                    "Serializable transaction failed: {}",
                    info_description(info)
                ),
                None,
                Some(retry_info),
            )
        }
        DatabaseErrorKind::ReadOnlyTransaction => Error::internal(format!(
            "Diesel-ReadOnlyTransaction {:?}",
            info_description(info)
        )),
        DatabaseErrorKind::NotNullViolation => Error::internal(format!(
            "Diesel-NotNullViolation {:?}",
            info_description(info)
        )),
        DatabaseErrorKind::CheckViolation => Error::internal_with(
            format!("Diesel-CheckViolation {:?}", info_description(info)),
            Some(DebugInfo {
                stack_entries: Vec::default(),
                detail: info.constraint_name().map(String::from).unwrap_or_default(),
            }),
        ),
        DatabaseErrorKind::ClosedConnection => Error::internal(format!(
            "Diesel-ClosedConnection {:?}",
            info_description(info)
        )),
        _ => Error::unknown(format!(
            "Database Error {kind:?}: {}",
            info_description(info)
        )),
    }
}

/// Returns constraint / column / table associated with database error.
fn associated_subject(info: &dyn DatabaseErrorInformation) -> Option<String> {
    info.constraint_name()
        .or_else(|| info.column_name())
        .or_else(|| info.table_name())
        .map(String::from)
}

fn info_description(info: &dyn DatabaseErrorInformation) -> String {
    format!(
        "{}\nDetails: {}\nHint: {}\nTable: {}\nColumn: {}\nConstraint: {}",
        info.message(),
        info.details().unwrap_or_default(),
        info.hint().unwrap_or_default(),
        info.table_name().unwrap_or_default(),
        info.column_name().unwrap_or_default(),
        info.constraint_name().unwrap_or_default(),
    )
}
