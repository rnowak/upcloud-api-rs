//! Host types.

use serde::{Deserialize, Serialize};

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
