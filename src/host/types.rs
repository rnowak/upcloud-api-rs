//! Host types.

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::types::YesNo;

/// A single host statistic measurement.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HostStat {
    pub name: String,
    /// Statistic value.
    pub value: serde_json::Value,
    /// Measurement unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// Measurement timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

/// Collection of host statistics.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct HostStats {
    /// List of statistics.
    #[serde(default)]
    pub stat: Vec<HostStat>,
}

/// A private cloud host.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Host {
    /// Host identifier.
    pub id: i64,
    pub description: String,
    pub zone: String,
    /// Whether Windows servers are supported.
    #[serde_as(as = "YesNo")]
    #[serde(default)]
    pub windows_enabled: bool,
    /// Host performance statistics.
    #[serde(default)]
    pub stats: HostStats,
}

/// Request body for modifying a host.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyHostRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
