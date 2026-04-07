use super::*;

// Source: https://developers.upcloud.com/1.3/14-hosts/#list-available-hosts
const LIST_HOSTS_JSON: &str = r#"
    {
        "hosts": {
            "host": [
                {
                    "id": 7653311107,
                    "description": "My Host #1",
                    "zone": "private-zone-id",
                    "windows_enabled": "no",
                    "stats": {
                        "stat": [
                            {
                                "name": "cpu_idle",
                                "timestamp": "2019-08-09T12:46:57Z",
                                "value": 95.2
                            },
                            {
                                "name": "memory_free",
                                "timestamp": "2019-08-09T12:46:57Z",
                                "value": 102
                            }
                        ]
                    }
                },
                {
                    "id": 8055964291,
                    "description": "My Host #2",
                    "zone": "private-zone-id",
                    "windows_enabled": "no",
                    "stats": {
                        "stat": [
                            {
                                "name": "cpu_idle",
                                "timestamp": "2019-08-09T12:46:57Z",
                                "value": 80.1
                            },
                            {
                                "name": "memory_free",
                                "timestamp": "2019-08-09T12:46:57Z",
                                "value": 61
                            }
                        ]
                    }
                }
            ]
        }
    }
"#;

// Source: https://developers.upcloud.com/1.3/14-hosts/#get-host-details
const GET_HOST_JSON: &str = r#"
    {
        "host": {
            "id": 7653311107,
            "description": "My Host #1",
            "zone": "private-zone-id",
            "windows_enabled": "no",
            "stats": {
                "stat": [
                    {
                        "name": "cpu_idle",
                        "timestamp": "2019-08-09T12:46:57Z",
                        "value": 95.2
                    },
                    {
                        "name": "memory_free",
                        "timestamp": "2019-08-09T12:46:57Z",
                        "value": 102
                    }
                ]
            }
        }
    }
"#;

#[test]
fn deserialize_host_list() {
    let resp: HostListResponse = serde_json::from_str(LIST_HOSTS_JSON).unwrap();
    let hosts = resp.hosts.host;
    assert_eq!(hosts.len(), 2);

    // Host #1 — all fields
    assert_eq!(hosts[0].id, 7653311107);
    assert_eq!(hosts[0].description, "My Host #1");
    assert_eq!(hosts[0].zone, "private-zone-id");
    assert!(!hosts[0].windows_enabled);
    assert_eq!(hosts[0].stats.stat.len(), 2);
    assert_eq!(hosts[0].stats.stat[0].name, "cpu_idle");
    assert_eq!(
        hosts[0].stats.stat[0].timestamp.as_deref(),
        Some("2019-08-09T12:46:57Z")
    );
    assert_eq!(hosts[0].stats.stat[0].value, serde_json::json!(95.2));
    assert_eq!(hosts[0].stats.stat[1].name, "memory_free");
    assert_eq!(
        hosts[0].stats.stat[1].timestamp.as_deref(),
        Some("2019-08-09T12:46:57Z")
    );
    assert_eq!(hosts[0].stats.stat[1].value, serde_json::json!(102));

    // Host #2 — all fields
    assert_eq!(hosts[1].id, 8055964291);
    assert_eq!(hosts[1].description, "My Host #2");
    assert_eq!(hosts[1].zone, "private-zone-id");
    assert!(!hosts[1].windows_enabled);
    assert_eq!(hosts[1].stats.stat.len(), 2);
    assert_eq!(hosts[1].stats.stat[0].name, "cpu_idle");
    assert_eq!(
        hosts[1].stats.stat[0].timestamp.as_deref(),
        Some("2019-08-09T12:46:57Z")
    );
    assert_eq!(hosts[1].stats.stat[0].value, serde_json::json!(80.1));
    assert_eq!(hosts[1].stats.stat[1].name, "memory_free");
    assert_eq!(
        hosts[1].stats.stat[1].timestamp.as_deref(),
        Some("2019-08-09T12:46:57Z")
    );
    assert_eq!(hosts[1].stats.stat[1].value, serde_json::json!(61));
}

#[test]
fn deserialize_host_detail() {
    let resp: HostResponse = serde_json::from_str(GET_HOST_JSON).unwrap();
    let host = resp.host;
    assert_eq!(host.id, 7653311107);
    assert_eq!(host.description, "My Host #1");
    assert_eq!(host.zone, "private-zone-id");
    assert!(!host.windows_enabled);
    assert_eq!(host.stats.stat.len(), 2);
    assert_eq!(host.stats.stat[0].name, "cpu_idle");
    assert_eq!(
        host.stats.stat[0].timestamp.as_deref(),
        Some("2019-08-09T12:46:57Z")
    );
    assert_eq!(host.stats.stat[0].value, serde_json::json!(95.2));
    assert_eq!(host.stats.stat[1].name, "memory_free");
    assert_eq!(
        host.stats.stat[1].timestamp.as_deref(),
        Some("2019-08-09T12:46:57Z")
    );
    assert_eq!(host.stats.stat[1].value, serde_json::json!(102));
}

// ─── Modify host request ────────────────────────────────────────────────

// Source: https://developers.upcloud.com/1.3/14-hosts/#modify-host-details
const MODIFY_HOST_REQUEST_JSON: &str = r#"
    {
        "host": {
            "description": "My New Host"
        }
    }
"#;

#[test]
fn deserialize_modify_host_request() {
    #[derive(Debug, serde::Deserialize)]
    struct Body {
        host: ModifyHostRequest,
    }
    let body: Body = serde_json::from_str(MODIFY_HOST_REQUEST_JSON).unwrap();
    assert_eq!(body.host.description.as_deref(), Some("My New Host"));
}

// ─── Modify host response ───────────────────────────────────────────────

// Source: https://developers.upcloud.com/1.3/14-hosts/#modify-host-details
const MODIFY_HOST_RESPONSE_JSON: &str = r#"
    {
        "host": {
            "id": 7653311107,
            "description": "My New Host",
            "zone": "private-zone-id",
            "windows_enabled": "no",
            "stats": {
                "stat": [
                    {
                        "name": "cpu_idle",
                        "timestamp": "2019-08-09T13:37:12Z",
                        "value": 93.3
                    },
                    {
                        "name": "memory_free",
                        "timestamp": "2019-08-09T13:37:12Z",
                        "value": 104
                    }
                ]
            }
        }
    }
"#;

#[test]
fn deserialize_modify_host_response() {
    let resp: HostResponse = serde_json::from_str(MODIFY_HOST_RESPONSE_JSON).unwrap();
    let host = resp.host;
    assert_eq!(host.id, 7653311107);
    assert_eq!(host.description, "My New Host");
    assert_eq!(host.zone, "private-zone-id");
    assert!(!host.windows_enabled);
    assert_eq!(host.stats.stat.len(), 2);
    assert_eq!(host.stats.stat[0].name, "cpu_idle");
    assert_eq!(
        host.stats.stat[0].timestamp.as_deref(),
        Some("2019-08-09T13:37:12Z")
    );
    assert_eq!(host.stats.stat[0].value, serde_json::json!(93.3));
    assert_eq!(host.stats.stat[1].name, "memory_free");
    assert_eq!(
        host.stats.stat[1].timestamp.as_deref(),
        Some("2019-08-09T13:37:12Z")
    );
    assert_eq!(host.stats.stat[1].value, serde_json::json!(104));
}
