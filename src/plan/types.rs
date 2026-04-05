//! Plan types.

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A predefined server plan (e.g. `1xCPU-1GB`).
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub name: String,
    #[serde_as(as = "DisplayFromStr")]
    pub core_number: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub memory_amount: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub storage_size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_tier: Option<String>,
    #[serde_as(as = "DisplayFromStr")]
    pub public_traffic_out: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_amount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_model: Option<String>,
}

/// A valid combination of CPU core count and memory for a custom server.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfiguration {
    #[serde_as(as = "DisplayFromStr")]
    pub core_number: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub memory_amount: u64,
}
