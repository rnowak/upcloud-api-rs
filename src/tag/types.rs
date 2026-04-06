//! Tag types.

use serde::{Deserialize, Serialize};

/// A resource tag for grouping servers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Servers associated with this tag.
    #[serde(default)]
    pub servers: TagServers,
}

/// Wrapper for the list of server UUIDs in a tag.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TagServers {
    /// List of server UUIDs.
    #[serde(default)]
    pub server: Vec<String>,
}

/// Request body for creating a tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTagRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Server UUIDs to associate.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub servers: Vec<String>,
}

/// Request body for modifying a tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyTagRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
