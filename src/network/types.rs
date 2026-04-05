//! Public types for Networks, Routers, and Network Peerings.

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::types::common::YesNo;

// ─── Network types ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NetworkType {
    Public,
    Utility,
    Private,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpNetwork {
    pub address: String,
    #[serde(default)]
    pub dhcp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dhcp_default_route: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dhcp_dns: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dhcp_routes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dhcp_routes_configuration: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dhcp_effective_routes: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dhcp_bootfile_url: Option<String>,
    pub family: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkIpNetworks {
    #[serde(default)]
    pub ip_network: Vec<IpNetwork>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkServer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub uuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkServers {
    #[serde(default)]
    pub server: Vec<NetworkServer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPeeringSummary {
    pub uuid: String,
    pub name: String,
    #[serde(default)]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkPeerings {
    #[serde(default)]
    pub peering: Vec<NetworkPeeringSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    pub uuid: String,
    pub name: String,
    #[serde(rename = "type")]
    pub network_type: NetworkType,
    pub zone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_networks: Option<NetworkIpNetworks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<NetworkServers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_routes: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peerings: Option<NetworkPeerings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNetworkRequest {
    pub name: String,
    pub zone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_networks: Option<NetworkIpNetworks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyNetworkRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_networks: Option<NetworkIpNetworks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router: Option<String>,
}

// ─── Server interface types ───────────────────────────────────────────────────

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInterfaceRequest {
    #[serde(rename = "type")]
    pub interface_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u32>,
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootable: Option<bool>,
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_filtering: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<InterfaceIpAddressesInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceIpAddressesInput {
    pub ip_address: Vec<InterfaceIpAddressInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceIpAddressInput {
    pub family: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyInterfaceRequest {
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootable: Option<bool>,
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_filtering: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignInterfaceIpRequest {
    pub family: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

// ─── Router types ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticRoute {
    pub name: Option<String>,
    pub route: String,
    pub nexthop: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub route_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterNetwork {
    pub uuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterNetworkSlice {
    #[serde(default)]
    pub network: Vec<RouterNetwork>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Router {
    pub uuid: String,
    pub name: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub router_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_networks: Option<RouterNetworkSlice>,
    #[serde(default)]
    pub static_routes: Vec<StaticRoute>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRouterRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub static_routes: Vec<StaticRoute>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyRouterRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_routes: Option<Vec<StaticRoute>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

// ─── Network Peering types ────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PeeringState {
    Active,
    PendingPeer,
    Provisioning,
    Disabled,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PeeringConfiguredStatus {
    Active,
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeeringNetwork {
    pub uuid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_networks: Option<NetworkIpNetworks>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPeering {
    pub uuid: String,
    pub name: String,
    pub configured_status: PeeringConfiguredStatus,
    pub state: PeeringState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<PeeringNetwork>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_network: Option<PeeringNetwork>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePeeringRequest {
    pub name: String,
    pub configured_status: Option<PeeringConfiguredStatus>,
    pub network: PeeringNetworkInput,
    pub peer_network: PeeringNetworkInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeeringNetworkInput {
    pub uuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyPeeringRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_status: Option<PeeringConfiguredStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}
