use crate::{
    Error, ErrorVariant,
    error_details::{
        BadRequest, ErrorInfo, PreconditionFailure, QuotaFailure, ResourceInfo, RetryInfo,
        bad_request::FieldViolation, precondition_failure, quota_failure,
    },
};
use async_graphql::{Error as GraphqlError, ErrorExtensionValues, Name, Value};
use indexmap::IndexMap;

const ERROR_DETAILS_FIELD_NAME: &str = "errorDetails";

impl From<Error> for GraphqlError {
    fn from(err: Error) -> Self {
        err.trace();
        GraphqlError {
            message: String::from(""),
            source: None,
            extensions: Some(ErrorExtensionValues::from(err)),
        }
    }
}

// TODO(huy): generate JSON examples for FE references
impl From<Error> for ErrorExtensionValues {
    fn from(err: Error) -> Self {
        err.trace();
        let mut extensions = ErrorExtensionValues::default();
        extensions.set("code", Value::String(err.client_code().to_string()));
        extensions.set("message", Value::String(err.message));

        match *err.variant {
            ErrorVariant::Cancelled
            | ErrorVariant::Unknown(_)
            | ErrorVariant::DeadlineExceeded
            | ErrorVariant::Unimplemented
            | ErrorVariant::Internal(_)
            | ErrorVariant::DataLoss(_) => {}

            ErrorVariant::InvalidArgument(bad_request) => {
                add_object_field(&mut extensions, ERROR_DETAILS_FIELD_NAME, bad_request)
            }
            ErrorVariant::NotFound(resource_info) => {
                add_object_field(&mut extensions, ERROR_DETAILS_FIELD_NAME, resource_info)
            }
            ErrorVariant::AlreadyExists(resource_info) => {
                add_object_field(&mut extensions, ERROR_DETAILS_FIELD_NAME, resource_info)
            }
            ErrorVariant::PermissionDenied(error_info) => {
                add_object_field(&mut extensions, ERROR_DETAILS_FIELD_NAME, error_info)
            }
            ErrorVariant::ResourceExhausted(quota_failure) => {
                add_object_field(&mut extensions, ERROR_DETAILS_FIELD_NAME, quota_failure)
            }
            ErrorVariant::FailedPrecondition(precondition_failure) => add_object_field(
                &mut extensions,
                ERROR_DETAILS_FIELD_NAME,
                precondition_failure,
            ),
            ErrorVariant::Aborted((error_info, retry_info)) => {
                let mut out = IndexMap::with_capacity(2);
                out.insert(
                    Name::new("errorInfo"),
                    error_info.map(Value::from).unwrap_or(Value::Null),
                );
                out.insert(
                    Name::new("retryInfo"),
                    retry_info.map(Value::from).unwrap_or(Value::Null),
                );
                add_object_field(
                    &mut extensions,
                    ERROR_DETAILS_FIELD_NAME,
                    Some(Value::Object(out)),
                );
            }
            ErrorVariant::OutOfRange(bad_request) => {
                add_object_field(&mut extensions, ERROR_DETAILS_FIELD_NAME, bad_request)
            }
            ErrorVariant::Unavailable((_, retry_info)) => {
                add_object_field(&mut extensions, ERROR_DETAILS_FIELD_NAME, retry_info)
            }
            ErrorVariant::Unauthenticated(error_info) => {
                add_object_field(&mut extensions, ERROR_DETAILS_FIELD_NAME, error_info)
            }
        };

        extensions
    }
}

/// Add a field with a value to an object.
/// `None` value equivalent to `null` in JSON.
fn add_object_field<T: Into<Value>>(
    extensions: &mut ErrorExtensionValues,
    field_name: &str,
    value: Option<T>,
) {
    extensions.set(field_name, value.map(Into::into).unwrap_or(Value::Null));
}

impl From<BadRequest> for Value {
    fn from(value: BadRequest) -> Self {
        let mut out = IndexMap::with_capacity(1);
        let field_violations = value
            .field_violations
            .into_iter()
            .map(Value::from)
            .collect::<Vec<_>>();
        out.insert(Name::new("fieldViolations"), Value::List(field_violations));
        Value::Object(out)
    }
}

impl From<FieldViolation> for Value {
    fn from(value: FieldViolation) -> Self {
        let mut out = IndexMap::with_capacity(2);
        out.insert(Name::new("field"), Value::String(value.field));
        out.insert(Name::new("description"), Value::String(value.description));
        Value::Object(out)
    }
}

impl From<ResourceInfo> for Value {
    fn from(value: ResourceInfo) -> Self {
        let mut out = IndexMap::with_capacity(4);
        out.insert(
            Name::new("resourceType"),
            Value::String(value.resource_type),
        );
        out.insert(
            Name::new("resourceName"),
            Value::String(value.resource_name),
        );
        out.insert(Name::new("owner"), Value::String(value.owner));
        out.insert(Name::new("description"), Value::String(value.description));
        Value::Object(out)
    }
}

impl From<ErrorInfo> for Value {
    fn from(value: ErrorInfo) -> Self {
        let mut out = IndexMap::with_capacity(3);
        out.insert(Name::new("reason"), Value::String(value.reason));
        out.insert(Name::new("domain"), Value::String(value.domain));
        let mut metadata = IndexMap::with_capacity(value.metadata.len());
        for (key, value) in value.metadata.into_iter() {
            metadata.insert(Name::new(key), Value::String(value));
        }
        out.insert(Name::new("metadata"), Value::Object(metadata));
        Value::Object(out)
    }
}

impl From<quota_failure::Violation> for Value {
    fn from(value: quota_failure::Violation) -> Self {
        let mut out = IndexMap::with_capacity(2);
        out.insert(Name::new("subject"), Value::String(value.subject));
        out.insert(Name::new("description"), Value::String(value.description));
        Value::Object(out)
    }
}

impl From<QuotaFailure> for Value {
    fn from(value: QuotaFailure) -> Self {
        let mut out = IndexMap::with_capacity(1);
        out.insert(
            Name::new("violations"),
            Value::List(value.violations.into_iter().map(Value::from).collect()),
        );
        Value::Object(out)
    }
}

impl From<precondition_failure::Violation> for Value {
    fn from(value: precondition_failure::Violation) -> Self {
        let mut out = IndexMap::with_capacity(3);
        out.insert(Name::new("type"), Value::String(value.r#type));
        out.insert(Name::new("subject"), Value::String(value.subject));
        out.insert(Name::new("description"), Value::String(value.description));
        Value::Object(out)
    }
}

impl From<PreconditionFailure> for Value {
    fn from(value: PreconditionFailure) -> Self {
        let mut out = IndexMap::with_capacity(1);
        out.insert(
            Name::new("violations"),
            Value::List(value.violations.into_iter().map(Value::from).collect()),
        );
        Value::Object(out)
    }
}

impl From<RetryInfo> for Value {
    fn from(value: RetryInfo) -> Self {
        let mut out = IndexMap::with_capacity(1);
        let retry_delay = if let Some(delay) = value.retry_delay {
            let mut retry_delay = IndexMap::with_capacity(2);
            retry_delay.insert(
                Name::new("seconds"),
                Value::Number(delay.seconds.min(i32::MAX as i64).into()),
            );
            retry_delay.insert(Name::new("nanos"), Value::Number(delay.nanos.into()));
            Value::Object(retry_delay)
        } else {
            Value::Null
        };

        out.insert(Name::new("retryDelay"), retry_delay);
        Value::Object(out)
    }
}

impl From<GraphqlError> for Error {
    fn from(err: GraphqlError) -> Self {
        Error::internal(format!("Internal GraphQL error {:?}", err))
    }
}
