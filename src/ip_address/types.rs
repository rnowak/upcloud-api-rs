//! IP address types.

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::types::common::YesNo;

// ─── Enums ────────────────────────────────────────────────────────────────────

/// IP address family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Family {
    /// IPv4 address.
    IPv4,
    /// IPv6 address.
    IPv6,
}

/// Network access type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Access {
    /// Public internet.
    Public,
    /// Utility network.
    Utility,
    /// Private network.
    Private,
}

/// IP address release behavior on server deletion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReleasePolicy {
    /// Release the address back to the pool.
    Release,
    /// Keep the address allocated.
    Keep,
}

// ─── Types ────────────────────────────────────────────────────────────────────

/// An IP address resource.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IpAddress {
    /// Network access type (`utility`, `public`, or `private`).
    pub access: Access,
    /// The IP address string.
    pub address: String,
    /// Address family (`IPv4` or `IPv6`).
    pub family: Family,
    /// Reverse DNS PTR record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ptr_record: Option<String>,
    /// UUID of the server this address is attached to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// MAC address of the interface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    /// Whether this is a floating IP.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floating: Option<bool>,
    /// Whether the address is DHCP-provided.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dhcp_provided: Option<bool>,
    /// Behavior on server deletion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_policy: Option<ReleasePolicy>,
    /// Whether the address is delegated.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_delegated: Option<bool>,
    /// Whether the address is included in a server plan.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_of_plan: Option<bool>,
    /// Availability zone identifier (e.g. `fi-hel2`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

/// Request body for assigning an IP address.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignRequest {
    /// Network access type (`utility` or `public`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<Access>,
    /// Address family (`IPv4` or `IPv6`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<Family>,
    /// Server UUID to assign to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// Zone identifier (required for detached floating IPs).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
    /// MAC address of the server interface to attach to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    /// Assign as a floating IP.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floating: Option<bool>,
    /// Behavior on server deletion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_policy: Option<ReleasePolicy>,
}

/// Request body for modifying an IP address.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyRequest {
    /// Reverse DNS PTR record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ptr_record: Option<String>,
    /// MAC address to re-attach to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    /// Release behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_policy: Option<ReleasePolicy>,
}
