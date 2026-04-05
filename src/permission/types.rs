//! Permission types.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetType {
    Server,
    Storage,
    Network,
    Router,
    IpAddress,
    NetworkGateway,
    ManagedDatabase,
    ManagedKubernetes,
    ManagedLoadbalancer,
    ManagedObjectStorage,
    TagAccess,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub target_identifier: String,
    pub target_type: TargetType,
    pub user: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantRequest {
    pub target_identifier: String,
    pub target_type: TargetType,
    pub user: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeRequest {
    pub target_identifier: String,
    pub target_type: TargetType,
    pub user: String,
}
