//! Audit Logs — `GET /1.3/audit-logs`, `GET /1.3/audit-logs/filter-options`

use serde::{Deserialize, Serialize};

use crate::{client::Client, error::Error};

// ─── Types ────────────────────────────────────────────────────────────────────

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

// ─── Client methods ───────────────────────────────────────────────────────────

impl Client {
    /// Returns a paginated list of audit events.
    ///
    /// The response includes a `total_count` from the `Upcloud-Total-Count` header.
    pub async fn list_audit_events(
        &self,
        params: AuditListParams,
    ) -> Result<AuditEventPage, Error> {
        let req = self
            .request(reqwest::Method::GET, "/1.3/audit-logs")
            .query(&params);
        let resp = req.send().await?;
        let total_count = resp
            .headers()
            .get("Upcloud-Total-Count")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok());
        let status = resp.status();
        if !status.is_success() {
            let body = resp.bytes().await?;
            if let Ok(env) =
                serde_json::from_slice::<crate::error::ErrorEnvelope>(&body)
            {
                return Err(crate::error::Error::Api(crate::error::ApiError {
                    error_code: env.error.error_code,
                    error_message: env.error.error_message,
                    status: status.as_u16(),
                }));
            }
            return Err(crate::error::Error::Api(crate::error::ApiError {
                error_code: "UNKNOWN".into(),
                error_message: format!("HTTP {}", status.as_u16()),
                status: status.as_u16(),
            }));
        }
        let body = resp.bytes().await?;
        #[derive(Deserialize)]
        struct AuditLogListResponse {
            audit_logs: Vec<AuditEvent>,
        }
        let parsed: AuditLogListResponse = serde_json::from_slice(&body)?;
        Ok(AuditEventPage { events: parsed.audit_logs, total_count })
    }

    /// Returns the available filter option values for audit events.
    pub async fn get_audit_filter_options(&self) -> Result<AuditFilterOptions, Error> {
        self.get("/1.3/audit-logs/filter-options").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const AUDIT_LOG_LIST_JSON: &str = r#"{
  "audit_logs": [
    {
      "time": "2024-04-02T11:52:25.719619Z",
      "auth_type": "gui",
      "upcloud_correlation_id": "01HTFAEGT48JP7M5YFHC7W21MG",
      "resource_type": "auth",
      "action": "login",
      "origin": "gui",
      "account_username": "mrmagoo",
      "ip_address": "1.2.3.4",
      "resource_id": null
    },
    {
      "time": "2024-03-13T17:50:26.647075Z",
      "auth_type": "daemon",
      "upcloud_correlation_id": "",
      "resource_type": "storage",
      "action": "delete",
      "origin": "upcloud_internal",
      "account_username": "mrmagoo",
      "ip_address": "",
      "resource_id": "00000000-0000-0000-0000-000000000000"
    }
  ]
}"#;

    const AUDIT_FILTER_OPTIONS_JSON: &str = r#"{
  "resource_types": [
    "account",
    "auth",
    "server",
    "storage"
  ],
  "actions": [
    "create",
    "read",
    "update",
    "delete",
    "login",
    "logout"
  ],
  "origins": ["api", "gui", "upcloud_internal"]
}"#;

    #[test]
    fn deserialize_audit_log_list() {
        #[derive(Deserialize)]
        struct AuditLogListResponse {
            audit_logs: Vec<AuditEvent>,
        }
        let resp: AuditLogListResponse =
            serde_json::from_str(AUDIT_LOG_LIST_JSON).unwrap();
        assert_eq!(resp.audit_logs.len(), 2);

        let first = &resp.audit_logs[0];
        assert_eq!(first.time.as_deref(), Some("2024-04-02T11:52:25.719619Z"));
        assert_eq!(first.auth_type.as_deref(), Some("gui"));
        assert_eq!(
            first.upcloud_correlation_id.as_deref(),
            Some("01HTFAEGT48JP7M5YFHC7W21MG")
        );
        assert_eq!(first.resource_type.as_deref(), Some("auth"));
        assert_eq!(first.action.as_deref(), Some("login"));
        assert_eq!(first.origin.as_deref(), Some("gui"));
        assert_eq!(first.account_username.as_deref(), Some("mrmagoo"));
        assert_eq!(first.ip_address.as_deref(), Some("1.2.3.4"));
        assert!(first.resource_id.is_none());
        assert!(first.request_id.is_none());

        let second = &resp.audit_logs[1];
        assert_eq!(second.time.as_deref(), Some("2024-03-13T17:50:26.647075Z"));
        assert_eq!(second.auth_type.as_deref(), Some("daemon"));
        assert_eq!(second.upcloud_correlation_id.as_deref(), Some(""));
        assert_eq!(second.resource_type.as_deref(), Some("storage"));
        assert_eq!(second.action.as_deref(), Some("delete"));
        assert_eq!(second.origin.as_deref(), Some("upcloud_internal"));
        assert_eq!(second.account_username.as_deref(), Some("mrmagoo"));
        assert_eq!(second.ip_address.as_deref(), Some(""));
        assert_eq!(
            second.resource_id.as_deref(),
            Some("00000000-0000-0000-0000-000000000000")
        );
        assert!(second.request_id.is_none());
    }

    #[test]
    fn deserialize_audit_filter_options() {
        let opts: AuditFilterOptions =
            serde_json::from_str(AUDIT_FILTER_OPTIONS_JSON).unwrap();
        assert_eq!(opts.resource_types.len(), 4);
        assert_eq!(opts.resource_types[0], "account");
        assert_eq!(opts.resource_types[1], "auth");
        assert_eq!(opts.resource_types[2], "server");
        assert_eq!(opts.resource_types[3], "storage");

        assert_eq!(opts.actions.len(), 6);
        assert_eq!(opts.actions[0], "create");
        assert_eq!(opts.actions[1], "read");
        assert_eq!(opts.actions[2], "update");
        assert_eq!(opts.actions[3], "delete");
        assert_eq!(opts.actions[4], "login");
        assert_eq!(opts.actions[5], "logout");

        assert_eq!(opts.origins.len(), 3);
        assert_eq!(opts.origins[0], "api");
        assert_eq!(opts.origins[1], "gui");
        assert_eq!(opts.origins[2], "upcloud_internal");
    }
}
