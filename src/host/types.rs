//! Host types.

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::types::common::YesNo;

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

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Host {
    pub id: i64,
    pub description: String,
    pub zone: String,
    #[serde_as(as = "YesNo")]
    #[serde(default)]
    pub windows_enabled: bool,
    #[serde(default)]
    pub stats: HostStats,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyHostRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
