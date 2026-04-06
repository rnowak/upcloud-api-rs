//! Permission types.

use serde::{Deserialize, Serialize};

/// Resource type for permission targeting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetType {
    /// Server resource.
    Server,
    /// Storage resource.
    Storage,
    /// Network resource.
    Network,
    /// Router resource.
    Router,
    /// IP address resource.
    IpAddress,
    /// Network gateway resource.
    NetworkGateway,
    /// Managed database resource.
    ManagedDatabase,
    /// Managed Kubernetes cluster.
    ManagedKubernetes,
    /// Managed load balancer.
    ManagedLoadbalancer,
    /// Managed object storage service.
    ManagedObjectStorage,
    /// Tag-based access.
    TagAccess,
}

/// A permission entry granting a user access to a resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    /// UUID of the target resource.
    pub target_identifier: String,
    /// Type of the target resource.
    pub target_type: TargetType,
    /// Username of the grantee.
    pub user: String,
    /// Additional permission options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
}

/// Request body for granting a permission.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantRequest {
    /// UUID of the target resource.
    pub target_identifier: String,
    /// Type of the target resource.
    pub target_type: TargetType,
    /// Username to grant access to.
    pub user: String,
    /// Additional permission options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
}

/// Request body for revoking a permission.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeRequest {
    /// UUID of the target resource.
    pub target_identifier: String,
    /// Type of the target resource.
    pub target_type: TargetType,
    /// Username to revoke access from.
    pub user: String,
}
