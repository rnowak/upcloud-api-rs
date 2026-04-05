//! Server plans — `GET /1.3/plan` and `GET /1.3/server_size`

mod types;
pub use types::*;

use serde::Deserialize;

use crate::client::Client;
use crate::error::Error;

// ─── Response wrappers ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct PlanListResponse {
    plans: PlanWrapper,
}

#[derive(Debug, Deserialize)]
struct PlanWrapper {
    plan: Vec<Plan>,
}

#[derive(Debug, Deserialize)]
struct ServerConfigurationListResponse {
    server_sizes: ServerConfigurationWrapper,
}

#[derive(Debug, Deserialize)]
struct ServerConfigurationWrapper {
    server_size: Vec<ServerConfiguration>,
}

// ─── Handler ──────────────────────────────────────────────────────────────────

/// Handler for server plan operations.
pub struct PlanHandler<'a> {
    client: &'a Client,
}

impl<'a> PlanHandler<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Returns a list of all available predefined server plans.
    ///
    /// `GET /1.3/plan`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/7-plans/#list-available-plans)
    pub async fn list(&self) -> Result<Vec<Plan>, Error> {
        let resp: PlanListResponse = self.client.get("/1.3/plan").await?;
        Ok(resp.plans.plan)
    }

    /// Returns a list of valid CPU/memory combinations for custom servers.
    ///
    /// `GET /1.3/server_size`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/8-servers/#list-server-configurations)
    pub async fn list_server_configurations(&self) -> Result<Vec<ServerConfiguration>, Error> {
        let resp: ServerConfigurationListResponse = self.client.get("/1.3/server_size").await?;
        Ok(resp.server_sizes.server_size)
    }
}

#[cfg(test)]
mod tests;
