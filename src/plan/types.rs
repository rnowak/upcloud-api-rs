//! Plan types.

use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

/// A predefined server plan (e.g. `1xCPU-1GB`).
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Plan {
    /// Display name of the plan (e.g. `1xCPU-2GB`). Used when creating or modifying a server.
    pub name: String,
    /// Number of CPU cores included in the plan.
    #[serde_as(as = "DisplayFromStr")]
    pub core_number: u32,
    /// Amount of RAM in MiB included in the plan.
    #[serde_as(as = "DisplayFromStr")]
    pub memory_amount: u64,
    /// Amount of storage in GiB included in the plan. `0` when the plan does not include storage.
    #[serde_as(as = "DisplayFromStr")]
    pub storage_size: u64,
    /// Storage tier used by the plan (e.g. `maxiops`). `None` when the plan does not include storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_tier: Option<String>,
    /// Amount of outbound transfer included in the plan, in GiB. Excess traffic is billed hourly.
    #[serde_as(as = "DisplayFromStr")]
    pub public_traffic_out: u64,
    /// Number of GPUs included in the plan. Only present on GPU plans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_amount: Option<u32>,
    /// GPU model identifier (e.g. `NVIDIA L40S`). Only present on GPU plans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_model: Option<String>,
}

/// A valid combination of CPU core count and memory for a custom server.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ServerConfiguration {
    #[serde_as(as = "DisplayFromStr")]
    pub core_number: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub memory_amount: u64,
}
