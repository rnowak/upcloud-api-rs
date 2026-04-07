use serde::{Deserialize, Serialize};

/// A single price item (amount + price per unit).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PriceItem {
    /// The number of units the price covers.
    pub amount: u64,
    /// The hourly price for the given amount of units, in the account's currency.
    pub price: f64,
}

/// Pricing for a single zone.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PriceZone {
    /// The zone identifier (e.g. `fi-hel1`).
    pub name: String,
    /// Hourly price per firewall.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall: Option<PriceItem>,
    /// Price per 1,000,000 I/O requests on backup storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_request_backup: Option<PriceItem>,
    /// Price per 1,000,000 I/O requests on HDD storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_request_hdd: Option<PriceItem>,
    /// Price per 1,000,000 I/O requests on MaxIOPS storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_request_maxiops: Option<PriceItem>,
    /// Hourly price per IPv4 address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_address: Option<PriceItem>,
    /// Hourly price per IPv6 address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<PriceItem>,
    /// Price per GB of inbound public IPv4 transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ipv4_bandwidth_in: Option<PriceItem>,
    /// Price per GB of outbound public IPv4 transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ipv4_bandwidth_out: Option<PriceItem>,
    /// Price per GB of inbound public IPv6 transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ipv6_bandwidth_in: Option<PriceItem>,
    /// Price per GB of outbound public IPv6 transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ipv6_bandwidth_out: Option<PriceItem>,
    /// Hourly price per server CPU core.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_core: Option<PriceItem>,
    /// Hourly price per 256 MB of server memory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_memory: Option<PriceItem>,
    /// Hourly price per GB of backup storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_backup: Option<PriceItem>,
    /// Hourly price per GB of HDD storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_hdd: Option<PriceItem>,
    /// Hourly price per GB of MaxIOPS storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_maxiops: Option<PriceItem>,
    /// Hourly price per GB of encrypted storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<PriceItem>,
    /// Hourly price for a specific server plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_plan_1xcpu_1gb: Option<PriceItem>,
}
