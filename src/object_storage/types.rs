use serde::{Deserialize, Serialize};

// ─── Enums ────────────────────────────────────────────────────────────────────

/// Object storage service lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OperationalState {
    /// Service is being created.
    Pending,
    /// Configuring DNS.
    SetupDns,
    /// Provisioning network.
    SetupNetwork,
    /// Setting up servers.
    SetupServer,
    /// Starting the service.
    SetupService,
    /// Service is operational.
    Running,
    /// Removing DNS records.
    DeleteDns,
    /// Removing network.
    DeleteNetwork,
    /// Removing servers.
    DeleteServer,
    /// Deleting the service.
    DeleteService,
}

/// Desired service status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfiguredStatus {
    /// Service should be running.
    Started,
    /// Service should be stopped.
    Stopped,
}

/// Status of a user access key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserAccessKeyStatus {
    /// Key is active.
    Active,
    /// Key is inactive.
    Inactive,
}

// ─── Response types ───────────────────────────────────────────────────────────

/// A zone within an object storage region.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RegionZone {
    pub name: String,
}

/// An object storage region.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Region {
    pub name: String,
    /// Primary availability zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_zone: Option<String>,
    /// Availability zones in the region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zones: Option<Vec<RegionZone>>,
    /// S3-compatible endpoint URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
}

/// A managed object storage service instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Service {
    pub uuid: String,
    pub name: String,
    pub region: String,
    /// Desired status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_status: Option<String>,
    /// Current lifecycle state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_state: Option<String>,
    /// Service endpoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<Endpoint>>,
    /// Attached networks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<Network>>,
    /// Service users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<UserSummary>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    /// Storage usage statistics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
    /// Status messages from the backend.
    #[serde(default)]
    pub state_messages: Vec<serde_json::Value>,
    /// Whether termination protection is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protection: Option<bool>,
    /// Custom domain mappings.
    #[serde(default)]
    pub custom_domains: Vec<serde_json::Value>,
    /// Static website configurations.
    #[serde(default)]
    pub static_websites: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Storage usage statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Usage {
    /// Total number of objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_objects: Option<u64>,
    /// Total size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size_bytes: Option<u64>,
}

/// A service endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Endpoint {
    /// Endpoint hostname.
    pub domain_name: String,
    /// Endpoint type.
    #[serde(rename = "type")]
    pub endpoint_type: String,
    /// Access mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// IAM endpoint URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_url: Option<String>,
    /// STS endpoint URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sts_url: Option<String>,
}

/// A network attached to an object storage service.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Network {
    /// Address family.
    pub family: String,
    pub name: String,
    /// Network type.
    #[serde(rename = "type")]
    pub network_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

/// Brief user summary (username only).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UserSummary {
    pub username: String,
}

/// An object storage service user.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct User {
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Amazon Resource Name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// User access keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_keys: Option<Vec<UserAccessKey>>,
    /// Attached IAM policies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<UserPolicy>>,
}

/// An IAM policy attached to a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UserPolicy {
    pub name: String,
    /// Policy ARN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

/// An access key for an object storage user.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UserAccessKey {
    pub access_key_id: String,
    /// Secret key (only returned at creation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Last usage timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<String>,
}

/// An IAM policy for object storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Policy {
    pub name: String,
    /// JSON policy document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    /// Whether this is a system-managed policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,
    /// Policy ARN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Number of users this policy is attached to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_count: Option<u64>,
    /// Default policy version ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version_id: Option<String>,
}

/// An object storage bucket.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Bucket {
    pub name: String,
    /// Number of objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_objects: Option<u64>,
    /// Total size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

/// A custom domain mapping for object storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomDomain {
    pub name: String,
    /// Custom domain name.
    pub domain_name: String,
    /// Domain verification status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

/// A version of an IAM policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PolicyVersion {
    /// Version identifier.
    pub version_id: String,
    /// JSON policy document.
    pub document: String,
    /// Whether this is the default version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_version: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

/// Aggregated object storage metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Metrics {
    /// Total storage bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bytes: Option<u64>,
    /// Total object count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_objects: Option<u64>,
    /// Per-bucket metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buckets: Option<Vec<BucketMetrics>>,
}

/// Metrics for a single bucket.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BucketMetrics {
    pub name: String,
    /// Total bytes in bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<u64>,
    /// Total objects in bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objects: Option<u64>,
}

// ─── Request types ────────────────────────────────────────────────────────────

/// Network reference for service creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NetworkInput {
    /// Address family.
    pub family: String,
    pub name: String,
    /// Network type.
    #[serde(rename = "type")]
    pub network_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

/// Request body for creating an object storage service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequest {
    pub name: String,
    pub region: String,
    /// Networks to attach.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub networks: Vec<NetworkInput>,
    /// Desired initial status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    /// Enable termination protection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protection: Option<bool>,
}

/// Request body for modifying an object storage service.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Desired status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    /// Termination protection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protection: Option<bool>,
    /// Networks to attach.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<NetworkInput>>,
}

/// Request body for creating an object storage user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
}

/// Request body for creating an IAM policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePolicyRequest {
    pub name: String,
    /// JSON policy document.
    pub document: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Request body for creating a custom domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomDomainRequest {
    /// The custom domain name.
    pub domain_name: String,
}

/// Request body for modifying a custom domain.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyCustomDomainRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

/// Request body for creating a bucket.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBucketRequest {
    pub name: String,
}

/// Request body for creating a new policy version.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePolicyVersionRequest {
    /// JSON policy document.
    pub document: String,
}

/// Request body for modifying an access key status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyUserAccessKeyRequest {
    /// New key status.
    pub status: UserAccessKeyStatus,
}
