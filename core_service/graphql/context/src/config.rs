use cookie::SameSite;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentConfig {
    /// Hostname of frontend, including scheme (http or https)
    pub frontend_host: String,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Configuration for yearnings server feature flags
pub struct Features {
    /// Placeholder feature flag
    pub foo: bool,
}

#[derive(Debug)]
pub struct CookieConfig {
    pub use_https: bool,
    pub same_site: SameSite,
    pub cookie_domain: String,
}
