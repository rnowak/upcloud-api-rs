//! Managed Kubernetes — `GET/POST/DELETE /1.3/kubernetes` + node groups.

use serde::{Deserialize, Serialize};

use crate::{client::Client, error::Error};

// ─── Enums ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KubernetesClusterState {
    Pending,
    Running,
    Terminating,
    Terminated,
    Failed,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum KubernetesNodeGroupState {
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
pub enum KubernetesNodeState {
    Pending,
    Running,
    Terminating,
    Failed,
    Unknown,
}

// ─── Types ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KubernetesPlan {
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
pub struct KubernetesCluster {
    pub uuid: String,
    pub name: String,
    pub state: KubernetesClusterState,
    pub zone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_cidr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_groups: Option<Vec<KubernetesNodeGroup>>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KubernetesNodeGroup {
    pub name: String,
    pub plan: String,
    pub count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<KubernetesNodeGroupState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_affinity: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taints: Option<Vec<KubernetesTaint>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubelet_args: Option<Vec<KubernetesKubeletArg>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_keys: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utility_network_access: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_affinity_status: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<KubernetesNode>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KubernetesNode {
    pub uuid: String,
    pub name: String,
    pub state: KubernetesNodeState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubelet_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KubernetesTaint {
    pub key: String,
    pub value: String,
    pub effect: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KubernetesKubeletArg {
    pub key: String,
    pub value: String,
}

/// The kubeconfig YAML for a cluster.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KubernetesKubeconfig {
    pub kubeconfig: String,
}

// ─── Request types ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateKubernetesClusterRequest {
    pub name: String,
    pub zone: String,
    pub network: String,
    pub network_cidr: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default)]
    pub node_groups: Vec<CreateKubernetesNodeGroupRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_node_groups: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_plane_ip_filter: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateKubernetesNodeGroupRequest {
    pub name: String,
    pub plan: String,
    pub count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_affinity: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taints: Option<Vec<KubernetesTaint>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubelet_args: Option<Vec<KubernetesKubeletArg>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_keys: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utility_network_access: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyKubernetesNodeGroupRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KubernetesUpgradeStrategy {
    #[serde(rename = "type")]
    pub strategy_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeKubernetesClusterRequest {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<KubernetesUpgradeStrategy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KubernetesClusterAvailableUpgrades {
    pub versions: Vec<String>,
}

// ─── Client methods ───────────────────────────────────────────────────────────

impl Client {
    /// Returns a list of available Kubernetes node plans.
    pub async fn list_kubernetes_plans(&self) -> Result<Vec<KubernetesPlan>, Error> {
        self.get("/1.3/kubernetes/plans").await
    }

    /// Returns a list of all Kubernetes clusters.
    pub async fn list_kubernetes_clusters(&self) -> Result<Vec<KubernetesCluster>, Error> {
        self.get("/1.3/kubernetes").await
    }

    /// Returns details for a specific Kubernetes cluster.
    pub async fn get_kubernetes_cluster(&self, uuid: &str) -> Result<KubernetesCluster, Error> {
        self.get(&format!("/1.3/kubernetes/{uuid}")).await
    }

    /// Creates a new Kubernetes cluster.
    pub async fn create_kubernetes_cluster(
        &self,
        req: CreateKubernetesClusterRequest,
    ) -> Result<KubernetesCluster, Error> {
        self.post("/1.3/kubernetes", &req).await
    }

    /// Deletes a Kubernetes cluster.
    pub async fn delete_kubernetes_cluster(&self, uuid: &str) -> Result<(), Error> {
        self.delete(&format!("/1.3/kubernetes/{uuid}")).await
    }

    /// Returns the kubeconfig YAML for a Kubernetes cluster.
    pub async fn get_kubernetes_kubeconfig(&self, uuid: &str) -> Result<String, Error> {
        let resp: KubernetesKubeconfig =
            self.get(&format!("/1.3/kubernetes/{uuid}/kubeconfig")).await?;
        Ok(resp.kubeconfig)
    }

    // ─── Node groups ──────────────────────────────────────────────────────────

    /// Returns a list of node groups for a cluster.
    pub async fn list_kubernetes_node_groups(&self, cluster_uuid: &str) -> Result<Vec<KubernetesNodeGroup>, Error> {
        self.get(&format!("/1.3/kubernetes/{cluster_uuid}/node-groups"))
            .await
    }

    /// Returns details for a specific node group.
    pub async fn get_kubernetes_node_group(
        &self,
        cluster_uuid: &str,
        name: &str,
    ) -> Result<KubernetesNodeGroup, Error> {
        self.get(&format!(
            "/1.3/kubernetes/{cluster_uuid}/node-groups/{name}"
        ))
        .await
    }

    /// Creates a node group in a cluster.
    pub async fn create_kubernetes_node_group(
        &self,
        cluster_uuid: &str,
        req: CreateKubernetesNodeGroupRequest,
    ) -> Result<KubernetesNodeGroup, Error> {
        self.post(
            &format!("/1.3/kubernetes/{cluster_uuid}/node-groups"),
            &req,
        )
        .await
    }

    /// Modifies a node group (e.g. scale count).
    pub async fn modify_kubernetes_node_group(
        &self,
        cluster_uuid: &str,
        name: &str,
        req: ModifyKubernetesNodeGroupRequest,
    ) -> Result<KubernetesNodeGroup, Error> {
        self.patch(
            &format!("/1.3/kubernetes/{cluster_uuid}/node-groups/{name}"),
            &req,
        )
        .await
    }

    /// Deletes a node group.
    pub async fn delete_kubernetes_node_group(&self, cluster_uuid: &str, name: &str) -> Result<(), Error> {
        self.delete(&format!(
            "/1.3/kubernetes/{cluster_uuid}/node-groups/{name}"
        ))
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const K8S_PLAN_LIST_JSON: &str = r#"[
  {
    "name": "dev-md",
    "max_nodes": 30,
    "deprecated": false
  },
  {
    "name": "prod-md",
    "max_nodes": 120,
    "deprecated": false
  }
]"#;

    const K8S_CLUSTER_LIST_JSON: &str = r#"[
  {
    "control_plane_ip_filter": [],
    "labels": [
      {"key": "env", "value": "staging"},
      {"key": "app", "value": "cats-dating-app"}
    ],
    "name": "example-cluster-1",
    "network": "03a85f64-5717-4562-b3fc-2c963f66afa6",
    "network_cidr": "172.16.0.0/24",
    "node_groups": [
      {
        "count": 3,
        "kubelet_args": [],
        "labels": [
          {"key": "team", "value": "development"}
        ],
        "name": "example-node-group-1",
        "plan": "4xCPU-8GB",
        "ssh_keys": [
          "ssh-rsa AAAAB3NzaC1yc2EAA"
        ],
        "state": "running",
        "storage": "01000000-0000-4000-8000-000160010100",
        "taints": [],
        "anti_affinity": false,
        "utility_network_access": true
      }
    ],
    "state": "running",
    "plan": "dev-md",
    "private_node_groups": false,
    "uuid": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "zone": "fi-hel2"
  },
  {
    "control_plane_ip_filter": ["0.0.0.0/0"],
    "name": "example-cluster-2",
    "network": "03a85f64-5717-4562-b3fc-2c963f66afa6",
    "network_cidr": "172.17.0.0/24",
    "node_groups": [],
    "state": "pending",
    "plan": "prod-md",
    "private_node_groups": true,
    "uuid": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "zone": "de-fra1"
  }
]"#;

    const K8S_CLUSTER_DETAIL_JSON: &str = r#"{
  "control_plane_ip_filter": ["0.0.0.0/0"],
  "labels": [
    {"key": "env", "value": "staging"},
    {"key": "app", "value": "cats-dating-app"}
  ],
  "name": "example-cluster-1",
  "network": "03a85f64-5717-4562-b3fc-2c963f66afa6",
  "network_cidr": "172.16.0.0/24",
  "node_groups": [
    {
      "count": 3,
      "kubelet_args": [],
      "labels": [{"key": "team", "value": "development"}],
      "name": "example-node-group-1",
      "plan": "4xCPU-8GB",
      "ssh_keys": ["ssh-rsa AAAAB3NzaC1yc2EAA"],
      "state": "running",
      "storage": "01000000-0000-4000-8000-000160010100",
      "taints": [],
      "anti_affinity": false,
      "utility_network_access": true
    }
  ],
  "state": "running",
  "plan": "dev-md",
  "private_node_groups": false,
  "uuid": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
  "zone": "fi-hel2"
}"#;

    const NODE_GROUP_LIST_JSON: &str = r#"[
  {
    "anti_affinity": false,
    "count": 3,
    "kubelet_args": [],
    "labels": [{"key": "team", "value": "development"}],
    "name": "example-node-group-1",
    "plan": "4xCPU-8GB",
    "ssh_keys": ["ssh-rsa AAAAB3NzaC1yc2EAA"],
    "state": "running",
    "storage": "01000000-0000-4000-8000-000160010100",
    "taints": [],
    "utility_network_access": true
  },
  {
    "anti_affinity": true,
    "count": 9,
    "kubelet_args": [{"key": "log-flush-frequency", "value": "5s"}],
    "labels": [{"key": "team", "value": "qa"}],
    "name": "example-node-group-2",
    "plan": "4xCPU-8GB",
    "ssh_keys": ["ssh-rsa AAAAB3NzaC1yc2EAA"],
    "state": "running",
    "storage": "01000000-0000-4000-8000-000160010100",
    "taints": [{"effect": "NoSchedule", "key": "team", "value": "qa"}],
    "utility_network_access": false
  }
]"#;

    #[test]
    fn deserialize_k8s_plan_list() {
        let plans: Vec<KubernetesPlan> = serde_json::from_str(K8S_PLAN_LIST_JSON).unwrap();
        assert_eq!(plans.len(), 2);
        assert_eq!(plans[0].name, "dev-md");
        assert_eq!(plans[0].max_nodes, Some(30));
        assert_eq!(plans[0].deprecated, Some(false));
        assert!(plans[0].description.is_none());
        assert!(plans[0].server_plan.is_none());
        assert_eq!(plans[1].name, "prod-md");
        assert_eq!(plans[1].max_nodes, Some(120));
        assert_eq!(plans[1].deprecated, Some(false));
        assert!(plans[1].description.is_none());
        assert!(plans[1].server_plan.is_none());
    }

    #[test]
    fn deserialize_k8s_cluster_list() {
        let clusters: Vec<KubernetesCluster> =
            serde_json::from_str(K8S_CLUSTER_LIST_JSON).unwrap();
        assert_eq!(clusters.len(), 2);

        let c1 = &clusters[0];
        assert_eq!(c1.name, "example-cluster-1");
        assert_eq!(c1.state, KubernetesClusterState::Running);
        assert_eq!(c1.zone, "fi-hel2");
        assert_eq!(c1.plan.as_deref(), Some("dev-md"));
        assert_eq!(c1.private_node_groups, Some(false));
        assert_eq!(c1.control_plane_ip_filter.as_ref().unwrap().len(), 0);
        assert_eq!(c1.network.as_deref(), Some("03a85f64-5717-4562-b3fc-2c963f66afa6"));
        assert_eq!(c1.network_cidr.as_deref(), Some("172.16.0.0/24"));
        assert_eq!(c1.uuid, "3fa85f64-5717-4562-b3fc-2c963f66afa6");
        assert!(c1.version.is_none());
        assert!(c1.created_at.is_none());

        let labels = c1.labels.as_ref().unwrap();
        assert_eq!(labels.len(), 2);
        assert_eq!(labels[0].key, "env");
        assert_eq!(labels[0].value, "staging");
        assert_eq!(labels[1].key, "app");
        assert_eq!(labels[1].value, "cats-dating-app");

        let groups = c1.node_groups.as_ref().unwrap();
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].name, "example-node-group-1");
        assert_eq!(groups[0].count, 3);
        assert_eq!(groups[0].state, Some(KubernetesNodeGroupState::Running));
        assert_eq!(groups[0].anti_affinity, Some(false));
        assert_eq!(groups[0].plan, "4xCPU-8GB");
        assert!(groups[0].kubelet_args.as_ref().unwrap().is_empty());
        let g0_labels = groups[0].labels.as_ref().unwrap();
        assert_eq!(g0_labels.len(), 1);
        assert_eq!(g0_labels[0].key, "team");
        assert_eq!(g0_labels[0].value, "development");
        let g0_ssh = groups[0].ssh_keys.as_ref().unwrap();
        assert_eq!(g0_ssh.len(), 1);
        assert_eq!(g0_ssh[0], "ssh-rsa AAAAB3NzaC1yc2EAA");
        assert_eq!(groups[0].storage.as_deref(), Some("01000000-0000-4000-8000-000160010100"));
        assert!(groups[0].taints.as_ref().unwrap().is_empty());
        assert_eq!(groups[0].utility_network_access, Some(true));
        assert!(groups[0].anti_affinity_status.is_none());
        assert!(groups[0].nodes.is_none());

        let c2 = &clusters[1];
        assert_eq!(c2.name, "example-cluster-2");
        assert_eq!(c2.state, KubernetesClusterState::Pending);
        assert_eq!(c2.zone, "de-fra1");
        assert_eq!(c2.plan.as_deref(), Some("prod-md"));
        assert_eq!(c2.private_node_groups, Some(true));
        assert_eq!(c2.uuid, "3fa85f64-5717-4562-b3fc-2c963f66afa6");
        assert_eq!(c2.network.as_deref(), Some("03a85f64-5717-4562-b3fc-2c963f66afa6"));
        assert_eq!(c2.network_cidr.as_deref(), Some("172.17.0.0/24"));
        let c2_filters = c2.control_plane_ip_filter.as_ref().unwrap();
        assert_eq!(c2_filters.len(), 1);
        assert_eq!(c2_filters[0], "0.0.0.0/0");
        assert!(c2.node_groups.as_ref().unwrap().is_empty());
        assert!(c2.labels.is_none());
        assert!(c2.version.is_none());
        assert!(c2.created_at.is_none());
    }

    #[test]
    fn deserialize_k8s_cluster_detail() {
        let cluster: KubernetesCluster =
            serde_json::from_str(K8S_CLUSTER_DETAIL_JSON).unwrap();
        assert_eq!(cluster.uuid, "3fa85f64-5717-4562-b3fc-2c963f66afa6");
        assert_eq!(cluster.name, "example-cluster-1");
        assert_eq!(cluster.state, KubernetesClusterState::Running);
        assert_eq!(cluster.zone, "fi-hel2");
        assert_eq!(cluster.plan.as_deref(), Some("dev-md"));
        assert_eq!(cluster.private_node_groups, Some(false));
        assert_eq!(cluster.network.as_deref(), Some("03a85f64-5717-4562-b3fc-2c963f66afa6"));
        assert_eq!(cluster.network_cidr.as_deref(), Some("172.16.0.0/24"));
        assert_eq!(cluster.control_plane_ip_filter.as_ref().unwrap(), &["0.0.0.0/0"]);
        assert!(cluster.version.is_none());
        assert!(cluster.created_at.is_none());

        let labels = cluster.labels.as_ref().unwrap();
        assert_eq!(labels.len(), 2);
        assert_eq!(labels[0].key, "env");
        assert_eq!(labels[0].value, "staging");
        assert_eq!(labels[1].key, "app");
        assert_eq!(labels[1].value, "cats-dating-app");

        let groups = cluster.node_groups.as_ref().unwrap();
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].name, "example-node-group-1");
        assert_eq!(groups[0].plan, "4xCPU-8GB");
        assert_eq!(groups[0].count, 3);
        assert_eq!(groups[0].state, Some(KubernetesNodeGroupState::Running));
        assert_eq!(groups[0].anti_affinity, Some(false));
        assert!(groups[0].kubelet_args.as_ref().unwrap().is_empty());
        let g0_labels = groups[0].labels.as_ref().unwrap();
        assert_eq!(g0_labels.len(), 1);
        assert_eq!(g0_labels[0].key, "team");
        assert_eq!(g0_labels[0].value, "development");
        let g0_ssh = groups[0].ssh_keys.as_ref().unwrap();
        assert_eq!(g0_ssh.len(), 1);
        assert_eq!(g0_ssh[0], "ssh-rsa AAAAB3NzaC1yc2EAA");
        assert_eq!(groups[0].storage.as_deref(), Some("01000000-0000-4000-8000-000160010100"));
        assert!(groups[0].taints.as_ref().unwrap().is_empty());
        assert_eq!(groups[0].utility_network_access, Some(true));
        assert!(groups[0].anti_affinity_status.is_none());
        assert!(groups[0].nodes.is_none());
    }

    #[test]
    fn deserialize_node_group_list() {
        let groups: Vec<KubernetesNodeGroup> = serde_json::from_str(NODE_GROUP_LIST_JSON).unwrap();
        assert_eq!(groups.len(), 2);

        assert_eq!(groups[0].name, "example-node-group-1");
        assert_eq!(groups[0].plan, "4xCPU-8GB");
        assert_eq!(groups[0].count, 3);
        assert_eq!(groups[0].state, Some(KubernetesNodeGroupState::Running));
        assert_eq!(groups[0].utility_network_access, Some(true));
        assert!(groups[0].taints.as_ref().unwrap().is_empty());
        assert_eq!(groups[0].anti_affinity, Some(false));
        assert!(groups[0].kubelet_args.as_ref().unwrap().is_empty());
        let g0_labels = groups[0].labels.as_ref().unwrap();
        assert_eq!(g0_labels.len(), 1);
        assert_eq!(g0_labels[0].key, "team");
        assert_eq!(g0_labels[0].value, "development");
        let g0_ssh = groups[0].ssh_keys.as_ref().unwrap();
        assert_eq!(g0_ssh.len(), 1);
        assert_eq!(g0_ssh[0], "ssh-rsa AAAAB3NzaC1yc2EAA");
        assert_eq!(groups[0].storage.as_deref(), Some("01000000-0000-4000-8000-000160010100"));
        assert!(groups[0].anti_affinity_status.is_none());
        assert!(groups[0].nodes.is_none());

        assert_eq!(groups[1].name, "example-node-group-2");
        assert_eq!(groups[1].count, 9);
        assert_eq!(groups[1].anti_affinity, Some(true));
        assert_eq!(groups[1].utility_network_access, Some(false));
        assert_eq!(groups[1].plan, "4xCPU-8GB");
        assert_eq!(groups[1].state, Some(KubernetesNodeGroupState::Running));
        assert_eq!(groups[1].storage.as_deref(), Some("01000000-0000-4000-8000-000160010100"));
        let g1_labels = groups[1].labels.as_ref().unwrap();
        assert_eq!(g1_labels.len(), 1);
        assert_eq!(g1_labels[0].key, "team");
        assert_eq!(g1_labels[0].value, "qa");
        let g1_ssh = groups[1].ssh_keys.as_ref().unwrap();
        assert_eq!(g1_ssh.len(), 1);
        assert_eq!(g1_ssh[0], "ssh-rsa AAAAB3NzaC1yc2EAA");
        assert!(groups[1].anti_affinity_status.is_none());
        assert!(groups[1].nodes.is_none());

        let args = groups[1].kubelet_args.as_ref().unwrap();
        assert_eq!(args.len(), 1);
        assert_eq!(args[0].key, "log-flush-frequency");
        assert_eq!(args[0].value, "5s");

        let taints = groups[1].taints.as_ref().unwrap();
        assert_eq!(taints.len(), 1);
        assert_eq!(taints[0].effect, "NoSchedule");
        assert_eq!(taints[0].key, "team");
        assert_eq!(taints[0].value, "qa");
    }

    // ─── New test JSON payloads (official docs, page 20) ──────────────────────

    const CREATE_CLUSTER_REQUEST_JSON: &str = r#"{
  "control_plane_ip_filter": [],
  "name": "example-cluster-1",
  "network": "03a85f64-5717-4562-b3fc-2c963f66afa6",
  "network_cidr": "172.16.0.0/24",
  "node_groups": [
    {
      "count": 3,
      "labels": [
        {"key": "team", "value": "development"}
      ],
      "name": "example-node-group-1",
      "plan": "4xCPU-8GB",
      "ssh_keys": ["ssh-rsa AAAAB3NzaC1yc2EAA"]
    }
  ],
  "plan": "dev-md",
  "private_node_groups": false,
  "zone": "fi-hel2",
  "labels": [
    {"key": "env", "value": "staging"},
    {"key": "app", "value": "cats-dating-app"}
  ]
}"#;

    const CREATE_CLUSTER_RESPONSE_JSON: &str = r#"{
  "control_plane_ip_filter": [],
  "labels": [
    {"key": "env", "value": "staging"},
    {"key": "app", "value": "cats-dating-app"}
  ],
  "name": "example-cluster-1",
  "network": "03a85f64-5717-4562-b3fc-2c963f66afa6",
  "network_cidr": "172.16.0.0/24",
  "node_groups": [
    {
      "count": 3,
      "kubelet_args": [],
      "labels": [{"key": "team", "value": "development"}],
      "name": "example-node-group-1",
      "plan": "4xCPU-8GB",
      "ssh_keys": ["ssh-rsa AAAAB3NzaC1yc2EAA"],
      "state": "pending",
      "storage": "01000000-0000-4000-8000-000160010100",
      "taints": [],
      "anti_affinity": false,
      "utility_network_access": true
    }
  ],
  "state": "pending",
  "plan": "dev-md",
  "private_node_groups": false,
  "uuid": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
  "zone": "fi-hel2"
}"#;

    const MODIFY_CLUSTER_REQUEST_JSON: &str = r#"{
  "control_plane_ip_filter": ["0.0.0.0/0"],
  "labels": [
    {"key": "env", "value": "production"},
    {"key": "app", "value": "dogs-dating-app"}
  ]
}"#;

    const MODIFY_CLUSTER_RESPONSE_JSON: &str = r#"{
  "control_plane_ip_filter": ["0.0.0.0/0"],
  "labels": [
    {"key": "env", "value": "production"},
    {"key": "app", "value": "dogs-dating-app"}
  ],
  "name": "example-cluster-1",
  "network": "03a85f64-5717-4562-b3fc-2c963f66afa6",
  "network_cidr": "172.16.0.0/24",
  "node_groups": [
    {
      "count": 3,
      "kubelet_args": [],
      "labels": [{"key": "team", "value": "development"}],
      "name": "example-node-group-1",
      "plan": "4xCPU-8GB",
      "ssh_keys": ["ssh-rsa AAAAB3NzaC1yc2EAA"],
      "state": "running",
      "storage": "01000000-0000-4000-8000-000160010100",
      "taints": [],
      "anti_affinity": false,
      "utility_network_access": true
    }
  ],
  "state": "running",
  "plan": "dev-md",
  "private_node_groups": false,
  "uuid": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
  "zone": "fi-hel2"
}"#;

    const AVAILABLE_UPGRADES_JSON: &str = r#"{"versions": ["1.31"]}"#;

    const UPGRADE_CLUSTER_REQUEST_JSON: &str = r#"{
  "version": "1.31",
  "strategy": {"type": "manual"}
}"#;

    const KUBECONFIG_RESPONSE_JSON: &str = r#"{
  "kubeconfig": "apiVersion: v1\nclusters:\n  - cluster:\n      certificate-authority-data: BASE64\n      server: https://server:6443\n    name: example-cluster-1\ncontexts:\n  - context:\n      cluster: example-cluster-1\n      user: example-cluster-1-admin\n    name: example-cluster-1-admin@example-cluster-1\ncurrent-context: example-cluster-1-admin@example-cluster-1\nkind: Config\npreferences: {}\nusers:\n  - name: example-cluster-1-admin\n    user:\n      client-certificate-data: BASE64\n      client-key-data: BASE64\n"
}"#;

    const CREATE_NODE_GROUP_REQUEST_JSON: &str = r#"{
  "anti_affinity": true,
  "count": 9,
  "kubelet_args": [{"key": "log-flush-frequency", "value": "5s"}],
  "labels": [{"key": "team", "value": "qa"}],
  "name": "example-node-group-2",
  "plan": "4xCPU-8GB",
  "ssh_keys": ["ssh-rsa AAAAB3NzaC1yc2EAA"],
  "storage": "01000000-0000-4000-8000-000160010100",
  "taints": [{"effect": "NoSchedule", "key": "team", "value": "qa"}],
  "utility_network_access": true
}"#;

    const CREATE_NODE_GROUP_RESPONSE_JSON: &str = r#"{
  "anti_affinity": true,
  "count": 9,
  "kubelet_args": [{"key": "log-flush-frequency", "value": "5s"}],
  "labels": [{"key": "team", "value": "qa"}],
  "name": "example-node-group-2",
  "plan": "4xCPU-8GB",
  "ssh_keys": ["ssh-rsa AAAAB3NzaC1yc2EAA"],
  "state": "pending",
  "storage": "01000000-0000-4000-8000-000160010100",
  "taints": [{"effect": "NoSchedule", "key": "team", "value": "qa"}],
  "utility_network_access": true
}"#;

    const NODE_GROUP_DETAIL_JSON: &str = r#"{
  "anti_affinity_status": true,
  "count": 2,
  "kubelet_args": [{"key": "log-flush-frequency", "value": "5s"}],
  "labels": [{"key": "team", "value": "qa"}],
  "name": "example-node-group-2",
  "plan": "4xCPU-8GB",
  "ssh_keys": ["ssh-rsa AAAAB3NzaC1yc2EAA"],
  "state": "running",
  "storage": "01000000-0000-4000-8000-000160010100",
  "taints": [{"effect": "NoSchedule", "key": "team", "value": "qa"}],
  "utility_network_access": true,
  "nodes": [
    {
      "uuid": "564c8cd9-bc16-4f89-a328-daf3f138cc44",
      "name": "example-fzwwt-8smpz",
      "state": "running",
      "kubelet_version": "1.30"
    },
    {
      "uuid": "2e8039c6-95d4-41a9-92ca-46808f560abb",
      "name": "example-fzwwt-zmjfs",
      "state": "running",
      "kubelet_version": "1.30"
    }
  ]
}"#;

    const MODIFY_NODE_GROUP_REQUEST_JSON: &str = r#"{"count": 15}"#;

    const MODIFY_NODE_GROUP_RESPONSE_JSON: &str = r#"{
  "anti_affinity": true,
  "count": 15,
  "kubelet_args": [{"key": "log-flush-frequency", "value": "5s"}],
  "labels": [{"key": "team", "value": "qa"}],
  "name": "example-node-group-2",
  "plan": "4xCPU-8GB",
  "ssh_keys": ["ssh-rsa AAAAB3NzaC1yc2EAA"],
  "state": "running",
  "storage": "01000000-0000-4000-8000-000160010100",
  "taints": [{"effect": "NoSchedule", "key": "team", "value": "qa"}],
  "utility_network_access": true
}"#;

    // ─── New test functions ───────────────────────────────────────────────────

    #[test]
    fn deserialize_create_cluster_request() {
        let req: CreateKubernetesClusterRequest =
            serde_json::from_str(CREATE_CLUSTER_REQUEST_JSON).unwrap();
        assert_eq!(req.name, "example-cluster-1");
        assert_eq!(req.zone, "fi-hel2");
        assert_eq!(req.network, "03a85f64-5717-4562-b3fc-2c963f66afa6");
        assert_eq!(req.network_cidr, "172.16.0.0/24");
        assert_eq!(req.node_groups.len(), 1);
        assert_eq!(req.plan.as_deref(), Some("dev-md"));
        assert_eq!(req.private_node_groups, Some(false));
        assert!(req.version.is_none());
        assert!(req.control_plane_ip_filter.as_ref().unwrap().is_empty());
        let labels = req.labels.as_ref().unwrap();
        assert_eq!(labels.len(), 2);
        assert_eq!(labels[0].key, "env");
        assert_eq!(labels[0].value, "staging");
        assert_eq!(labels[1].key, "app");
        assert_eq!(labels[1].value, "cats-dating-app");

        let ng = &req.node_groups[0];
        assert_eq!(ng.name, "example-node-group-1");
        assert_eq!(ng.plan, "4xCPU-8GB");
        assert_eq!(ng.count, 3);
        let ng_labels = ng.labels.as_ref().unwrap();
        assert_eq!(ng_labels.len(), 1);
        assert_eq!(ng_labels[0].key, "team");
        assert_eq!(ng_labels[0].value, "development");
        let ng_ssh = ng.ssh_keys.as_ref().unwrap();
        assert_eq!(ng_ssh.len(), 1);
        assert_eq!(ng_ssh[0], "ssh-rsa AAAAB3NzaC1yc2EAA");
        assert!(ng.anti_affinity.is_none());
        assert!(ng.taints.is_none());
        assert!(ng.kubelet_args.is_none());
        assert!(ng.utility_network_access.is_none());
        assert!(ng.storage.is_none());
    }

    #[test]
    fn deserialize_create_cluster_response() {
        let cluster: KubernetesCluster =
            serde_json::from_str(CREATE_CLUSTER_RESPONSE_JSON).unwrap();
        assert_eq!(cluster.state, KubernetesClusterState::Pending);
        assert_eq!(cluster.name, "example-cluster-1");
        assert_eq!(cluster.uuid, "3fa85f64-5717-4562-b3fc-2c963f66afa6");
        assert_eq!(cluster.zone, "fi-hel2");
        assert_eq!(cluster.plan.as_deref(), Some("dev-md"));
        assert_eq!(cluster.private_node_groups, Some(false));
        assert_eq!(cluster.network.as_deref(), Some("03a85f64-5717-4562-b3fc-2c963f66afa6"));
        assert_eq!(cluster.network_cidr.as_deref(), Some("172.16.0.0/24"));
        assert!(cluster.control_plane_ip_filter.as_ref().unwrap().is_empty());
        assert!(cluster.version.is_none());
        assert!(cluster.created_at.is_none());

        let labels = cluster.labels.as_ref().unwrap();
        assert_eq!(labels.len(), 2);
        assert_eq!(labels[0].key, "env");
        assert_eq!(labels[0].value, "staging");
        assert_eq!(labels[1].key, "app");
        assert_eq!(labels[1].value, "cats-dating-app");

        let groups = cluster.node_groups.as_ref().unwrap();
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].state, Some(KubernetesNodeGroupState::Pending));
        assert_eq!(groups[0].name, "example-node-group-1");
        assert_eq!(groups[0].plan, "4xCPU-8GB");
        assert_eq!(groups[0].count, 3);
        assert_eq!(groups[0].anti_affinity, Some(false));
        assert_eq!(groups[0].utility_network_access, Some(true));
        assert_eq!(groups[0].storage.as_deref(), Some("01000000-0000-4000-8000-000160010100"));
        assert!(groups[0].kubelet_args.as_ref().unwrap().is_empty());
        assert!(groups[0].taints.as_ref().unwrap().is_empty());
        let g0_labels = groups[0].labels.as_ref().unwrap();
        assert_eq!(g0_labels.len(), 1);
        assert_eq!(g0_labels[0].key, "team");
        assert_eq!(g0_labels[0].value, "development");
        let g0_ssh = groups[0].ssh_keys.as_ref().unwrap();
        assert_eq!(g0_ssh.len(), 1);
        assert_eq!(g0_ssh[0], "ssh-rsa AAAAB3NzaC1yc2EAA");
        assert!(groups[0].anti_affinity_status.is_none());
        assert!(groups[0].nodes.is_none());
    }

    #[test]
    fn deserialize_modify_cluster_request() {
        let val: serde_json::Value =
            serde_json::from_str(MODIFY_CLUSTER_REQUEST_JSON).unwrap();
        assert!(val.get("control_plane_ip_filter").is_some());
        assert!(val.get("labels").is_some());
        let filters = val["control_plane_ip_filter"].as_array().unwrap();
        assert_eq!(filters.len(), 1);
        assert_eq!(filters[0].as_str().unwrap(), "0.0.0.0/0");
        let labels = val["labels"].as_array().unwrap();
        assert_eq!(labels.len(), 2);
        assert_eq!(labels[0]["key"].as_str().unwrap(), "env");
        assert_eq!(labels[0]["value"].as_str().unwrap(), "production");
        assert_eq!(labels[1]["key"].as_str().unwrap(), "app");
        assert_eq!(labels[1]["value"].as_str().unwrap(), "dogs-dating-app");
    }

    #[test]
    fn deserialize_modify_cluster_response() {
        let cluster: KubernetesCluster =
            serde_json::from_str(MODIFY_CLUSTER_RESPONSE_JSON).unwrap();
        assert_eq!(cluster.state, KubernetesClusterState::Running);
        assert_eq!(cluster.name, "example-cluster-1");
        assert_eq!(cluster.uuid, "3fa85f64-5717-4562-b3fc-2c963f66afa6");
        assert_eq!(cluster.zone, "fi-hel2");
        assert_eq!(cluster.plan.as_deref(), Some("dev-md"));
        assert_eq!(cluster.private_node_groups, Some(false));
        assert_eq!(cluster.network.as_deref(), Some("03a85f64-5717-4562-b3fc-2c963f66afa6"));
        assert_eq!(cluster.network_cidr.as_deref(), Some("172.16.0.0/24"));
        assert_eq!(cluster.control_plane_ip_filter.as_ref().unwrap(), &["0.0.0.0/0"]);
        assert!(cluster.version.is_none());
        assert!(cluster.created_at.is_none());

        let labels = cluster.labels.as_ref().unwrap();
        assert_eq!(labels.len(), 2);
        assert_eq!(labels[0].key, "env");
        assert_eq!(labels[0].value, "production");
        assert_eq!(labels[1].key, "app");
        assert_eq!(labels[1].value, "dogs-dating-app");

        let groups = cluster.node_groups.as_ref().unwrap();
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].name, "example-node-group-1");
        assert_eq!(groups[0].plan, "4xCPU-8GB");
        assert_eq!(groups[0].count, 3);
        assert_eq!(groups[0].state, Some(KubernetesNodeGroupState::Running));
        assert_eq!(groups[0].anti_affinity, Some(false));
        assert_eq!(groups[0].utility_network_access, Some(true));
        assert_eq!(groups[0].storage.as_deref(), Some("01000000-0000-4000-8000-000160010100"));
        assert!(groups[0].kubelet_args.as_ref().unwrap().is_empty());
        assert!(groups[0].taints.as_ref().unwrap().is_empty());
        let g0_labels = groups[0].labels.as_ref().unwrap();
        assert_eq!(g0_labels.len(), 1);
        assert_eq!(g0_labels[0].key, "team");
        assert_eq!(g0_labels[0].value, "development");
        let g0_ssh = groups[0].ssh_keys.as_ref().unwrap();
        assert_eq!(g0_ssh.len(), 1);
        assert_eq!(g0_ssh[0], "ssh-rsa AAAAB3NzaC1yc2EAA");
        assert!(groups[0].anti_affinity_status.is_none());
        assert!(groups[0].nodes.is_none());
    }

    #[test]
    fn deserialize_available_upgrades() {
        let resp: KubernetesClusterAvailableUpgrades =
            serde_json::from_str(AVAILABLE_UPGRADES_JSON).unwrap();
        assert_eq!(resp.versions.len(), 1);
        assert_eq!(resp.versions[0], "1.31");
    }

    #[test]
    fn deserialize_upgrade_cluster_request() {
        let req: UpgradeKubernetesClusterRequest =
            serde_json::from_str(UPGRADE_CLUSTER_REQUEST_JSON).unwrap();
        assert_eq!(req.version, "1.31");
        let strategy = req.strategy.as_ref().unwrap();
        assert_eq!(strategy.strategy_type, "manual");
    }

    // ─── Upgrade cluster response (official docs, page 20) ───────────────────

    const UPGRADE_CLUSTER_RESPONSE_JSON: &str = r#"{
  "version": "1.31",
  "strategy": {"type": "manual"}
}"#;

    #[test]
    fn deserialize_upgrade_cluster_response() {
        let resp: UpgradeKubernetesClusterRequest =
            serde_json::from_str(UPGRADE_CLUSTER_RESPONSE_JSON).unwrap();
        assert_eq!(resp.version, "1.31");
        let strategy = resp.strategy.as_ref().unwrap();
        assert_eq!(strategy.strategy_type, "manual");
    }

    #[test]
    fn deserialize_kubeconfig_response() {
        let resp: KubernetesKubeconfig =
            serde_json::from_str(KUBECONFIG_RESPONSE_JSON).unwrap();
        assert!(resp.kubeconfig.starts_with("apiVersion"));
        assert!(resp.kubeconfig.contains("example-cluster-1"));
        assert!(resp.kubeconfig.contains("certificate-authority-data: BASE64"));
        assert!(resp.kubeconfig.contains("client-certificate-data: BASE64"));
        assert!(resp.kubeconfig.contains("client-key-data: BASE64"));
    }

    #[test]
    fn deserialize_create_kubernetes_node_group_request() {
        let req: CreateKubernetesNodeGroupRequest =
            serde_json::from_str(CREATE_NODE_GROUP_REQUEST_JSON).unwrap();
        assert_eq!(req.name, "example-node-group-2");
        assert_eq!(req.plan, "4xCPU-8GB");
        assert_eq!(req.count, 9);
        assert_eq!(req.anti_affinity, Some(true));
        assert_eq!(req.utility_network_access, Some(true));
        assert_eq!(
            req.storage.as_deref(),
            Some("01000000-0000-4000-8000-000160010100")
        );
        let labels = req.labels.as_ref().unwrap();
        assert_eq!(labels.len(), 1);
        assert_eq!(labels[0].key, "team");
        assert_eq!(labels[0].value, "qa");
        let ssh_keys = req.ssh_keys.as_ref().unwrap();
        assert_eq!(ssh_keys.len(), 1);
        assert_eq!(ssh_keys[0], "ssh-rsa AAAAB3NzaC1yc2EAA");
        let args = req.kubelet_args.as_ref().unwrap();
        assert_eq!(args.len(), 1);
        assert_eq!(args[0].key, "log-flush-frequency");
        assert_eq!(args[0].value, "5s");
        let taints = req.taints.as_ref().unwrap();
        assert_eq!(taints.len(), 1);
        assert_eq!(taints[0].effect, "NoSchedule");
        assert_eq!(taints[0].key, "team");
        assert_eq!(taints[0].value, "qa");
    }

    #[test]
    fn deserialize_create_kubernetes_node_group_response() {
        let group: KubernetesNodeGroup =
            serde_json::from_str(CREATE_NODE_GROUP_RESPONSE_JSON).unwrap();
        assert_eq!(group.state, Some(KubernetesNodeGroupState::Pending));
        assert_eq!(group.name, "example-node-group-2");
        assert_eq!(group.count, 9);
        assert_eq!(group.plan, "4xCPU-8GB");
        assert_eq!(group.anti_affinity, Some(true));
        assert_eq!(group.utility_network_access, Some(true));
        assert_eq!(group.storage.as_deref(), Some("01000000-0000-4000-8000-000160010100"));
        let labels = group.labels.as_ref().unwrap();
        assert_eq!(labels.len(), 1);
        assert_eq!(labels[0].key, "team");
        assert_eq!(labels[0].value, "qa");
        let ssh_keys = group.ssh_keys.as_ref().unwrap();
        assert_eq!(ssh_keys.len(), 1);
        assert_eq!(ssh_keys[0], "ssh-rsa AAAAB3NzaC1yc2EAA");
        let args = group.kubelet_args.as_ref().unwrap();
        assert_eq!(args.len(), 1);
        assert_eq!(args[0].key, "log-flush-frequency");
        assert_eq!(args[0].value, "5s");
        let taints = group.taints.as_ref().unwrap();
        assert_eq!(taints.len(), 1);
        assert_eq!(taints[0].effect, "NoSchedule");
        assert_eq!(taints[0].key, "team");
        assert_eq!(taints[0].value, "qa");
        assert!(group.anti_affinity_status.is_none());
        assert!(group.nodes.is_none());
    }

    #[test]
    fn deserialize_node_group_detail() {
        let group: KubernetesNodeGroup =
            serde_json::from_str(NODE_GROUP_DETAIL_JSON).unwrap();
        assert_eq!(group.anti_affinity_status, Some(true));
        assert_eq!(group.state, Some(KubernetesNodeGroupState::Running));
        assert_eq!(group.count, 2);
        assert_eq!(group.name, "example-node-group-2");
        assert_eq!(group.plan, "4xCPU-8GB");
        assert_eq!(group.utility_network_access, Some(true));
        assert_eq!(group.storage.as_deref(), Some("01000000-0000-4000-8000-000160010100"));
        assert!(group.anti_affinity.is_none());
        let labels = group.labels.as_ref().unwrap();
        assert_eq!(labels.len(), 1);
        assert_eq!(labels[0].key, "team");
        assert_eq!(labels[0].value, "qa");
        let ssh_keys = group.ssh_keys.as_ref().unwrap();
        assert_eq!(ssh_keys.len(), 1);
        assert_eq!(ssh_keys[0], "ssh-rsa AAAAB3NzaC1yc2EAA");
        let args = group.kubelet_args.as_ref().unwrap();
        assert_eq!(args.len(), 1);
        assert_eq!(args[0].key, "log-flush-frequency");
        assert_eq!(args[0].value, "5s");
        let taints = group.taints.as_ref().unwrap();
        assert_eq!(taints.len(), 1);
        assert_eq!(taints[0].effect, "NoSchedule");
        assert_eq!(taints[0].key, "team");
        assert_eq!(taints[0].value, "qa");
        let nodes = group.nodes.as_ref().unwrap();
        assert_eq!(nodes.len(), 2);
        assert_eq!(nodes[0].uuid, "564c8cd9-bc16-4f89-a328-daf3f138cc44");
        assert_eq!(nodes[0].name, "example-fzwwt-8smpz");
        assert_eq!(nodes[0].state, KubernetesNodeState::Running);
        assert_eq!(nodes[0].kubelet_version.as_deref(), Some("1.30"));
        assert_eq!(nodes[1].uuid, "2e8039c6-95d4-41a9-92ca-46808f560abb");
        assert_eq!(nodes[1].name, "example-fzwwt-zmjfs");
        assert_eq!(nodes[1].state, KubernetesNodeState::Running);
        assert_eq!(nodes[1].kubelet_version.as_deref(), Some("1.30"));
    }

    #[test]
    fn deserialize_modify_kubernetes_node_group_request() {
        let req: ModifyKubernetesNodeGroupRequest =
            serde_json::from_str(MODIFY_NODE_GROUP_REQUEST_JSON).unwrap();
        assert_eq!(req.count, Some(15));
        assert!(req.labels.is_none());
    }

    #[test]
    fn deserialize_modify_kubernetes_node_group_response() {
        let group: KubernetesNodeGroup =
            serde_json::from_str(MODIFY_NODE_GROUP_RESPONSE_JSON).unwrap();
        assert_eq!(group.count, 15);
        assert_eq!(group.name, "example-node-group-2");
        assert_eq!(group.state, Some(KubernetesNodeGroupState::Running));
        assert_eq!(group.plan, "4xCPU-8GB");
        assert_eq!(group.anti_affinity, Some(true));
        assert_eq!(group.utility_network_access, Some(true));
        assert_eq!(group.storage.as_deref(), Some("01000000-0000-4000-8000-000160010100"));
        let labels = group.labels.as_ref().unwrap();
        assert_eq!(labels.len(), 1);
        assert_eq!(labels[0].key, "team");
        assert_eq!(labels[0].value, "qa");
        let ssh_keys = group.ssh_keys.as_ref().unwrap();
        assert_eq!(ssh_keys.len(), 1);
        assert_eq!(ssh_keys[0], "ssh-rsa AAAAB3NzaC1yc2EAA");
        let args = group.kubelet_args.as_ref().unwrap();
        assert_eq!(args.len(), 1);
        assert_eq!(args[0].key, "log-flush-frequency");
        assert_eq!(args[0].value, "5s");
        let taints = group.taints.as_ref().unwrap();
        assert_eq!(taints.len(), 1);
        assert_eq!(taints[0].effect, "NoSchedule");
        assert_eq!(taints[0].key, "team");
        assert_eq!(taints[0].value, "qa");
        assert!(group.anti_affinity_status.is_none());
        assert!(group.nodes.is_none());
    }
}
