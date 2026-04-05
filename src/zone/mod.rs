//! Zones — available UpCloud data center zones.

mod types;

pub use types::*;

use crate::client::Client;
use crate::error::Error;

/// Handler for zone operations.
pub struct ZoneHandler<'a> {
    client: &'a Client,
}

impl<'a> ZoneHandler<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Returns a list of all available zones.
    ///
    /// `GET /1.3/zone`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/5-zones/#list-available-zones)
    pub async fn list(&self) -> Result<Vec<Zone>, Error> {
        let resp: ZoneListResponse = self.client.get("/1.3/zone").await?;
        Ok(resp.zones.zone)
    }
}

// ─── Response wrappers (crate-private) ────────────────────────────────────────

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ZoneListResponse {
    zones: ZoneWrapper,
}

#[derive(Debug, Deserialize)]
struct ZoneWrapper {
    zone: Vec<Zone>,
}

#[cfg(test)]
mod tests;
