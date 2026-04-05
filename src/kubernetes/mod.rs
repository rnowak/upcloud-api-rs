//! Managed Kubernetes — `GET/POST/DELETE /1.3/kubernetes` + node groups.

mod types;
pub use types::*;

use crate::client::Client;
use crate::error::Error;

// ─── Handler ──────────────────────────────────────────────────────────────────

pub struct KubernetesHandler<'a> {
    client: &'a Client,
}

impl<'a> KubernetesHandler<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Returns a list of available Kubernetes node plans.
    pub async fn list_plans(&self) -> Result<Vec<Plan>, Error> {
        self.client.get("/1.3/kubernetes/plans").await
    }

    /// Returns a list of all Kubernetes clusters.
    pub async fn list(&self) -> Result<Vec<Cluster>, Error> {
        self.client.get("/1.3/kubernetes").await
    }

    /// Returns details for a specific Kubernetes cluster.
    pub async fn get(&self, uuid: &str) -> Result<Cluster, Error> {
        self.client
            .get(&format!("/1.3/kubernetes/{uuid}"))
            .await
    }

    /// Creates a new Kubernetes cluster.
    pub async fn create(&self, req: CreateClusterRequest) -> Result<Cluster, Error> {
        self.client.post("/1.3/kubernetes", &req).await
    }

    /// Deletes a Kubernetes cluster.
    pub async fn delete(&self, uuid: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/kubernetes/{uuid}"))
            .await
    }

    /// Returns the kubeconfig YAML for a Kubernetes cluster.
    pub async fn get_kubeconfig(&self, uuid: &str) -> Result<String, Error> {
        let resp: Kubeconfig = self
            .client
            .get(&format!("/1.3/kubernetes/{uuid}/kubeconfig"))
            .await?;
        Ok(resp.kubeconfig)
    }

    // ─── Node groups ──────────────────────────────────────────────────────────

    /// Returns a list of node groups for a cluster.
    pub async fn list_node_groups(
        &self,
        cluster_uuid: &str,
    ) -> Result<Vec<NodeGroup>, Error> {
        self.client
            .get(&format!("/1.3/kubernetes/{cluster_uuid}/node-groups"))
            .await
    }

    /// Returns details for a specific node group.
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
}

#[cfg(test)]
mod tests;
