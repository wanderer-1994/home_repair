use tokio::sync::mpsc;

#[derive(Debug)]
/// Bridge for manipulating http response from within graphql resolvers.
pub enum HttpEffect {
    /// Set response cookies
    SetCookies(Vec<String>),
}

pub type HttpEffectSender = mpsc::UnboundedSender<HttpEffect>;
pub type HttpEffectReceiver = mpsc::UnboundedReceiver<HttpEffect>;

impl HttpEffect {
    pub fn unbounded_channel() -> (HttpEffectSender, HttpEffectReceiver) {
        tokio::sync::mpsc::unbounded_channel::<HttpEffect>()
    }
}
