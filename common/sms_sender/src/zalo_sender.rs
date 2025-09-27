use crate::{SendSmsInput, SmsSender};
use async_trait::async_trait;
use error::Result;

pub struct ZaloSmsSender {}

#[async_trait]
impl SmsSender for ZaloSmsSender {
    async fn send(&self, _input: SendSmsInput) -> Result<()> {
        todo!()
    }
}
