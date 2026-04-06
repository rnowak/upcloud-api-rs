//! Server-related types.

use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, PickFirst, serde_as};

use crate::types::common::YesNo;

// ─── Enums ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum State {
    Started,
    Stopped,
    Maintenance,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum StopType {
    #[default]
    Soft,
    Hard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeleteStorageBackupsMode {
    Keep,
    KeepLatest,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimeoutAction {
    Destroy,
    Ignore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NicModel {
    E1000,
    Virtio,
    #[serde(rename = "rtl8139")]
    Rtl8139,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VideoModel {
    Vga,
    Cirrus,
}

/// The method used to create or attach a storage device during server creation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StorageDeviceAction {
    Create,
    Clone,
    Attach,
}

/// The device type a storage will be attached as.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StorageDeviceType {
    Disk,
    Cdrom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RemoteAccessType {
    Vnc,
    Spice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PasswordDelivery {
    None,
    Email,
    Sms,
}

// ─── Sub-types ────────────────────────────────────────────────────────────────

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageDevice {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
    #[serde_as(as = "Option<PickFirst<(DisplayFromStr, _)>>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_tier: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_disk: Option<String>,
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_of_plan: Option<bool>,
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpAddressEntry {
    pub access: String,
    pub address: String,
    pub family: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_of_plan: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interface {
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootable: Option<bool>,
    pub index: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<InterfaceIpAddresses>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_filtering: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub interface_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceIpAddresses {
    #[serde(default)]
    pub ip_address: Vec<InterfaceIpAddress>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceIpAddress {
    pub address: String,
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dhcp_provided: Option<bool>,
    pub family: String,
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floating: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_policy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoginUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_password: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_keys: Option<SshKeys>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshKeys {
    #[serde(default)]
    pub ssh_key: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub model: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerDevices {
    #[serde(default)]
    pub device: Vec<ServerDevice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerDevice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ServerDeviceMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerDeviceMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cores: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<u64>,
}

// ─── Server struct (basic fields returned by list) ────────────────────────────

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub uuid: String,
    pub hostname: String,
    pub title: String,
    pub state: State,
    pub zone: String,
    #[serde_as(as = "PickFirst<(DisplayFromStr, _)>")]
    pub core_number: u32,
    #[serde_as(as = "PickFirst<(DisplayFromStr, _)>")]
    pub memory_amount: u64,
    pub plan: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<f64>,
    #[serde_as(as = "Option<PickFirst<(DisplayFromStr, _)>>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<ServerTags>,
}

// ─── ServerDetails struct (full detail returned by get/create/modify/actions) ─

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerDetails {
    #[serde(flatten)]
    pub server: Server,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nic_model: Option<NicModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_model: Option<VideoModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_enabled: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_port: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_backup: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_ipv4_bytes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_ipv6_bytes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<ServerDevices>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl std::ops::Deref for ServerDetails {
    type Target = Server;
    fn deref(&self) -> &Server {
        &self.server
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerIpAddresses {
    #[serde(default)]
    pub ip_address: Vec<IpAddressEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStorageDevices {
    #[serde(default)]
    pub storage_device: Vec<StorageDevice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerTags {
    #[serde(default)]
    pub tag: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerNetworking {
    pub interfaces: ServerInterfaces,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInterfaces {
    #[serde(default)]
    pub interface: Vec<Interface>,
}

// ─── Request types ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelocateRequest {
    pub zone: String,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequest {
    pub zone: String,
    pub hostname: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_number: Option<u32>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_amount: Option<u64>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nic_model: Option<NicModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_model: Option<VideoModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_backup: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<crate::types::common::Labels>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avoid_host: Option<i64>,
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

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateServerStorageDevice {
    pub action: StorageDeviceAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<StorageDeviceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_disk: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_rule: Option<crate::storage::BackupRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateServerStorageDevices {
    pub storage_device: Vec<CreateServerStorageDevice>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateServerNetworking {
    pub interfaces: CreateServerInterfaces,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateServerInterfaces {
    #[serde(default)]
    pub interface: Vec<CreateServerInterface>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateServerInterface {
    #[serde(rename = "type")]
    pub interface_type: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateServerInterfaceIpAddresses {
    #[serde(default)]
    pub ip_address: Vec<CreateServerInterfaceIpAddress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateServerInterfaceIpAddress {
    pub family: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dhcp_provided: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_number: Option<u32>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StopRequest {
    pub stop_type: StopType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RestartRequest {
    pub stop_type: StopType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_action: Option<TimeoutAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachStorageRequest {
    pub storage: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetachStorageRequest {
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadCdromRequest {
    pub storage: String,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebuildRequest {
    pub clone_source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_disk: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub uuid: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_affinity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_affinity_status: Option<Vec<AntiAffinityStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<GroupServers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<crate::types::common::Labels>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiAffinityStatus {
    pub uuid: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupServers {
    #[serde(default)]
    pub server: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGroupRequest {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_affinity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<crate::types::common::Labels>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<GroupServers>,
}

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
    Title,
    Zone,
    Size,
    Cores,
    Memory,
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
