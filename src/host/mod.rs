//! Dedicated hosts — `GET/PATCH /1.3/host`

mod types;
pub use types::*;

use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::Error;

// ─── Response wrappers ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct HostListResponse {
    hosts: HostWrapper,
}

#[derive(Debug, Deserialize)]
struct HostWrapper {
    host: Vec<Host>,
}

#[derive(Debug, Deserialize)]
struct HostResponse {
    host: Host,
}

// ─── Handler ──────────────────────────────────────────────────────────────────

/// Handler for dedicated host operations.
pub struct HostHandler<'a> {
    client: &'a Client,
}

impl<'a> HostHandler<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Returns a list of all available dedicated hosts.
    ///
    /// `GET /1.3/host`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/14-hosts/#list-available-hosts)
    pub async fn list(&self) -> Result<Vec<Host>, Error> {
        let resp: HostListResponse = self.client.get("/1.3/host").await?;
        Ok(resp.hosts.host)
    }

    /// Returns details about a specific dedicated host.
    ///
    /// `GET /1.3/host/{id}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/14-hosts/#get-host-details)
    pub async fn get(&self, id: i64) -> Result<Host, Error> {
        let resp: HostResponse = self.client.get(&format!("/1.3/host/{id}")).await?;
        Ok(resp.host)
    }

    /// Modifies the description of a dedicated host.
    ///
    /// `PATCH /1.3/host/{id}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/14-hosts/#modify-host-details)
    pub async fn modify(&self, id: i64, req: ModifyHostRequest) -> Result<Host, Error> {
        #[derive(Serialize)]
        struct Body {
            host: ModifyHostRequest,
        }
        let resp: HostResponse = self
            .client
            .patch(&format!("/1.3/host/{id}"), &Body { host: req })
            .await?;
        Ok(resp.host)
    }
}

#[cfg(test)]
mod tests;
