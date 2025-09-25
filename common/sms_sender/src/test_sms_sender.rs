//! Implement SMS sender & receiver with [`tokio::sync::mpsc::unbounded_channel`].
//! Used for automatio test purpose.

use crate::{SendSmsInput, SmsSender};
use async_trait::async_trait;
use error::{Error, Result};
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};

/// The sms sender for development and testing purposes.
/// It's only visible when the `test` feature is enabled.
#[derive(Debug)]
pub struct TestSmsSender {
    sender: mpsc::UnboundedSender<SendSmsInput>,
    /// If receiver drop, sender can not send message.
    /// Since local deployment would use [`TestSmsSender`], holding receiver here to make server not error out.
    _receiver: TestSmsReceiver,
}

impl TestSmsSender {
    /// Create a new `TestSmsSender` and return a `TestSmsReceiver` to receive sms.
    /// The `TestSmsReceiver` can be used to assert that the sms was sent correctly.
    pub fn new() -> (Self, TestSmsReceiver) {
        let (tx, rx) = mpsc::unbounded_channel();
        let receiver = Arc::new(Mutex::new(rx));
        (
            Self {
                sender: tx,
                _receiver: TestSmsReceiver(Arc::clone(&receiver)),
            },
            TestSmsReceiver(receiver),
        )
    }
}

#[async_trait]
impl SmsSender for TestSmsSender {
    async fn send(&self, input: SendSmsInput) -> Result<()> {
        self.sender
            .send(input)
            .map_err(|e| Error::internal(format!("Unable to send sms {e:?}")))?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct TestSmsReceiver(Arc<Mutex<mpsc::UnboundedReceiver<SendSmsInput>>>);

impl TestSmsReceiver {
    /// Receive all sms that have been sent.
    pub async fn receive_sms(&self) -> Vec<SendSmsInput> {
        let mut result = Vec::new();
        let mut receiver = self.0.as_ref().lock().await;
        while let Ok(received) = receiver.try_recv() {
            result.push(received)
        }
        result
    }
}
