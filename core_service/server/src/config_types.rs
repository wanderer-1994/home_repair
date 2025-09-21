use std::sync::Arc;

use cookie::SameSite;
use core_service_graphql_context::CookieConfig;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SameSiteConfig {
    Strict,
    Lax,
    None,
}

impl From<SameSiteConfig> for SameSite {
    fn from(value: SameSiteConfig) -> Self {
        match value {
            SameSiteConfig::Strict => SameSite::Strict,
            SameSiteConfig::Lax => SameSite::Lax,
            SameSiteConfig::None => SameSite::None,
        }
    }
}

pub struct HttpConfig {
    pub cookie_config: Arc<CookieConfig>,
    pub cors_origins: Vec<String>,
}
