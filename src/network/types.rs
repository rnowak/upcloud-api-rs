//! Public types for Networks, Routers, and Network Peerings.

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::types::common::YesNo;

// ─── Network types ────────────────────────────────────────────────────────────

/// The type of an UpCloud SDN network.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NetworkType {
    /// Public internet network.
    Public,
    /// UpCloud utility network (shared private).
    Utility,
    /// Private SDN network.
    Private,
}

/// An IP network (subnet) within an SDN network.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IpNetwork {
    /// CIDR notation address (e.g. `"10.0.0.0/24"`).
    pub address: String,
    /// Whether DHCP is enabled (`"yes"` or `"no"`).
    #[serde(default)]
    pub dhcp: Option<String>,
    /// Default route provided via DHCP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dhcp_default_route: Option<String>,
    /// DNS servers provided via DHCP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dhcp_dns: Option<Vec<String>>,
    /// Static routes provided via DHCP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dhcp_routes: Option<Vec<String>>,
    /// DHCP routes configuration as free-form JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dhcp_routes_configuration: Option<serde_json::Value>,
    /// Effective routes computed from DHCP and static configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dhcp_effective_routes: Option<Vec<serde_json::Value>>,
    /// DHCP boot file URL for PXE booting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dhcp_bootfile_url: Option<String>,
    /// Address family (`"IPv4"` or `"IPv6"`).
    pub family: String,
    /// Gateway address for this subnet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
}

/// Wrapper for a list of IP networks within a network.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NetworkIpNetworks {
    /// IP network (subnet) definitions.
    #[serde(default)]
    pub ip_network: Vec<IpNetwork>,
}

/// A server attached to a network.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NetworkServer {
    /// IP address assigned to the server on this network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// Server title (display name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Unique identifier for the server.
    pub uuid: String,
}

/// Wrapper for a list of servers attached to a network.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NetworkServers {
    /// Servers attached to the network.
    #[serde(default)]
    pub server: Vec<NetworkServer>,
}

/// Summary of a network peering associated with a network.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NetworkPeeringSummary {
    /// Unique identifier for the peering.
    pub uuid: String,
    /// Peering name.
    pub name: String,
    /// Current peering state.
    #[serde(default)]
    pub state: Option<String>,
}

/// Wrapper for a list of network peering summaries.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct NetworkPeerings {
    /// Peerings associated with the network.
    #[serde(default)]
    pub peering: Vec<NetworkPeeringSummary>,
}

/// An UpCloud SDN network.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Network {
    /// Unique identifier for the network.
    pub uuid: String,
    /// Network name.
    pub name: String,
    /// Network type (public, utility, or private).
    #[serde(rename = "type")]
    pub network_type: NetworkType,
    /// Zone where the network exists (e.g. `"fi-hel1"`).
    pub zone: String,
    /// IP networks (subnets) within this network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_networks: Option<NetworkIpNetworks>,
    /// Servers attached to this network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<NetworkServers>,
    /// Key-value labels on the network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    /// UUID of the router attached to this network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router: Option<String>,
    /// Effective routes computed for this network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_routes: Option<Vec<serde_json::Value>>,
    /// Network peerings associated with this network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peerings: Option<NetworkPeerings>,
}

/// Request to create a new SDN network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNetworkRequest {
    /// Network name.
    pub name: String,
    /// Zone to create the network in (e.g. `"fi-hel1"`).
    pub zone: String,
    /// IP networks (subnets) to configure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_networks: Option<NetworkIpNetworks>,
    /// UUID of a router to attach to the network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router: Option<String>,
    /// Key-value labels on the network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

/// Request to modify an existing SDN network.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyNetworkRequest {
    /// Network name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Zone where the network exists (e.g. `"fi-hel1"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
    /// Updated IP networks (subnets).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_networks: Option<NetworkIpNetworks>,
    /// Key-value labels on the network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    /// UUID of a router to attach (or detach by omitting).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub router: Option<String>,
}

// ─── Server interface types ───────────────────────────────────────────────────

/// Request to create a new network interface on a server.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInterfaceRequest {
    /// Interface type (e.g. `"public"`, `"private"`, `"utility"`).
    #[serde(rename = "type")]
    pub interface_type: String,
    /// UUID of the network to attach to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// Interface index (position in the server's NIC list).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u32>,
    /// Whether this interface is the boot interface.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootable: Option<bool>,
    /// Whether source IP filtering is enabled.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_filtering: Option<bool>,
    /// IP addresses to assign to the interface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<InterfaceIpAddressesInput>,
}

/// Wrapper for IP address inputs when creating an interface.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InterfaceIpAddressesInput {
    /// IP address assignments.
    pub ip_address: Vec<InterfaceIpAddressInput>,
}

/// An IP address input for a server network interface.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InterfaceIpAddressInput {
    /// Address family (`"IPv4"` or `"IPv6"`).
    pub family: String,
    /// Specific IP address to assign, or omit for automatic assignment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

/// Request to modify an existing server network interface.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyInterfaceRequest {
    /// Whether this interface is the boot interface.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootable: Option<bool>,
    /// Whether source IP filtering is enabled.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_filtering: Option<bool>,
}

/// Request to assign an IP address to a server interface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignInterfaceIpRequest {
    /// Address family (`"IPv4"` or `"IPv6"`).
    pub family: String,
    /// Specific IP address to assign, or omit for automatic assignment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

// ─── Router types ─────────────────────────────────────────────────────────────

/// Type of a static route on a router.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RouterStaticRouteType {
    /// System-managed service route.
    Service,
    /// User-defined static route.
    User,
}

/// A static route entry on a router.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StaticRoute {
    /// Route name.
    pub name: Option<String>,
    /// Destination CIDR (e.g. `"10.0.0.0/24"`).
    pub route: String,
    /// Next-hop IP address.
    pub nexthop: String,
    /// Route type (service or user).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub route_type: Option<RouterStaticRouteType>,
}

/// A network reference attached to a router.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RouterNetwork {
    /// Network UUID.
    pub uuid: String,
}

/// Wrapper for a list of networks attached to a router.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RouterNetworkSlice {
    /// Networks attached to the router.
    #[serde(default)]
    pub network: Vec<RouterNetwork>,
}

/// An UpCloud router for connecting SDN networks.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Router {
    /// Unique identifier for the router.
    pub uuid: String,
    /// Router name.
    pub name: String,
    /// Router type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub router_type: Option<String>,
    /// Networks attached to this router.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_networks: Option<RouterNetworkSlice>,
    /// Static routes configured on this router.
    #[serde(default)]
    pub static_routes: Vec<StaticRoute>,
    /// Key-value labels on the router.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

/// Request to create a new router.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRouterRequest {
    /// Router name.
    pub name: String,
    /// Static routes to configure on the router.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub static_routes: Vec<StaticRoute>,
    /// Key-value labels on the router.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

/// Request to modify an existing router.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyRouterRequest {
    /// Router name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated static routes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_routes: Option<Vec<StaticRoute>>,
    /// Key-value labels on the router.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

// ─── Network Peering types ────────────────────────────────────────────────────

/// State of a network peering connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PeeringState {
    /// Peering is active and routing traffic.
    Active,
    /// Waiting for the peer to accept the peering.
    PendingPeer,
    /// Peering is being provisioned.
    Provisioning,
    /// Peering has been disabled.
    Disabled,
    /// Peering is in an error state.
    Error,
    /// Subnet conflict between peered networks.
    ConflictSubnet,
    /// Local network is missing its router.
    MissingLocalRouter,
    /// Peer network is missing its router.
    MissingPeerRouter,
    /// Peer network has been deleted.
    DeletedPeerNetwork,
    /// Peer side has disabled the peering.
    PeerDisabled,
}

/// Configured (desired) status of a network peering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PeeringConfiguredStatus {
    /// Peering should be active.
    Active,
    /// Peering should be disabled.
    Disabled,
}

/// A network reference in a peering, with optional IP network details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PeeringNetwork {
    /// Network UUID.
    pub uuid: String,
    /// IP networks (subnets) exposed through the peering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_networks: Option<NetworkIpNetworks>,
}

/// A network peering between two UpCloud SDN networks.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NetworkPeering {
    /// Unique identifier for the peering.
    pub uuid: String,
    /// Peering name.
    pub name: String,
    /// Desired status of the peering.
    pub configured_status: PeeringConfiguredStatus,
    /// Current state of the peering.
    pub state: PeeringState,
    /// Local network in the peering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<PeeringNetwork>,
    /// Remote (peer) network in the peering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_network: Option<PeeringNetwork>,
    /// Key-value labels on the peering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

/// Request to create a new network peering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePeeringRequest {
    /// Peering name.
    pub name: String,
    /// Desired initial status of the peering.
    pub configured_status: Option<PeeringConfiguredStatus>,
    /// Local network for the peering.
    pub network: PeeringNetworkInput,
    /// Remote (peer) network for the peering.
    pub peer_network: PeeringNetworkInput,
    /// Key-value labels on the peering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

/// Network UUID input for creating a peering.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PeeringNetworkInput {
    /// Network UUID.
    pub uuid: String,
}

/// Request to modify an existing network peering.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyPeeringRequest {
    /// Peering name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Desired status of the peering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_status: Option<PeeringConfiguredStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

/// IP network address family for network peering filters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkPeeringIPNetworkFamily {
    /// IPv4 address family.
    IPv4,
    /// IPv6 address family.
    IPv6,
}
