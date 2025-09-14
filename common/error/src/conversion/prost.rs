use crate::Error;
use prost::UnknownEnumValue;

impl From<UnknownEnumValue> for Error {
    fn from(value: UnknownEnumValue) -> Self {
        Error::internal(format!("Failed to convert prost i32 to proto enum {value}"))
    }
}
