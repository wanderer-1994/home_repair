//! Log SMS to terminal. Used for local development

use crate::{SendSmsInput, SmsSender};
use async_trait::async_trait;
use error::Result;

pub struct TerminalSmsSender;

#[async_trait]
impl SmsSender for TerminalSmsSender {
    async fn send(&self, input: SendSmsInput) -> Result<()> {
        // Log message to console for local development debugging purpose
        tracing::info!(
            ?input,
            "\nTerminalSmsSender message emit\n---------------------------------------------\n"
        );
        Ok(())
    }
}
