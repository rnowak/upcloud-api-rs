use serde::{Deserialize, Serialize};

// Re-export the shared ConfiguredStatus enum.
pub use crate::types::common::ConfiguredStatus;

// ─── Enums ────────────────────────────────────────────────────────────────────

/// Access permission for a file storage share ACL entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ShareACLPermission {
    Ro,
    Rw,
}

// ─── Types ────────────────────────────────────────────────────────────────────

/// File storage service plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Plan {
    /// Plan name identifier.
    pub name: String,
    /// Monthly price in EUR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_month_price_eur: Option<f64>,
    /// Hourly price in EUR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_hour_price_eur: Option<f64>,
    /// Maximum storage size in GiB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size_gib: Option<u64>,
}

/// An ACL entry for a file storage share (allows a specific IP/CIDR to mount).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Acl {
    /// ACL entry name.
    pub name: String,
    /// The IP address or CIDR block allowed to access this share.
    pub target: String,
    /// Access permission (`ro` or `rw`).
    pub permission: ShareACLPermission,
}

/// An NFS export share within a file storage service.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Share {
    /// Share name.
    pub name: String,
    /// Share path (e.g. `/data`).
    pub path: String,
    /// Access control list entries.
    #[serde(default)]
    pub acl: Vec<Acl>,
    /// Whether the share is being deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleting: Option<bool>,
}

/// A network attached to a file storage service.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Network {
    /// Address family.
    pub family: String,
    /// Network name.
    pub name: String,
    /// Network type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// Network UUID.
    pub uuid: String,
    /// DNS hostname for the mount target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    /// IP address of the mount target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

/// A file storage service instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Service {
    /// Unique identifier for the file storage service.
    pub uuid: String,
    /// Service name (unique within account).
    pub name: String,
    /// Zone in which the service is hosted.
    pub zone: String,
    /// Service plan name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    /// Desired status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_status: Option<ConfiguredStatus>,
    /// Current lifecycle state.
    #[serde(alias = "state")]
    pub operational_state: Option<String>,
    /// Storage size in GiB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_gib: Option<u64>,
    /// Whether storage is encrypted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// NFS shares.
    #[serde(default)]
    pub shares: Vec<Share>,
    /// Attached networks.
    #[serde(default)]
    pub networks: Vec<Network>,
    /// Key-value labels for service filtering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    /// Creation timestamp (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Last update timestamp (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Status messages.
    #[serde(default)]
    pub state_messages: Vec<serde_json::Value>,
}

// ─── Request types ────────────────────────────────────────────────────────────

/// Request body for creating a file storage service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequest {
    /// Service name (unique within account).
    pub name: String,
    /// Zone in which the service will be hosted.
    pub zone: String,
    /// Service plan name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    /// Storage size in GiB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_gib: Option<u64>,
    /// Enable encryption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// Networks to attach.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub networks: Vec<NetworkInput>,
    /// Key-value labels for service filtering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    /// Desired service status (`started` / `stopped`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_status: Option<String>,
    /// Initial NFS shares.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub shares: Vec<CreateShareRequest>,
}

/// Network reference for file storage creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NetworkInput {
    /// Address family.
    pub family: String,
    /// Network name.
    pub name: String,
    /// Network type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    /// Network UUID.
    pub uuid: String,
    /// Desired IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

/// Request body for modifying a file storage service (PATCH).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyRequest {
    /// Service name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New storage size in GiB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_gib: Option<u64>,
    /// Desired service status (`started` / `stopped`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_status: Option<String>,
    /// Networks to attach.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<NetworkInput>>,
    /// Key-value labels for service filtering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

/// Request body for replacing a file storage service (PUT).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplaceRequest {
    /// Service name.
    pub name: String,
    /// Desired service status (`started` / `stopped`).
    pub configured_status: String,
    /// Storage size in GiB.
    pub size_gib: u64,
    /// Networks to attach.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub networks: Vec<NetworkInput>,
    /// Key-value labels for service filtering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

/// Request body for creating an NFS share.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateShareRequest {
    /// Share name.
    pub name: String,
    /// Share path (e.g. `/data`).
    pub path: String,
    /// Access control list entries.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub acl: Vec<CreateShareAcl>,
}

/// ACL entry for share creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateShareAcl {
    /// ACL entry name.
    pub name: String,
    /// Allowed IP/CIDR.
    pub target: String,
    /// Access permission.
    pub permission: ShareACLPermission,
}

/// Request body for modifying an NFS share.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyShareRequest {
    /// Share name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated access control list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl: Option<Vec<CreateShareAcl>>,
}
