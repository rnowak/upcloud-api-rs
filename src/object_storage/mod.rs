//! Object Storage — `GET/POST/PATCH/DELETE /1.3/object-storage-2` + sub-resources.
//!
//! NOTE: The base path is `/object-storage-2` (not `/object-storage`).
//! The legacy Object Storage was at `/object-storage`.

mod types;
pub use types::*;

use serde::Serialize;

use crate::client::Client;
use crate::error::Error;
use crate::types::common::PageParams;

pub struct ObjectStorageHandler<'a> {
    client: &'a Client,
}

impl<'a> ObjectStorageHandler<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    // ─── Regions ──────────────────────────────────────────────────────────────

    /// Returns a list of available managed object storage regions.
    pub async fn list_regions(&self) -> Result<Vec<Region>, Error> {
        self.client.get("/1.3/object-storage-2/regions").await
    }

    /// Returns details for a specific managed object storage region.
    pub async fn get_region(&self, name: &str) -> Result<Region, Error> {
        self.client
            .get(&format!("/1.3/object-storage-2/regions/{name}"))
            .await
    }

    // ─── Services ─────────────────────────────────────────────────────────────

    /// Returns a list of all managed object storage services.
    pub async fn list(&self, params: PageParams) -> Result<Vec<Service>, Error> {
        self.client
            .get_with_query("/1.3/object-storage-2", &params)
            .await
    }

    /// Returns details for a specific managed object storage service.
    pub async fn get(&self, uuid: &str) -> Result<Service, Error> {
        self.client
            .get(&format!("/1.3/object-storage-2/{uuid}"))
            .await
    }

    /// Creates a new managed object storage service.
    pub async fn create(&self, req: CreateRequest) -> Result<Service, Error> {
        self.client.post("/1.3/object-storage-2", &req).await
    }

    /// Modifies a managed object storage service.
    pub async fn modify(&self, uuid: &str, req: ModifyRequest) -> Result<Service, Error> {
        self.client
            .patch(&format!("/1.3/object-storage-2/{uuid}"), &req)
            .await
    }

    /// Deletes a managed object storage service.
    pub async fn delete(&self, uuid: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/object-storage-2/{uuid}"))
            .await
    }

    /// Returns usage statistics for a managed object storage service.
    pub async fn get_metrics(&self, uuid: &str) -> Result<Metrics, Error> {
        self.client
            .get(&format!("/1.3/object-storage-2/{uuid}/stats"))
            .await
    }

    // ─── Users ────────────────────────────────────────────────────────────────

    /// Returns a list of users for a managed object storage service.
    pub async fn list_users(&self, uuid: &str) -> Result<Vec<User>, Error> {
        self.client
            .get(&format!("/1.3/object-storage-2/{uuid}/users"))
            .await
    }

    /// Returns details for a specific user.
    pub async fn get_user(&self, uuid: &str, username: &str) -> Result<User, Error> {
        self.client
            .get(&format!("/1.3/object-storage-2/{uuid}/users/{username}"))
            .await
    }

    /// Creates a user for a managed object storage service.
    pub async fn create_user(
        &self,
        uuid: &str,
        req: CreateUserRequest,
    ) -> Result<User, Error> {
        self.client
            .post(&format!("/1.3/object-storage-2/{uuid}/users"), &req)
            .await
    }

    /// Deletes a user from a managed object storage service.
    pub async fn delete_user(&self, uuid: &str, username: &str) -> Result<(), Error> {
        self.client
            .delete(&format!(
                "/1.3/object-storage-2/{uuid}/users/{username}"
            ))
            .await
    }

    // ─── User credentials ─────────────────────────────────────────────────────

    /// Returns a list of access keys for a user.
    pub async fn list_user_access_keys(
        &self,
        uuid: &str,
        username: &str,
    ) -> Result<Vec<UserAccessKey>, Error> {
        self.client
            .get(&format!(
                "/1.3/object-storage-2/{uuid}/users/{username}/access-keys"
            ))
            .await
    }

    /// Creates an access key for a user.
    pub async fn create_user_access_key(
        &self,
        uuid: &str,
        username: &str,
    ) -> Result<UserAccessKey, Error> {
        self.client
            .post_empty(&format!(
                "/1.3/object-storage-2/{uuid}/users/{username}/access-keys"
            ))
            .await
    }

    /// Deletes an access key for a user.
    pub async fn delete_user_access_key(
        &self,
        uuid: &str,
        username: &str,
        access_key_id: &str,
    ) -> Result<(), Error> {
        self.client
            .delete(&format!(
                "/1.3/object-storage-2/{uuid}/users/{username}/access-keys/{access_key_id}"
            ))
            .await
    }

    // ─── Policies ─────────────────────────────────────────────────────────────

    /// Returns a list of policies for a managed object storage service.
    pub async fn list_policies(&self, uuid: &str) -> Result<Vec<Policy>, Error> {
        self.client
            .get(&format!("/1.3/object-storage-2/{uuid}/policies"))
            .await
    }

    /// Creates a policy for a managed object storage service.
    pub async fn create_policy(
        &self,
        uuid: &str,
        req: CreatePolicyRequest,
    ) -> Result<Policy, Error> {
        self.client
            .post(&format!("/1.3/object-storage-2/{uuid}/policies"), &req)
            .await
    }

    /// Deletes a policy.
    pub async fn delete_policy(&self, uuid: &str, policy_name: &str) -> Result<(), Error> {
        self.client
            .delete(&format!(
                "/1.3/object-storage-2/{uuid}/policies/{policy_name}"
            ))
            .await
    }

    /// Returns policies attached to a user.
    pub async fn list_user_policies(
        &self,
        uuid: &str,
        username: &str,
    ) -> Result<Vec<Policy>, Error> {
        self.client
            .get(&format!(
                "/1.3/object-storage-2/{uuid}/users/{username}/policies"
            ))
            .await
    }

    /// Attaches a policy to a user.
    pub async fn attach_user_policy(
        &self,
        uuid: &str,
        username: &str,
        policy_name: &str,
    ) -> Result<(), Error> {
        #[derive(Serialize)]
        struct Body {
            name: String,
        }
        self.client
            .post_no_response(
                &format!("/1.3/object-storage-2/{uuid}/users/{username}/policies"),
                &Body {
                    name: policy_name.to_string(),
                },
            )
            .await
    }

    /// Detaches a policy from a user.
    pub async fn detach_user_policy(
        &self,
        uuid: &str,
        username: &str,
        policy_name: &str,
    ) -> Result<(), Error> {
        self.client
            .delete(&format!(
                "/1.3/object-storage-2/{uuid}/users/{username}/policies/{policy_name}"
            ))
            .await
    }

    // ─── Buckets ──────────────────────────────────────────────────────────────

    /// Returns a list of buckets in a managed object storage service.
    pub async fn list_buckets(&self, uuid: &str) -> Result<Vec<Bucket>, Error> {
        self.client
            .get(&format!("/1.3/object-storage-2/{uuid}/buckets"))
            .await
    }

    /// Returns CORS configurations for buckets.
    pub async fn list_bucket_cors(&self, uuid: &str) -> Result<serde_json::Value, Error> {
        self.client
            .get(&format!("/1.3/object-storage-2/{uuid}/buckets/cors"))
            .await
    }

    // ─── Networks ─────────────────────────────────────────────────────────────

    /// Returns a list of networks for a managed object storage service.
    pub async fn list_networks(&self, uuid: &str) -> Result<Vec<Network>, Error> {
        self.client
            .get(&format!("/1.3/object-storage-2/{uuid}/networks"))
            .await
    }

    /// Creates a network for a managed object storage service.
    pub async fn create_network(
        &self,
        uuid: &str,
        req: NetworkInput,
    ) -> Result<Network, Error> {
        self.client
            .post(&format!("/1.3/object-storage-2/{uuid}/networks"), &req)
            .await
    }

    /// Deletes a network from a managed object storage service.
    pub async fn delete_network(&self, uuid: &str, network_name: &str) -> Result<(), Error> {
        self.client
            .delete(&format!(
                "/1.3/object-storage-2/{uuid}/networks/{network_name}"
            ))
            .await
    }

    // ─── Metrics ──────────────────────────────────────────────────────────────

    /// Returns service-level metrics (total objects, total size).
    pub async fn get_usage(&self, uuid: &str) -> Result<Usage, Error> {
        self.client
            .get(&format!("/1.3/object-storage-2/{uuid}/metrics"))
            .await
    }
}

#[cfg(test)]
mod tests;
