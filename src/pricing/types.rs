use serde::{Deserialize, Serialize};

/// A single price item (amount + price per unit).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PriceItem {
    pub amount: u64,
    pub price: f64,
}

/// Pricing for a single zone.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PriceZone {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_request_backup: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_request_hdd: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_request_maxiops: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_address: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ipv4_bandwidth_in: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ipv4_bandwidth_out: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ipv6_bandwidth_in: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ipv6_bandwidth_out: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_core: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_memory: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_backup: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_hdd: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_maxiops: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_plan_1xcpu_1gb: Option<PriceItem>,
}
