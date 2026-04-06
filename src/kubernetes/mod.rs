//! Managed Kubernetes — `GET/POST/DELETE /1.3/kubernetes` + node groups.

mod types;
pub use types::*;

use crate::client::Client;
use crate::error::Error;

// ─── Handler ──────────────────────────────────────────────────────────────────

/// Handler for managed Kubernetes operations.
pub struct KubernetesHandler<'a> {
    client: &'a Client,
}

impl<'a> KubernetesHandler<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Returns a list of available Kubernetes node plans.
    ///
    /// `GET /1.3/kubernetes/plans`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/20-managed-kubernetes/#list-plans)
    pub async fn list_plans(&self) -> Result<Vec<Plan>, Error> {
        self.client.get("/1.3/kubernetes/plans").await
    }

    /// Returns a list of all Kubernetes clusters.
    ///
    /// `GET /1.3/kubernetes`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/20-managed-kubernetes/#list-clusters)
    pub async fn list(&self) -> Result<Vec<Cluster>, Error> {
        self.client.get("/1.3/kubernetes").await
    }

    /// Returns details for a specific Kubernetes cluster.
    ///
    /// `GET /1.3/kubernetes/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/20-managed-kubernetes/#get-cluster-details)
    pub async fn get(&self, uuid: &str) -> Result<Cluster, Error> {
        self.client
            .get(&format!("/1.3/kubernetes/{uuid}"))
            .await
    }

    /// Creates a new Kubernetes cluster.
    ///
    /// `POST /1.3/kubernetes`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/20-managed-kubernetes/#create-cluster)
    pub async fn create(&self, req: CreateClusterRequest) -> Result<Cluster, Error> {
        self.client.post("/1.3/kubernetes", &req).await
    }

    /// Deletes a Kubernetes cluster.
    ///
    /// `DELETE /1.3/kubernetes/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/20-managed-kubernetes/#delete-cluster)
    pub async fn delete(&self, uuid: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/kubernetes/{uuid}"))
            .await
    }

    /// Modifies a Kubernetes cluster.
    ///
    /// `PATCH /1.3/kubernetes/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/20-managed-kubernetes/#modify-cluster)
    pub async fn modify_cluster(
        &self,
        uuid: &str,
        req: ModifyClusterRequest,
    ) -> Result<Cluster, Error> {
        self.client
            .patch(&format!("/1.3/kubernetes/{uuid}"), &req)
            .await
    }

    /// Returns a list of available Kubernetes versions.
    ///
    /// `GET /1.3/kubernetes/versions`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/20-managed-kubernetes/#list-versions)
    pub async fn list_versions(&self) -> Result<Vec<KubernetesVersion>, Error> {
        self.client.get("/1.3/kubernetes/versions").await
    }

    /// Returns the kubeconfig YAML for a Kubernetes cluster.
    ///
    /// `GET /1.3/kubernetes/{uuid}/kubeconfig`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/20-managed-kubernetes/#get-kubeconfig)
    pub async fn get_kubeconfig(&self, uuid: &str) -> Result<String, Error> {
        let resp: Kubeconfig = self
            .client
            .get(&format!("/1.3/kubernetes/{uuid}/kubeconfig"))
            .await?;
        Ok(resp.kubeconfig)
    }

    // ─── Node groups ──────────────────────────────────────────────────────────

    /// Returns a list of node groups for a cluster.
    ///
    /// `GET /1.3/kubernetes/{uuid}/node-groups`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/20-managed-kubernetes/#list-node-groups)
    pub async fn list_node_groups(
        &self,
        cluster_uuid: &str,
    ) -> Result<Vec<NodeGroup>, Error> {
        self.client
            .get(&format!("/1.3/kubernetes/{cluster_uuid}/node-groups"))
            .await
    }

    /// Returns details for a specific node group.
    ///
    /// `GET /1.3/kubernetes/{uuid}/node-groups/{name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/20-managed-kubernetes/#get-node-group-details)
    pub async fn get_node_group(
        &self,
        cluster_uuid: &str,
        name: &str,
    ) -> Result<NodeGroup, Error> {
        self.client
            .get(&format!(
                "/1.3/kubernetes/{cluster_uuid}/node-groups/{name}"
            ))
            .await
    }

    /// Creates a node group in a cluster.
    ///
    /// `POST /1.3/kubernetes/{uuid}/node-groups`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/20-managed-kubernetes/#create-node-group)
    pub async fn create_node_group(
        &self,
        cluster_uuid: &str,
        req: CreateNodeGroupRequest,
    ) -> Result<NodeGroup, Error> {
        self.client
            .post(
                &format!("/1.3/kubernetes/{cluster_uuid}/node-groups"),
                &req,
            )
            .await
    }

    /// Modifies a node group (e.g. scale count).
    ///
    /// `PATCH /1.3/kubernetes/{uuid}/node-groups/{name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/20-managed-kubernetes/#modify-node-group)
    pub async fn modify_node_group(
        &self,
        cluster_uuid: &str,
        name: &str,
        req: ModifyNodeGroupRequest,
    ) -> Result<NodeGroup, Error> {
        self.client
            .patch(
                &format!("/1.3/kubernetes/{cluster_uuid}/node-groups/{name}"),
                &req,
            )
            .await
    }

    /// Deletes a node group.
    ///
    /// `DELETE /1.3/kubernetes/{uuid}/node-groups/{name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/20-managed-kubernetes/#delete-node-group)
    pub async fn delete_node_group(
        &self,
        cluster_uuid: &str,
        name: &str,
    ) -> Result<(), Error> {
        self.client
            .delete(&format!(
                "/1.3/kubernetes/{cluster_uuid}/node-groups/{name}"
            ))
            .await
    }

    /// Deletes a specific node from a node group.
    ///
    /// `DELETE /1.3/kubernetes/{uuid}/node-groups/{node_group_name}/nodes/{node_name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/20-managed-kubernetes/#delete-node)
    pub async fn delete_node(
        &self,
        cluster_uuid: &str,
        node_group_name: &str,
        node_name: &str,
    ) -> Result<(), Error> {
        self.client
            .delete(&format!(
                "/1.3/kubernetes/{cluster_uuid}/node-groups/{node_group_name}/nodes/{node_name}"
            ))
            .await
    }

    // ─── Wait helpers ─────────────────────────────────────────────────────────

    /// Polls until the cluster reaches the desired state or the timeout expires.
    pub async fn wait_for_cluster_state(
        &self,
        uuid: &str,
        desired_state: ClusterState,
        timeout: std::time::Duration,
    ) -> Result<Cluster, Error> {
        let start = std::time::Instant::now();
        loop {
            let cluster = self.get(uuid).await?;
            if cluster.state == desired_state {
                return Ok(cluster);
            }
            if start.elapsed() >= timeout {
                return Err(crate::error::Error::InvalidConfig(format!(
                    "timeout waiting for cluster {} to reach state {:?}",
                    uuid, desired_state
                )));
            }
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    }

    /// Polls until the node group reaches the desired state or the timeout expires.
    pub async fn wait_for_node_group_state(
        &self,
        cluster_uuid: &str,
        group_name: &str,
        desired_state: NodeGroupState,
        timeout: std::time::Duration,
    ) -> Result<NodeGroup, Error> {
        let start = std::time::Instant::now();
        loop {
            let group = self.get_node_group(cluster_uuid, group_name).await?;
            if group.state == Some(desired_state) {
                return Ok(group);
            }
            if start.elapsed() >= timeout {
                return Err(crate::error::Error::InvalidConfig(format!(
                    "timeout waiting for node group {} to reach state {:?}",
                    group_name, desired_state
                )));
            }
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    }
}

#[cfg(test)]
mod tests;
