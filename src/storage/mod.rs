//! Storages — `GET/POST/PUT/DELETE /1.3/storage` + operations.

mod types;

pub use types::*;

use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use tokio::time::sleep;

use crate::client::Client;
use crate::error::Error;

// ─── Handler ──────────────────────────────────────────────────────────────────

/// Handler for storage operations.
pub struct StorageHandler<'a> {
    client: &'a Client,
}

impl<'a> StorageHandler<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Returns a list of all storages, optionally filtered by labels.
    ///
    /// `GET /1.3/storage`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/9-storages/#list-storages)
    ///
    /// Pass `ListParams::default()` to list all storages without filtering.
    pub async fn list(&self, params: &ListParams) -> Result<Vec<Storage>, Error> {
        let pairs = params.to_query_pairs();
        let resp: StorageListResponse = if pairs.is_empty() {
            self.client.get("/1.3/storage").await?
        } else {
            self.client
                .get_with_query_pairs("/1.3/storage", &pairs)
                .await?
        };
        Ok(resp.storages.storage)
    }

    /// Returns a list of public storages (templates, CDROMs).
    ///
    /// `GET /1.3/storage/public`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/9-storages/#public-storages)
    pub async fn list_public(&self) -> Result<Vec<Storage>, Error> {
        let resp: StorageListResponse = self.client.get("/1.3/storage/public").await?;
        Ok(resp.storages.storage)
    }

    /// Returns a list of private storages.
    ///
    /// `GET /1.3/storage/private`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/9-storages/#private-storages)
    pub async fn list_private(&self) -> Result<Vec<Storage>, Error> {
        let resp: StorageListResponse = self.client.get("/1.3/storage/private").await?;
        Ok(resp.storages.storage)
    }

    /// Returns a list of normal (disk) storages.
    ///
    /// `GET /1.3/storage/normal`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/9-storages/#normal-storages)
    pub async fn list_normal(&self) -> Result<Vec<Storage>, Error> {
        let resp: StorageListResponse = self.client.get("/1.3/storage/normal").await?;
        Ok(resp.storages.storage)
    }

    /// Returns a list of backup storages.
    ///
    /// `GET /1.3/storage/backup`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/9-storages/#backups)
    pub async fn list_backups(&self) -> Result<Vec<Storage>, Error> {
        let resp: StorageListResponse = self.client.get("/1.3/storage/backup").await?;
        Ok(resp.storages.storage)
    }

    /// Returns a list of CDROM storages.
    ///
    /// `GET /1.3/storage/cdrom`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/9-storages/#cd-roms)
    pub async fn list_cdroms(&self) -> Result<Vec<Storage>, Error> {
        let resp: StorageListResponse = self.client.get("/1.3/storage/cdrom").await?;
        Ok(resp.storages.storage)
    }

    /// Returns a list of template storages.
    ///
    /// `GET /1.3/storage/template`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/9-storages/#list-storages)
    pub async fn list_templates(&self) -> Result<Vec<Storage>, Error> {
        let resp: StorageListResponse = self.client.get("/1.3/storage/template").await?;
        Ok(resp.storages.storage)
    }

    /// Returns a list of favorite storages.
    ///
    /// `GET /1.3/storage/favorite`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/9-storages/#favorite-storages)
    pub async fn list_favorites(&self) -> Result<Vec<Storage>, Error> {
        let resp: StorageListResponse = self.client.get("/1.3/storage/favorite").await?;
        Ok(resp.storages.storage)
    }

    /// Returns details for a specific storage.
    ///
    /// `GET /1.3/storage/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/9-storages/#get-storage-details)
    pub async fn get(&self, uuid: &str) -> Result<StorageDetails, Error> {
        let resp: StorageResponse = self.client.get(&format!("/1.3/storage/{uuid}")).await?;
        Ok(resp.storage)
    }

    /// Creates a new storage.
    ///
    /// `POST /1.3/storage`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/9-storages/#create-storage)
    pub async fn create(&self, req: CreateRequest) -> Result<StorageDetails, Error> {
        #[derive(Serialize)]
        struct Body {
            storage: CreateRequest,
        }
        let resp: StorageResponse = self
            .client
            .post("/1.3/storage", &Body { storage: req })
            .await?;
        Ok(resp.storage)
    }

    /// Modifies an existing storage (title, size, backup rule).
    ///
    /// `PUT /1.3/storage/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/9-storages/#modify-storage)
    pub async fn modify(&self, uuid: &str, req: ModifyRequest) -> Result<StorageDetails, Error> {
        #[derive(Serialize)]
        struct Body {
            storage: ModifyRequest,
        }
        let resp: StorageResponse = self
            .client
            .put(&format!("/1.3/storage/{uuid}"), &Body { storage: req })
            .await?;
        Ok(resp.storage)
    }

    /// Deletes a storage.
    ///
    /// `DELETE /1.3/storage/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/9-storages/#delete-storage)
    pub async fn delete(&self, uuid: &str) -> Result<(), Error> {
        self.client.delete(&format!("/1.3/storage/{uuid}")).await
    }

    /// Clones a storage.
    ///
    /// `POST /1.3/storage/{uuid}/clone`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/9-storages/#clone-storage)
    pub async fn clone(&self, uuid: &str, req: CloneRequest) -> Result<StorageDetails, Error> {
        #[derive(Serialize)]
        struct Body {
            storage: CloneRequest,
        }
        let resp: StorageResponse = self
            .client
            .post(
                &format!("/1.3/storage/{uuid}/clone"),
                &Body { storage: req },
            )
            .await?;
        Ok(resp.storage)
    }

    /// Creates a template from a storage.
    ///
    /// `POST /1.3/storage/{uuid}/templatize`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/9-storages/#list-storages)
    pub async fn templatize(
        &self,
        uuid: &str,
        req: TemplatizeRequest,
    ) -> Result<StorageDetails, Error> {
        #[derive(Serialize)]
        struct Body {
            storage: TemplatizeRequest,
        }
        let resp: StorageResponse = self
            .client
            .post(
                &format!("/1.3/storage/{uuid}/templatize"),
                &Body { storage: req },
            )
            .await?;
        Ok(resp.storage)
    }

    /// Creates a backup of a storage.
    ///
    /// `POST /1.3/storage/{uuid}/backup`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/9-storages/#create-backup)
    pub async fn create_backup(
        &self,
        uuid: &str,
        req: CreateBackupRequest,
    ) -> Result<StorageDetails, Error> {
        #[derive(Serialize)]
        struct Body {
            storage: CreateBackupRequest,
        }
        let resp: StorageResponse = self
            .client
            .post(
                &format!("/1.3/storage/{uuid}/backup"),
                &Body { storage: req },
            )
            .await?;
        Ok(resp.storage)
    }

    /// Restores a backup storage to the original storage.
    ///
    /// `POST /1.3/storage/{uuid}/restore`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/9-storages/#backups)
    pub async fn restore_backup(&self, uuid: &str) -> Result<(), Error> {
        self.client
            .post_no_response_empty(&format!("/1.3/storage/{uuid}/restore"))
            .await
    }

    /// Marks a storage as a favorite.
    ///
    /// `POST /1.3/storage/{uuid}/favorite`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/9-storages/#add-storage-to-favorites)
    pub async fn add_to_favorites(&self, uuid: &str) -> Result<(), Error> {
        self.client
            .post_no_response_empty(&format!("/1.3/storage/{uuid}/favorite"))
            .await
    }

    /// Removes a storage from favorites.
    ///
    /// `DELETE /1.3/storage/{uuid}/favorite`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/9-storages/#remove-storage-from-favorites)
    pub async fn remove_from_favorites(&self, uuid: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/storage/{uuid}/favorite"))
            .await
    }

    /// Creates a storage import job.
    ///
    /// `POST /1.3/storage/{uuid}/import`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/9-storages/#create-import)
    pub async fn create_import(
        &self,
        uuid: &str,
        req: CreateImportRequest,
    ) -> Result<ImportDetails, Error> {
        #[derive(Serialize)]
        struct Body {
            storage_import: CreateImportRequest,
        }
        let resp: StorageImportResponse = self
            .client
            .post(
                &format!("/1.3/storage/{uuid}/import"),
                &Body {
                    storage_import: req,
                },
            )
            .await?;
        Ok(resp.storage_import)
    }

    /// Returns the status of a storage import.
    ///
    /// `GET /1.3/storage/{uuid}/import`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/9-storages/#get-import-details)
    pub async fn get_import_details(&self, uuid: &str) -> Result<ImportDetails, Error> {
        let resp: StorageImportResponse = self
            .client
            .get(&format!("/1.3/storage/{uuid}/import"))
            .await?;
        Ok(resp.storage_import)
    }

    /// Cancels an in-progress storage import.
    ///
    /// `DELETE /1.3/storage/{uuid}/import`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/9-storages/#cancel-import)
    pub async fn cancel_import(&self, uuid: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/storage/{uuid}/import"))
            .await
    }

    /// Resizes the filesystem of a storage device.
    ///
    /// `POST /1.3/storage/{uuid}/resize`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/9-storages/#resize-storage-partition-filesystem)
    pub async fn resize_filesystem(&self, uuid: &str) -> Result<ResizeFilesystemBackup, Error> {
        let resp: ResizeBackupResponse = self
            .client
            .post_empty(&format!("/1.3/storage/{uuid}/resize"))
            .await?;
        Ok(resp.storage)
    }

    // ─── Polling helpers ──────────────────────────────────────────────────────

    /// Polls until a storage reaches the desired state or the timeout expires.
    pub async fn wait_for_state(
        &self,
        uuid: &str,
        desired_state: State,
        timeout: Duration,
    ) -> Result<StorageDetails, Error> {
        let start = Instant::now();
        loop {
            let storage = self.get(uuid).await?;
            if storage.state == desired_state {
                return Ok(storage);
            }
            if start.elapsed() >= timeout {
                return Err(Error::InvalidConfig(format!(
                    "timeout waiting for storage {} to reach state {:?}",
                    uuid, desired_state
                )));
            }
            sleep(Duration::from_secs(5)).await;
        }
    }
}

// ─── Response wrappers (crate-private) ────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct StorageListResponse {
    storages: StorageWrapper,
}

#[derive(Debug, Deserialize)]
struct StorageWrapper {
    storage: Vec<Storage>,
}

#[derive(Debug, Deserialize)]
struct StorageResponse {
    storage: StorageDetails,
}

#[derive(Debug, Deserialize)]
struct StorageImportResponse {
    storage_import: ImportDetails,
}

#[derive(Debug, Deserialize)]
struct ResizeBackupResponse {
    storage: ResizeFilesystemBackup,
}

#[cfg(test)]
mod tests;
