#![allow(dead_code)]
use error::ErrorVariant;
use graphql_client::Response;
use serde_json::Value;
use std::collections::HashMap;

/// Assert a [graphql_client::Response] having an error of expected type
pub fn assert_error_response<T>(
    response: Response<T>,
    variant: ErrorVariant,
    error_details: Option<Value>,
) {
    let extension = extract_error_extensions(response);
    assert_eq!(
        extension.get("code"),
        Some(&serde_json::json!(variant.client_code()))
    );

    let err_details = extension.get("errorDetails");

    assert_eq!(err_details, error_details.as_ref());
}

pub fn assert_aborted_error_with_retry_info_response<T>(
    response: Response<T>,
    error_info: Option<Value>,
    has_retry_delay_time_elapse: bool,
) {
    let extension = extract_error_extensions(response);
    assert_eq!(
        extension.get("code"),
        Some(&serde_json::json!(
            ErrorVariant::Aborted((None, None)).client_code()
        ))
    );

    let err_details = extension.get("errorDetails").unwrap();
    let err_info = err_details.get("errorInfo");
    let has_retry_delay_time_elapse_actual = err_details
        .get("retryInfo")
        .and_then(|i| i.get("retryDelay"))
        .and_then(|d| d.get("seconds"))
        .is_some();

    assert_eq!(err_info, error_info.as_ref());
    assert_eq!(
        has_retry_delay_time_elapse,
        has_retry_delay_time_elapse_actual
    );
}

/// Extract error extensisons from a [graphql_client::Response]
pub fn extract_error_extensions<T>(response: Response<T>) -> HashMap<String, Value> {
    response
        .errors
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
        .extensions
        .unwrap()
}

pub fn extract_error_details<T>(response: Response<T>) -> ErrorDetails {
    let error = response.errors.unwrap().into_iter().next().unwrap();
    ErrorDetails {
        message: error.message,
        extensions: error.extensions.unwrap(),
    }
}

pub struct ErrorDetails {
    message: String,
    extensions: HashMap<String, Value>,
}

impl ErrorDetails {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.extensions.get(key)
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    pub fn get_code(&self) -> &str {
        self.get("code").unwrap().as_str().unwrap()
    }
}
