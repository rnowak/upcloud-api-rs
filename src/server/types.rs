//! Server-related types.

use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, PickFirst, serde_as};

use crate::types::common::YesNo;

// ─── Enums ────────────────────────────────────────────────────────────────────

/// Cloud server state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum State {
    /// Server is running.
    Started,
    /// Server is powered off.
    Stopped,
    /// Server is in maintenance mode.
    Maintenance,
    /// Server is in an error state.
    Error,
}

/// How a server should be stopped or restarted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum StopType {
    /// Graceful ACPI shutdown.
    #[default]
    Soft,
    /// Immediate power-off.
    Hard,
}

/// What to do with storage backups when deleting a server.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeleteStorageBackupsMode {
    /// Keep all backups.
    Keep,
    /// Keep only the latest backup.
    KeepLatest,
    /// Delete all backups.
    Delete,
}

impl std::fmt::Display for DeleteStorageBackupsMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Keep => write!(f, "keep"),
            Self::KeepLatest => write!(f, "keep_latest"),
            Self::Delete => write!(f, "delete"),
        }
    }
}

/// Action to take when a stop or restart operation times out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimeoutAction {
    /// Force-destroy the server after timeout.
    Destroy,
    /// Ignore the timeout and leave the server running.
    Ignore,
}

/// Network interface card model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NicModel {
    /// Intel E1000 (emulated).
    E1000,
    /// VirtIO (paravirtualised, recommended).
    Virtio,
    /// Realtek RTL8139 (emulated).
    #[serde(rename = "rtl8139")]
    Rtl8139,
}

/// Video adapter model for the server console.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VideoModel {
    /// Standard VGA.
    Vga,
    /// Cirrus Logic GD5446.
    Cirrus,
}

/// The method used to create or attach a storage device during server creation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StorageDeviceAction {
    /// Create a new storage device.
    Create,
    /// Clone an existing storage or template.
    Clone,
    /// Attach an existing storage by UUID.
    Attach,
}

/// The device type a storage will be attached as.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StorageDeviceType {
    /// Regular disk device.
    Disk,
    /// CD-ROM device.
    Cdrom,
}

/// Remote console access protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RemoteAccessType {
    /// VNC remote access.
    Vnc,
    /// SPICE remote access.
    Spice,
}

/// How to deliver the server root password to the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PasswordDelivery {
    /// Do not deliver the password.
    None,
    /// Deliver via email.
    Email,
    /// Deliver via SMS.
    Sms,
}

// ─── Sub-types ────────────────────────────────────────────────────────────────

/// A storage device attached to a server.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageDevice {
    /// Device address (e.g. `"virtio:0"`).
    pub address: String,
    /// UUID of the storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
    /// Storage size in GiB.
    #[serde_as(as = "Option<PickFirst<(DisplayFromStr, _)>>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_title: Option<String>,
    /// Storage tier (`"hdd"` or `"maxiops"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_tier: Option<String>,
    /// Device type (`"disk"` or `"cdrom"`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    /// `"1"` if this is the boot disk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_disk: Option<String>,
    /// Whether this storage is included in the server's plan.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_of_plan: Option<bool>,
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// String representation of the encrypted flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

/// IP address assigned to a server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpAddressEntry {
    /// Access type (`"public"` or `"private"`).
    pub access: String,
    pub address: String,
    /// Address family (`"IPv4"` or `"IPv6"`).
    pub family: String,
    /// Whether the IP is included in the server's plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_of_plan: Option<String>,
}

/// Network interface attached to a server.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interface {
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootable: Option<bool>,
    /// Interface index (position order).
    pub index: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<InterfaceIpAddresses>,
    /// MAC address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    /// UUID of the network this interface belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// Whether source IP filtering is enabled.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_filtering: Option<bool>,
    /// Interface type (`"public"`, `"private"`, or `"utility"`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub interface_type: Option<String>,
}

/// Wrapper for IP addresses on a network interface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceIpAddresses {
    #[serde(default)]
    pub ip_address: Vec<InterfaceIpAddress>,
}

/// IP address on a network interface.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceIpAddress {
    pub address: String,
    /// Whether the address was provided by DHCP.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dhcp_provided: Option<bool>,
    /// Address family (`"IPv4"` or `"IPv6"`).
    pub family: String,
    /// Whether this is a floating IP.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floating: Option<bool>,
    /// When the IP is released (`"delete"` or `"unassign"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_policy: Option<String>,
}

/// Login user configuration for server creation.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoginUser {
    /// Set to `1` to auto-generate a password for the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_password: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_keys: Option<SshKeys>,
}

/// Wrapper for a list of SSH public keys.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshKeys {
    #[serde(default)]
    pub ssh_key: Vec<String>,
}

/// Summary GPU information for a server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub model: String,
    /// Number of GPUs of this model.
    pub count: u32,
}

/// Detailed GPU information returned by the `GET /server/{uuid}/gpus` endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuDetail {
    pub model: String,
    pub cores: u64,
    pub firmware: String,
    pub memory_gib: u64,
    pub part: String,
    pub serial: String,
}

// ─── Device types (GPU/PCIe) ──────────────────────────────────────────────────

/// Wrapper for a list of devices (GPU/PCIe) on a server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerDevices {
    #[serde(default)]
    pub device: Vec<ServerDevice>,
}

/// A device (GPU or PCIe) attached to a server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerDevice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Device type (e.g. `"gpu"`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ServerDeviceMetadata>,
}

/// Hardware metadata for a server device.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerDeviceMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    /// Number of GPU cores.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cores: Option<u64>,
    /// Device memory in MiB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<u64>,
}

// ─── Server struct (basic fields returned by list) ────────────────────────────

/// Cloud server summary returned by the list endpoint.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub uuid: String,
    pub hostname: String,
    pub title: String,
    pub state: State,
    /// Data center zone (e.g. `"de-fra1"`).
    pub zone: String,
    /// Number of CPU cores.
    #[serde_as(as = "PickFirst<(DisplayFromStr, _)>")]
    pub core_number: u32,
    /// RAM in MiB.
    #[serde_as(as = "PickFirst<(DisplayFromStr, _)>")]
    pub memory_amount: u64,
    /// Preconfigured plan name, or `"custom"`.
    pub plan: String,
    /// Licensing cost (per-hour).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<f64>,
    /// Provisioning progress percentage (0–100).
    #[serde_as(as = "Option<PickFirst<(DisplayFromStr, _)>>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<ServerTags>,
}

// ─── ServerDetails struct (full detail returned by get/create/modify/actions) ─

/// Full cloud server details returned by get, create, and modify endpoints.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerDetails {
    /// Base server fields (flattened).
    #[serde(flatten)]
    pub server: Server,
    /// Firewall state (`"on"` or `"off"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall: Option<String>,
    /// Boot device order (e.g. `"disk"`, `"cdrom"`, `"network"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_order: Option<String>,
    /// Private cloud host ID, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nic_model: Option<NicModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_model: Option<VideoModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// UUID of the server group this server belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<ServerIpAddresses>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_devices: Option<ServerStorageDevices>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networking: Option<ServerNetworking>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<crate::types::common::Labels>,
    /// Whether the metadata service is enabled (`"yes"` or `"no"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    /// Whether remote access is enabled (`"yes"` or `"no"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_enabled: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_password: Option<String>,
    /// Remote access hostname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_host: Option<String>,
    /// Remote access port number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_port: Option<String>,
    /// Simple backup schedule string (e.g. `"0300,dailies"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_backup: Option<String>,
    /// Server creation timestamp (Unix epoch).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    /// Outbound IPv4 transfer included in plan (bytes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_ipv4_bytes: Option<String>,
    /// Outbound IPv6 transfer included in plan (bytes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_ipv6_bytes: Option<String>,
    /// Attached GPU/PCIe devices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<ServerDevices>,
    /// Generated root password (only in create responses).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl std::ops::Deref for ServerDetails {
    type Target = Server;
    fn deref(&self) -> &Server {
        &self.server
    }
}

/// Wrapper for a server's IP address list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerIpAddresses {
    #[serde(default)]
    pub ip_address: Vec<IpAddressEntry>,
}

/// Wrapper for a server's storage device list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStorageDevices {
    #[serde(default)]
    pub storage_device: Vec<StorageDevice>,
}

/// Wrapper for a server's tag list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerTags {
    #[serde(default)]
    pub tag: Vec<String>,
}

/// Server networking configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerNetworking {
    pub interfaces: ServerInterfaces,
}

/// Wrapper for a list of network interfaces.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInterfaces {
    #[serde(default)]
    pub interface: Vec<Interface>,
}

// ─── Request types ────────────────────────────────────────────────────────────

/// Request to relocate a server to a different zone.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelocateRequest {
    /// Target data center zone.
    pub zone: String,
}

/// Request to create a new cloud server.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequest {
    /// Data center zone (e.g. `"de-fra1"`).
    pub zone: String,
    /// Fully qualified domain name.
    pub hostname: String,
    pub title: String,
    /// Preconfigured plan name (alternative to `core_number` + `memory_amount`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    /// Number of CPU cores (alternative to `plan`).
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_number: Option<u32>,
    /// RAM in MiB (alternative to `plan`).
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_amount: Option<u64>,
    /// Boot device order (e.g. `"disk"`, `"cdrom"`, `"network"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_devices: Option<CreateServerStorageDevices>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networking: Option<CreateServerNetworking>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_user: Option<LoginUser>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_delivery: Option<PasswordDelivery>,
    /// Firewall state (`"on"` or `"off"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall: Option<String>,
    /// Enable the metadata service (`"yes"` or `"no"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nic_model: Option<NicModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_model: Option<VideoModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// UUID of a server group to join.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group: Option<String>,
    /// Simple backup schedule string (e.g. `"0300,dailies"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_backup: Option<String>,
    /// Base64-encoded or plaintext cloud-init user data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<crate::types::common::Labels>,
    /// Host ID to avoid for scheduling (private cloud).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avoid_host: Option<i64>,
    /// Preferred host ID for scheduling (private cloud).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<i64>,
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_type: Option<RemoteAccessType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_password: Option<String>,
}

/// Storage device definition for server creation.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateServerStorageDevice {
    pub action: StorageDeviceAction,
    /// UUID of the storage to clone or attach.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
    /// Size in GiB (required when `action` is `Create`).
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Storage tier (e.g. `"maxiops"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<StorageDeviceType>,
    /// Device address (e.g. `"virtio:0"`); auto-assigned if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// `"1"` to mark as the boot disk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_disk: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_rule: Option<crate::storage::BackupRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

/// Wrapper for storage devices in a server create request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateServerStorageDevices {
    pub storage_device: Vec<CreateServerStorageDevice>,
}

/// Networking configuration for server creation.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateServerNetworking {
    pub interfaces: CreateServerInterfaces,
}

/// Wrapper for interfaces in a server create request.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateServerInterfaces {
    #[serde(default)]
    pub interface: Vec<CreateServerInterface>,
}

/// Network interface definition for server creation.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateServerInterface {
    /// Interface type (`"public"`, `"private"`, or `"utility"`).
    #[serde(rename = "type")]
    pub interface_type: String,
    /// UUID of an existing network (required for private interfaces).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootable: Option<bool>,
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_filtering: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<CreateServerInterfaceIpAddresses>,
}

/// Wrapper for IP addresses in a create-server interface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateServerInterfaceIpAddresses {
    #[serde(default)]
    pub ip_address: Vec<CreateServerInterfaceIpAddress>,
}

/// IP address definition for a create-server interface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateServerInterfaceIpAddress {
    /// Address family (`"IPv4"` or `"IPv6"`).
    pub family: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Whether the address should be assigned via DHCP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dhcp_provided: Option<String>,
}

/// Request to modify an existing cloud server.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    /// Number of CPU cores.
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_number: Option<u32>,
    /// RAM in MiB.
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_order: Option<String>,
    /// Firewall state (`"on"` or `"off"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall: Option<String>,
    /// Whether the metadata service is enabled (`"yes"` or `"no"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nic_model: Option<NicModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_model: Option<VideoModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_backup: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_enabled: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_type: Option<RemoteAccessType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_password: Option<String>,
}

/// Request to stop a server.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StopRequest {
    pub stop_type: StopType,
    /// Timeout in seconds before the `timeout_action` is taken.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

/// Request to restart a server.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RestartRequest {
    pub stop_type: StopType,
    /// Timeout in seconds before the `timeout_action` is taken.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_action: Option<TimeoutAction>,
    /// Host ID to restart on (private cloud).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<i64>,
}

/// Request to attach a storage device to a server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachStorageRequest {
    /// UUID of the storage to attach.
    pub storage: String,
    /// Device type (`"disk"` or `"cdrom"`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    /// Target device address (e.g. `"virtio:1"`); auto-assigned if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

/// Request to detach a storage device from a server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetachStorageRequest {
    /// Device address to detach (e.g. `"virtio:0"`).
    pub address: String,
}

/// Request to load a storage as a CD-ROM on a server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadCdromRequest {
    /// UUID of the CD-ROM storage to load.
    pub storage: String,
}

/// Request to rebuild a server from a clone source.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebuildRequest {
    /// UUID of the storage to clone as the new boot disk.
    pub clone_source: String,
    /// Title for the newly cloned storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_title: Option<String>,
    /// Address of the disk to detach during rebuild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_disk: Option<String>,
    /// Whether to delete the detached disk.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_detached_disk: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_delivery: Option<String>,
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_user: Option<LoginUser>,
}

// ─── Server Group types ───────────────────────────────────────────────────────

/// Server group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub uuid: String,
    pub title: String,
    /// Anti-affinity policy (`"yes"`, `"no"`, or `"strict"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_affinity: Option<String>,
    /// Per-server anti-affinity status within the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_affinity_status: Option<Vec<AntiAffinityStatus>>,
    /// Server UUIDs belonging to this group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<GroupServers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<crate::types::common::Labels>,
}

/// Anti-affinity status for a server in a group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiAffinityStatus {
    /// Server UUID.
    pub uuid: String,
    /// Status string (e.g. `"met"` or `"unmet"`).
    pub status: String,
}

/// Wrapper for server UUIDs in a group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupServers {
    #[serde(default)]
    pub server: Vec<String>,
}

/// Request to create a server group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGroupRequest {
    pub title: String,
    /// Anti-affinity policy (`"yes"`, `"no"`, or `"strict"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_affinity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<crate::types::common::Labels>,
    /// Initial server UUIDs to add to the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<GroupServers>,
}

/// Request to modify a server group.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyGroupRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_affinity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<crate::types::common::Labels>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<GroupServers>,
}

// ─── List / filter parameters ─────────────────────────────────────────────────

/// Sort field for server list ordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderBy {
    /// Sort by title.
    Title,
    /// Sort by data center zone.
    Zone,
    /// Sort by total storage size.
    Size,
    /// Sort by number of CPU cores.
    Cores,
    /// Sort by memory amount.
    Memory,
    /// Sort by popularity.
    Popularity,
}

impl std::fmt::Display for OrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Title => write!(f, "title"),
            Self::Zone => write!(f, "zone"),
            Self::Size => write!(f, "size"),
            Self::Cores => write!(f, "cores"),
            Self::Memory => write!(f, "memory"),
            Self::Popularity => write!(f, "popularity"),
        }
    }
}

/// Query parameters for filtering, ordering, and paginating the server list.
///
/// # Examples
///
/// ```
/// use upcloud_api_rs::server::{ListParams, State, OrderBy};
/// use upcloud_api_rs::types::common::SortOrder;
///
/// let params = ListParams::new()
///     .state(State::Started)
///     .label("env", "prod")
///     .search("web")
///     .order_by(OrderBy::Title, SortOrder::Asc)
///     .limit(25);
/// ```
#[derive(Debug, Clone, Default)]
pub struct ListParams {
    state: Option<State>,
    labels: Vec<String>,
    tags: Vec<String>,
    search: Option<String>,
    hosts: Vec<i64>,
    uuids: Vec<String>,
    order_by: Option<OrderBy>,
    sort_order: Option<crate::types::common::SortOrder>,
    limit: Option<u32>,
    offset: Option<u32>,
}

impl ListParams {
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by server state (e.g. `State::Started`).
    pub fn state(mut self, state: State) -> Self {
        self.state = Some(state);
        self
    }

    /// Add a label filter. Each call adds `key=value` as a separate `?label=` parameter.
    /// Multiple labels are AND-combined by the API.
    pub fn label(mut self, key: &str, value: &str) -> Self {
        self.labels.push(format!("{key}={value}"));
        self
    }

    /// Add a tag filter. Multiple tags can be specified to match servers having any of them.
    pub fn tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }

    /// Full-text search across server name, title, UUID, zone, IP, hostname, and tags.
    pub fn search(mut self, search: &str) -> Self {
        self.search = Some(search.to_string());
        self
    }

    /// Filter by private cloud host ID. Multiple host IDs are OR-combined.
    pub fn host(mut self, host_id: i64) -> Self {
        self.hosts.push(host_id);
        self
    }

    /// Filter by server UUID. Multiple UUIDs are OR-combined.
    pub fn uuid(mut self, uuid: &str) -> Self {
        self.uuids.push(uuid.to_string());
        self
    }

    /// Sort results by the given field and direction.
    pub fn order_by(mut self, field: OrderBy, direction: crate::types::common::SortOrder) -> Self {
        self.order_by = Some(field);
        self.sort_order = Some(direction);
        self
    }

    /// Maximum number of results to return.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Offset from the start of the result set.
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Returns `true` if no filters, ordering, or pagination are set.
    pub fn is_empty(&self) -> bool {
        self.state.is_none()
            && self.labels.is_empty()
            && self.tags.is_empty()
            && self.search.is_none()
            && self.hosts.is_empty()
            && self.uuids.is_empty()
            && self.order_by.is_none()
            && self.limit.is_none()
            && self.offset.is_none()
    }

    /// Encode as query parameter key-value pairs suitable for `reqwest::RequestBuilder::query`.
    pub(crate) fn to_query_pairs(&self) -> Vec<(&str, String)> {
        let mut pairs = Vec::new();

        if let Some(state) = &self.state {
            pairs.push(("state", format!("{state:?}").to_lowercase()));
        }
        for label in &self.labels {
            pairs.push(("label", label.clone()));
        }
        if !self.tags.is_empty() {
            pairs.push(("tag", self.tags.join(",")));
        }
        if let Some(search) = &self.search {
            pairs.push(("search", search.clone()));
        }
        for host in &self.hosts {
            pairs.push(("host", host.to_string()));
        }
        for uuid in &self.uuids {
            pairs.push(("uuid", uuid.clone()));
        }
        if let Some(order_by) = &self.order_by {
            pairs.push(("order_by", order_by.to_string()));
        }
        if let Some(sort_order) = &self.sort_order {
            pairs.push(("sort_by", sort_order.to_string()));
        }
        if let Some(limit) = self.limit {
            pairs.push(("limit", limit.to_string()));
        }
        if let Some(offset) = self.offset {
            pairs.push(("offset", offset.to_string()));
        }

        pairs
    }
}

// ─── Device Availability ──────────────────────────────────────────────────────

/// Availability of devices (e.g. GPUs) across zones.
///
/// The outer map key is the zone ID (e.g. `"fi-hel1"`).
pub type DevicesAvailability = std::collections::HashMap<String, Devices>;

/// Available device plans within a zone.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Devices {
    /// GPU plans keyed by plan name (e.g. `"NVIDIA A100"`).
    #[serde(default)]
    pub gpu_plans: std::collections::HashMap<String, DeviceAvailability>,
}

/// Availability count for a single device plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceAvailability {
    #[serde(default)]
    pub amount: i64,
}
