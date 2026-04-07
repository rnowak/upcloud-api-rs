//! Audit Logs — `GET /1.3/audit-logs`, `GET /1.3/audit-logs/filter-options`

mod types;

pub use types::*;

use serde::Deserialize;

use crate::client::Client;
use crate::error::Error;

/// Handler for audit log operations.
pub struct AuditLogHandler<'a> {
    client: &'a Client,
}

impl<'a> AuditLogHandler<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Returns a paginated list of audit events.
    ///
    /// The response includes a `total_count` from the `Upcloud-Total-Count` header.
    ///
    /// `GET /1.3/audit-logs`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/22-audit-logs/#list-audit-logs)
    pub async fn list(&self, params: AuditListParams) -> Result<AuditEventPage, Error> {
        let req = self
            .client
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
            if let Ok(env) = serde_json::from_slice::<crate::error::ErrorEnvelope>(&body) {
                return Err(crate::error::Error::Api(crate::error::ApiError {
                    error_code: crate::error::ErrorCode::from(env.error.error_code),
                    error_message: env.error.error_message,
                    status: status.as_u16(),
                }));
            }
            return Err(crate::error::Error::Api(crate::error::ApiError {
                error_code: crate::error::ErrorCode::Unknown("UNKNOWN".to_string()),
                error_message: format!("HTTP {}", status.as_u16()),
                status: status.as_u16(),
            }));
        }
        let body = resp.bytes().await?;
        let parsed: AuditLogListResponse = serde_json::from_slice(&body)?;
        Ok(AuditEventPage {
            events: parsed.audit_logs,
            total_count,
        })
    }

    /// Returns the available filter option values for audit events.
    ///
    /// `GET /1.3/audit-logs/filter-options`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/22-audit-logs/#list-options-to-filter-by)
    pub async fn get_filter_options(&self) -> Result<AuditFilterOptions, Error> {
        self.client.get("/1.3/audit-logs/filter-options").await
    }
}

// ─── Response wrappers (crate-private) ────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct AuditLogListResponse {
    audit_logs: Vec<AuditEvent>,
}

#[cfg(test)]
mod tests;
