//! Public types for the managed database module.

use serde::{Deserialize, Serialize};

// ─── Enums ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum State {
    Pending,
    SetupNetwork,
    CheckNetwork,
    SetupPeer,
    CheckPeer,
    SetupService,
    Rebuilding,
    Rebalancing,
    Running,
    Stopped,
    Error,
    CleanupService,
    CleanupNetwork,
    DeleteService,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MetricPeriod {
    Hour,
    Day,
    Week,
    Month,
    Year,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserType {
    Primary,
    Normal,
    Readonly,
}

// ─── Types ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceTime {
    pub dow: String,
    pub time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Backup {
    pub backup_name: String,
    pub backup_time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenSearchAccessControl {
    pub access_control: Option<bool>,
    pub extended_access_control: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenSearchAccessControlRule {
    pub index: String,
    pub permission: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenSearchIndex {
    pub index_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_replicas: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_shards: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only_allow_delete: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserOpenSearchAccessControl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<OpenSearchAccessControlRule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Database {
    pub uuid: String,
    pub name: String,
    pub title: String,
    pub plan: String,
    pub zone: String,
    #[serde(rename = "type")]
    pub db_type: String,
    pub state: State,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_database: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_access_control: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_uri_params: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_states: Option<Vec<NodeState>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub powered: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<Network>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backups: Option<Vec<Backup>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<MaintenanceTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub component: String,
    pub host: String,
    pub port: u16,
    pub route: String,
    pub usage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeState {
    pub name: String,
    pub role: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    pub family: String,
    pub name: String,
    #[serde(rename = "type")]
    pub network_type: String,
    pub uuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicePlan {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_config: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_memory_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zones: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    #[serde(rename = "type")]
    pub user_type: UserType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pg_allow_replication: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicalDatabase {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lc_collate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lc_ctype: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_addr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usename: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extension {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseType {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_available_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PgExtensionDetail {
    pub name: String,
    pub default_version: String,
    #[serde(default)]
    pub versions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PgVersionExtensions {
    pub version: String,
    pub extensions: Vec<PgExtensionDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInput {
    pub family: String,
    pub name: String,
    #[serde(rename = "type")]
    pub network_type: String,
    pub uuid: String,
}

// ─── Request types ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequest {
    pub name: String,
    pub title: String,
    pub plan: String,
    pub zone: String,
    #[serde(rename = "type")]
    pub db_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub powered: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<NetworkInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_name_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<MaintenanceTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_disk_space_gib: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub powered: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protection: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub user_type: Option<UserType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pg_allow_replication: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyUserRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pg_allow_replication: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetUserPasswordRequest {
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLogicalDatabaseRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lc_collate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lc_ctype: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetLogsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetSessionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminateQueriesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminate_after: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetMetricsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<MetricPeriod>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloneRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clone_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname_prefix: Option<String>,
    pub plan: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub zone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<MaintenanceTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeVersionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyAccessControlRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_access_control: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyUserAccessControlRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opensearch_access_control: Option<UserOpenSearchAccessControl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pg_access_control: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_access_control: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valkey_access_control: Option<serde_json::Value>,
}
