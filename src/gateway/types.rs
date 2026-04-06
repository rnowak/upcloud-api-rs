//! Public types for the Gateway API.

use serde::{Deserialize, Serialize};

// ─── Enums ────────────────────────────────────────────────────────────────────

/// Gateway feature flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Feature {
    /// Network address translation.
    Nat,
    /// Virtual private network.
    Vpn,
}

/// Desired gateway status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfiguredStatus {
    /// The gateway should be running.
    Started,
    /// The gateway should be stopped.
    Stopped,
}

/// Gateway lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OperationalState {
    /// Waiting to be provisioned.
    Pending,
    /// Setting up the management agent.
    SetupAgent,
    /// Setting up the link network.
    SetupLinkNetwork,
    /// Setting up the backend server.
    SetupServer,
    /// Setting up networking.
    SetupNetwork,
    /// Setting up the gateway service.
    SetupGw,
    /// Setting up DNS records.
    SetupDns,
    /// Running health checks.
    Checkup,
    /// Gateway is running and operational.
    Running,
    /// Deleting DNS records.
    DeleteDns,
    /// Deleting networking.
    DeleteNetwork,
    /// Deleting the backend server.
    DeleteServer,
    /// Deleting the link network.
    DeleteLinkNetwork,
    /// Deleting the gateway service.
    DeleteService,
}

/// VPN connection type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionType {
    /// IPSec tunnel.
    #[serde(rename = "ipsec")]
    IPSec,
}

/// VPN tunnel lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TunnelOperationalState {
    /// Tunnel has not been initialised yet.
    Uninitialized,
    /// Tunnel has been created.
    Created,
    /// Tunnel is establishing a connection.
    Connecting,
    /// Tunnel connection is established.
    Established,
    /// Tunnel is rekeying.
    Rekeying,
    /// Tunnel has been rekeyed.
    Rekeyed,
    /// Tunnel is being deleted.
    Deleting,
    /// Tunnel is being destroyed.
    Destroying,
    /// Tunnel is in an unknown state.
    Unknown,
}

/// Network route type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RouteType {
    /// Static route.
    Static,
}

/// IPSec authentication method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IPSecAuthType {
    /// Pre-shared key.
    #[serde(rename = "psk")]
    PSK,
}

/// IPSec encryption algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IPSecAlgorithm {
    #[serde(rename = "aes128gcm16")]
    Aes128Gcm16,
    #[serde(rename = "aes128gcm128")]
    Aes128Gcm128,
    #[serde(rename = "aes192gcm16")]
    Aes192Gcm16,
    #[serde(rename = "aes192gcm128")]
    Aes192Gcm128,
    #[serde(rename = "aes256gcm16")]
    Aes256Gcm16,
    #[serde(rename = "aes256gcm128")]
    Aes256Gcm128,
    #[serde(rename = "aes128")]
    Aes128,
    #[serde(rename = "aes192")]
    Aes192,
    #[serde(rename = "aes256")]
    Aes256,
}

/// IPSec integrity/MAC algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IPSecIntegrityAlgorithm {
    #[serde(rename = "aes128gmac")]
    Aes128Gmac,
    #[serde(rename = "aes256gmac")]
    Aes256Gmac,
    #[serde(rename = "sha1")]
    Sha1,
    #[serde(rename = "sha256")]
    Sha256,
    #[serde(rename = "sha384")]
    Sha384,
    #[serde(rename = "sha512")]
    Sha512,
}

// ─── Types ────────────────────────────────────────────────────────────────────

/// A gateway plan describing bandwidth, max connections, and available features.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub name: String,
    /// Maximum bandwidth in Mbps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_gateway_bandwidth_mbps: Option<u64>,
    /// Maximum number of concurrent connections.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_gateway_max_connections: Option<u64>,
    /// Number of backend servers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_number: Option<u32>,
    /// Features available on this plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_features: Option<Vec<Feature>>,
    /// Number of VPN tunnels allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn_tunnel_amount: Option<u32>,
}

/// A named address allocated on a gateway.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub name: String,
    /// IP address string (may be auto-assigned).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

/// An UpCloud network gateway resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gateway {
    pub uuid: String,
    pub name: String,
    pub zone: String,
    pub plan: String,
    /// Current lifecycle state of the gateway.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_state: Option<OperationalState>,
    /// Desired status of the gateway.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_status: Option<ConfiguredStatus>,
    /// Enabled feature flags.
    #[serde(default)]
    pub features: Vec<Feature>,
    /// Routers attached to this gateway.
    #[serde(default)]
    pub routers: Vec<Router>,
    /// Addresses allocated on this gateway.
    #[serde(default)]
    pub addresses: Vec<Address>,
    /// VPN connections configured on this gateway.
    #[serde(default)]
    pub connections: Vec<Connection>,
    /// Key-value labels attached to the gateway.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    /// Whether internal tunnel IPs are automatically allocated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tunnel_internal_ip_allocation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// A router attached to the gateway.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Router {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    pub uuid: String,
}

/// A VPN connection on a gateway.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    /// Type of VPN connection.
    #[serde(rename = "type")]
    pub connection_type: ConnectionType,
    /// Routes on the local (gateway) side.
    #[serde(default)]
    pub local_routes: Vec<Route>,
    /// Routes on the remote (peer) side.
    #[serde(default)]
    pub remote_routes: Vec<Route>,
    /// VPN tunnels within this connection.
    #[serde(default)]
    pub tunnels: Vec<Tunnel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// A network route in a VPN connection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub name: String,
    /// CIDR network (e.g. `10.0.0.0/24`).
    pub static_network: String,
    /// Route type (currently always `Static`).
    #[serde(rename = "type")]
    pub route_type: RouteType,
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

/// A VPN tunnel within a connection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tunnel {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    /// Local (gateway-side) tunnel endpoint address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_address: Option<TunnelAddress>,
    /// Remote (peer-side) tunnel endpoint address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_address: Option<TunnelAddress>,
    /// IPSec configuration for this tunnel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipsec: Option<IpsecProperties>,
    /// Current lifecycle state of the tunnel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_state: Option<TunnelOperationalState>,
    /// Whether the tunnel is healthy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnel_healthy: Option<bool>,
    /// Whether the tunnel is currently up.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnel_up: Option<bool>,
    /// Internal IP address assigned to the tunnel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnel_internal_ip: Option<String>,
    /// Interval (seconds) between internal peer ping probes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_peer_ping_interval: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// IPSec authentication configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpsecAuthentication {
    /// Authentication method type.
    pub authentication: IPSecAuthType,
    /// Pre-shared key value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psk: Option<String>,
}

/// IPSec tunnel configuration properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpsecProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<IpsecAuthentication>,
    /// Child SA rekey time in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_rekey_time: Option<u32>,
    /// Dead peer detection delay in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpd_delay: Option<u32>,
    /// Dead peer detection timeout in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpd_timeout: Option<u32>,
    /// IKE SA lifetime in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ike_lifetime: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase1_algorithms: Option<Vec<IPSecAlgorithm>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase1_dh_group_numbers: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase1_integrity_algorithms: Option<Vec<IPSecIntegrityAlgorithm>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase2_algorithms: Option<Vec<IPSecAlgorithm>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase2_dh_group_numbers: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase2_integrity_algorithms: Option<Vec<IPSecIntegrityAlgorithm>>,
    /// IKE SA rekey time in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rekey_time: Option<u64>,
}

// ─── Request types ────────────────────────────────────────────────────────────

/// Request body for creating a gateway.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequest {
    pub name: String,
    pub zone: String,
    pub features: Vec<Feature>,
    /// Routers to attach to the gateway.
    pub routers: Vec<RouterInput>,
    /// Desired initial status.
    pub configured_status: ConfiguredStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    /// Initial VPN connections to create.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<CreateConnectionRequest>>,
    /// Addresses to allocate on the gateway.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<AddressInput>>,
    /// Key-value labels to attach.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    /// Whether to automatically allocate internal tunnel IPs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tunnel_internal_ip_allocation: Option<bool>,
}

/// Router reference for gateway creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterInput {
    pub uuid: String,
}

/// Named address input for gateway creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressInput {
    pub name: String,
}

/// Request body for modifying a gateway.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_status: Option<ConfiguredStatus>,
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

/// Request body for creating a VPN connection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConnectionRequest {
    pub name: String,
    /// Type of VPN connection.
    #[serde(rename = "type")]
    pub connection_type: ConnectionType,
    /// Routes on the local (gateway) side.
    #[serde(default)]
    pub local_routes: Vec<RouteInput>,
    /// Routes on the remote (peer) side.
    #[serde(default)]
    pub remote_routes: Vec<RouteInput>,
    /// Tunnels to create within this connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnels: Option<Vec<CreateTunnelRequest>>,
}

/// Route input for connection creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteInput {
    pub name: String,
    /// CIDR network (e.g. `10.0.0.0/24`).
    pub static_network: String,
    /// Route type.
    #[serde(rename = "type")]
    pub route_type: RouteType,
}

/// Request body for creating a VPN tunnel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTunnelRequest {
    pub name: String,
    /// Local (gateway-side) tunnel endpoint address.
    pub local_address: TunnelAddress,
    /// Remote (peer-side) tunnel endpoint address.
    pub remote_address: TunnelAddress,
    /// IPSec configuration for this tunnel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipsec: Option<IpsecProperties>,
    /// Internal IP address to assign to the tunnel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnel_internal_ip: Option<String>,
    /// Interval (seconds) between internal peer ping probes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_peer_ping_interval: Option<u32>,
}

/// Request body for modifying a VPN connection.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyConnectionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_routes: Option<Vec<RouteInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_routes: Option<Vec<RouteInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnels: Option<Vec<CreateTunnelRequest>>,
}

/// Request body for modifying a VPN tunnel.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyTunnelRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_address: Option<TunnelAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_address: Option<TunnelAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipsec: Option<IpsecProperties>,
}
