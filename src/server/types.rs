//! Server-related types.

use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, PickFirst, serde_as};

pub use crate::types::common::{Family, NetworkType, OnOff};
use crate::types::common::YesNo;

// ─── Enums ────────────────────────────────────────────────────────────────────

/// Cloud server state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum State {
    /// The server is running.
    Started,
    /// The server is stopped.
    Stopped,
    /// The server is in maintenance mode.
    Maintenance,
    /// The server has encountered an error and is inaccessible.
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

/// Boot device order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BootOrder {
    Disk,
    Cdrom,
    Network,
    #[serde(rename = "disk,cdrom")]
    DiskCdrom,
    #[serde(rename = "cdrom,disk")]
    CdromDisk,
}

/// Server group anti-affinity policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AntiAffinityPolicy {
    Yes,
    No,
    Strict,
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
#[non_exhaustive]
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
    /// Title of the storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_title: Option<String>,
    /// Storage tier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_tier: Option<crate::storage::Tier>,
    /// Device type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<StorageDeviceType>,
    /// `"1"` if this is the boot disk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_disk: Option<String>,
    /// Whether this storage is included in the server's plan.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_of_plan: Option<bool>,
    /// Whether the storage is encrypted at rest.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// String representation of the encrypted flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<String>,
    /// Labels attached to the storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

/// IP address assigned to a server.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IpAddressEntry {
    /// Access type.
    pub access: NetworkType,
    /// IP address string.
    pub address: String,
    /// Address family.
    pub family: Family,
    /// Whether the IP is included in the server's plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_of_plan: Option<String>,
}

/// Network interface attached to a server.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Interface {
    /// Whether to try booting through the interface.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootable: Option<bool>,
    /// Interface index (position order).
    pub index: u32,
    /// IP addresses assigned to this interface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<InterfaceIpAddresses>,
    /// MAC address of the interface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    /// UUID of the network this interface belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// Whether source IP filtering is enabled.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_filtering: Option<bool>,
    /// Interface type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub interface_type: Option<NetworkType>,
}

/// Wrapper for IP addresses on a network interface.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InterfaceIpAddresses {
    #[serde(default)]
    pub ip_address: Vec<InterfaceIpAddress>,
}

/// IP address on a network interface.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InterfaceIpAddress {
    /// IP address string.
    pub address: String,
    /// Whether the address was provided by DHCP.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dhcp_provided: Option<bool>,
    /// Address family.
    pub family: Family,
    /// Whether this is a floating IP.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floating: Option<bool>,
    /// IP release policy (e.g. `"release"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_policy: Option<String>,
}

/// Login user configuration for server creation.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct LoginUser {
    /// Whether to auto-generate a one-time password (`"yes"` or `"no"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_password: Option<u8>,
    /// Login username (defaults to `"root"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// SSH public keys for the login user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_keys: Option<SshKeys>,
}

/// Wrapper for a list of SSH public keys.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SshKeys {
    #[serde(default)]
    pub ssh_key: Vec<String>,
}

/// Summary GPU information for a server.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GpuInfo {
    pub model: String,
    /// Number of GPUs of this model.
    pub count: u32,
}

/// Detailed GPU information returned by the `GET /server/{uuid}/gpus` endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GpuDetail {
    /// GPU model name.
    pub model: String,
    /// Number of GPU cores.
    pub cores: u64,
    /// GPU firmware version.
    pub firmware: String,
    /// GPU memory in GiB.
    pub memory_gib: u64,
    /// GPU part number.
    pub part: String,
    /// GPU serial number.
    pub serial: String,
}

// ─── Device types (GPU/PCIe) ──────────────────────────────────────────────────

/// Wrapper for a list of devices (GPU/PCIe) on a server.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ServerDevices {
    #[serde(default)]
    pub device: Vec<ServerDevice>,
}

/// A device (GPU or PCIe) attached to a server.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ServerDevice {
    /// Device model name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Device type (e.g. `"gpu"`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    /// Device serial number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    /// Hardware metadata for the device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ServerDeviceMetadata>,
}

/// Hardware metadata for a server device.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ServerDeviceMetadata {
    /// Device serial number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    /// Number of GPU cores.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cores: Option<u64>,
    /// Device memory in GiB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<u64>,
}

// ─── Server struct (basic fields returned by list) ────────────────────────────

/// Cloud server summary returned by the list endpoint.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Server {
    /// Unique server identifier.
    pub uuid: String,
    /// Server hostname (FQDN).
    pub hostname: String,
    /// Short descriptive title.
    pub title: String,
    /// Current server state.
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
    /// Sum of all attached storages' license costs (credits/hour/core).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<f64>,
    /// Provisioning progress percentage (0–100).
    #[serde_as(as = "Option<PickFirst<(DisplayFromStr, _)>>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<u32>,
    /// Server tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<ServerTags>,
}

// ─── ServerDetails struct (full detail returned by get/create/modify/actions) ─

/// Full cloud server details returned by get, create, and modify endpoints.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ServerDetails {
    /// Base server fields (flattened).
    #[serde(flatten)]
    pub server: Server,
    /// Firewall state.
    #[serde_as(as = "Option<OnOff>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall: Option<bool>,
    /// Boot device order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_order: Option<BootOrder>,
    /// Private cloud host ID, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<i64>,
    /// Network interface card model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nic_model: Option<NicModel>,
    /// Video adapter model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_model: Option<VideoModel>,
    /// Server timezone identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// UUID of the server group this server belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group: Option<String>,
    /// IP addresses assigned to the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<ServerIpAddresses>,
    /// Storage devices attached to the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_devices: Option<ServerStorageDevices>,
    /// Network interface configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networking: Option<ServerNetworking>,
    /// Labels assigned to the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<crate::types::common::Labels>,
    /// Whether the metadata service is enabled.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<bool>,
    /// Whether remote access is enabled.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_enabled: Option<bool>,
    /// Remote access type (e.g. `"vnc"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_type: Option<String>,
    /// Remote access password.
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
#[non_exhaustive]
pub struct ServerIpAddresses {
    #[serde(default)]
    pub ip_address: Vec<IpAddressEntry>,
}

/// Wrapper for a server's storage device list.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ServerStorageDevices {
    #[serde(default)]
    pub storage_device: Vec<StorageDevice>,
}

/// Wrapper for a server's tag list.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ServerTags {
    #[serde(default)]
    pub tag: Vec<String>,
}

/// Server networking configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ServerNetworking {
    pub interfaces: ServerInterfaces,
}

/// Wrapper for a list of network interfaces.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
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
    /// Short descriptive title for the server.
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
    /// Boot device order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_order: Option<BootOrder>,
    /// Storage devices to attach during creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_devices: Option<CreateServerStorageDevices>,
    /// Network interface configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networking: Option<CreateServerNetworking>,
    /// Login user configuration (username, SSH keys).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_user: Option<LoginUser>,
    /// How to deliver the root password (`none`/`email`/`sms`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_delivery: Option<PasswordDelivery>,
    /// Firewall state.
    #[serde_as(as = "Option<OnOff>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall: Option<bool>,
    /// Whether the metadata service is enabled.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<bool>,
    /// Network interface card model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nic_model: Option<NicModel>,
    /// Video adapter model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_model: Option<VideoModel>,
    /// Server timezone identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// UUID of a server group to join.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group: Option<String>,
    /// Simple backup schedule string (e.g. `"0300,dailies"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_backup: Option<String>,
    /// Cloud-init user data (URL or script body).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    /// Labels to assign to the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<crate::types::common::Labels>,
    /// Host ID to avoid for scheduling (private cloud).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avoid_host: Option<i64>,
    /// Preferred host ID for scheduling (private cloud).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<i64>,
    /// Whether to enable remote access.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_enabled: Option<bool>,
    /// Remote access type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_type: Option<RemoteAccessType>,
    /// Remote access password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_password: Option<String>,
}

/// Storage device definition for server creation.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateServerStorageDevice {
    /// Method to create/attach storage (`create`/`clone`/`attach`).
    pub action: StorageDeviceAction,
    /// UUID of the storage to clone or attach.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
    /// Size in GiB (required when `action` is `Create`).
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    /// Short descriptive title for the storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Storage tier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<crate::storage::Tier>,
    /// Device type (`disk` or `cdrom`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<StorageDeviceType>,
    /// Device address (e.g. `"virtio:0"`); auto-assigned if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Whether to encrypt the storage at rest.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// `"1"` to mark as the boot disk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_disk: Option<String>,
    /// Backup rule for the storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_rule: Option<crate::storage::BackupRule>,
    /// Labels to assign to the storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

/// Wrapper for storage devices in a server create request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateServerStorageDevices {
    pub storage_device: Vec<CreateServerStorageDevice>,
}

/// Networking configuration for server creation.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct CreateServerNetworking {
    pub interfaces: CreateServerInterfaces,
}

/// Wrapper for interfaces in a server create request.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct CreateServerInterfaces {
    #[serde(default)]
    pub interface: Vec<CreateServerInterface>,
}

/// Network interface definition for server creation.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateServerInterface {
    /// Interface type.
    #[serde(rename = "type")]
    pub interface_type: NetworkType,
    /// UUID of an existing network (required for private interfaces).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// Whether to try booting through this interface.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootable: Option<bool>,
    /// Whether source IP filtering is enabled.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_filtering: Option<bool>,
    /// IP addresses to assign to the interface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<CreateServerInterfaceIpAddresses>,
}

/// Wrapper for IP addresses in a create-server interface.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateServerInterfaceIpAddresses {
    #[serde(default)]
    pub ip_address: Vec<CreateServerInterfaceIpAddress>,
}

/// IP address definition for a create-server interface.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateServerInterfaceIpAddress {
    /// Address family.
    pub family: Family,
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
    /// New hostname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// New title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// New pricing plan.
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
    /// Boot device order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_order: Option<BootOrder>,
    /// Firewall state.
    #[serde_as(as = "Option<OnOff>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall: Option<bool>,
    /// Whether the metadata service is enabled.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<bool>,
    /// Network interface card model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nic_model: Option<NicModel>,
    /// Video adapter model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_model: Option<VideoModel>,
    /// Server timezone identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Simple backup schedule string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_backup: Option<String>,
    /// Labels to assign to the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<crate::types::common::Labels>,
    /// Whether to enable remote access.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_enabled: Option<bool>,
    /// Remote access type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_type: Option<RemoteAccessType>,
    /// Remote access password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_password: Option<String>,
}

/// Request to start a stopped server.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StartRequest {
    /// Host ID to avoid for scheduling (private cloud).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avoid_host: Option<i64>,
    /// Preferred host ID for scheduling (private cloud).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<i64>,
}

/// Request to stop a server.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StopRequest {
    /// Stop type (`soft` or `hard`).
    pub stop_type: StopType,
    /// Timeout in seconds before the `timeout_action` is taken.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

/// Request to restart a server.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RestartRequest {
    /// Stop type (`soft` or `hard`).
    pub stop_type: StopType,
    /// Timeout in seconds before the `timeout_action` is taken.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    /// Action to take when timeout expires (`destroy` or `ignore`).
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
    /// Device type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<StorageDeviceType>,
    /// Target device address (e.g. `"virtio:1"`); auto-assigned if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// `"0"` or `"1"` — set to `"1"` to mark this as the boot disk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_disk: Option<String>,
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
    /// UUID of the disk to detach during rebuild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_disk: Option<String>,
    /// Whether to delete the detached disk.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_detached_disk: Option<bool>,
    /// Password delivery method (`none`/`email`/`sms`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_delivery: Option<String>,
    /// Whether to encrypt the new storage.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// Login user configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_user: Option<LoginUser>,
}

// ─── Server Group types ───────────────────────────────────────────────────────

/// Server group.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Group {
    /// Unique server group identifier.
    pub uuid: String,
    /// Server group title.
    pub title: String,
    /// Anti-affinity policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_affinity: Option<AntiAffinityPolicy>,
    /// Per-server anti-affinity status within the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_affinity_status: Option<Vec<AntiAffinityStatus>>,
    /// Server UUIDs belonging to this group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<GroupServers>,
    /// Labels assigned to the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<crate::types::common::Labels>,
}

/// Anti-affinity status for a server in a group.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AntiAffinityStatus {
    /// Server UUID.
    pub uuid: String,
    /// Status string (e.g. `"met"` or `"unmet"`).
    pub status: String,
}

/// Wrapper for server UUIDs in a group.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GroupServers {
    #[serde(default)]
    pub server: Vec<String>,
}

/// Request to create a server group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGroupRequest {
    /// Server group title.
    pub title: String,
    /// Anti-affinity policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_affinity: Option<AntiAffinityPolicy>,
    /// Labels to assign to the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<crate::types::common::Labels>,
    /// Initial server UUIDs to add to the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<GroupServers>,
}

/// Request to modify a server group.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyGroupRequest {
    /// New group title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Anti-affinity policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_affinity: Option<AntiAffinityPolicy>,
    /// Labels to assign to the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<crate::types::common::Labels>,
    /// Server UUIDs to set in the group.
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
/// use upcloud_api::server::{ListParams, State, OrderBy};
/// use upcloud_api::types::common::SortOrder;
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
#[non_exhaustive]
pub struct Devices {
    /// GPU plans keyed by plan name (e.g. `"NVIDIA A100"`).
    #[serde(default)]
    pub gpu_plans: std::collections::HashMap<String, DeviceAvailability>,
}

/// Availability count for a single device plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DeviceAvailability {
    #[serde(default)]
    pub amount: i64,
}
