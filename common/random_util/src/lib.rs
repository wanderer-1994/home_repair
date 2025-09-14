use error::{Error, Result};
use rand::{Rng, distr::Alphanumeric, rngs::ThreadRng};

pub struct Random;

impl Random {
    pub fn gen_u128() -> u128 {
        let mut rng = ThreadRng::default();
        let mut id = 0u128.to_le_bytes();
        rng.fill(&mut id);
        u128::from_le_bytes(id)
    }

    pub fn gen_numeric_string(digits: u8) -> Result<String> {
        if digits == 0 {
            return Err(Error::internal("Digits must be a positive integer"));
        }
        let mut rng = ThreadRng::default();
        let mut code = String::with_capacity(digits as usize);
        for _ in 0..digits {
            code.push(rng.random_range(b'0'..=b'9') as char);
        }

        Ok(code)
    }

    /// Generate a set of recovery codes in alphanumeric uppercase string format
    ///
    /// # Arguments
    ///
    /// * `num_codes` - how many codes to generate
    /// * `code_length` - how many chars a code has.
    pub async fn gen_recovery_codes(num_codes: u8, code_length: u8) -> Vec<String> {
        let mut rng = ThreadRng::default();
        let mut codes = Vec::with_capacity(num_codes as usize);

        for _ in 0..num_codes {
            let code = (0..code_length)
                .map(|_| char::from(rng.sample(Alphanumeric)))
                .collect::<String>()
                .to_uppercase();

            codes.push(code);
        }

        codes
    }
}
