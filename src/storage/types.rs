//! Public types for the storage module.

use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, PickFirst, serde_as};

use crate::types::common::YesNo;

// ─── Enums ────────────────────────────────────────────────────────────────────

/// Storage resource state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum State {
    /// Storage is available and operational.
    Online,
    /// Storage is under maintenance.
    Maintenance,
    /// Storage is being cloned.
    Cloning,
    /// Storage backup is in progress.
    Backuping,
    /// Storage is in an error state.
    Error,
    /// Storage is syncing (e.g. after resize).
    Syncing,
}

/// Storage performance tier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tier {
    /// HDD-backed storage.
    Hdd,
    /// MaxIOPS (SSD-backed, legacy name).
    Maxiops,
    /// Standard SSD-backed storage.
    Standard,
}

/// Storage type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    /// Regular storage.
    Normal,
    /// Disk image.
    Disk,
    /// Backup of another storage.
    Backup,
    /// CD-ROM image.
    Cdrom,
    /// Public or private template.
    Template,
}

/// Storage access level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Access {
    /// Publicly visible (e.g. OS templates).
    Public,
    /// Visible only to the account owner.
    Private,
}

/// State of a storage import operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageImportState {
    /// Import slot has been prepared.
    Prepared,
    /// Import is pending start.
    Pending,
    /// Import is in progress.
    Importing,
    /// Import completed successfully.
    Completed,
    /// Import failed.
    Failed,
    /// Import is being cancelled.
    Cancelling,
    /// Import was cancelled.
    Cancelled,
}

// ─── Types ────────────────────────────────────────────────────────────────────

/// A storage resource.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Storage {
    pub uuid: String,
    pub title: String,
    pub access: Access,
    pub state: State,
    /// Storage type (serialised as `"type"` in JSON).
    #[serde(rename = "type")]
    pub storage_type: Kind,
    /// Size in GiB.
    #[serde_as(as = "PickFirst<(DisplayFromStr, _)>")]
    pub size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<Tier>,
    /// Data center zone (e.g. `"de-fra1"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
    /// UUID of the origin storage (set for clones/backups).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// Whether this storage is included in a server plan.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_of_plan: Option<bool>,
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// Creation timestamp (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// Per-hour license cost.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    /// Template type (e.g. `"linux"`, `"windows"`); set for templates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_type: Option<String>,
    /// Operation progress percentage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
}

/// Full storage details returned by get, create, and modify endpoints.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StorageDetails {
    /// Base storage fields (flattened).
    #[serde(flatten)]
    pub storage: Storage,
    /// Servers this storage is attached to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<StorageServers>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::types::common::deserialize_empty_string_as_none"
    )]
    pub backup_rule: Option<BackupRule>,
    /// UUIDs of backups created from this storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backups: Option<StorageBackups>,
}

impl std::ops::Deref for StorageDetails {
    type Target = Storage;
    fn deref(&self) -> &Storage {
        &self.storage
    }
}

/// Wrapper for server UUIDs that a storage is attached to.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StorageServers {
    #[serde(default)]
    pub server: Vec<String>,
}

/// Wrapper for backup UUIDs of a storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StorageBackups {
    #[serde(default)]
    pub backup: Vec<String>,
}

/// Automatic backup schedule rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BackupRule {
    /// Backup interval (`"daily"`, `"mon"`, `"tue"`, etc.).
    pub interval: String,
    /// Time of day for the backup in `HHMM` format (UTC).
    #[serde(alias = "hour")]
    pub time: String,
    /// Number of backups to retain.
    pub retention: String,
}

/// Details of a storage import operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ImportDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    /// MD5 checksum of the imported data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5sum: Option<String>,
    /// Number of bytes read from the source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_bytes: Option<u64>,
    /// SHA-256 checksum of the imported data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256sum: Option<String>,
    /// Import source type (e.g. `"http_import"` or `"direct_upload"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// URL of the source for HTTP imports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<StorageImportState>,
    /// Number of bytes written to the storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub written_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// Content length reported by the client (direct uploads).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_content_length: Option<u64>,
    /// Content type reported by the client (direct uploads).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_content_type: Option<String>,
    /// Completion timestamp (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,
    /// Creation timestamp (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// UUID of the import operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    /// Pre-signed URL for direct file uploads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_upload_url: Option<String>,
}

// ─── Request types ────────────────────────────────────────────────────────────

/// Request to create a new storage resource.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequest {
    /// Size in GiB.
    #[serde_as(as = "DisplayFromStr")]
    pub size: u64,
    pub tier: Tier,
    pub title: String,
    /// Data center zone (e.g. `"de-fra1"`).
    pub zone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_rule: Option<BackupRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

/// Request to modify an existing storage resource.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// New size in GiB (can only be increased).
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_rule: Option<BackupRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

/// Request to clone an existing storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloneRequest {
    pub title: String,
    /// Target data center zone.
    pub zone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<Tier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
}

/// Request to templatize a storage (create a template from it).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplatizeRequest {
    pub title: String,
}

/// Request to create a backup of a storage.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateBackupRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// Request to import data into a storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateImportRequest {
    /// Import source type (e.g. `"http_import"` or `"direct_upload"`).
    pub source: String,
    /// URL to import from (required for `http_import`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_location: Option<String>,
    /// Expected SHA-256 checksum of the imported data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256sum: Option<String>,
    /// Expected MD5 checksum of the imported data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5sum: Option<String>,
}

/// Backup metadata returned by a resize-filesystem operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ResizeFilesystemBackup {
    pub uuid: String,
    pub title: String,
    /// Storage type.
    #[serde(rename = "type")]
    pub storage_type: String,
    pub state: String,
    /// Size in GiB.
    pub size: u64,
    pub zone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    /// UUID of the origin storage.
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
/// use upcloud_api::storage::ListParams;
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
