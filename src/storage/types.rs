//! Public types for the storage module.

use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, PickFirst, serde_as};

use crate::types::common::YesNo;

// ─── Enums ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum State {
    Online,
    Maintenance,
    Cloning,
    Backuping,
    Error,
    Syncing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tier {
    Hdd,
    Maxiops,
    Standard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Normal,
    Disk,
    Backup,
    Cdrom,
    Template,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Access {
    Public,
    Private,
}

// ─── Types ────────────────────────────────────────────────────────────────────

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storage {
    pub uuid: String,
    pub title: String,
    pub access: Access,
    pub state: State,
    #[serde(rename = "type")]
    pub storage_type: Kind,
    #[serde_as(as = "PickFirst<(DisplayFromStr, _)>")]
    pub size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<Tier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_of_plan: Option<bool>,
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageDetails {
    #[serde(flatten)]
    pub storage: Storage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<StorageServers>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::types::common::deserialize_empty_string_as_none"
    )]
    pub backup_rule: Option<BackupRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backups: Option<StorageBackups>,
}

impl std::ops::Deref for StorageDetails {
    type Target = Storage;
    fn deref(&self) -> &Storage {
        &self.storage
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageServers {
    #[serde(default)]
    pub server: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageBackups {
    #[serde(default)]
    pub backup: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupRule {
    pub interval: String,
    #[serde(alias = "hour")]
    pub time: String,
    pub retention: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5sum: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256sum: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub written_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_content_length: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_upload_url: Option<String>,
}

// ─── Request types ────────────────────────────────────────────────────────────

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequest {
    #[serde_as(as = "DisplayFromStr")]
    pub size: u64,
    pub tier: Tier,
    pub title: String,
    pub zone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_rule: Option<BackupRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_rule: Option<BackupRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloneRequest {
    pub title: String,
    pub zone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<Tier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplatizeRequest {
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateBackupRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateImportRequest {
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256sum: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5sum: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResizeFilesystemBackup {
    pub uuid: String,
    pub title: String,
    #[serde(rename = "type")]
    pub storage_type: String,
    pub state: String,
    pub size: u64,
    pub zone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
}

// ─── List / filter parameters ─────────────────────────────────────────────────

/// Query parameters for filtering the storage list.
///
/// # Examples
///
/// ```
/// use upcloud_api_rs::storage::ListParams;
///
/// let params = ListParams::new()
///     .label("env", "prod")
///     .label("team", "backend");
/// ```
#[derive(Debug, Clone, Default)]
pub struct ListParams {
    labels: Vec<String>,
}

impl ListParams {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a label filter. Each call adds `key=value` as a separate `?label=` parameter.
    /// Multiple labels are AND-combined by the API.
    pub fn label(mut self, key: &str, value: &str) -> Self {
        self.labels.push(format!("{key}={value}"));
        self
    }

    /// Returns `true` if no filters are set.
    pub fn is_empty(&self) -> bool {
        self.labels.is_empty()
    }

    /// Encode as query parameter key-value pairs.
    pub(crate) fn to_query_pairs(&self) -> Vec<(&str, String)> {
        self.labels.iter().map(|l| ("label", l.clone())).collect()
    }
}
