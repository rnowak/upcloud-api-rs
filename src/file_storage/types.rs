use serde::{Deserialize, Serialize};

// ─── Enums ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfiguredStatus {
    Started,
    Stopped,
}

/// Access permission for a file storage share ACL entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ShareACLPermission {
    Ro,
    Rw,
}

// ─── Types ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_month_price_eur: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_hour_price_eur: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size_gib: Option<u64>,
}

/// An ACL entry for a file storage share (allows a specific IP/CIDR to mount).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Acl {
    pub name: String,
    /// The IP address or CIDR block allowed to access this share.
    pub target: String,
    pub permission: ShareACLPermission,
}

/// An NFS export share within a file storage service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Share {
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub acl: Vec<Acl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleting: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    pub family: String,
    pub name: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    pub uuid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub uuid: String,
    pub name: String,
    pub zone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_status: Option<ConfiguredStatus>,
    #[serde(alias = "state")]
    pub operational_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_gib: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(default)]
    pub shares: Vec<Share>,
    #[serde(default)]
    pub networks: Vec<Network>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub state_messages: Vec<serde_json::Value>,
}

// ─── Request types ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequest {
    pub name: String,
    pub zone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_gib: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub networks: Vec<NetworkInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_status: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub shares: Vec<CreateShareRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInput {
    pub family: String,
    pub name: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    pub uuid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_gib: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateShareRequest {
    pub name: String,
    pub path: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub acl: Vec<CreateShareAcl>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateShareAcl {
    pub name: String,
    pub target: String,
    pub permission: ShareACLPermission,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyShareRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl: Option<Vec<CreateShareAcl>>,
}
