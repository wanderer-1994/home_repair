use async_trait::async_trait;
use error::Result;
use phonenumber::PhoneNumber;

#[async_trait]
pub trait SmsSender: Send + Sync {
    async fn send(&self, input: SendSmsInput) -> Result<()>;
}

#[derive(Debug)]
pub struct SendSmsInput {
    pub to: PhoneNumber,
    pub message: MessageType,
}

#[derive(Debug)]
pub enum MessageType {
    OtpVerificationForRegistration(OtpVerificationForRegistration),
}

impl MessageType {
    pub fn to_message_str(self) -> String {
        match self {
            MessageType::OtpVerificationForRegistration(inner) => {
                format!("Mã xác thực của bạn là {}", inner.code)
            }
        }
    }
}

#[derive(Debug)]
pub struct OtpVerificationForRegistration {
    pub code: String,
}

#[cfg(feature = "test")]
mod test {
    use super::*;
    use error::Result;

    impl MessageType {
        pub fn try_otp_verification_for_registration(
            self,
        ) -> Result<OtpVerificationForRegistration> {
            match self {
                MessageType::OtpVerificationForRegistration(inner) => Ok(inner),
            }
        }
    }
}
