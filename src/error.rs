use thiserror::Error;

/// Errors returned by the UpCloud API client.
#[derive(Debug, Error)]
pub enum Error {
    /// An HTTP transport error (connection refused, timeout, DNS failure, etc.).
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// The UpCloud API returned a structured error response.
    #[error("API error: {0}")]
    Api(ApiError),

    /// Failed to deserialize a response body as JSON.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Failed to parse a URL.
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    /// Invalid client configuration (missing auth, bad base URL, etc.).
    #[error("invalid configuration: {0}")]
    InvalidConfig(String),
}

impl Error {
    /// Returns the HTTP status code if this is an API error.
    pub fn status(&self) -> Option<u16> {
        match self {
            Error::Api(e) => Some(e.status),
            _ => None,
        }
    }

    /// Returns `true` if this is a 404 Not Found API error.
    pub fn is_not_found(&self) -> bool {
        self.status() == Some(404)
    }

    /// Returns `true` if this is a 401 or 403 authentication/authorization error.
    pub fn is_unauthorized(&self) -> bool {
        matches!(self.status(), Some(401) | Some(403))
    }

    /// Returns `true` if this is a 409 Conflict API error.
    pub fn is_conflict(&self) -> bool {
        self.status() == Some(409)
    }
}

/// An error returned by the UpCloud API.
///
/// Contains the HTTP status code and the structured error body from the API
/// response envelope: `{"error": {"error_code": "...", "error_message": "..."}}`.
#[derive(Debug, Clone)]
pub struct ApiError {
    /// Machine-readable error code (e.g. `"SERVER_NOT_FOUND"`, `"AUTHENTICATION_FAILED"`).
    pub error_code: String,
    /// Human-readable error description.
    pub error_message: String,
    /// HTTP status code (e.g. 404, 409).
    pub status: u16,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.error_message, self.error_code)
    }
}

impl std::error::Error for ApiError {}

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
