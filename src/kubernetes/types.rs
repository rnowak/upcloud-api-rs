//! Public types for the Managed Kubernetes API.

use serde::{Deserialize, Serialize};

// ─── Enums ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClusterState {
    Pending,
    Running,
    Terminating,
    Terminated,
    Failed,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NodeGroupState {
    Pending,
    Running,
    ScalingUp,
    ScalingDown,
    Terminating,
    Failed,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeState {
    Pending,
    Running,
    Terminating,
    Failed,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaintEffect {
    NoExecute,
    NoSchedule,
    PreferNoSchedule,
}

// ─── Types ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_nodes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cluster {
    pub uuid: String,
    pub name: String,
    pub state: ClusterState,
    pub zone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_cidr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_groups: Option<Vec<NodeGroup>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_plane_ip_filter: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_node_groups: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encryption: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeGroup {
    pub name: String,
    pub plan: String,
    pub count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<NodeGroupState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_affinity: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taints: Option<Vec<Taint>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubelet_args: Option<Vec<KubeletArg>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_keys: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utility_network_access: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_affinity_status: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_plan: Option<CustomPlan>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_native_plan: Option<CloudNativePlan>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_plan: Option<GPUPlan>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encryption: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub uuid: String,
    pub name: String,
    pub state: NodeState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubelet_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Taint {
    pub key: String,
    pub value: String,
    pub effect: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KubeletArg {
    pub key: String,
    pub value: String,
}

/// The kubeconfig YAML for a cluster.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kubeconfig {
    pub kubeconfig: String,
}

// ─── Node group plan types ────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomPlan {
    pub cores: u32,
    pub memory: u64,
    pub storage_size: u64,
    pub storage_tier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudNativePlan {
    pub storage_size: u64,
    pub storage_tier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GPUPlan {
    pub storage_size: u64,
    pub storage_tier: String,
}

// ─── Response types ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KubernetesVersion {
    pub id: String,
    pub version: String,
}

// ─── Request types ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateClusterRequest {
    pub name: String,
    pub zone: String,
    pub network: String,
    pub network_cidr: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default)]
    pub node_groups: Vec<CreateNodeGroupRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_node_groups: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_plane_ip_filter: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyClusterRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_filter: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_plane_ip_filter: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNodeGroupRequest {
    pub name: String,
    pub plan: String,
    pub count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_affinity: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taints: Option<Vec<Taint>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubelet_args: Option<Vec<KubeletArg>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_keys: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utility_network_access: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_plan: Option<CustomPlan>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_native_plan: Option<CloudNativePlan>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_plan: Option<GPUPlan>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encryption: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyNodeGroupRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeStrategy {
    #[serde(rename = "type")]
    pub strategy_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeClusterRequest {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<UpgradeStrategy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterAvailableUpgrades {
    pub versions: Vec<String>,
}
