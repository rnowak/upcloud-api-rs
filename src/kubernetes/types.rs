//! Public types for the Managed Kubernetes API.

use serde::{Deserialize, Serialize};

// ─── Enums ────────────────────────────────────────────────────────────────────

/// Kubernetes cluster lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClusterState {
    /// Cluster is being provisioned.
    Pending,
    /// Cluster is operational.
    Running,
    /// Cluster is being deleted.
    Terminating,
    /// Cluster has been deleted.
    Terminated,
    /// Cluster provisioning or operation failed.
    Failed,
    /// State could not be determined.
    Unknown,
}

/// Node group lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NodeGroupState {
    /// Node group is being provisioned.
    Pending,
    /// Node group is operational.
    Running,
    /// Nodes are being added.
    ScalingUp,
    /// Nodes are being removed.
    ScalingDown,
    /// Node group is being deleted.
    Terminating,
    /// Node group operation failed.
    Failed,
    /// State could not be determined.
    Unknown,
}

/// Individual node lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeState {
    /// Node is being provisioned.
    Pending,
    /// Node is operational.
    Running,
    /// Node is being removed.
    Terminating,
    /// Node operation failed.
    Failed,
    /// State could not be determined.
    Unknown,
}

/// Kubernetes taint effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaintEffect {
    /// Evict existing pods that do not tolerate the taint.
    NoExecute,
    /// Do not schedule pods that do not tolerate the taint.
    NoSchedule,
    /// Prefer not to schedule intolerant pods.
    PreferNoSchedule,
}

// ─── Types ────────────────────────────────────────────────────────────────────

/// A Kubernetes node group plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Plan {
    /// Plan name identifier.
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Underlying server plan name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_plan: Option<String>,
    /// Maximum nodes allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_nodes: Option<u32>,
    /// Whether this plan is deprecated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
}

/// A managed Kubernetes cluster.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Cluster {
    /// Unique identifier for the cluster.
    pub uuid: String,
    /// Cluster name (unique within account).
    pub name: String,
    /// Current lifecycle state.
    pub state: ClusterState,
    /// Zone in which the cluster is hosted.
    pub zone: String,
    /// Kubernetes version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Private network UUID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// Network CIDR block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_cidr: Option<String>,
    /// Node groups in the cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_groups: Option<Vec<NodeGroup>>,
    /// Key-value labels on the cluster API resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    /// Creation timestamp (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Allowed CIDRs for control plane access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_plane_ip_filter: Option<Vec<String>>,
    /// Control plane plan name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    /// Whether node groups use private networking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_node_groups: Option<bool>,
    /// Storage encryption setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encryption: Option<String>,
}

/// A group of worker nodes in a cluster.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NodeGroup {
    /// Node group name.
    pub name: String,
    /// Server plan for each node.
    pub plan: String,
    /// Desired number of nodes.
    pub count: u32,
    /// Current lifecycle state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<NodeGroupState>,
    /// Whether to spread nodes across hosts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_affinity: Option<bool>,
    /// Key-value labels applied to nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    /// Kubernetes taints applied to nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taints: Option<Vec<Taint>>,
    /// Extra kubelet arguments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubelet_args: Option<Vec<KubeletArg>>,
    /// SSH public keys injected into nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_keys: Option<Vec<String>>,
    /// Whether nodes have utility network access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utility_network_access: Option<bool>,
    /// Storage device plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
    /// Whether anti-affinity is in effect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_affinity_status: Option<bool>,
    /// Individual nodes in this group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<Node>>,
    /// Custom plan configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_plan: Option<CustomPlan>,
    /// Cloud-native plan configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_native_plan: Option<CloudNativePlan>,
    /// GPU plan configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_plan: Option<GPUPlan>,
    /// Storage encryption setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encryption: Option<String>,
}

/// A single worker node.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Node {
    /// Unique identifier for the node.
    pub uuid: String,
    /// Node name.
    pub name: String,
    /// Current lifecycle state.
    pub state: NodeState,
    /// Running kubelet version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubelet_version: Option<String>,
}

/// A Kubernetes node taint (key/value/effect).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Taint {
    /// Taint key.
    pub key: String,
    /// Taint value.
    pub value: String,
    /// Taint effect (NoExecute, NoSchedule, PreferNoSchedule).
    pub effect: String,
}

/// A kubelet command-line argument (key/value pair).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct KubeletArg {
    /// Argument key.
    pub key: String,
    /// Argument value.
    pub value: String,
}

/// The kubeconfig YAML for a cluster.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Kubeconfig {
    pub kubeconfig: String,
}

// ─── Node group plan types ────────────────────────────────────────────────────

/// Custom node group plan with explicit resource allocation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomPlan {
    /// Number of CPU cores.
    pub cores: u32,
    /// Memory in MiB.
    pub memory: u64,
    /// Storage size in GiB.
    pub storage_size: u64,
    /// Storage tier (e.g. `"maxiops"`).
    pub storage_tier: String,
}

/// Cloud-native node group plan configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CloudNativePlan {
    /// Storage size in GiB.
    pub storage_size: u64,
    /// Storage tier.
    pub storage_tier: String,
}

/// GPU node group plan configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GPUPlan {
    /// Storage size in GiB.
    pub storage_size: u64,
    /// Storage tier.
    pub storage_tier: String,
}

// ─── Response types ──────────────────────────────────────────────────────────

/// An available Kubernetes version.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct KubernetesVersion {
    /// Version identifier.
    pub id: String,
    /// Kubernetes version string.
    pub version: String,
}

// ─── Request types ────────────────────────────────────────────────────────────

/// Request body for creating a Kubernetes cluster.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateClusterRequest {
    /// Cluster name (unique within account).
    pub name: String,
    /// Zone in which the cluster will be hosted.
    pub zone: String,
    /// Private network UUID.
    pub network: String,
    /// Network CIDR block.
    pub network_cidr: String,
    /// Kubernetes version to deploy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Initial node groups.
    #[serde(default)]
    pub node_groups: Vec<CreateNodeGroupRequest>,
    /// Key-value labels on the cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    /// Enable private networking for node groups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_node_groups: Option<bool>,
    /// Control plane plan name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    /// Allowed CIDRs for control plane access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_plane_ip_filter: Option<Vec<String>>,
    /// Storage encryption setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encryption: Option<String>,
}

/// Request body for modifying a Kubernetes cluster.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyClusterRequest {
    /// Legacy IP filter field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_filter: Option<Vec<String>>,
    /// Key-value labels to set on the cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    /// Allowed CIDRs for control plane access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_plane_ip_filter: Option<Vec<String>>,
    /// Control plane plan name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
}

/// Request body for creating a node group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNodeGroupRequest {
    /// Node group name.
    pub name: String,
    /// Server plan for each node.
    pub plan: String,
    /// Desired number of nodes.
    pub count: u32,
    /// Spread nodes across hosts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_affinity: Option<bool>,
    /// Key-value labels applied to nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    /// Kubernetes taints applied to nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taints: Option<Vec<Taint>>,
    /// Extra kubelet arguments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubelet_args: Option<Vec<KubeletArg>>,
    /// SSH public keys injected into nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_keys: Option<Vec<String>>,
    /// Whether nodes have utility network access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utility_network_access: Option<bool>,
    /// Storage device plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
    /// Custom plan configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_plan: Option<CustomPlan>,
    /// Cloud-native plan configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_native_plan: Option<CloudNativePlan>,
    /// GPU plan configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_plan: Option<GPUPlan>,
    /// Storage encryption setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encryption: Option<String>,
}

/// Request body for modifying a node group.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyNodeGroupRequest {
    /// Desired number of nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    /// Key-value labels on the node group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

/// Strategy for cluster version upgrades.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UpgradeStrategy {
    /// Strategy type identifier.
    #[serde(rename = "type")]
    pub strategy_type: String,
}

/// Request body for upgrading a cluster version.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeClusterRequest {
    /// Target Kubernetes version.
    pub version: String,
    /// Upgrade strategy to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<UpgradeStrategy>,
}

/// Available upgrade versions for a cluster.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ClusterAvailableUpgrades {
    /// List of available Kubernetes versions.
    pub versions: Vec<String>,
}
