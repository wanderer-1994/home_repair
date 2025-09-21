use error::{Error, Result};
use paste::paste;

pub struct HexConverter;

macro_rules! to_hex {
    ($(($numeric:ty, $hex_with:literal)),+,) => {
        impl HexConverter {
            $(
                paste! {
                    /// Convert to lowercase hexadecimal string
                    pub fn [<$numeric _to_hex>](value: $numeric) -> String {
                        format!("{:0with$x}", value, with = $hex_with)
                    }
                }
            )+
        }
    };
}

macro_rules! from_hex {
    ($($usign:ty)+) => {
        impl HexConverter {
            $(
                paste! {
                    pub fn [<$usign _from_hex>](hex_str: &str) -> Result<$usign> {
                        let value = $usign::from_str_radix(hex_str, 16)
                            .map_err(|_| Error::invalid_argument("Malformed hex string"))?;
                        Ok(value)
                    }
                }
            )+
        }
    };
}

macro_rules! from_hex_with_intermediate {
    ($(($sign:ty, $usign:ty)),+,) => {
        impl HexConverter {
            $(
                paste! {
                    pub fn [<$sign _from_hex>](hex_str: &str) -> Result<$sign> {
                        let intermediate = $usign::from_str_radix(hex_str, 16)
                            .map_err(|_| Error::invalid_argument("Malformed hex string"))?;
                        Ok(intermediate as $sign)
                    }
                }
            )+
        }
    };
}

// Each hex digit has 4-bits length. `i8` has 8-bits length
// hence max hex length is 2. Same rule for other data types.
to_hex! {
    (i8, 2),
    (u8, 2),
    (i16, 4),
    (u16, 4),
    (i32, 8),
    (u32, 8),
    (i64, 16),
    (u64, 16),
    (i128, 32),
    (u128, 32),
}

from_hex! {u8 u16 u32 u64 u128}

from_hex_with_intermediate! {
    (i8, u8),
    (i16, u16),
    (i32, u32),
    (i64, u64),
    (i128, u128),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hex_char_length() {
        let hex_str = HexConverter::i8_to_hex(-3);
        assert_eq!(hex_str.len(), 2);
        assert_eq!(HexConverter::i8_from_hex(&hex_str).unwrap(), -3);

        let hex_str = HexConverter::u8_to_hex(3);
        assert_eq!(hex_str.len(), 2);
        assert_eq!(HexConverter::u8_from_hex(&hex_str).unwrap(), 3);

        let hex_str = HexConverter::i64_to_hex(-3);
        assert_eq!(hex_str.len(), 16);
        assert_eq!(HexConverter::i64_from_hex(&hex_str).unwrap(), -3);

        let hex_str = HexConverter::u64_to_hex(3);
        assert_eq!(hex_str.len(), 16);
        assert_eq!(HexConverter::u64_from_hex(&hex_str).unwrap(), 3);
    }
}
