use std::fmt;

use serde::{Deserialize, Serialize};

/// Export format for audit log data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Csv,
    Json,
}

impl fmt::Display for ExportFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExportFormat::Csv => write!(f, "csv"),
            ExportFormat::Json => write!(f, "json"),
        }
    }
}

/// A single audit log event.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AuditEvent {
    /// When the event occurred.
    pub time: Option<String>,
    /// Authentication method used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    /// Correlation ID for request tracing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upcloud_correlation_id: Option<String>,
    /// Type of the affected resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// Action performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// Origin of the action (e.g. `"api"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// Username that performed the action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_username: Option<String>,
    /// Source IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// UUID of the affected resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// Unique request identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// Query parameters for listing audit events.
#[derive(Debug, Clone, Serialize, Default)]
pub struct AuditListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub actions: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub origins: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub resource_types: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Full-text search query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
}

/// Paginated list of audit events with total count.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct AuditEventPage {
    pub events: Vec<AuditEvent>,
    /// Total count from the `Upcloud-Total-Count` response header.
    pub total_count: Option<u64>,
}

/// Available filter options for audit events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditFilterOptions {
    #[serde(default)]
    pub actions: Vec<String>,
    #[serde(default)]
    pub origins: Vec<String>,
    #[serde(default)]
    pub resource_types: Vec<String>,
}
