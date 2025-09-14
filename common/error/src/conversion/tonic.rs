use crate::{
    Error, ErrorVariant,
    error_details::{DebugInfo, ErrorInfo, RetryInfo},
};
use prost::Message;
use tonic::{Code, Status};

// Map error for GRPC client
impl From<Status> for Error {
    fn from(status: Status) -> Self {
        let message = status.message();
        match status.code() {
            Code::Ok => Error::internal(format!("Found tonic OK status code {message}")),
            Code::Cancelled => Error::cancelled(message),
            Code::Unknown => Error::unknown(message),
            Code::InvalidArgument => Error::invalid_argument_with(message, decode(&status)),
            Code::DeadlineExceeded => Error::deadline_exceeded(message),
            Code::NotFound => Error::not_found_with(message, decode(&status)),
            Code::AlreadyExists => Error::already_exists_with(message, decode(&status)),
            Code::PermissionDenied => Error::permission_denied_with(message, decode(&status)),
            Code::ResourceExhausted => Error::resource_exhausted_with(message, decode(&status)),
            Code::FailedPrecondition => Error::failed_precondition_with(message, decode(&status)),
            Code::Aborted => match decode::<AbortedErrorDetail>(&status) {
                Some(AbortedErrorDetail {
                    error_info,
                    retry_info,
                }) => Error::aborted_with(message, error_info, retry_info),
                None => Error::aborted(message),
            },
            Code::OutOfRange => Error::out_of_range_with(message, decode(&status)),
            Code::Unimplemented => Error::unimplemented(message),
            Code::Internal => Error::internal_with(message, decode(&status)),
            Code::Unavailable => match decode::<UnavailableErrorDetail>(&status) {
                Some(UnavailableErrorDetail {
                    debug_info,
                    retry_info,
                }) => Error::unavailable_with(message, debug_info, retry_info),
                None => Error::aborted(message),
            },
            Code::DataLoss => Error::data_loss_with(message, decode(&status)),
            Code::Unauthenticated => Error::unauthenticated_with(message, decode(&status)),
        }
    }
}

// Error conversion for GRPC server implementation
impl From<Error> for Status {
    fn from(err: Error) -> Self {
        let message = &err.message;
        match *err.variant {
            ErrorVariant::Cancelled => Status::cancelled(message),
            ErrorVariant::Unknown(debug_info) => {
                Status::with_details(Code::Unknown, message, encode(debug_info).into())
            }
            ErrorVariant::InvalidArgument(bad_request) => {
                Status::with_details(Code::InvalidArgument, message, encode(bad_request).into())
            }
            ErrorVariant::DeadlineExceeded => Status::deadline_exceeded(message),
            ErrorVariant::NotFound(resource_info) => {
                Status::with_details(Code::NotFound, message, encode(resource_info).into())
            }
            ErrorVariant::AlreadyExists(resource_info) => {
                Status::with_details(Code::AlreadyExists, message, encode(resource_info).into())
            }
            ErrorVariant::PermissionDenied(error_info) => {
                Status::with_details(Code::PermissionDenied, message, encode(error_info).into())
            }
            ErrorVariant::ResourceExhausted(quota_failure) => Status::with_details(
                Code::ResourceExhausted,
                message,
                encode(quota_failure).into(),
            ),
            ErrorVariant::FailedPrecondition(precondition_failure) => Status::with_details(
                Code::FailedPrecondition,
                message,
                encode(precondition_failure).into(),
            ),
            ErrorVariant::Aborted((error_info, retry_info)) => Status::with_details(
                Code::Aborted,
                message,
                encode(Some(AbortedErrorDetail {
                    error_info,
                    retry_info,
                }))
                .into(),
            ),
            ErrorVariant::OutOfRange(bad_request) => {
                Status::with_details(Code::OutOfRange, message, encode(bad_request).into())
            }
            ErrorVariant::Unimplemented => Status::unimplemented(message),
            ErrorVariant::Internal(debug_info) => {
                Status::with_details(Code::Internal, message, encode(debug_info).into())
            }
            ErrorVariant::Unavailable((debug_info, retry_info)) => Status::with_details(
                Code::Unavailable,
                message,
                encode(Some(UnavailableErrorDetail {
                    debug_info,
                    retry_info,
                }))
                .into(),
            ),
            ErrorVariant::DataLoss(debug_info) => {
                Status::with_details(Code::DataLoss, message, encode(debug_info).into())
            }
            ErrorVariant::Unauthenticated(error_info) => {
                Status::with_details(Code::Unauthenticated, message, encode(error_info).into())
            }
        }
    }
}

#[derive(Message)]
struct AbortedErrorDetail {
    #[prost(message, optional, tag = "1")]
    error_info: Option<ErrorInfo>,
    #[prost(message, optional, tag = "2")]
    retry_info: Option<RetryInfo>,
}

#[derive(Message)]
struct UnavailableErrorDetail {
    #[prost(message, optional, tag = "1")]
    debug_info: Option<DebugInfo>,
    #[prost(message, optional, tag = "2")]
    retry_info: Option<RetryInfo>,
}

fn encode<T>(input: Option<T>) -> Vec<u8>
where
    T: Message + Default,
{
    input.as_ref().map(T::encode_to_vec).unwrap_or_default()
}

fn decode<T>(status: &Status) -> Option<T>
where
    T: Message + Default,
{
    T::decode(status.details()).ok()
}

impl From<tonic::transport::Error> for Error {
    fn from(error: tonic::transport::Error) -> Self {
        Error::internal(error.to_string())
    }
}

#[test]
fn convert_between_error_and_tonic_status() {
    use crate::error_details::{BadRequest, bad_request::FieldViolation};
    use prost_types::Duration;

    // Single error detail
    let original_error = Error::invalid_argument_with(
        "Foo error",
        Some(BadRequest {
            field_violations: vec![FieldViolation {
                field: String::from("foo"),
                description: String::from("BAR"),
            }],
        }),
    );
    let tonic_status = Status::from(original_error);
    let error = Error::from(tonic_status);
    assert_eq!(error.message.as_str(), "Foo error");
    let variant = *error.variant;
    let field_violation = match variant {
        ErrorVariant::InvalidArgument(bad_request) => {
            bad_request.unwrap().field_violations.pop().unwrap()
        }
        _ => panic!("Expect invalid argument"),
    };
    assert_eq!(field_violation.field.as_str(), "foo");
    assert_eq!(field_violation.description.as_str(), "BAR");

    // Tuple error detail
    let original_error = Error::aborted_with(
        "Foo error",
        Some(ErrorInfo {
            reason: "FOO".into(),
            domain: "BAR".into(),
            ..Default::default()
        }),
        Some(RetryInfo {
            retry_delay: Some(Duration {
                seconds: 10,
                nanos: 0,
            }),
        }),
    );
    let tonic_status = Status::from(original_error);
    let error = Error::from(tonic_status);
    let variant = *error.variant;
    match variant {
        ErrorVariant::Aborted((error_info, retry_info)) => {
            let error_info = error_info.unwrap();
            let retry_delay = retry_info.unwrap().retry_delay.unwrap().seconds;
            assert_eq!(error_info.reason.as_str(), "FOO");
            assert_eq!(error_info.domain.as_str(), "BAR");
            assert_eq!(retry_delay, 10);
        }
        _ => panic!("Expect invalid argument"),
    };
}
