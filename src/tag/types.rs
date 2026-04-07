//! Tag types.

use serde::{Deserialize, Serialize};

/// A resource tag for grouping servers.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Tag {
    /// The tag name.
    pub name: String,
    /// Description of the tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Servers associated with this tag.
    #[serde(default)]
    pub servers: TagServers,
}

/// Wrapper for the list of server UUIDs in a tag.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TagServers {
    /// List of server UUIDs.
    #[serde(default)]
    pub server: Vec<String>,
}

/// Request body for creating a tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTagRequest {
    /// The tag name.
    pub name: String,
    /// Description of the tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Servers associated with this tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<TagServers>,
}

/// Request body for modifying a tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyTagRequest {
    /// New tag name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description of the tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Servers associated with this tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<TagServers>,
}
