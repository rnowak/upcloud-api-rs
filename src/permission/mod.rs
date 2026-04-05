//! Permissions — `GET /1.3/permission`, `POST /1.3/permission/grant|revoke`

mod types;
pub use types::*;

use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::Error;

// ─── Response wrappers ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct PermissionListResponse {
    permissions: PermissionWrapper,
}

#[derive(Debug, Deserialize)]
struct PermissionWrapper {
    permission: Vec<Permission>,
}

// ─── Handler ──────────────────────────────────────────────────────────────────

/// Handler for permission operations.
pub struct PermissionHandler<'a> {
    client: &'a Client,
}

impl<'a> PermissionHandler<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Returns a list of all permissions for the current account.
    ///
    /// `GET /1.3/permission`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/18-permissions/#get-permissions)
    pub async fn list(&self) -> Result<Vec<Permission>, Error> {
        let resp: PermissionListResponse = self.client.get("/1.3/permission").await?;
        Ok(resp.permissions.permission)
    }

    /// Grants a permission to a subaccount.
    ///
    /// `POST /1.3/permission/grant`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/18-permissions/#grant-permission)
    pub async fn grant(&self, req: GrantRequest) -> Result<(), Error> {
        #[derive(Serialize)]
        struct Body {
            permission: GrantRequest,
        }
        self.client
            .post_no_response("/1.3/permission/grant", &Body { permission: req })
            .await
    }

    /// Revokes a permission from a subaccount.
    ///
    /// `POST /1.3/permission/revoke`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/18-permissions/#revoke-permission)
    pub async fn revoke(&self, req: RevokeRequest) -> Result<(), Error> {
        #[derive(Serialize)]
        struct Body {
            permission: RevokeRequest,
        }
        self.client
            .post_no_response("/1.3/permission/revoke", &Body { permission: req })
            .await
    }
}

#[cfg(test)]
mod tests;
