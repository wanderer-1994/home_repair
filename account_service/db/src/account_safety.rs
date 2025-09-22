//! Utility functions facilitate account safety check

use std::str::FromStr;

use error::{
    Error, Result,
    error_details::{
        BadRequest, PreconditionFailure, bad_request::FieldViolation,
        precondition_failure::Violation,
    },
};
use phonenumber::PhoneNumber;

pub fn check_password_safety(password: &str) -> Result<()> {
    static MIN_PASSWORD_LENGTH: usize = 8;

    if password.len() < MIN_PASSWORD_LENGTH {
        return Err(Error::failed_precondition_with(
            "Password too short",
            Some(PreconditionFailure {
                violations: vec![Violation {
                    r#type: String::from("TOO_SHORT"),
                    subject: String::from("password"),
                    description: format!(
                        "Password should be at-least {MIN_PASSWORD_LENGTH} characters long"
                    ),
                }],
            }),
        ));
    }

    Ok(())
}

/// Normalize phone number to E.164 format for consistent storage and lookups.
pub fn phone_number_to_e164_format(phone_number: &PhoneNumber) -> String {
    phone_number
        .format()
        .mode(phonenumber::Mode::E164)
        .to_string()
}

pub fn phone_number_from_str(phone_str: &str) -> Result<PhoneNumber> {
    PhoneNumber::from_str(phone_str).map_err(|e| {
        Error::invalid_argument_with(
            format!("Invalid phone number {phone_str} - {e:?}"),
            Some(BadRequest {
                field_violations: vec![FieldViolation {
                    field: "phone_number".into(),
                    description: "INVALID".into(),
                }],
            }),
        )
    })
}

/// Check if string is trimmed and not empty
pub fn require_trimmed_and_not_empty_str(
    input: &str,
    error_field_name: impl Into<String>,
) -> Result<()> {
    if input.is_empty() {
        return Err(Error::invalid_argument_with(
            "Require input not empty",
            Some(BadRequest {
                field_violations: vec![FieldViolation {
                    field: error_field_name.into(),
                    description: "EMPTY".into(),
                }],
            }),
        ));
    }

    let trimmed = input.trim();
    if input != trimmed {
        return Err(Error::invalid_argument_with(
            "Require input to be trimmed and not empty",
            Some(BadRequest {
                field_violations: vec![FieldViolation {
                    field: error_field_name.into(),
                    description: "NOT_TRIMMED".into(),
                }],
            }),
        ));
    }
    Ok(())
}
