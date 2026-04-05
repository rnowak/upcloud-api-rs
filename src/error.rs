use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error: {0}")]
    Api(ApiError),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    #[error("invalid configuration: {0}")]
    InvalidConfig(String),
}

/// An error returned by the UpCloud API.
#[derive(Debug, Clone)]
pub struct ApiError {
    pub error_code: String,
    pub error_message: String,
    pub status: u16,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.error_message, self.error_code)
    }
}

/// The UpCloud error response envelope: `{ "error": { "error_code": ..., "error_message": ... } }`.
#[derive(serde::Deserialize, Debug)]
pub(crate) struct ErrorEnvelope {
    pub error: ErrorBody,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct ErrorBody {
    pub error_code: String,
    pub error_message: String,
}
