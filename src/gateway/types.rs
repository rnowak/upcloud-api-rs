//! Public types for the Gateway API.

use serde::{Deserialize, Serialize};

// ─── Enums ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Feature {
    Nat,
    Vpn,
}

// ─── Types ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_gateway_bandwidth_mbps: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_gateway_max_connections: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_number: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_features: Option<Vec<Feature>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn_tunnel_amount: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gateway {
    pub uuid: String,
    pub name: String,
    pub zone: String,
    pub plan: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_status: Option<String>,
    #[serde(default)]
    pub features: Vec<Feature>,
    #[serde(default)]
    pub routers: Vec<Router>,
    #[serde(default)]
    pub addresses: Vec<Address>,
    #[serde(default)]
    pub connections: Vec<Connection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tunnel_internal_ip_allocation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Router {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    pub uuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(rename = "type")]
    pub connection_type: String,
    #[serde(default)]
    pub local_routes: Vec<Route>,
    #[serde(default)]
    pub remote_routes: Vec<Route>,
    #[serde(default)]
    pub tunnels: Vec<Tunnel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub name: String,
    pub static_network: String,
    #[serde(rename = "type")]
    pub route_type: String,
}

/// A tunnel endpoint address. Can contain `name` (referencing a gateway address)
/// or `address` (a literal IP), or both.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tunnel {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_address: Option<TunnelAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_address: Option<TunnelAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipsec: Option<IpsecProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnel_healthy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnel_up: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnel_internal_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_peer_ping_interval: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpsecAuthentication {
    pub authentication: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psk: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpsecProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<IpsecAuthentication>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_rekey_time: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpd_delay: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpd_timeout: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ike_lifetime: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase1_algorithms: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase1_dh_group_numbers: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase1_integrity_algorithms: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase2_algorithms: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase2_dh_group_numbers: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase2_integrity_algorithms: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rekey_time: Option<u64>,
}

// ─── Request types ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequest {
    pub name: String,
    pub zone: String,
    pub features: Vec<Feature>,
    pub routers: Vec<RouterInput>,
    pub configured_status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<CreateConnectionRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<AddressInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tunnel_internal_ip_allocation: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterInput {
    pub uuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressInput {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<Feature>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<CreateConnectionRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tunnel_internal_ip_allocation: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConnectionRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub connection_type: String,
    #[serde(default)]
    pub local_routes: Vec<RouteInput>,
    #[serde(default)]
    pub remote_routes: Vec<RouteInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnels: Option<Vec<CreateTunnelRequest>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteInput {
    pub name: String,
    pub static_network: String,
    #[serde(rename = "type")]
    pub route_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTunnelRequest {
    pub name: String,
    pub local_address: TunnelAddress,
    pub remote_address: TunnelAddress,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipsec: Option<IpsecProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnel_internal_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_peer_ping_interval: Option<u32>,
}
