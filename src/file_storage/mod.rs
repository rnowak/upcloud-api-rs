//! File Storage — `GET/POST/PATCH/DELETE /1.3/file-storage` + shares, networks, plans.

mod types;

pub use types::*;

use crate::client::Client;
use crate::error::Error;
use crate::types::PageParams;

/// Handler for file storage operations.
pub struct FileStorageHandler<'a> {
    client: &'a Client,
}

impl<'a> FileStorageHandler<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Returns a list of available file storage plans.
    ///
    /// `GET /1.3/file-storage/plans`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/25-file-storage/)
    pub async fn list_plans(&self) -> Result<Vec<Plan>, Error> {
        self.client.get("/1.3/file-storage/plans").await
    }

    // ─── Services ─────────────────────────────────────────────────────────────

    /// Returns a list of all file storage services.
    ///
    /// `GET /1.3/file-storage`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/25-file-storage/#list-services)
    pub async fn list(&self, params: PageParams) -> Result<Vec<Service>, Error> {
        self.client
            .get_with_query("/1.3/file-storage", &params)
            .await
    }

    /// Returns details for a specific file storage service.
    ///
    /// `GET /1.3/file-storage/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/25-file-storage/#get-service-details)
    pub async fn get(&self, uuid: &str) -> Result<Service, Error> {
        self.client.get(&format!("/1.3/file-storage/{uuid}")).await
    }

    /// Creates a new file storage service.
    ///
    /// `POST /1.3/file-storage`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/25-file-storage/#create-service)
    pub async fn create(&self, req: CreateRequest) -> Result<Service, Error> {
        self.client.post("/1.3/file-storage", &req).await
    }

    /// Modifies an existing file storage service.
    ///
    /// `PATCH /1.3/file-storage/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/25-file-storage/#modify-service)
    pub async fn modify(&self, uuid: &str, req: ModifyRequest) -> Result<Service, Error> {
        self.client
            .patch(&format!("/1.3/file-storage/{uuid}"), &req)
            .await
    }

    /// Deletes a file storage service.
    ///
    /// `DELETE /1.3/file-storage/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/25-file-storage/#delete-service)
    pub async fn delete(&self, uuid: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/file-storage/{uuid}"))
            .await
    }

    // ─── Shares ───────────────────────────────────────────────────────────────

    /// Returns a list of NFS shares for a file storage service.
    ///
    /// `GET /1.3/file-storage/{uuid}/shares`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/25-file-storage/#list-shares)
    pub async fn list_shares(&self, uuid: &str) -> Result<Vec<Share>, Error> {
        self.client
            .get(&format!("/1.3/file-storage/{uuid}/shares"))
            .await
    }

    /// Returns details for a specific share.
    ///
    /// `GET /1.3/file-storage/{uuid}/shares/{share_name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/25-file-storage/#get-share-details)
    pub async fn get_share(&self, uuid: &str, share_name: &str) -> Result<Share, Error> {
        self.client
            .get(&format!("/1.3/file-storage/{uuid}/shares/{share_name}"))
            .await
    }

    /// Creates an NFS share for a file storage service.
    ///
    /// `POST /1.3/file-storage/{uuid}/shares`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/25-file-storage/#create-share)
    pub async fn create_share(&self, uuid: &str, req: CreateShareRequest) -> Result<Share, Error> {
        self.client
            .post(&format!("/1.3/file-storage/{uuid}/shares"), &req)
            .await
    }

    /// Modifies an NFS share.
    ///
    /// `PATCH /1.3/file-storage/{uuid}/shares/{share_name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/25-file-storage/#modify-share)
    pub async fn modify_share(
        &self,
        uuid: &str,
        share_name: &str,
        req: ModifyShareRequest,
    ) -> Result<Share, Error> {
        self.client
            .patch(
                &format!("/1.3/file-storage/{uuid}/shares/{share_name}"),
                &req,
            )
            .await
    }

    /// Deletes an NFS share.
    ///
    /// `DELETE /1.3/file-storage/{uuid}/shares/{share_name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/25-file-storage/#delete-share)
    pub async fn delete_share(&self, uuid: &str, share_name: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/file-storage/{uuid}/shares/{share_name}"))
            .await
    }

    // ─── Networks ─────────────────────────────────────────────────────────────

    /// Returns a list of networks attached to a file storage service.
    ///
    /// `GET /1.3/file-storage/{uuid}/networks`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/25-file-storage/#list-networks)
    pub async fn list_networks(&self, uuid: &str) -> Result<Vec<Network>, Error> {
        self.client
            .get(&format!("/1.3/file-storage/{uuid}/networks"))
            .await
    }

    /// Attaches a network to a file storage service.
    ///
    /// `POST /1.3/file-storage/{uuid}/networks`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/25-file-storage/#create-network)
    pub async fn create_network(&self, uuid: &str, req: NetworkInput) -> Result<Network, Error> {
        self.client
            .post(&format!("/1.3/file-storage/{uuid}/networks"), &req)
            .await
    }

    /// Detaches a network from a file storage service.
    ///
    /// `DELETE /1.3/file-storage/{uuid}/networks/{network_name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/25-file-storage/#delete-network)
    pub async fn delete_network(&self, uuid: &str, network_name: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/file-storage/{uuid}/networks/{network_name}"))
            .await
    }
}

#[cfg(test)]
mod tests;
