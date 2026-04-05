//! Dedicated hosts — `GET/PATCH /1.3/host`

use serde::{Deserialize, Serialize};

use crate::{client::Client, error::Error};

// ─── Types ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostStat {
    pub name: String,
    pub value: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HostStats {
    #[serde(default)]
    pub stat: Vec<HostStat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Host {
    pub id: i64,
    pub description: String,
    pub zone: String,
    #[serde(default)]
    pub windows_enabled: String,
    #[serde(default)]
    pub stats: HostStats,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyHostRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

// ─── Response wrappers ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct HostListResponse {
    hosts: HostWrapper,
}

#[derive(Debug, Deserialize)]
struct HostWrapper {
    host: Vec<Host>,
}

#[derive(Debug, Deserialize)]
struct HostResponse {
    host: Host,
}

// ─── Client methods ───────────────────────────────────────────────────────────

impl Client {
    /// Returns a list of all available dedicated hosts.
    pub async fn list_hosts(&self) -> Result<Vec<Host>, Error> {
        let resp: HostListResponse = self.get("/1.3/host").await?;
        Ok(resp.hosts.host)
    }

    /// Returns details about a specific dedicated host.
    pub async fn get_host(&self, id: i64) -> Result<Host, Error> {
        let resp: HostResponse = self.get(&format!("/1.3/host/{id}")).await?;
        Ok(resp.host)
    }

    /// Modifies the description of a dedicated host.
    pub async fn modify_host(&self, id: i64, req: ModifyHostRequest) -> Result<Host, Error> {
        #[derive(Serialize)]
        struct Body {
            host: ModifyHostRequest,
        }
        let resp: HostResponse = self
            .patch(&format!("/1.3/host/{id}"), &Body { host: req })
            .await?;
        Ok(resp.host)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIST_HOSTS_JSON: &str = r#"{
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
}"#;

    const GET_HOST_JSON: &str = r#"{
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
}"#;

    #[test]
    fn deserialize_host_list() {
        let resp: HostListResponse = serde_json::from_str(LIST_HOSTS_JSON).unwrap();
        let hosts = resp.hosts.host;
        assert_eq!(hosts.len(), 2);

        // Host #1 — all fields
        assert_eq!(hosts[0].id, 7653311107);
        assert_eq!(hosts[0].description, "My Host #1");
        assert_eq!(hosts[0].zone, "private-zone-id");
        assert_eq!(hosts[0].windows_enabled, "no");
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
        assert_eq!(hosts[1].windows_enabled, "no");
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
        assert_eq!(host.windows_enabled, "no");
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

    const MODIFY_HOST_REQUEST_JSON: &str = r#"{
  "host": {
    "description": "My New Host"
  }
}"#;

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

    const MODIFY_HOST_RESPONSE_JSON: &str = r#"{
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
}"#;

    #[test]
    fn deserialize_modify_host_response() {
        let resp: HostResponse = serde_json::from_str(MODIFY_HOST_RESPONSE_JSON).unwrap();
        let host = resp.host;
        assert_eq!(host.id, 7653311107);
        assert_eq!(host.description, "My New Host");
        assert_eq!(host.zone, "private-zone-id");
        assert_eq!(host.windows_enabled, "no");
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
}
