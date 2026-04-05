//! Servers and Server Groups.
//!
//! Servers: `GET/POST/PUT/DELETE /1.3/server` + actions.
//! Server Groups: `GET/POST/PUT/DELETE /1.3/server-group`.

use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, PickFirst, serde_as};
use tokio::time::sleep;

use crate::{
    api::firewall::{CreateFirewallRuleRequest, FirewallRule},
    client::Client,
    error::Error,
    types::common::YesNo,
};

// ─── Enums ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServerState {
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
    pub state: ServerState,
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
pub struct RelocateServerRequest {
    pub zone: String,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateServerRequest {
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
    pub password_delivery: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
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
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateServerStorageDevice {
    pub action: String,
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
    pub device_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
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
pub struct ModifyServerRequest {
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
    pub remote_access_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_access_password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StopServerRequest {
    pub stop_type: StopType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RestartServerRequest {
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
pub struct RebuildServerRequest {
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
pub struct ServerGroup {
    pub uuid: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_affinity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_affinity_status: Option<Vec<AntiAffinityStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<ServerGroupServers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<crate::types::common::Labels>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiAffinityStatus {
    pub uuid: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerGroupServers {
    #[serde(default)]
    pub server: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateServerGroupRequest {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_affinity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<crate::types::common::Labels>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<ServerGroupServers>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyServerGroupRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_affinity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<crate::types::common::Labels>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<ServerGroupServers>,
}

// ─── Response wrappers ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct ServerListResponse {
    servers: ServerListWrapper,
}

#[derive(Debug, Deserialize)]
struct ServerListWrapper {
    server: Vec<Server>,
}

#[derive(Debug, Deserialize)]
struct ServerResponse {
    server: ServerDetails,
}

#[derive(Debug, Deserialize)]
struct FirewallRuleListResponse {
    firewall_rules: FirewallRuleWrapper,
}

#[derive(Debug, Deserialize)]
struct FirewallRuleWrapper {
    firewall_rule: Vec<FirewallRule>,
}

#[derive(Debug, Deserialize)]
struct FirewallRuleResponse {
    firewall_rule: FirewallRule,
}

#[derive(Debug, Deserialize)]
struct ServerGroupListResponse {
    server_groups: ServerGroupListWrapper,
}

#[derive(Debug, Deserialize)]
struct ServerGroupListWrapper {
    server_group: Vec<ServerGroup>,
}

#[derive(Debug, Deserialize)]
struct ServerGroupResponse {
    server_group: ServerGroup,
}

// ─── Client methods — Servers ─────────────────────────────────────────────────

impl Client {
    /// Returns a list of all servers.
    pub async fn list_servers(&self) -> Result<Vec<Server>, Error> {
        let resp: ServerListResponse = self.get("/1.3/server").await?;
        Ok(resp.servers.server)
    }

    /// Returns details for a specific server.
    pub async fn get_server(&self, uuid: &str) -> Result<ServerDetails, Error> {
        let resp: ServerResponse = self.get(&format!("/1.3/server/{uuid}")).await?;
        Ok(resp.server)
    }

    /// Creates a new server.
    pub async fn create_server(&self, req: CreateServerRequest) -> Result<ServerDetails, Error> {
        #[derive(Serialize)]
        struct Body {
            server: CreateServerRequest,
        }
        let resp: ServerResponse = self.post("/1.3/server", &Body { server: req }).await?;
        Ok(resp.server)
    }

    /// Modifies an existing server.
    pub async fn modify_server(
        &self,
        uuid: &str,
        req: ModifyServerRequest,
    ) -> Result<ServerDetails, Error> {
        #[derive(Serialize)]
        struct Body {
            server: ModifyServerRequest,
        }
        let resp: ServerResponse = self
            .put(&format!("/1.3/server/{uuid}"), &Body { server: req })
            .await?;
        Ok(resp.server)
    }

    /// Deletes a server.
    pub async fn delete_server(&self, uuid: &str) -> Result<(), Error> {
        self.delete(&format!("/1.3/server/{uuid}")).await
    }

    /// Deletes a server and its attached storage devices.
    pub async fn delete_server_and_storages(
        &self,
        uuid: &str,
        backups: DeleteStorageBackupsMode,
    ) -> Result<(), Error> {
        let req = self
            .request(
                reqwest::Method::DELETE,
                &format!("/1.3/server/{uuid}"),
            )
            .query(&[("storages", "1"), ("backups", &backups.to_string())]);
        self.send_empty(req).await
    }

    // ─── Actions ──────────────────────────────────────────────────────────────

    /// Starts a stopped server.
    pub async fn start_server(&self, uuid: &str) -> Result<ServerDetails, Error> {
        let resp: ServerResponse = self
            .post_empty(&format!("/1.3/server/{uuid}/start"))
            .await?;
        Ok(resp.server)
    }

    /// Relocates a server to a different zone.
    pub async fn relocate_server(
        &self,
        uuid: &str,
        req: &RelocateServerRequest,
    ) -> Result<ServerDetails, Error> {
        let resp: ServerResponse = self
            .post(&format!("/1.3/server/{uuid}/relocate"), req)
            .await?;
        Ok(resp.server)
    }

    /// Stops a running server.
    pub async fn stop_server(&self, uuid: &str, req: StopServerRequest) -> Result<ServerDetails, Error> {
        #[derive(Serialize)]
        struct Body {
            stop_server: StopServerRequest,
        }
        let resp: ServerResponse = self
            .post(
                &format!("/1.3/server/{uuid}/stop"),
                &Body { stop_server: req },
            )
            .await?;
        Ok(resp.server)
    }

    /// Restarts a running server.
    pub async fn restart_server(
        &self,
        uuid: &str,
        req: RestartServerRequest,
    ) -> Result<ServerDetails, Error> {
        #[derive(Serialize)]
        struct Body {
            restart_server: RestartServerRequest,
        }
        let resp: ServerResponse = self
            .post(
                &format!("/1.3/server/{uuid}/restart"),
                &Body {
                    restart_server: req,
                },
            )
            .await?;
        Ok(resp.server)
    }

    /// Cancels an ongoing operation on a server.
    pub async fn cancel_server_operation(&self, uuid: &str) -> Result<(), Error> {
        self.post_no_response_empty(&format!("/1.3/server/{uuid}/cancel"))
            .await
    }

    /// Rebuilds a server from a template (re-installs the OS).
    pub async fn rebuild_server(
        &self,
        uuid: &str,
        req: RebuildServerRequest,
    ) -> Result<ServerDetails, Error> {
        #[derive(Serialize)]
        struct Body {
            server_rebuild: RebuildServerRequest,
        }
        let resp: ServerResponse = self
            .post(
                &format!("/1.3/server/{uuid}/rebuild"),
                &Body {
                    server_rebuild: req,
                },
            )
            .await?;
        Ok(resp.server)
    }

    // ─── Storage devices ──────────────────────────────────────────────────────

    /// Attaches a storage device to a server.
    pub async fn attach_storage(
        &self,
        server_uuid: &str,
        req: AttachStorageRequest,
    ) -> Result<ServerDetails, Error> {
        #[derive(Serialize)]
        struct Body {
            storage_device: AttachStorageRequest,
        }
        let resp: ServerResponse = self
            .post(
                &format!("/1.3/server/{server_uuid}/storage/attach"),
                &Body {
                    storage_device: req,
                },
            )
            .await?;
        Ok(resp.server)
    }

    /// Detaches a storage device from a server.
    pub async fn detach_storage(
        &self,
        server_uuid: &str,
        req: DetachStorageRequest,
    ) -> Result<ServerDetails, Error> {
        #[derive(Serialize)]
        struct Body {
            storage_device: DetachStorageRequest,
        }
        let resp: ServerResponse = self
            .post(
                &format!("/1.3/server/{server_uuid}/storage/detach"),
                &Body {
                    storage_device: req,
                },
            )
            .await?;
        Ok(resp.server)
    }

    /// Loads a CDROM into the server's virtual optical drive.
    pub async fn load_cdrom(
        &self,
        server_uuid: &str,
        req: LoadCdromRequest,
    ) -> Result<ServerDetails, Error> {
        #[derive(Serialize)]
        struct Body {
            storage_device: LoadCdromRequest,
        }
        let resp: ServerResponse = self
            .post(
                &format!("/1.3/server/{server_uuid}/cdrom/load"),
                &Body {
                    storage_device: req,
                },
            )
            .await?;
        Ok(resp.server)
    }

    /// Ejects the CDROM from the server's virtual optical drive.
    pub async fn eject_cdrom(&self, server_uuid: &str) -> Result<ServerDetails, Error> {
        let resp: ServerResponse = self
            .post_empty(&format!("/1.3/server/{server_uuid}/cdrom/eject"))
            .await?;
        Ok(resp.server)
    }

    // ─── GPUs ─────────────────────────────────────────────────────────────────

    /// Returns GPU information for a server.
    pub async fn get_server_gpus(&self, server_uuid: &str) -> Result<Vec<GpuInfo>, Error> {
        self.get(&format!("/1.3/server/{server_uuid}/gpus")).await
    }

    // ─── Firewall rules ───────────────────────────────────────────────────────

    /// Returns all firewall rules for a server.
    pub async fn get_server_firewall_rules(
        &self,
        server_uuid: &str,
    ) -> Result<Vec<FirewallRule>, Error> {
        let resp: FirewallRuleListResponse = self
            .get(&format!("/1.3/server/{server_uuid}/firewall_rule"))
            .await?;
        Ok(resp.firewall_rules.firewall_rule)
    }

    /// Creates a firewall rule for a server.
    pub async fn create_server_firewall_rule(
        &self,
        server_uuid: &str,
        req: CreateFirewallRuleRequest,
    ) -> Result<FirewallRule, Error> {
        #[derive(Serialize)]
        struct Body {
            firewall_rule: CreateFirewallRuleRequest,
        }
        let resp: FirewallRuleResponse = self
            .post(
                &format!("/1.3/server/{server_uuid}/firewall_rule"),
                &Body { firewall_rule: req },
            )
            .await?;
        Ok(resp.firewall_rule)
    }

    /// Returns details for a specific firewall rule.
    pub async fn get_server_firewall_rule(
        &self,
        server_uuid: &str,
        position: u32,
    ) -> Result<FirewallRule, Error> {
        let resp: FirewallRuleResponse = self
            .get(&format!(
                "/1.3/server/{server_uuid}/firewall_rule/{position}"
            ))
            .await?;
        Ok(resp.firewall_rule)
    }

    /// Deletes a firewall rule from a server.
    pub async fn delete_server_firewall_rule(
        &self,
        server_uuid: &str,
        position: u32,
    ) -> Result<(), Error> {
        self.delete(&format!(
            "/1.3/server/{server_uuid}/firewall_rule/{position}"
        ))
        .await
    }

    /// Replaces all firewall rules for a server.
    pub async fn create_server_firewall_rules(
        &self,
        server_uuid: &str,
        rules: &[CreateFirewallRuleRequest],
    ) -> Result<Vec<FirewallRule>, Error> {
        #[derive(Serialize)]
        struct Inner<'a> {
            firewall_rule: &'a [CreateFirewallRuleRequest],
        }
        #[derive(Serialize)]
        struct Body<'a> {
            firewall_rules: Inner<'a>,
        }
        let resp: FirewallRuleListResponse = self
            .put(
                &format!("/1.3/server/{server_uuid}/firewall_rule"),
                &Body {
                    firewall_rules: Inner {
                        firewall_rule: rules,
                    },
                },
            )
            .await?;
        Ok(resp.firewall_rules.firewall_rule)
    }

    // ─── Server Groups ────────────────────────────────────────────────────────

    /// Returns a list of all server groups.
    pub async fn list_server_groups(&self) -> Result<Vec<ServerGroup>, Error> {
        let resp: ServerGroupListResponse = self.get("/1.3/server-group").await?;
        Ok(resp.server_groups.server_group)
    }

    /// Returns details for a specific server group.
    pub async fn get_server_group(&self, uuid: &str) -> Result<ServerGroup, Error> {
        let resp: ServerGroupResponse = self.get(&format!("/1.3/server-group/{uuid}")).await?;
        Ok(resp.server_group)
    }

    /// Creates a new server group.
    pub async fn create_server_group(
        &self,
        req: CreateServerGroupRequest,
    ) -> Result<ServerGroup, Error> {
        #[derive(Serialize)]
        struct Body {
            server_group: CreateServerGroupRequest,
        }
        let resp: ServerGroupResponse = self
            .post("/1.3/server-group", &Body { server_group: req })
            .await?;
        Ok(resp.server_group)
    }

    /// Modifies an existing server group.
    pub async fn modify_server_group(
        &self,
        uuid: &str,
        req: ModifyServerGroupRequest,
    ) -> Result<ServerGroup, Error> {
        #[derive(Serialize)]
        struct Body {
            server_group: ModifyServerGroupRequest,
        }
        let resp: ServerGroupResponse = self
            .put(
                &format!("/1.3/server-group/{uuid}"),
                &Body { server_group: req },
            )
            .await?;
        Ok(resp.server_group)
    }

    /// Deletes a server group.
    pub async fn delete_server_group(&self, uuid: &str) -> Result<(), Error> {
        self.delete(&format!("/1.3/server-group/{uuid}")).await
    }

    // ─── Polling helpers ──────────────────────────────────────────────────────

    /// Polls until a server reaches the desired state or the timeout expires.
    pub async fn wait_for_server_state(
        &self,
        uuid: &str,
        desired_state: ServerState,
        timeout: Duration,
    ) -> Result<ServerDetails, Error> {
        let start = Instant::now();
        loop {
            let details = self.get_server(uuid).await?;
            if details.server.state == desired_state {
                return Ok(details);
            }
            if start.elapsed() >= timeout {
                return Err(Error::InvalidConfig(format!(
                    "timeout waiting for server {} to reach state {:?}",
                    uuid, desired_state
                )));
            }
            sleep(Duration::from_secs(5)).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct GpusResponse {
        gpus: Vec<GpuDetail>,
    }

    #[derive(Debug, Deserialize)]
    struct ServerGroupServersResponse {
        servers: ServerGroupServers,
    }

    #[derive(Debug, Deserialize)]
    struct ServerGroupRequestBody {
        server_group: ServerGroupRequestInner,
    }

    #[allow(dead_code)]
    #[derive(Debug, Deserialize)]
    struct ServerGroupRequestInner {
        title: Option<String>,
        #[serde(default)]
        servers: Option<ServerGroupServers>,
        anti_affinity: Option<String>,
        labels: Option<crate::types::common::Labels>,
    }

    #[derive(Debug, Deserialize)]
    struct RebuildServerRequestBody {
        server_rebuild: RebuildServerRequest,
    }

    #[derive(Debug, Deserialize)]
    struct StopServerRequestBody {
        stop_server: StopServerRequest,
    }

    #[derive(Debug, Deserialize)]
    struct RestartServerRequestBody {
        restart_server: RestartServerRequest,
    }

    #[derive(Debug, Deserialize)]
    struct ModifyServerRequestBody {
        server: ModifyServerRequest,
    }

    #[derive(Debug, Deserialize)]
    struct CreateServerRequestBody {
        server: CreateServerRequest,
    }

    /// Full "Get server details" response from the UpCloud 1.3 docs,
    /// including GPU devices, storage_encrypted, remote_access_host/port.
    #[test]
    fn deserialize_get_server_details() {
        let json = r#"{
  "server": {
    "boot_order": "disk",
    "core_number": "12",
    "firewall": "on",
    "host": 7653311107,
    "hostname": "server1.example.com",
    "labels": {
      "label": [
        { "key": "env", "value": "dev" }
      ]
    },
    "ip_addresses": {
      "ip_address": [
        { "access": "utility", "address": "10.0.0.00", "family": "IPv4" },
        { "access": "public",  "address": "0.0.0.0",   "family": "IPv4" },
        { "access": "public",  "address": "xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx", "family": "IPv6" }
      ]
    },
    "license": 0,
    "memory_amount": "130048",
    "networking": {
      "interfaces": {
        "interface": [
          {
            "index": 1,
            "ip_addresses": {
              "ip_address": [
                { "address": "94.237.0.207", "dhcp_provided": "yes", "family": "IPv4", "release_policy": "release", "floating": "no" }
              ]
            },
            "mac": "de:ff:ff:ff:66:89",
            "network": "037fcf2a-6745-45dd-867e-f9479ea8c044",
            "type": "public",
            "bootable": "no"
          }
        ]
      }
    },
    "nic_model": "virtio",
    "plan": "GPU-12xCPU-128GB-2xL40S",
    "plan_ipv4_bytes": "3565675343",
    "plan_ipv6_bytes": "4534432",
    "simple_backup": "0100,dailies",
    "state": "started",
    "server_group": "0b3d85b4-b3be-46ca-a1e2-3b5d40d60fb1",
    "storage_devices": {
      "storage_device": [
        {
          "address": "virtio:0",
          "part_of_plan": "yes",
          "labels": [
            { "key": "foo", "value": "bar" }
          ],
          "storage": "012580a1-32a1-466e-a323-689ca16f2d43",
          "storage_size": "20",
          "storage_encrypted": "yes",
          "storage_tier": "maxiops",
          "storage_title": "Storage for server1.example.com",
          "type": "disk",
          "boot_disk": "0"
        }
      ]
    },
    "devices": {
      "device": [
        {
          "model": "NVIDIA L40S",
          "type": "gpu",
          "serial": "1790925007018",
          "metadata": {
            "serial": "1723925007011",
            "cores": 18176,
            "memory": 48
          }
        },
        {
          "model": "NVIDIA L40S",
          "type": "gpu",
          "serial": "1790325030002",
          "metadata": {
            "serial": "1723382930206",
            "cores": 18176,
            "memory": 48
          }
        }
      ]
    },
    "tags": {
      "tag": ["DEV", "Ubuntu"]
    },
    "timezone": "UTC",
    "title": "server1.example.com",
    "uuid": "0077fa3d-32db-4b09-9f5f-30d9e9afb565",
    "video_model": "cirrus",
    "remote_access_enabled": "yes",
    "remote_access_type": "vnc",
    "remote_access_host": "fi-hel1.vnc.upcloud.com",
    "remote_access_password": "aabbccdd",
    "remote_access_port": "3000",
    "zone": "fi-hel1"
  }
}"#;

        let resp: ServerResponse = serde_json::from_str(json).expect("deserialize server details");
        let s = resp.server;
        assert_eq!(s.uuid, "0077fa3d-32db-4b09-9f5f-30d9e9afb565");
        assert_eq!(s.core_number, 12);
        assert_eq!(s.memory_amount, 130048);
        assert_eq!(s.state, ServerState::Started);
        assert_eq!(s.plan, "GPU-12xCPU-128GB-2xL40S");
        assert_eq!(s.plan_ipv4_bytes.as_deref(), Some("3565675343"));
        assert_eq!(s.plan_ipv6_bytes.as_deref(), Some("4534432"));
        assert_eq!(
            s.remote_access_host.as_deref(),
            Some("fi-hel1.vnc.upcloud.com")
        );
        assert_eq!(s.remote_access_port.as_deref(), Some("3000"));
        assert_eq!(s.host, Some(7653311107));
        assert_eq!(
            s.server_group.as_deref(),
            Some("0b3d85b4-b3be-46ca-a1e2-3b5d40d60fb1")
        );

        // Verify devices (GPU)
        let devices = s.devices.expect("devices present");
        assert_eq!(devices.device.len(), 2);
        assert_eq!(devices.device[0].device_type.as_deref(), Some("gpu"));
        assert_eq!(devices.device[0].model.as_deref(), Some("NVIDIA L40S"));
        let meta = devices.device[0]
            .metadata
            .as_ref()
            .expect("metadata present");
        assert_eq!(meta.cores, Some(18176));
        assert_eq!(meta.memory, Some(48));

        // Verify storage_encrypted on storage device
        let sd = &s
            .storage_devices
            .expect("storage_devices present")
            .storage_device;
        assert_eq!(sd.len(), 1);
        assert_eq!(sd[0].storage_encrypted.as_deref(), Some("yes"));
        assert_eq!(sd[0].labels.as_ref().map(|l| l.len()), Some(1));

        // Verify labels (old-style wrapper)
        let labels = s.labels.expect("labels present");
        assert_eq!(labels.label.len(), 1);
        assert_eq!(labels.label[0].key, "env");
    }

    /// List servers response from docs.
    #[test]
    fn deserialize_list_servers() {
        let json = r#"{
  "servers": {
    "server": [
      {
        "core_number": "1",
        "hostname": "fi.example.com",
        "labels": { "label": [{ "key": "env", "value": "prod" }] },
        "license": 0,
        "memory_amount": "2048",
        "plan": "1xCPU-2GB",
        "plan_ipv4_bytes": "34253332",
        "plan_ipv6_bytes": "0",
        "state": "started",
        "tags": { "tag": ["PROD", "CentOS"] },
        "title": "Helsinki server",
        "uuid": "00798b85-efdc-41ca-8021-f6ef457b8531",
        "zone": "fi-hel1"
      },
      {
        "core_number": "1",
        "hostname": "uk.example.com",
        "labels": { "label": [] },
        "license": 0,
        "memory_amount": "512",
        "plan": "custom",
        "state": "stopped",
        "tags": { "tag": ["DEV", "Ubuntu"] },
        "title": "London server",
        "uuid": "009d64ef-31d1-4684-a26b-c86c955cbf46",
        "zone": "uk-lon1"
      }
    ]
  }
}"#;

        let resp: ServerListResponse = serde_json::from_str(json).expect("deserialize server list");
        assert_eq!(resp.servers.server.len(), 2);
        // Server 0
        let s0 = &resp.servers.server[0];
        assert_eq!(s0.uuid, "00798b85-efdc-41ca-8021-f6ef457b8531");
        assert_eq!(s0.hostname, "fi.example.com");
        assert_eq!(s0.title, "Helsinki server");
        assert_eq!(s0.state, ServerState::Started);
        assert_eq!(s0.zone, "fi-hel1");
        assert_eq!(s0.plan, "1xCPU-2GB");
        assert_eq!(s0.core_number, 1);
        assert_eq!(s0.memory_amount, 2048);
        assert_eq!(s0.license, Some(0.0));
        let tags0 = &s0.tags.as_ref().unwrap().tag;
        assert_eq!(tags0.len(), 2);
        assert_eq!(tags0[0], "PROD");
        assert_eq!(tags0[1], "CentOS");
        // Server 1
        let s1 = &resp.servers.server[1];
        assert_eq!(s1.uuid, "009d64ef-31d1-4684-a26b-c86c955cbf46");
        assert_eq!(s1.hostname, "uk.example.com");
        assert_eq!(s1.title, "London server");
        assert_eq!(s1.state, ServerState::Stopped);
        assert_eq!(s1.zone, "uk-lon1");
        assert_eq!(s1.plan, "custom");
        assert_eq!(s1.core_number, 1);
        assert_eq!(s1.memory_amount, 512);
        assert_eq!(s1.license, Some(0.0));
        let tags1 = &s1.tags.as_ref().unwrap().tag;
        assert_eq!(tags1.len(), 2);
        assert_eq!(tags1[0], "DEV");
        assert_eq!(tags1[1], "Ubuntu");
    }

    /// Minimal server (only required fields) still deserializes.
    #[test]
    fn deserialize_minimal_server() {
        let json = r#"{
  "server": {
    "core_number": "1",
    "hostname": "test.example.com",
    "memory_amount": "1024",
    "plan": "custom",
    "state": "stopped",
    "title": "test",
    "uuid": "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
    "zone": "fi-hel1"
  }
}"#;

        let resp: ServerResponse = serde_json::from_str(json).expect("deserialize minimal server");
        let s = resp.server;
        assert_eq!(s.uuid, "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee");
        assert!(s.devices.is_none());
        assert!(s.remote_access_host.is_none());
        assert!(s.remote_access_port.is_none());
        assert!(s.plan_ipv4_bytes.is_none());
        assert!(s.storage_devices.is_none());
    }

    /// Server group list response from docs.
    #[test]
    fn deserialize_server_group_list() {
        let json = r#"{
  "server_groups": {
    "server_group": [
      {
        "anti_affinity": "yes",
        "anti_affinity_status": [
          { "uuid": "0016dadf-eba4-4331-bd34-a361841f7af1", "status": "met" }
        ],
        "labels": { "label": [{ "key": "foo", "value": "bar" }] },
        "servers": { "server": ["0016dadf-eba4-4331-bd34-a361841f7af2"] },
        "title": "nameserver group",
        "uuid": "0b3d85b4-b3be-46ca-a1e2-3b5d30d60ab0"
      }
    ]
  }
}"#;

        let resp: ServerGroupListResponse =
            serde_json::from_str(json).expect("deserialize server group list");
        assert_eq!(resp.server_groups.server_group.len(), 1);
        let g = &resp.server_groups.server_group[0];
        assert_eq!(g.title, "nameserver group");
        assert_eq!(g.anti_affinity.as_deref(), Some("yes"));
        assert_eq!(g.servers.as_ref().map(|s| s.server.len()), Some(1));
    }

    // ─── NEW TESTS: every JSON example from the UpCloud 1.3 Servers docs ──────

    /// Get server GPUs response (GET /1.3/server/{uuid}/gpus).
    #[test]
    fn deserialize_get_server_gpus() {
        let json = r#"{
  "gpus": [
    {
      "cores": 18176,
      "firmware": "95.02.66.00.15",
      "memory_gib": 48,
      "model": "NVIDIA L40S",
      "part": "900-2G133-0180-030",
      "serial": "1723925007011"
    },
    {
      "cores": 18176,
      "firmware": "95.02.66.00.15",
      "memory_gib": 48,
      "model": "NVIDIA L40S",
      "part": "900-2G133-0180-030",
      "serial": "1723382930206"
    }
  ]
}"#;
        let resp: GpusResponse = serde_json::from_str(json).expect("deserialize gpus");
        assert_eq!(resp.gpus.len(), 2);
        assert_eq!(resp.gpus[0].model, "NVIDIA L40S");
        assert_eq!(resp.gpus[0].cores, 18176);
        assert_eq!(resp.gpus[0].memory_gib, 48);
        assert_eq!(resp.gpus[0].firmware, "95.02.66.00.15");
        assert_eq!(resp.gpus[1].serial, "1723382930206");
    }

    /// Create server from template request (POST /1.3/server).
    #[test]
    fn deserialize_create_server_from_template_request() {
        let json = r#"{
  "server": {
    "zone": "fi-hel1",
    "title": "My Debian server",
    "hostname": "debian.example.com",
    "labels": {
      "label": [
        {
          "key": "test",
          "value": ""
        }
      ]
    },
    "plan": "2xCPU-4GB",
    "storage_devices": {
      "storage_device": [
        {
          "action": "clone",
          "labels": [
            {
              "key": "foo",
              "value": "bar"
            }
          ],
          "storage": "01000000-0000-4000-8000-000020030100",
          "encrypted": "yes",
          "title": "Debian from a template",
          "size": "50",
          "tier": "maxiops"
        }
      ]
    },
    "networking": {
      "interfaces": {
        "interface": [
          {
            "ip_addresses": {
              "ip_address": [
                { "family": "IPv4" }
              ]
            },
            "type": "public"
          },
          {
            "ip_addresses": {
              "ip_address": [
                { "family": "IPv4" }
              ]
            },
            "type": "utility"
          },
          {
            "ip_addresses": {
              "ip_address": [
                { "family": "IPv6" }
              ]
            },
            "type": "public"
          },
          {
            "type": "private",
            "network": "03b5b0a0-ad4c-4817-9632-dafdb3ace5d9",
            "ip_addresses": {
              "ip_address": [
                {
                  "family": "IPv4",
                  "address": "10.0.0.50",
                  "dhcp_provided": "yes"
                },
                {
                  "family": "IPv4",
                  "address": "10.0.0.60"
                }
              ]
            }
          }
        ]
      }
    },
    "login_user": {
      "username": "upclouduser",
      "ssh_keys": {
        "ssh_key": [
          "ssh-rsa AAAAB3NzaC1yc2EAA user@example.com",
          "ssh-dss AAAAB3NzaC1kc3MAA user@example.com"
        ]
      }
    }
  }
}"#;
        let body: CreateServerRequestBody =
            serde_json::from_str(json).expect("deserialize create from template");
        let s = body.server;
        // Top-level scalar fields
        assert_eq!(s.zone, "fi-hel1");
        assert_eq!(s.title, "My Debian server");
        assert_eq!(s.hostname, "debian.example.com");
        assert_eq!(s.plan.as_deref(), Some("2xCPU-4GB"));
        assert!(s.core_number.is_none());
        assert!(s.memory_amount.is_none());
        // labels
        let labels = &s.labels.as_ref().unwrap().label;
        assert_eq!(labels.len(), 1);
        assert_eq!(labels[0].key, "test");
        assert_eq!(labels[0].value, "");
        // storage_devices
        let sd = &s.storage_devices.unwrap().storage_device;
        assert_eq!(sd.len(), 1);
        assert_eq!(sd[0].action, "clone");
        assert_eq!(
            sd[0].storage.as_deref(),
            Some("01000000-0000-4000-8000-000020030100")
        );
        assert_eq!(sd[0].encrypted, Some(true));
        assert_eq!(sd[0].title.as_deref(), Some("Debian from a template"));
        assert_eq!(sd[0].size, Some(50));
        assert_eq!(sd[0].tier.as_deref(), Some("maxiops"));
        assert_eq!(sd[0].labels.as_ref().unwrap()[0].key, "foo");
        assert_eq!(sd[0].labels.as_ref().unwrap()[0].value, "bar");
        // networking interfaces
        let ifaces = &s.networking.unwrap().interfaces.interface;
        assert_eq!(ifaces.len(), 4);
        assert_eq!(ifaces[0].interface_type, "public");
        assert_eq!(
            ifaces[0].ip_addresses.as_ref().unwrap().ip_address[0].family,
            "IPv4"
        );
        assert_eq!(ifaces[1].interface_type, "utility");
        assert_eq!(ifaces[2].interface_type, "public");
        assert_eq!(
            ifaces[2].ip_addresses.as_ref().unwrap().ip_address[0].family,
            "IPv6"
        );
        assert_eq!(ifaces[3].interface_type, "private");
        assert_eq!(
            ifaces[3].network.as_deref(),
            Some("03b5b0a0-ad4c-4817-9632-dafdb3ace5d9")
        );
        let priv_ips = &ifaces[3].ip_addresses.as_ref().unwrap().ip_address;
        assert_eq!(priv_ips.len(), 2);
        assert_eq!(priv_ips[0].family, "IPv4");
        assert_eq!(priv_ips[0].address.as_deref(), Some("10.0.0.50"));
        assert_eq!(priv_ips[0].dhcp_provided.as_deref(), Some("yes"));
        assert_eq!(priv_ips[1].address.as_deref(), Some("10.0.0.60"));
        assert!(priv_ips[1].dhcp_provided.is_none());
        // login_user
        let lu = s.login_user.unwrap();
        assert_eq!(lu.username.as_deref(), Some("upclouduser"));
        assert!(lu.create_password.is_none());
        let ssh = lu.ssh_keys.unwrap().ssh_key;
        assert_eq!(ssh.len(), 2);
        assert_eq!(ssh[0], "ssh-rsa AAAAB3NzaC1yc2EAA user@example.com");
        assert_eq!(ssh[1], "ssh-dss AAAAB3NzaC1kc3MAA user@example.com");
    }

    /// Create server by cloning request (POST /1.3/server).
    #[test]
    fn deserialize_create_server_by_cloning_request() {
        let json = r#"{
  "server": {
    "zone": "fi-hel1",
    "title": "Another Debian server",
    "hostname": "debian2.example.com",
    "core_number": "4",
    "memory_amount": "8192",
    "storage_devices": {
      "storage_device": [
        {
          "action": "clone",
          "storage": "0169b4f8-051c-4a86-9484-f5b798249949",
          "title": "Storage for another Debian server"
        }
      ]
    },
    "networking": {
      "interfaces": {
        "interface": [
          {
            "ip_addresses": {
              "ip_address": [
                { "family": "IPv4" }
              ]
            },
            "type": "public"
          },
          {
            "ip_addresses": {
              "ip_address": [
                { "family": "IPv4" }
              ]
            },
            "type": "utility"
          },
          {
            "ip_addresses": {
              "ip_address": [
                { "family": "IPv6" }
              ]
            },
            "type": "public"
          },
          {
            "type": "private",
            "network": "03b5b0a0-ad4c-4817-9632-dafdb3ace5d9",
            "ip_addresses": {
              "ip_address": [
                {
                  "family": "IPv4",
                  "address": "10.0.0.50"
                }
              ]
            }
          }
        ]
      }
    }
  }
}"#;
        let body: CreateServerRequestBody =
            serde_json::from_str(json).expect("deserialize create by cloning");
        let s = body.server;
        // Top-level scalar fields
        assert_eq!(s.zone, "fi-hel1");
        assert_eq!(s.title, "Another Debian server");
        assert_eq!(s.hostname, "debian2.example.com");
        assert_eq!(s.core_number, Some(4));
        assert_eq!(s.memory_amount, Some(8192));
        assert!(s.plan.is_none());
        // storage_devices
        let sd = &s.storage_devices.unwrap().storage_device;
        assert_eq!(sd.len(), 1);
        assert_eq!(sd[0].action, "clone");
        assert_eq!(
            sd[0].storage.as_deref(),
            Some("0169b4f8-051c-4a86-9484-f5b798249949")
        );
        assert_eq!(
            sd[0].title.as_deref(),
            Some("Storage for another Debian server")
        );
        assert!(sd[0].size.is_none());
        assert!(sd[0].tier.is_none());
        // networking
        let ifaces = &s.networking.unwrap().interfaces.interface;
        assert_eq!(ifaces.len(), 4);
        assert_eq!(ifaces[0].interface_type, "public");
        assert_eq!(
            ifaces[0].ip_addresses.as_ref().unwrap().ip_address[0].family,
            "IPv4"
        );
        assert_eq!(ifaces[1].interface_type, "utility");
        assert_eq!(ifaces[2].interface_type, "public");
        assert_eq!(
            ifaces[2].ip_addresses.as_ref().unwrap().ip_address[0].family,
            "IPv6"
        );
        assert_eq!(ifaces[3].interface_type, "private");
        assert_eq!(
            ifaces[3].network.as_deref(),
            Some("03b5b0a0-ad4c-4817-9632-dafdb3ace5d9")
        );
        let priv_ips = &ifaces[3].ip_addresses.as_ref().unwrap().ip_address;
        assert_eq!(priv_ips.len(), 1);
        assert_eq!(priv_ips[0].family, "IPv4");
        assert_eq!(priv_ips[0].address.as_deref(), Some("10.0.0.50"));
    }

    /// Create server from scratch request (POST /1.3/server).
    #[test]
    fn deserialize_create_server_from_scratch_request() {
        let json = r#"{
  "server": {
    "zone": "fi-hel1",
    "title": "My Debian server",
    "hostname": "debian.example.com",
    "avoid_host": 7653311107,
    "core_number": "4",
    "memory_amount": "8192",
    "storage_devices": {
      "storage_device": [
        {
          "action": "create",
          "size": "20",
          "tier": "maxiops",
          "title": "Debian from scratch"
        },
        {
          "action": "attach",
          "storage": "01000000-0000-4000-8000-000020010301",
          "type": "cdrom"
        }
      ]
    },
    "simple_backup": "0430,monthlies",
    "networking": {
      "interfaces": {
        "interface": [
          {
            "ip_addresses": {
              "ip_address": [
                { "family": "IPv4" }
              ]
            },
            "type": "public"
          },
          {
            "ip_addresses": {
              "ip_address": [
                { "family": "IPv4" }
              ]
            },
            "type": "utility"
          },
          {
            "ip_addresses": {
              "ip_address": [
                { "family": "IPv6" }
              ]
            },
            "type": "public"
          },
          {
            "type": "private",
            "network": "03b5b0a0-ad4c-4817-9632-dafdb3ace5d9",
            "ip_addresses": {
              "ip_address": [
                {
                  "family": "IPv4",
                  "address": "10.0.0.50"
                }
              ]
            }
          }
        ]
      }
    }
  }
}"#;
        let body: CreateServerRequestBody =
            serde_json::from_str(json).expect("deserialize create from scratch");
        let s = body.server;
        // Top-level scalar fields
        assert_eq!(s.zone, "fi-hel1");
        assert_eq!(s.title, "My Debian server");
        assert_eq!(s.hostname, "debian.example.com");
        assert_eq!(s.avoid_host, Some(7653311107));
        assert_eq!(s.core_number, Some(4));
        assert_eq!(s.memory_amount, Some(8192));
        assert!(s.plan.is_none());
        assert_eq!(s.simple_backup.as_deref(), Some("0430,monthlies"));
        // storage_devices
        let sd = &s.storage_devices.unwrap().storage_device;
        assert_eq!(sd.len(), 2);
        assert_eq!(sd[0].action, "create");
        assert_eq!(sd[0].size, Some(20));
        assert_eq!(sd[0].tier.as_deref(), Some("maxiops"));
        assert_eq!(sd[0].title.as_deref(), Some("Debian from scratch"));
        assert!(sd[0].storage.is_none());
        assert_eq!(sd[1].action, "attach");
        assert_eq!(
            sd[1].storage.as_deref(),
            Some("01000000-0000-4000-8000-000020010301")
        );
        assert_eq!(sd[1].device_type.as_deref(), Some("cdrom"));
        assert!(sd[1].size.is_none());
        // networking
        let ifaces = &s.networking.unwrap().interfaces.interface;
        assert_eq!(ifaces.len(), 4);
        assert_eq!(ifaces[0].interface_type, "public");
        assert_eq!(
            ifaces[0].ip_addresses.as_ref().unwrap().ip_address[0].family,
            "IPv4"
        );
        assert_eq!(ifaces[1].interface_type, "utility");
        assert_eq!(ifaces[2].interface_type, "public");
        assert_eq!(
            ifaces[2].ip_addresses.as_ref().unwrap().ip_address[0].family,
            "IPv6"
        );
        assert_eq!(ifaces[3].interface_type, "private");
        assert_eq!(
            ifaces[3].network.as_deref(),
            Some("03b5b0a0-ad4c-4817-9632-dafdb3ace5d9")
        );
        let priv_ips = &ifaces[3].ip_addresses.as_ref().unwrap().ip_address;
        assert_eq!(priv_ips.len(), 1);
        assert_eq!(priv_ips[0].family, "IPv4");
        assert_eq!(priv_ips[0].address.as_deref(), Some("10.0.0.50"));
    }

    /// Create server response (POST /1.3/server — 202 Accepted).
    #[test]
    fn deserialize_create_server_response() {
        let json = r#"{
  "server": {
    "boot_order": "disk",
    "core_number": "2",
    "firewall": "on",
    "hostname": "debian.example.com",
    "ip_addresses": {
      "ip_address": [
        { "access": "utility", "address": "10.0.0.0", "family": "IPv4" },
        { "access": "public",  "address": "x.x.x.x",  "family": "IPv4" },
        { "access": "public",  "address": "xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx", "family": "IPv6" }
      ]
    },
    "license": 0,
    "memory_amount": "4096",
    "nic_model": "virtio",
    "password": "aabbccdd",
    "plan": "2xCPU-4GB",
    "plan_ipv4_bytes": "0",
    "plan_ipv6_bytes": "0",
    "simple_backup": "0430,dailies",
    "state": "maintenance",
    "storage_devices": {
      "storage_device": [
        {
          "address": "virtio:0",
          "part_of_plan": "yes",
          "storage": "0169b4f8-051c-4a86-9484-f5b798249949",
          "storage_size": 1,
          "storage_tier": "maxiops",
          "storage_title": "Debian from a template",
          "type": "disk",
          "boot_disk": "0"
        }
      ]
    },
    "tags": { "tag": [] },
    "timezone": "UTC",
    "title": "My Debian server",
    "uuid": "00c78863-db86-44ea-af70-d6edc4d162bf",
    "video_model": "cirrus",
    "remote_access_enabled": "no",
    "remote_access_password": "aabbccdd",
    "zone": "fi-hel1"
  }
}"#;
        let resp: ServerResponse =
            serde_json::from_str(json).expect("deserialize create server response");
        let s = resp.server;
        // Top-level scalar fields
        assert_eq!(s.uuid, "00c78863-db86-44ea-af70-d6edc4d162bf");
        assert_eq!(s.hostname, "debian.example.com");
        assert_eq!(s.title, "My Debian server");
        assert_eq!(s.state, ServerState::Maintenance);
        assert_eq!(s.zone, "fi-hel1");
        assert_eq!(s.plan, "2xCPU-4GB");
        assert_eq!(s.core_number, 2);
        assert_eq!(s.memory_amount, 4096);
        assert_eq!(s.firewall.as_deref(), Some("on"));
        assert_eq!(s.boot_order.as_deref(), Some("disk"));
        assert_eq!(s.nic_model, Some(NicModel::Virtio));
        assert_eq!(s.timezone.as_deref(), Some("UTC"));
        assert_eq!(s.video_model, Some(VideoModel::Cirrus));
        assert_eq!(s.license, Some(0.0));
        assert_eq!(s.plan_ipv4_bytes.as_deref(), Some("0"));
        assert_eq!(s.plan_ipv6_bytes.as_deref(), Some("0"));
        assert_eq!(s.simple_backup.as_deref(), Some("0430,dailies"));
        assert_eq!(s.password.as_deref(), Some("aabbccdd"));
        assert_eq!(s.remote_access_enabled.as_deref(), Some("no"));
        assert_eq!(s.remote_access_password.as_deref(), Some("aabbccdd"));
        // ip_addresses
        let ips = s.ip_addresses.as_ref().unwrap();
        assert_eq!(ips.ip_address.len(), 3);
        assert_eq!(ips.ip_address[0].access, "utility");
        assert_eq!(ips.ip_address[0].address, "10.0.0.0");
        assert_eq!(ips.ip_address[0].family, "IPv4");
        assert_eq!(ips.ip_address[1].access, "public");
        assert_eq!(ips.ip_address[2].family, "IPv6");
        // storage_devices
        let sd = &s.storage_devices.as_ref().unwrap().storage_device;
        assert_eq!(sd.len(), 1);
        assert_eq!(sd[0].address, "virtio:0");
        assert_eq!(sd[0].part_of_plan, Some(true));
        assert_eq!(
            sd[0].storage.as_deref(),
            Some("0169b4f8-051c-4a86-9484-f5b798249949")
        );
        assert_eq!(sd[0].storage_size, Some(1));
        assert_eq!(sd[0].storage_tier.as_deref(), Some("maxiops"));
        assert_eq!(sd[0].storage_title.as_deref(), Some("Debian from a template"));
        assert_eq!(sd[0].device_type.as_deref(), Some("disk"));
        assert_eq!(sd[0].boot_disk.as_deref(), Some("0"));
        // tags
        assert!(s.tags.as_ref().unwrap().tag.is_empty());
    }

    /// Rebuild server request (POST /1.3/server/{uuid}/rebuild).
    #[test]
    fn deserialize_rebuild_server_request() {
        let json = r#"{
  "server_rebuild": {
    "storage_title": "new encrypted storage",
    "clone_source": "01000000-0000-4000-8000-000020050100",
    "detach_disk": "0169b4f8-051c-4a86-9484-f5b798249949",
    "delete_detached_disk": "yes",
    "password_delivery": "email",
    "encrypted": "yes",
    "login_user": {
      "create_password": 0,
      "username": "foobar",
      "ssh_keys": {
        "ssh_key": [
          "ssh-ed25519 ..."
        ]
      }
    }
  }
}"#;
        let body: RebuildServerRequestBody =
            serde_json::from_str(json).expect("deserialize rebuild request");
        let r = body.server_rebuild;
        assert_eq!(r.clone_source, "01000000-0000-4000-8000-000020050100");
        assert_eq!(r.delete_detached_disk, Some(true));
        assert_eq!(r.encrypted, Some(true));
        assert_eq!(r.password_delivery.as_deref(), Some("email"));
        let lu = r.login_user.unwrap();
        assert_eq!(lu.username.as_deref(), Some("foobar"));
        assert_eq!(lu.ssh_keys.unwrap().ssh_key.len(), 1);
    }

    /// Rebuild server response (POST /1.3/server/{uuid}/rebuild — 202 Accepted).
    #[test]
    fn deserialize_rebuild_server_response() {
        let json = r#"{
  "server": {
    "boot_order": "disk",
    "core_number": "1",
    "created": 1716400530,
    "firewall": "off",
    "hostname": "my-awesome-server",
    "ip_addresses": {
      "ip_address": [
        { "access": "utility", "address": "10.10.15.1", "family": "IPv4" },
        { "access": "public", "address": "150.127.16.103", "family": "IPv4", "part_of_plan": "yes" }
      ]
    },
    "labels": { "label": [] },
    "license": 0,
    "memory_amount": "1024",
    "metadata": "yes",
    "networking": {
      "interfaces": {
        "interface": [
          {
            "bootable": "no",
            "index": 1,
            "ip_addresses": {
              "ip_address": [
                {
                  "address": "150.127.16.103",
                  "dhcp_provided": "yes",
                  "family": "IPv4",
                  "floating": "no",
                  "release_policy": "release"
                }
              ]
            },
            "mac": "de:10:ff:ff:17:79",
            "network": "03030473-8e9a-4f5f-bffe-b3cffe378a28",
            "source_ip_filtering": "yes",
            "type": "public"
          },
          {
            "bootable": "no",
            "index": 2,
            "ip_addresses": {
              "ip_address": [
                {
                  "address": "10.10.15.1",
                  "dhcp_provided": "yes",
                  "family": "IPv4",
                  "floating": "no",
                  "release_policy": "release"
                }
              ]
            },
            "mac": "de:10:ff:ff:17:78",
            "network": "039bd564-c991-4fdd-bedd-37e936fab099",
            "source_ip_filtering": "yes",
            "type": "utility"
          }
        ]
      }
    },
    "nic_model": "virtio",
    "plan": "1xCPU-1GB",
    "plan_ipv4_bytes": "0",
    "plan_ipv6_bytes": "0",
    "remote_access_enabled": "no",
    "remote_access_password": "osd9xRqufdlqwe_1481-7",
    "remote_access_type": "vnc",
    "server_group": null,
    "simple_backup": "no",
    "state": "maintenance",
    "storage_devices": {
      "storage_device": [
        {
          "address": "virtio:0",
          "boot_disk": "0",
          "labels": [
            { "key": "_os_architecture", "value": "64" },
            { "key": "_os_brand_name", "value": "Debian" },
            { "key": "_os_brand_version", "value": "10.1" },
            { "key": "_os_main_category", "value": "Linux" },
            { "key": "_os_type", "value": "debian" },
            { "key": "_template_uuid", "value": "01000000-0000-4000-8000-000020050100" }
          ],
          "part_of_plan": "yes",
          "storage": "0117aae1-b8a9-4aee-affe-89771922cabf",
          "storage_encrypted": "yes",
          "storage_size": 25,
          "storage_tier": "maxiops",
          "storage_title": "my new re-installed os",
          "type": "disk"
        }
      ]
    },
    "tags": { "tag": [] },
    "timezone": "UTC",
    "title": "my awesome server",
    "uuid": "00b5d217-3aa8-4820-8b6a-aef77f8e84f5",
    "video_model": "cirrus",
    "zone": "fi-hel1"
  }
}"#;
        let resp: ServerResponse =
            serde_json::from_str(json).expect("deserialize rebuild response");
        let s = resp.server;
        // Top-level scalar fields
        assert_eq!(s.uuid, "00b5d217-3aa8-4820-8b6a-aef77f8e84f5");
        assert_eq!(s.hostname, "my-awesome-server");
        assert_eq!(s.title, "my awesome server");
        assert_eq!(s.state, ServerState::Maintenance);
        assert_eq!(s.zone, "fi-hel1");
        assert_eq!(s.plan, "1xCPU-1GB");
        assert_eq!(s.core_number, 1);
        assert_eq!(s.memory_amount, 1024);
        assert_eq!(s.created, Some(1716400530));
        assert_eq!(s.firewall.as_deref(), Some("off"));
        assert_eq!(s.boot_order.as_deref(), Some("disk"));
        assert_eq!(s.nic_model, Some(NicModel::Virtio));
        assert_eq!(s.timezone.as_deref(), Some("UTC"));
        assert_eq!(s.video_model, Some(VideoModel::Cirrus));
        assert_eq!(s.license, Some(0.0));
        assert_eq!(s.metadata.as_deref(), Some("yes"));
        assert_eq!(s.plan_ipv4_bytes.as_deref(), Some("0"));
        assert_eq!(s.plan_ipv6_bytes.as_deref(), Some("0"));
        assert_eq!(s.simple_backup.as_deref(), Some("no"));
        assert_eq!(s.remote_access_enabled.as_deref(), Some("no"));
        assert_eq!(
            s.remote_access_password.as_deref(),
            Some("osd9xRqufdlqwe_1481-7")
        );
        assert_eq!(s.remote_access_type.as_deref(), Some("vnc"));
        // server_group is null in the JSON
        assert!(s.server_group.is_none());
        // ip_addresses
        let ips = s.ip_addresses.as_ref().unwrap();
        assert_eq!(ips.ip_address.len(), 2);
        assert_eq!(ips.ip_address[0].access, "utility");
        assert_eq!(ips.ip_address[0].address, "10.10.15.1");
        assert_eq!(ips.ip_address[0].family, "IPv4");
        assert_eq!(ips.ip_address[1].access, "public");
        assert_eq!(ips.ip_address[1].address, "150.127.16.103");
        assert_eq!(ips.ip_address[1].part_of_plan.as_deref(), Some("yes"));
        // networking interfaces
        let ifaces = &s.networking.as_ref().unwrap().interfaces.interface;
        assert_eq!(ifaces.len(), 2);
        assert_eq!(ifaces[0].index, 1);
        assert_eq!(ifaces[0].bootable, Some(false));
        assert_eq!(ifaces[0].interface_type.as_deref(), Some("public"));
        assert_eq!(ifaces[0].mac.as_deref(), Some("de:10:ff:ff:17:79"));
        assert_eq!(
            ifaces[0].network.as_deref(),
            Some("03030473-8e9a-4f5f-bffe-b3cffe378a28")
        );
        assert_eq!(ifaces[0].source_ip_filtering, Some(true));
        let iface0_ips = &ifaces[0].ip_addresses.as_ref().unwrap().ip_address;
        assert_eq!(iface0_ips.len(), 1);
        assert_eq!(iface0_ips[0].address, "150.127.16.103");
        assert_eq!(iface0_ips[0].dhcp_provided, Some(true));
        assert_eq!(iface0_ips[0].family, "IPv4");
        assert_eq!(iface0_ips[0].floating, Some(false));
        assert_eq!(iface0_ips[0].release_policy.as_deref(), Some("release"));
        assert_eq!(ifaces[1].index, 2);
        assert_eq!(ifaces[1].interface_type.as_deref(), Some("utility"));
        assert_eq!(ifaces[1].mac.as_deref(), Some("de:10:ff:ff:17:78"));
        // storage_devices
        let sd = &s.storage_devices.as_ref().unwrap().storage_device;
        assert_eq!(sd.len(), 1);
        assert_eq!(sd[0].address, "virtio:0");
        assert_eq!(sd[0].boot_disk.as_deref(), Some("0"));
        assert_eq!(sd[0].part_of_plan, Some(true));
        assert_eq!(
            sd[0].storage.as_deref(),
            Some("0117aae1-b8a9-4aee-affe-89771922cabf")
        );
        assert_eq!(sd[0].storage_encrypted.as_deref(), Some("yes"));
        assert_eq!(sd[0].storage_size, Some(25));
        assert_eq!(sd[0].storage_tier.as_deref(), Some("maxiops"));
        assert_eq!(
            sd[0].storage_title.as_deref(),
            Some("my new re-installed os")
        );
        assert_eq!(sd[0].device_type.as_deref(), Some("disk"));
        assert_eq!(sd[0].labels.as_ref().unwrap().len(), 6);
        assert_eq!(sd[0].labels.as_ref().unwrap()[0].key, "_os_architecture");
        assert_eq!(sd[0].labels.as_ref().unwrap()[0].value, "64");
        // tags & labels
        assert!(s.tags.as_ref().unwrap().tag.is_empty());
        assert!(s.labels.as_ref().unwrap().label.is_empty());
    }

    /// Start server request (POST /1.3/server/{uuid}/start).
    #[test]
    fn deserialize_start_server_request() {
        let json = r#"{
  "server": {
    "host": 8055964291
  }
}"#;
        let v: serde_json::Value = serde_json::from_str(json).expect("valid JSON");
        assert_eq!(v["server"]["host"], 8055964291_i64);
    }

    /// Start server response (POST /1.3/server/{uuid}/start — 200 OK).
    #[test]
    fn deserialize_start_server_response() {
        // Note: docs have a typo "plart_of_plan" (instead of "part_of_plan") — serde ignores unknown fields.
        let json = r#"{
  "server": {
    "boot_order": "disk",
    "core_number": "2",
    "firewall": "on",
    "hostname": "debian.example.com",
    "host": 8055964291,
    "ip_addresses": {
      "ip_address": [
        { "access": "utility", "address": "10.0.0.0", "family": "IPv4" },
        { "access": "public",  "address": "x.x.x.x",  "family": "IPv4" },
        { "access": "public",  "address": "xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx", "family": "IPv6" }
      ]
    },
    "labels": { "label": [] },
    "license": 0,
    "memory_amount": "4096",
    "nic_model": "virtio",
    "plan": "2xCPU-4GB",
    "plan_ipv4_bytes": "435376546",
    "plan_ipv6_bytes": "0",
    "state": "started",
    "storage_devices": {
      "storage_device": [
        {
          "address": "virtio:0",
          "plart_of_plan": "yes",
          "storage": "0169b4f8-051c-4a86-9484-f5b798249949",
          "storage_size": "10",
          "storage_tier": "maxiops",
          "storage_title": "Debian from a template",
          "type": "disk",
          "boot_disk": "0"
        }
      ]
    },
    "tags": { "tag": [] },
    "timezone": "UTC",
    "title": "My Debian server",
    "uuid": "00c78863-db86-44ea-af70-d6edc4d162bf",
    "video_model": "cirrus",
    "remote_access_enabled": "no",
    "remote_access_password": "aabbccdd",
    "zone": "fi-hel1"
  }
}"#;
        let resp: ServerResponse =
            serde_json::from_str(json).expect("deserialize start server response");
        let s = resp.server;
        // Top-level scalar fields
        assert_eq!(s.uuid, "00c78863-db86-44ea-af70-d6edc4d162bf");
        assert_eq!(s.hostname, "debian.example.com");
        assert_eq!(s.title, "My Debian server");
        assert_eq!(s.state, ServerState::Started);
        assert_eq!(s.zone, "fi-hel1");
        assert_eq!(s.plan, "2xCPU-4GB");
        assert_eq!(s.core_number, 2);
        assert_eq!(s.memory_amount, 4096);
        assert_eq!(s.host, Some(8055964291));
        assert_eq!(s.firewall.as_deref(), Some("on"));
        assert_eq!(s.boot_order.as_deref(), Some("disk"));
        assert_eq!(s.nic_model, Some(NicModel::Virtio));
        assert_eq!(s.timezone.as_deref(), Some("UTC"));
        assert_eq!(s.video_model, Some(VideoModel::Cirrus));
        assert_eq!(s.license, Some(0.0));
        assert_eq!(s.plan_ipv4_bytes.as_deref(), Some("435376546"));
        assert_eq!(s.plan_ipv6_bytes.as_deref(), Some("0"));
        assert_eq!(s.remote_access_enabled.as_deref(), Some("no"));
        assert_eq!(s.remote_access_password.as_deref(), Some("aabbccdd"));
        // ip_addresses
        let ips = s.ip_addresses.as_ref().unwrap();
        assert_eq!(ips.ip_address.len(), 3);
        assert_eq!(ips.ip_address[0].access, "utility");
        assert_eq!(ips.ip_address[0].address, "10.0.0.0");
        assert_eq!(ips.ip_address[0].family, "IPv4");
        assert_eq!(ips.ip_address[1].access, "public");
        assert_eq!(ips.ip_address[2].family, "IPv6");
        // storage_devices
        let sd = s.storage_devices.as_ref().unwrap();
        assert_eq!(sd.storage_device.len(), 1);
        assert_eq!(sd.storage_device[0].address, "virtio:0");
        assert_eq!(
            sd.storage_device[0].storage.as_deref(),
            Some("0169b4f8-051c-4a86-9484-f5b798249949")
        );
        assert_eq!(sd.storage_device[0].storage_size, Some(10));
        assert_eq!(sd.storage_device[0].storage_tier.as_deref(), Some("maxiops"));
        assert_eq!(
            sd.storage_device[0].storage_title.as_deref(),
            Some("Debian from a template")
        );
        assert_eq!(sd.storage_device[0].device_type.as_deref(), Some("disk"));
        assert_eq!(sd.storage_device[0].boot_disk.as_deref(), Some("0"));
        // tags & labels
        assert!(s.tags.as_ref().unwrap().tag.is_empty());
        assert!(s.labels.as_ref().unwrap().label.is_empty());
    }

    /// Stop server request (POST /1.3/server/{uuid}/stop).
    #[test]
    fn deserialize_stop_server_request() {
        let json = r#"{
  "stop_server": {
    "stop_type": "soft",
    "timeout": "60"
  }
}"#;
        let body: StopServerRequestBody =
            serde_json::from_str(json).expect("deserialize stop request");
        assert_eq!(body.stop_server.stop_type, StopType::Soft);
        assert_eq!(body.stop_server.timeout.as_deref(), Some("60"));
    }

    /// Stop server response (POST /1.3/server/{uuid}/stop — 200 OK).
    /// Note: The docs use a bare array for `ip_addresses` instead of the usual
    /// `{"ip_address":[...]}` wrapper. We validate with `serde_json::Value`.
    #[test]
    fn deserialize_stop_server_response() {
        let json = r#"{
  "server": {
    "boot_order": "disk",
    "core_number": "2",
    "firewall": "on",
    "hostname": "debian.example.com",
    "host": 8055964291,
    "ip_addresses": [
      { "access": "utility", "address": "10.0.0.0",  "family": "IPv4" },
      { "access": "public",  "address": "x.x.x.x",   "family": "IPv4" },
      { "access": "public",  "address": "xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx", "family": "IPv6" }
    ],
    "labels": { "label": [] },
    "license": 0,
    "memory_amount": "4096",
    "nic_model": "virtio",
    "plan": "2xCPU-4GB",
    "plan_ipv4_bytes": "34543646754",
    "plan_ipv6_bytes": "242432",
    "state": "started",
    "storage_devices": {
      "storage_device": [
        {
          "address": "virtio:0",
          "part_of_plan": "yes",
          "storage": "0169b4f8-051c-4a86-9484-f5b798249949",
          "storage_size": "10",
          "storage_tier": "maxiops",
          "storage_title": "Debian from a template",
          "type": "disk"
        }
      ]
    },
    "tags": { "tag": [] },
    "timezone": "UTC",
    "title": "My Debian server",
    "uuid": "00c78863-db86-44ea-af70-d6edc4d162bf",
    "video_model": "cirrus",
    "remote_access_enabled": "no",
    "remote_access_password": "aabbccdd",
    "zone": "fi-hel1"
  }
}"#;
        // ip_addresses is a bare array in the docs (inconsistent with other responses).
        let v: serde_json::Value = serde_json::from_str(json).expect("valid JSON");
        let sv = &v["server"];
        // Top-level scalar fields
        assert_eq!(sv["uuid"], "00c78863-db86-44ea-af70-d6edc4d162bf");
        assert_eq!(sv["hostname"], "debian.example.com");
        assert_eq!(sv["title"], "My Debian server");
        assert_eq!(sv["state"], "started");
        assert_eq!(sv["zone"], "fi-hel1");
        assert_eq!(sv["plan"], "2xCPU-4GB");
        assert_eq!(sv["core_number"], "2");
        assert_eq!(sv["memory_amount"], "4096");
        assert_eq!(sv["host"], 8055964291_i64);
        assert_eq!(sv["firewall"], "on");
        assert_eq!(sv["boot_order"], "disk");
        assert_eq!(sv["nic_model"], "virtio");
        assert_eq!(sv["timezone"], "UTC");
        assert_eq!(sv["video_model"], "cirrus");
        assert_eq!(sv["license"], 0);
        assert_eq!(sv["plan_ipv4_bytes"], "34543646754");
        assert_eq!(sv["plan_ipv6_bytes"], "242432");
        assert_eq!(sv["remote_access_enabled"], "no");
        assert_eq!(sv["remote_access_password"], "aabbccdd");
        // ip_addresses (bare array)
        assert_eq!(sv["ip_addresses"].as_array().unwrap().len(), 3);
        assert_eq!(sv["ip_addresses"][0]["access"], "utility");
        assert_eq!(sv["ip_addresses"][0]["family"], "IPv4");
        assert_eq!(sv["ip_addresses"][1]["access"], "public");
        // storage_devices
        let sd = &sv["storage_devices"]["storage_device"];
        assert_eq!(sd.as_array().unwrap().len(), 1);
        assert_eq!(sd[0]["address"], "virtio:0");
        assert_eq!(sd[0]["storage"], "0169b4f8-051c-4a86-9484-f5b798249949");
        assert_eq!(sd[0]["storage_size"], "10");
        assert_eq!(sd[0]["storage_tier"], "maxiops");
        assert_eq!(sd[0]["storage_title"], "Debian from a template");
        assert_eq!(sd[0]["type"], "disk");
        assert_eq!(sd[0]["part_of_plan"], "yes");
        // tags & labels
        assert!(sv["tags"]["tag"].as_array().unwrap().is_empty());
        assert!(sv["labels"]["label"].as_array().unwrap().is_empty());
    }

    /// Restart server request (POST /1.3/server/{uuid}/restart).
    #[test]
    fn deserialize_restart_server_request() {
        let json = r#"{
  "restart_server": {
    "stop_type": "soft",
    "timeout": "60",
    "timeout_action": "destroy"
  }
}"#;
        let body: RestartServerRequestBody =
            serde_json::from_str(json).expect("deserialize restart request");
        assert_eq!(body.restart_server.stop_type, StopType::Soft);
        assert_eq!(body.restart_server.timeout.as_deref(), Some("60"));
        assert_eq!(
            body.restart_server.timeout_action,
            Some(TimeoutAction::Destroy)
        );
    }

    /// Restart server response (POST /1.3/server/{uuid}/restart — 200 OK).
    #[test]
    fn deserialize_restart_server_response() {
        // Note: docs typo "plart_of_plan" — serde ignores unknown fields.
        let json = r#"{
  "server": {
    "boot_order": "disk",
    "core_number": "2",
    "firewall": "on",
    "hostname": "debian.example.com",
    "host": 8055964291,
    "ip_addresses": {
      "ip_address": [
        { "access": "utility", "address": "10.0.0.0",  "family": "IPv4" },
        { "access": "public",  "address": "x.x.x.x",   "family": "IPv4" },
        { "access": "public",  "address": "xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx", "family": "IPv6" }
      ]
    },
    "labels": { "label": [] },
    "license": 0,
    "memory_amount": "4096",
    "nic_model": "virtio",
    "plan": "2xCPU-4GB",
    "plan_ipv4_bytes": "34543646754",
    "plan_ipv6_bytes": "242432",
    "state": "started",
    "storage_devices": {
      "storage_device": [
        {
          "address": "virtio:0",
          "plart_of_plan": "yes",
          "storage": "0169b4f8-051c-4a86-9484-f5b798249949",
          "storage_size": "10",
          "storage_tier": "maxiops",
          "storage_title": "Debian from a template",
          "type": "disk"
        }
      ]
    },
    "tags": { "tag": [] },
    "timezone": "UTC",
    "title": "My Debian server",
    "uuid": "00c78863-db86-44ea-af70-d6edc4d162bf",
    "video_model": "cirrus",
    "remote_access_enabled": "no",
    "remote_access_password": "aabbccdd",
    "zone": "fi-hel1"
  }
}"#;
        let resp: ServerResponse =
            serde_json::from_str(json).expect("deserialize restart response");
        let s = resp.server;
        // Top-level scalar fields
        assert_eq!(s.uuid, "00c78863-db86-44ea-af70-d6edc4d162bf");
        assert_eq!(s.hostname, "debian.example.com");
        assert_eq!(s.title, "My Debian server");
        assert_eq!(s.state, ServerState::Started);
        assert_eq!(s.zone, "fi-hel1");
        assert_eq!(s.plan, "2xCPU-4GB");
        assert_eq!(s.core_number, 2);
        assert_eq!(s.memory_amount, 4096);
        assert_eq!(s.host, Some(8055964291));
        assert_eq!(s.firewall.as_deref(), Some("on"));
        assert_eq!(s.boot_order.as_deref(), Some("disk"));
        assert_eq!(s.nic_model, Some(NicModel::Virtio));
        assert_eq!(s.timezone.as_deref(), Some("UTC"));
        assert_eq!(s.video_model, Some(VideoModel::Cirrus));
        assert_eq!(s.license, Some(0.0));
        assert_eq!(s.plan_ipv4_bytes.as_deref(), Some("34543646754"));
        assert_eq!(s.plan_ipv6_bytes.as_deref(), Some("242432"));
        assert_eq!(s.remote_access_enabled.as_deref(), Some("no"));
        assert_eq!(s.remote_access_password.as_deref(), Some("aabbccdd"));
        // ip_addresses
        let ips = s.ip_addresses.as_ref().unwrap();
        assert_eq!(ips.ip_address.len(), 3);
        assert_eq!(ips.ip_address[0].access, "utility");
        assert_eq!(ips.ip_address[0].address, "10.0.0.0");
        assert_eq!(ips.ip_address[0].family, "IPv4");
        assert_eq!(ips.ip_address[1].access, "public");
        assert_eq!(ips.ip_address[2].family, "IPv6");
        // storage_devices (note: "plart_of_plan" typo in JSON is ignored by serde)
        let sd = s.storage_devices.as_ref().unwrap();
        assert_eq!(sd.storage_device.len(), 1);
        assert_eq!(sd.storage_device[0].address, "virtio:0");
        assert_eq!(
            sd.storage_device[0].storage.as_deref(),
            Some("0169b4f8-051c-4a86-9484-f5b798249949")
        );
        assert_eq!(sd.storage_device[0].storage_size, Some(10));
        assert_eq!(sd.storage_device[0].storage_tier.as_deref(), Some("maxiops"));
        assert_eq!(
            sd.storage_device[0].storage_title.as_deref(),
            Some("Debian from a template")
        );
        assert_eq!(sd.storage_device[0].device_type.as_deref(), Some("disk"));
        // tags & labels
        assert!(s.tags.as_ref().unwrap().tag.is_empty());
        assert!(s.labels.as_ref().unwrap().label.is_empty());
    }

    /// Modify server request (PUT /1.3/server/{uuid}).
    #[test]
    fn deserialize_modify_server_request() {
        let json = r#"{
  "server": {
    "core_number": "8",
    "memory_amount": "16384",
    "plan": "custom"
  }
}"#;
        let body: ModifyServerRequestBody =
            serde_json::from_str(json).expect("deserialize modify request");
        let s = body.server;
        assert_eq!(s.core_number, Some(8));
        assert_eq!(s.memory_amount, Some(16384));
        assert_eq!(s.plan.as_deref(), Some("custom"));
    }

    /// Modify server response (PUT /1.3/server/{uuid} — 202 Accepted).
    #[test]
    fn deserialize_modify_server_response() {
        // Note: docs typo "plart_of_plan" — serde ignores unknown fields.
        let json = r#"{
  "server": {
    "boot_order": "disk",
    "core_number": "8",
    "firewall": "on",
    "hostname": "debian.example.com",
    "host": 8055964291,
    "ip_addresses": {
      "ip_address": [
        { "access": "utility", "address": "10.0.0.0",  "family": "IPv4" },
        { "access": "public",  "address": "x.x.x.x",   "family": "IPv4" },
        { "access": "public",  "address": "xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx", "family": "IPv6" }
      ]
    },
    "labels": { "label": [] },
    "license": 0,
    "memory_amount": "16384",
    "nic_model": "virtio",
    "plan": "custom",
    "plan_ipv4_bytes": "0",
    "plan_ipv6_bytes": "0",
    "simple_backup": "no",
    "state": "stopped",
    "storage_devices": {
      "storage_device": [
        {
          "address": "virtio:0",
          "plart_of_plan": "yes",
          "storage": "0169b4f8-051c-4a86-9484-f5b798249949",
          "storage_size": "10",
          "storage_tier": "maxiops",
          "storage_title": "Debian from a template",
          "type": "disk",
          "boot_disk": "0"
        }
      ]
    },
    "tags": { "tag": [] },
    "timezone": "UTC",
    "title": "My Debian server",
    "uuid": "00c78863-db86-44ea-af70-d6edc4d162bf",
    "video_model": "cirrus",
    "remote_access_enabled": "no",
    "remote_access_password": "aabbccdd",
    "zone": "fi-hel1"
  }
}"#;
        let resp: ServerResponse = serde_json::from_str(json).expect("deserialize modify response");
        let s = resp.server;
        // Top-level scalar fields
        assert_eq!(s.uuid, "00c78863-db86-44ea-af70-d6edc4d162bf");
        assert_eq!(s.hostname, "debian.example.com");
        assert_eq!(s.title, "My Debian server");
        assert_eq!(s.state, ServerState::Stopped);
        assert_eq!(s.zone, "fi-hel1");
        assert_eq!(s.plan, "custom");
        assert_eq!(s.core_number, 8);
        assert_eq!(s.memory_amount, 16384);
        assert_eq!(s.host, Some(8055964291));
        assert_eq!(s.firewall.as_deref(), Some("on"));
        assert_eq!(s.boot_order.as_deref(), Some("disk"));
        assert_eq!(s.nic_model, Some(NicModel::Virtio));
        assert_eq!(s.timezone.as_deref(), Some("UTC"));
        assert_eq!(s.video_model, Some(VideoModel::Cirrus));
        assert_eq!(s.license, Some(0.0));
        assert_eq!(s.plan_ipv4_bytes.as_deref(), Some("0"));
        assert_eq!(s.plan_ipv6_bytes.as_deref(), Some("0"));
        assert_eq!(s.simple_backup.as_deref(), Some("no"));
        assert_eq!(s.remote_access_enabled.as_deref(), Some("no"));
        assert_eq!(s.remote_access_password.as_deref(), Some("aabbccdd"));
        // ip_addresses
        let ips = s.ip_addresses.as_ref().unwrap();
        assert_eq!(ips.ip_address.len(), 3);
        assert_eq!(ips.ip_address[0].access, "utility");
        assert_eq!(ips.ip_address[0].address, "10.0.0.0");
        assert_eq!(ips.ip_address[0].family, "IPv4");
        assert_eq!(ips.ip_address[1].access, "public");
        assert_eq!(ips.ip_address[2].family, "IPv6");
        // storage_devices
        let sd = s.storage_devices.as_ref().unwrap();
        assert_eq!(sd.storage_device.len(), 1);
        assert_eq!(sd.storage_device[0].address, "virtio:0");
        assert_eq!(
            sd.storage_device[0].storage.as_deref(),
            Some("0169b4f8-051c-4a86-9484-f5b798249949")
        );
        assert_eq!(sd.storage_device[0].storage_size, Some(10));
        assert_eq!(sd.storage_device[0].storage_tier.as_deref(), Some("maxiops"));
        assert_eq!(
            sd.storage_device[0].storage_title.as_deref(),
            Some("Debian from a template")
        );
        assert_eq!(sd.storage_device[0].device_type.as_deref(), Some("disk"));
        assert_eq!(sd.storage_device[0].boot_disk.as_deref(), Some("0"));
        // tags & labels
        assert!(s.tags.as_ref().unwrap().tag.is_empty());
        assert!(s.labels.as_ref().unwrap().label.is_empty());
    }

    /// Server group details response (GET /1.3/server-group/{uuid}).
    #[test]
    fn deserialize_server_group_details() {
        let json = r#"{
  "server_group": {
    "anti_affinity": "yes",
    "anti_affinity_status": [
      { "uuid": "0016dadf-eba4-4331-bd34-a361841f7af1", "status": "met" },
      { "uuid": "00c77bbe-fc0e-436f-a753-37f5b5b76270", "status": "met" }
    ],
    "labels": {
      "label": [
        { "key": "foo", "value": "bar" }
      ]
    },
    "servers": {
      "server": [
        "0016dadf-eba4-4331-bd34-a361841f7af2",
        "00c77bbe-fc0e-436f-a753-37f5b5b76271"
      ]
    },
    "title": "nameserver group",
    "uuid": "0b3d85b4-b3be-46ca-a1e2-3b5d30d60ab0"
  }
}"#;
        let resp: ServerGroupResponse =
            serde_json::from_str(json).expect("deserialize server group details");
        let g = resp.server_group;
        assert_eq!(g.uuid, "0b3d85b4-b3be-46ca-a1e2-3b5d30d60ab0");
        assert_eq!(g.title, "nameserver group");
        assert_eq!(g.anti_affinity.as_deref(), Some("yes"));
        let statuses = g.anti_affinity_status.unwrap();
        assert_eq!(statuses.len(), 2);
        assert_eq!(statuses[0].status, "met");
        let servers = g.servers.unwrap().server;
        assert_eq!(servers.len(), 2);
    }

    /// Full server group list from docs (2 groups, second has anti_affinity_status: null).
    #[test]
    fn deserialize_server_group_list_full() {
        let json = r#"{
  "server_groups": {
    "server_group": [
      {
        "anti_affinity": "yes",
        "anti_affinity_status": [
          { "uuid": "0016dadf-eba4-4331-bd34-a361841f7af1", "status": "met" },
          { "uuid": "00c77bbe-fc0e-436f-a753-37f5b5b76270", "status": "met" }
        ],
        "labels": {
          "label": [
            { "key": "foo", "value": "bar" }
          ]
        },
        "servers": {
          "server": [
            "0016dadf-eba4-4331-bd34-a361841f7af2",
            "00c77bbe-fc0e-436f-a753-37f5b5b76271"
          ]
        },
        "title": "nameserver group",
        "uuid": "0b3d85b4-b3be-46ca-a1e2-3b5d30d60ab0"
      },
      {
        "anti_affinity": "no",
        "anti_affinity_status": null,
        "labels": {
          "label": [
            { "key": "foo", "value": "bar" }
          ]
        },
        "servers": {
          "server": [
            "001789da-3aa4-4431-bd34-b4ad841f23e2"
          ]
        },
        "title": "my group",
        "uuid": "0bebfe54-7532-4ca4-b8c0-0fa294ea9b7a"
      }
    ]
  }
}"#;
        let resp: ServerGroupListResponse =
            serde_json::from_str(json).expect("deserialize full server group list");
        assert_eq!(resp.server_groups.server_group.len(), 2);
        // Group 0
        let g0 = &resp.server_groups.server_group[0];
        assert_eq!(g0.uuid, "0b3d85b4-b3be-46ca-a1e2-3b5d30d60ab0");
        assert_eq!(g0.title, "nameserver group");
        assert_eq!(g0.anti_affinity.as_deref(), Some("yes"));
        let statuses0 = g0.anti_affinity_status.as_ref().unwrap();
        assert_eq!(statuses0.len(), 2);
        assert_eq!(statuses0[0].uuid, "0016dadf-eba4-4331-bd34-a361841f7af1");
        assert_eq!(statuses0[0].status, "met");
        assert_eq!(statuses0[1].uuid, "00c77bbe-fc0e-436f-a753-37f5b5b76270");
        assert_eq!(statuses0[1].status, "met");
        let labels0 = &g0.labels.as_ref().unwrap().label;
        assert_eq!(labels0.len(), 1);
        assert_eq!(labels0[0].key, "foo");
        assert_eq!(labels0[0].value, "bar");
        let servers0 = &g0.servers.as_ref().unwrap().server;
        assert_eq!(servers0.len(), 2);
        assert_eq!(servers0[0], "0016dadf-eba4-4331-bd34-a361841f7af2");
        assert_eq!(servers0[1], "00c77bbe-fc0e-436f-a753-37f5b5b76271");
        // Group 1
        let g1 = &resp.server_groups.server_group[1];
        assert_eq!(g1.uuid, "0bebfe54-7532-4ca4-b8c0-0fa294ea9b7a");
        assert_eq!(g1.title, "my group");
        assert_eq!(g1.anti_affinity.as_deref(), Some("no"));
        assert!(g1.anti_affinity_status.is_none());
        let labels1 = &g1.labels.as_ref().unwrap().label;
        assert_eq!(labels1.len(), 1);
        assert_eq!(labels1[0].key, "foo");
        assert_eq!(labels1[0].value, "bar");
        let servers1 = &g1.servers.as_ref().unwrap().server;
        assert_eq!(servers1.len(), 1);
        assert_eq!(servers1[0], "001789da-3aa4-4431-bd34-b4ad841f23e2");
    }

    /// List servers in a group response (GET /1.3/server-group/{uuid}/servers).
    #[test]
    fn deserialize_list_servers_in_group() {
        let json = r#"{
  "servers": {
    "server": [
      "0016dadf-eba4-4331-bd34-a361841f7af2",
      "00c77bbe-fc0e-436f-a753-37f5b5b76271"
    ]
  }
}"#;
        let resp: ServerGroupServersResponse =
            serde_json::from_str(json).expect("deserialize servers in group");
        assert_eq!(resp.servers.server.len(), 2);
        assert_eq!(
            resp.servers.server[0],
            "0016dadf-eba4-4331-bd34-a361841f7af2"
        );
    }

    /// Create server group request (POST /1.3/server-group).
    #[test]
    fn deserialize_create_server_group_request() {
        let json = r#"{
  "server_group": {
    "title": "My brand new server group",
    "servers": {
      "server": [
        "00e2dc05-379d-6b90-a354-e5c3f9293918",
        "00451e12-46ee-41a0-8a52-1aff9039298a",
        "00651e23-44ff-41a0-9a82-2aff9039298b"
      ]
    },
    "labels": {
      "label": [
        { "key": "foo", "value": "bar" }
      ]
    },
    "anti_affinity": "yes"
  }
}"#;
        let body: ServerGroupRequestBody =
            serde_json::from_str(json).expect("deserialize create group request");
        assert_eq!(
            body.server_group.title.as_deref(),
            Some("My brand new server group")
        );
        assert_eq!(body.server_group.anti_affinity.as_deref(), Some("yes"));
        let servers = body.server_group.servers.unwrap().server;
        assert_eq!(servers.len(), 3);
        assert_eq!(servers[0], "00e2dc05-379d-6b90-a354-e5c3f9293918");
    }

    /// Modify server group request (PATCH /1.3/server-group/{uuid}).
    #[test]
    fn deserialize_modify_server_group_request() {
        let json = r#"{
  "server_group": {
    "title": "New fancy title for the group",
    "servers": {
      "server": [
        "00e2dc05-379d-6b90-a354-e5c3f9293918",
        "00451e12-46ee-41a0-8a52-1aff9039298a"
      ]
    },
    "anti_affinity": "no"
  }
}"#;
        let body: ServerGroupRequestBody =
            serde_json::from_str(json).expect("deserialize modify group request");
        assert_eq!(
            body.server_group.title.as_deref(),
            Some("New fancy title for the group")
        );
        assert_eq!(body.server_group.anti_affinity.as_deref(), Some("no"));
        let servers = body.server_group.servers.unwrap().server;
        assert_eq!(servers.len(), 2);
    }

    /// Add server to group request (POST /1.3/server-group/{uuid}/servers).
    #[test]
    fn deserialize_add_server_to_group_request() {
        let json = r#"{
  "server": {
    "uuid": "00235adf-add4-66f1-ef34-e331516bb6a1"
  }
}"#;
        let v: serde_json::Value = serde_json::from_str(json).expect("valid JSON");
        assert_eq!(v["server"]["uuid"], "00235adf-add4-66f1-ef34-e331516bb6a1");
    }

    /// Relocate server request (POST /1.3/server/{uuid}/relocate).
    #[test]
    fn deserialize_relocate_server_request() {
        let json = r#"{
  "zone": "fi-priv-example"
}"#;
        let v: serde_json::Value = serde_json::from_str(json).expect("valid JSON");
        assert_eq!(v["zone"], "fi-priv-example");
    }

    /// Relocate server response (POST /1.3/server/{uuid}/relocate — 200 OK).
    #[test]
    fn deserialize_relocate_server_response() {
        // Note: docs typo "plart_of_plan" — serde ignores unknown fields.
        let json = r#"{
  "server": {
    "boot_order": "disk",
    "core_number": "2",
    "firewall": "on",
    "hostname": "debian.example.com",
    "ip_addresses": {
      "ip_address": [
        { "access": "utility", "address": "x.x.x.x", "family": "IPv4" },
        { "access": "public",  "address": "x.x.x.x", "family": "IPv4" },
        { "access": "public",  "address": "xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx", "family": "IPv6" }
      ]
    },
    "labels": { "label": [] },
    "license": 0,
    "memory_amount": "4096",
    "nic_model": "virtio",
    "plan": "2xCPU-4GB",
    "plan_ipv4_bytes": "435376546",
    "plan_ipv6_bytes": "0",
    "remote_access_enabled": "no",
    "remote_access_password": "aabbccdd",
    "state": "stopped",
    "storage_devices": {
      "storage_device": [
        {
          "address": "virtio:0",
          "plart_of_plan": "yes",
          "storage": "0169b4f8-051c-4a86-9484-f5b798249949",
          "storage_size": "10",
          "storage_tier": "maxiops",
          "storage_title": "Debian from a template",
          "type": "disk",
          "boot_disk": "0"
        }
      ]
    },
    "tags": { "tag": [] },
    "timezone": "UTC",
    "title": "My Debian server",
    "uuid": "00c78863-db86-44ea-af70-d6edc4d162bf",
    "video_model": "cirrus",
    "zone": "fi-priv-example"
  }
}"#;
        let resp: ServerResponse =
            serde_json::from_str(json).expect("deserialize relocate response");
        let s = resp.server;
        // Top-level scalar fields
        assert_eq!(s.uuid, "00c78863-db86-44ea-af70-d6edc4d162bf");
        assert_eq!(s.hostname, "debian.example.com");
        assert_eq!(s.title, "My Debian server");
        assert_eq!(s.state, ServerState::Stopped);
        assert_eq!(s.zone, "fi-priv-example");
        assert_eq!(s.plan, "2xCPU-4GB");
        assert_eq!(s.core_number, 2);
        assert_eq!(s.memory_amount, 4096);
        assert_eq!(s.firewall.as_deref(), Some("on"));
        assert_eq!(s.boot_order.as_deref(), Some("disk"));
        assert_eq!(s.nic_model, Some(NicModel::Virtio));
        assert_eq!(s.timezone.as_deref(), Some("UTC"));
        assert_eq!(s.video_model, Some(VideoModel::Cirrus));
        assert_eq!(s.license, Some(0.0));
        assert_eq!(s.plan_ipv4_bytes.as_deref(), Some("435376546"));
        assert_eq!(s.plan_ipv6_bytes.as_deref(), Some("0"));
        assert_eq!(s.remote_access_enabled.as_deref(), Some("no"));
        assert_eq!(s.remote_access_password.as_deref(), Some("aabbccdd"));
        // ip_addresses
        let ips = s.ip_addresses.as_ref().unwrap();
        assert_eq!(ips.ip_address.len(), 3);
        assert_eq!(ips.ip_address[0].access, "utility");
        assert_eq!(ips.ip_address[0].family, "IPv4");
        assert_eq!(ips.ip_address[1].access, "public");
        assert_eq!(ips.ip_address[2].family, "IPv6");
        // storage_devices
        let sd = s.storage_devices.as_ref().unwrap();
        assert_eq!(sd.storage_device.len(), 1);
        assert_eq!(sd.storage_device[0].address, "virtio:0");
        assert_eq!(
            sd.storage_device[0].storage.as_deref(),
            Some("0169b4f8-051c-4a86-9484-f5b798249949")
        );
        assert_eq!(sd.storage_device[0].storage_size, Some(10));
        assert_eq!(sd.storage_device[0].storage_tier.as_deref(), Some("maxiops"));
        assert_eq!(
            sd.storage_device[0].storage_title.as_deref(),
            Some("Debian from a template")
        );
        assert_eq!(sd.storage_device[0].device_type.as_deref(), Some("disk"));
        assert_eq!(sd.storage_device[0].boot_disk.as_deref(), Some("0"));
        // tags & labels
        assert!(s.tags.as_ref().unwrap().tag.is_empty());
        assert!(s.labels.as_ref().unwrap().label.is_empty());
    }

    /// Metadata service JSON response (curl http://169.254.169.254/metadata/v1.json).
    #[test]
    fn deserialize_metadata_service_response() {
        let json = r#"{
  "cloud_name": "upcloud",
  "instance_id": "00bf9504-a4cb-4839-88ff-124a2c95e169",
  "hostname": "metadata.example.com",
  "platform": "servers",
  "subplatform": "metadata (http://169.254.169.254)",
  "public_keys": [
    "ssh-rsa AAAAB3 user@example.com"
  ],
  "region": "de-fra1",
  "network": {
    "interfaces": [
      {
        "index": 1,
        "ip_addresses": [
          {
            "address": "94.237.90.209",
            "dhcp": true,
            "dns": ["94.237.127.9", "94.237.40.9"],
            "family": "IPv4",
            "floating": false,
            "gateway": "94.237.90.1",
            "network": "94.237.90.0/24"
          }
        ],
        "mac": "de:ad:be:ef:3f:c5",
        "network_id": "03030473-8e9d-4f4f-bcfe-b2c300391a07",
        "type": "public"
      },
      {
        "index": 2,
        "ip_addresses": [
          {
            "address": "10.199.12.11",
            "dhcp": true,
            "dns": null,
            "family": "IPv4",
            "floating": false,
            "gateway": "10.199.12.1",
            "network": "10.199.12.0/24"
          }
        ],
        "mac": "de:ad:be:ef:9f:ff",
        "network_id": "03318153-4e70-4ba5-8e74-69538582188d",
        "type": "utility"
      },
      {
        "index": 3,
        "ip_addresses": [
          {
            "address": "2a04:3540:1000:811:9809:21ff:fe8b:5962",
            "dhcp": true,
            "dns": ["2a04:3540:53::1", "2a04:3544:53::1"],
            "family": "IPv6",
            "floating": false,
            "gateway": "2a04:3540:1000:811::1",
            "network": "2a04:3540:1000:811::/64"
          }
        ],
        "mac": "9a:09:21:8b:59:62",
        "network_id": "03000000-0000-4000-8002-000000000000",
        "type": "public"
      }
    ],
    "dns": ["94.237.127.9", "94.237.40.9"]
  },
  "storage": {
    "disks": [
      {
        "id": "0187b8c5-7220-4c90-9026-047dff8be0b3",
        "serial": "0187b8c572204c909026",
        "size": 25,
        "type": "disk",
        "tier": "maxiops"
      }
    ]
  },
  "tags": ["dev", "metadata"],
  "user_data": "apt-get update && apt-get -y upgrade",
  "vendor_data": ""
}"#;
        let v: serde_json::Value = serde_json::from_str(json).expect("valid metadata JSON");
        // Top-level fields
        assert_eq!(v["cloud_name"], "upcloud");
        assert_eq!(v["instance_id"], "00bf9504-a4cb-4839-88ff-124a2c95e169");
        assert_eq!(v["hostname"], "metadata.example.com");
        assert_eq!(v["platform"], "servers");
        assert_eq!(
            v["subplatform"],
            "metadata (http://169.254.169.254)"
        );
        assert_eq!(v["region"], "de-fra1");
        assert_eq!(
            v["user_data"],
            "apt-get update && apt-get -y upgrade"
        );
        assert_eq!(v["vendor_data"], "");
        // public_keys
        let pkeys = v["public_keys"].as_array().unwrap();
        assert_eq!(pkeys.len(), 1);
        assert_eq!(pkeys[0], "ssh-rsa AAAAB3 user@example.com");
        // tags
        let tags = v["tags"].as_array().unwrap();
        assert_eq!(tags.len(), 2);
        assert_eq!(tags[0], "dev");
        assert_eq!(tags[1], "metadata");
        // network.interfaces
        let ifaces = v["network"]["interfaces"].as_array().unwrap();
        assert_eq!(ifaces.len(), 3);
        assert_eq!(ifaces[0]["index"], 1);
        assert_eq!(ifaces[0]["type"], "public");
        assert_eq!(ifaces[0]["mac"], "de:ad:be:ef:3f:c5");
        assert_eq!(
            ifaces[0]["network_id"],
            "03030473-8e9d-4f4f-bcfe-b2c300391a07"
        );
        let ip0 = &ifaces[0]["ip_addresses"][0];
        assert_eq!(ip0["address"], "94.237.90.209");
        assert_eq!(ip0["dhcp"], true);
        assert_eq!(ip0["family"], "IPv4");
        assert_eq!(ip0["floating"], false);
        assert_eq!(ip0["gateway"], "94.237.90.1");
        assert_eq!(ip0["network"], "94.237.90.0/24");
        let dns0 = ip0["dns"].as_array().unwrap();
        assert_eq!(dns0.len(), 2);
        assert_eq!(dns0[0], "94.237.127.9");
        assert_eq!(ifaces[1]["index"], 2);
        assert_eq!(ifaces[1]["type"], "utility");
        assert!(ifaces[1]["ip_addresses"][0]["dns"].is_null());
        assert_eq!(ifaces[2]["index"], 3);
        assert_eq!(ifaces[2]["type"], "public");
        assert_eq!(ifaces[2]["ip_addresses"][0]["family"], "IPv6");
        // network.dns
        let net_dns = v["network"]["dns"].as_array().unwrap();
        assert_eq!(net_dns.len(), 2);
        assert_eq!(net_dns[0], "94.237.127.9");
        assert_eq!(net_dns[1], "94.237.40.9");
        // storage.disks
        let disks = v["storage"]["disks"].as_array().unwrap();
        assert_eq!(disks.len(), 1);
        assert_eq!(disks[0]["id"], "0187b8c5-7220-4c90-9026-047dff8be0b3");
        assert_eq!(disks[0]["serial"], "0187b8c572204c909026");
        assert_eq!(disks[0]["size"], 25);
        assert_eq!(disks[0]["type"], "disk");
        assert_eq!(disks[0]["tier"], "maxiops");
    }
}
