#[macro_export]
/// Convert `Option` arguments to non-optional.
/// If any argument is `None`, returns invalid_argument error.
macro_rules! assert_argument_is_some {
    ($($arg:ident),+) => {
        $(
            let $arg = $arg
                .ok_or_else(|| error::Error::invalid_argument_with(
                    "Missing required fields",
                    Some(error::error_details::BadRequest {
                        field_violations: vec![error::error_details::bad_request::FieldViolation {
                            field: String::from(std::stringify!($arg)),
                            description: String::from("missing"),
                        }]
                    })
                ))?;
        )+
    };
}

#[macro_export]
/// Convert `Option` arguments to non-optional.
/// If any argument is `None`, returns internal error.
macro_rules! assert_internal_variable_is_some {
    ($($arg:ident),+) => {
        $(
            let $arg = $arg
                .ok_or_else(|| error::Error::internal(format!("Missing required fields {}", std::stringify!($arg))))?;
        )+
    };
}
