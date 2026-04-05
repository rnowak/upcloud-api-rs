//! Tags — `GET/POST/PUT/DELETE /1.3/tag`, tag/untag server.

mod types;
pub use types::*;

use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::Error;

// ─── Response wrappers ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct TagListResponse {
    tags: TagWrapper,
}

#[derive(Debug, Deserialize)]
struct TagWrapper {
    tag: Vec<Tag>,
}

#[derive(Debug, Deserialize)]
struct TagResponse {
    tag: Tag,
}

// ─── Handler ──────────────────────────────────────────────────────────────────

pub struct TagHandler<'a> {
    client: &'a Client,
}

impl<'a> TagHandler<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Returns a list of all tags.
    pub async fn list(&self) -> Result<Vec<Tag>, Error> {
        let resp: TagListResponse = self.client.get("/1.3/tag").await?;
        Ok(resp.tags.tag)
    }

    /// Creates a new tag.
    pub async fn create(&self, req: CreateTagRequest) -> Result<Tag, Error> {
        #[derive(Serialize)]
        struct Body {
            tag: CreateTagRequest,
        }
        let resp: TagResponse = self.client.post("/1.3/tag", &Body { tag: req }).await?;
        Ok(resp.tag)
    }

    /// Modifies an existing tag.
    pub async fn modify(&self, name: &str, req: ModifyTagRequest) -> Result<Tag, Error> {
        #[derive(Serialize)]
        struct Body {
            tag: ModifyTagRequest,
        }
        let resp: TagResponse = self
            .client
            .put(&format!("/1.3/tag/{name}"), &Body { tag: req })
            .await?;
        Ok(resp.tag)
    }

    /// Deletes a tag.
    pub async fn delete(&self, name: &str) -> Result<(), Error> {
        self.client.delete(&format!("/1.3/tag/{name}")).await
    }

    /// Adds one or more tags to a server.
    ///
    /// `tags` is a comma-separated list of tag names, e.g. `"prod,web"`.
    pub async fn tag_server(&self, server_uuid: &str, tags: &str) -> Result<(), Error> {
        self.client
            .post_no_response_empty(&format!("/1.3/server/{server_uuid}/tag/{tags}"))
            .await
    }

    /// Removes one or more tags from a server.
    ///
    /// `tags` is a comma-separated list of tag names, e.g. `"prod,web"`.
    pub async fn untag_server(&self, server_uuid: &str, tags: &str) -> Result<(), Error> {
        self.client
            .post_no_response_empty(&format!("/1.3/server/{server_uuid}/untag/{tags}"))
            .await
    }
}

#[cfg(test)]
mod tests;
