//! Public types for the managed database module.

use serde::{Deserialize, Serialize};

// ─── Enums ────────────────────────────────────────────────────────────────────

/// Managed database lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum State {
    /// Database is being created.
    Pending,
    /// Provisioning network.
    SetupNetwork,
    /// Verifying network.
    CheckNetwork,
    /// Setting up peer replication.
    SetupPeer,
    /// Verifying peer replication.
    CheckPeer,
    /// Starting the database service.
    SetupService,
    /// Rebuilding database nodes.
    Rebuilding,
    /// Rebalancing data across nodes.
    Rebalancing,
    /// Database is operational.
    Running,
    /// Database is stopped.
    Stopped,
    /// Database encountered an error.
    Error,
    /// Cleaning up the service.
    CleanupService,
    /// Cleaning up the network.
    CleanupNetwork,
    /// Deleting the service.
    DeleteService,
}

/// Time period for database metrics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MetricPeriod {
    /// One hour.
    Hour,
    /// One day.
    Day,
    /// One week.
    Week,
    /// One month.
    Month,
    /// One year.
    Year,
}

/// Database user privilege level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserType {
    /// Primary (superuser) account.
    Primary,
    /// Regular user.
    Normal,
    /// Read-only user.
    Readonly,
}

// ─── Types ────────────────────────────────────────────────────────────────────

/// Preferred maintenance window (day of week + time).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MaintenanceTime {
    /// Day of week (e.g. `"monday"`).
    pub dow: String,
    /// Time in HH:MM:SS format.
    pub time: String,
}

/// A database backup entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Backup {
    pub backup_name: String,
    pub backup_time: String,
    /// Backup size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_size: Option<u64>,
}

/// OpenSearch service-level access control settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OpenSearchAccessControl {
    /// Enable basic access control.
    pub access_control: Option<bool>,
    /// Enable extended access control.
    pub extended_access_control: Option<bool>,
}

/// An access control rule for an OpenSearch index.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OpenSearchAccessControlRule {
    /// Index pattern.
    pub index: String,
    /// Permission level.
    pub permission: String,
}

/// An OpenSearch index with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OpenSearchIndex {
    pub index_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// Document count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs: Option<u64>,
    /// Index health (e.g. `"green"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_replicas: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_shards: Option<u64>,
    /// Whether index is read-only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only_allow_delete: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Per-user OpenSearch access control rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UserOpenSearchAccessControl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<OpenSearchAccessControlRule>>,
}

/// Per-user PostgreSQL access control settings.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PgAccessControl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_replication: Option<bool>,
}

/// Per-user Redis access control settings.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RedisAccessControl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
}

/// Per-user Valkey access control settings.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ValkeyAccessControl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
}

/// A managed database instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Database {
    pub uuid: String,
    pub name: String,
    pub title: String,
    pub plan: String,
    pub zone: String,
    /// Database engine type (e.g. `"pg"`, `"mysql"`).
    #[serde(rename = "type")]
    pub db_type: String,
    pub state: State,
    /// UUID of the primary if this is a read replica.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_database: Option<String>,
    /// Whether access control is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control: Option<bool>,
    /// Whether extended access control is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_access_control: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Connection URI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_uri: Option<String>,
    /// Parsed connection parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_uri_params: Option<serde_json::Value>,
    /// Service components (endpoints).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component>>,
    /// State of each database node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_states: Option<Vec<NodeState>>,
    /// Engine-specific properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    /// Whether the database is powered on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub powered: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Attached networks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<Network>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    /// Whether termination protection is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backups: Option<Vec<Backup>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<MaintenanceTime>,
}

/// A database service component (connection endpoint).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Component {
    /// Component name (e.g. `"pg"`).
    pub component: String,
    /// Hostname.
    pub host: String,
    /// Port number.
    pub port: u16,
    /// Network route (e.g. `"dynamic"`).
    pub route: String,
    /// Usage type (e.g. `"primary"`).
    pub usage: String,
}

/// State of a single database node.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NodeState {
    pub name: String,
    /// Node role (e.g. `"master"`, `"standby"`).
    pub role: String,
    pub state: String,
}

/// A network attached to a managed database.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Network {
    /// Address family (e.g. `"IPv4"`).
    pub family: String,
    pub name: String,
    /// Network type (e.g. `"private"`).
    #[serde(rename = "type")]
    pub network_type: String,
    pub uuid: String,
}

/// A managed database service plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ServicePlan {
    pub name: String,
    /// Number of database nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_count: Option<u32>,
    /// Backup configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_config: Option<serde_json::Value>,
    /// Storage size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_size: Option<u64>,
    /// Max memory usage percentage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_memory_percent: Option<f64>,
    /// Zones where this plan is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zones: Option<Vec<String>>,
}

/// A managed database user.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct User {
    pub username: String,
    /// Privilege level.
    #[serde(rename = "type")]
    pub user_type: UserType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Authentication method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<String>,
    /// PostgreSQL access control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pg_access_control: Option<PgAccessControl>,
    /// Redis access control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_access_control: Option<RedisAccessControl>,
    /// Valkey access control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valkey_access_control: Option<ValkeyAccessControl>,
    /// OpenSearch access control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opensearch_access_control: Option<UserOpenSearchAccessControl>,
}

/// A logical database within a managed database service.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LogicalDatabase {
    pub name: String,
    /// Collation locale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lc_collate: Option<String>,
    /// Character classification locale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lc_ctype: Option<String>,
}

/// An active database session (PostgreSQL).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Session {
    /// Process ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i64>,
    /// Current query text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Time the current query has been running.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Client IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_addr: Option<String>,
    /// Database name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datname: Option<String>,
    /// Username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usename: Option<String>,
}

/// A database extension.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Extension {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// A supported database engine type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DatabaseType {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Latest available engine version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_available_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Detail of a single PostgreSQL extension.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PgExtensionDetail {
    pub name: String,
    /// Default extension version.
    pub default_version: String,
    /// All available versions.
    #[serde(default)]
    pub versions: Vec<String>,
}

/// Extensions available for a specific PostgreSQL version.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PgVersionExtensions {
    pub version: String,
    pub extensions: Vec<PgExtensionDetail>,
}

/// Network reference for database creation/modification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NetworkInput {
    /// Address family.
    pub family: String,
    pub name: String,
    /// Network type.
    #[serde(rename = "type")]
    pub network_type: String,
    pub uuid: String,
}

// ─── Request types ────────────────────────────────────────────────────────────

/// Request body for creating a managed database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequest {
    /// Hostname prefix for the database service.
    pub hostname_prefix: String,
    pub title: String,
    pub plan: String,
    pub zone: String,
    /// Database engine type.
    #[serde(rename = "type")]
    pub db_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub powered: Option<bool>,
    /// Engine-specific properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    /// Networks to attach.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<NetworkInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    /// Enable termination protection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protection: Option<bool>,
    /// Preferred maintenance window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<MaintenanceTime>,
    /// Additional disk space in GiB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_disk_space_gib: Option<u64>,
}

/// Request body for modifying a managed database.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    /// Engine-specific properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    /// Power state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub powered: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    /// Termination protection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protection: Option<bool>,
    /// Target zone for migration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
    /// Networks to attach.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<NetworkInput>>,
    /// Additional disk space in GiB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_disk_space_gib: Option<u64>,
    /// Preferred maintenance window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<MaintenanceTime>,
}

/// Request body for creating a database user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    /// Privilege level.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub user_type: Option<UserType>,
    /// Authentication method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<String>,
    /// PostgreSQL access control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pg_access_control: Option<PgAccessControl>,
    /// Redis access control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_access_control: Option<RedisAccessControl>,
    /// Valkey access control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valkey_access_control: Option<ValkeyAccessControl>,
    /// OpenSearch access control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opensearch_access_control: Option<UserOpenSearchAccessControl>,
}

/// Request body for modifying a database user.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyUserRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Authentication method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<String>,
}

/// Request body for resetting a user password.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetUserPasswordRequest {
    pub password: String,
}

/// Request body for creating a logical database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLogicalDatabaseRequest {
    pub name: String,
    /// Collation locale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lc_collate: Option<String>,
    /// Character classification locale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lc_ctype: Option<String>,
}

/// Query parameters for fetching database logs.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetLogsRequest {
    /// Pagination offset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
    /// Maximum number of entries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Sort order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

/// Query parameters for listing database sessions.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetSessionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
}

/// Request body for terminating long-running queries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminateQueriesRequest {
    /// Terminate queries running longer than this (seconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminate_after: Option<u32>,
}

/// Query parameters for fetching database metrics.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetMetricsRequest {
    /// Metrics time period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<MetricPeriod>,
}

/// Request body for cloning a managed database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloneRequest {
    /// Point-in-time to clone from (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clone_time: Option<String>,
    /// Name of backup to clone from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_name: Option<String>,
    /// Custom hostname prefix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname_prefix: Option<String>,
    pub plan: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub zone: String,
    /// Maintenance window for the clone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<MaintenanceTime>,
    /// Engine-specific properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

/// Request body for upgrading the database engine version.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeVersionRequest {
    /// Target engine version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_version: Option<String>,
}

/// Request body for modifying service-level access control.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyAccessControlRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_access_control: Option<bool>,
}

/// Request body for modifying per-user access control.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyUserAccessControlRequest {
    /// OpenSearch ACL rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opensearch_access_control: Option<UserOpenSearchAccessControl>,
    /// PostgreSQL access control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pg_access_control: Option<PgAccessControl>,
    /// Redis access control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_access_control: Option<RedisAccessControl>,
    /// Valkey access control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valkey_access_control: Option<ValkeyAccessControl>,
}
