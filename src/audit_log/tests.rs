use super::*;
// Source: https://developers.upcloud.com/1.3/22-audit-logs/#list-audit-logs
const AUDIT_LOG_LIST_JSON: &str = r#"
    {
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
    }
"#;

#[test]
fn deserialize_audit_log_list() {
    let resp: AuditLogListResponse = serde_json::from_str(AUDIT_LOG_LIST_JSON).unwrap();
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

// Source: https://developers.upcloud.com/1.3/22-audit-logs/#list-options-to-filter-by
const AUDIT_FILTER_OPTIONS_JSON: &str = r#"
    {
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
        "origins": [
            "api",
            "gui",
            "upcloud_internal"
        ]
    }
"#;

#[test]
fn deserialize_audit_filter_options() {
    let opts: AuditFilterOptions = serde_json::from_str(AUDIT_FILTER_OPTIONS_JSON).unwrap();
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

#[test]
fn export_format_serialization() {
    assert_eq!(ExportFormat::Csv.to_string(), "csv");
    assert_eq!(ExportFormat::Json.to_string(), "json");

    let csv: ExportFormat = serde_json::from_str(r#""csv""#).unwrap();
    assert_eq!(csv, ExportFormat::Csv);
    let json: ExportFormat = serde_json::from_str(r#""json""#).unwrap();
    assert_eq!(json, ExportFormat::Json);

    assert_eq!(serde_json::to_string(&ExportFormat::Csv).unwrap(), r#""csv""#);
    assert_eq!(serde_json::to_string(&ExportFormat::Json).unwrap(), r#""json""#);
}
