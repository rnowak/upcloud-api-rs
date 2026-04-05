use std::time::Duration;

/// Authentication mode for the UpCloud API.
#[derive(Debug, Clone)]
pub enum AuthMode {
    /// Bearer token authentication (preferred, `Authorization: Bearer ucat_...`).
    Bearer(String),
    /// HTTP Basic authentication (legacy, `Authorization: Basic <base64>`).
    Basic { username: String, password: String },
}

/// Configuration for the UpCloud API client.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub(crate) base_url: String,
    pub(crate) auth: AuthMode,
    pub(crate) timeout: Duration,
}

impl ClientConfig {
    pub fn new(auth: AuthMode) -> Self {
        Self {
            base_url: "https://api.upcloud.com".to_string(),
            auth,
            timeout: Duration::from_secs(30),
        }
    }

    /// Override the base URL (useful for testing or alternative endpoints).
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Override the request timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}
