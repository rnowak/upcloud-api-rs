use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upcloud_correlation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
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
