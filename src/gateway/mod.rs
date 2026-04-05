//! Network Gateways — `GET/POST/PATCH/DELETE /1.3/gateway` + connections, tunnels, routes.

mod types;
pub use types::*;

use crate::client::Client;
use crate::error::Error;
use crate::types::common::PageParams;

// ─── Handler ──────────────────────────────────────────────────────────────────

pub struct GatewayHandler<'a> {
    client: &'a Client,
}

impl<'a> GatewayHandler<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    // ─── Plans ────────────────────────────────────────────────────────────────

    /// Returns a list of available gateway plans.
    pub async fn list_plans(&self) -> Result<Vec<Plan>, Error> {
        self.client.get("/1.3/gateway/plans").await
    }

    /// Returns details for a specific gateway plan.
    pub async fn get_plan(&self, name: &str) -> Result<Plan, Error> {
        self.client
            .get(&format!("/1.3/gateway/plans/{name}"))
            .await
    }

    // ─── Gateways ─────────────────────────────────────────────────────────────

    /// Returns a list of all gateways.
    pub async fn list(&self, params: PageParams) -> Result<Vec<Gateway>, Error> {
        self.client
            .get_with_query("/1.3/gateway", &params)
            .await
    }

    /// Returns details for a specific gateway.
    pub async fn get(&self, uuid: &str) -> Result<Gateway, Error> {
        self.client.get(&format!("/1.3/gateway/{uuid}")).await
    }

    /// Creates a new gateway.
    pub async fn create(&self, req: CreateRequest) -> Result<Gateway, Error> {
        self.client.post("/1.3/gateway", &req).await
    }

    /// Modifies an existing gateway.
    pub async fn modify(
        &self,
        uuid: &str,
        req: ModifyRequest,
    ) -> Result<Gateway, Error> {
        self.client
            .patch(&format!("/1.3/gateway/{uuid}"), &req)
            .await
    }

    /// Deletes a gateway.
    pub async fn delete(&self, uuid: &str) -> Result<(), Error> {
        self.client.delete(&format!("/1.3/gateway/{uuid}")).await
    }

    // ─── Connections ──────────────────────────────────────────────────────────

    /// Returns a list of connections for a gateway.
    pub async fn list_connections(
        &self,
        gateway_uuid: &str,
    ) -> Result<Vec<Connection>, Error> {
        self.client
            .get(&format!("/1.3/gateway/{gateway_uuid}/connections"))
            .await
    }

    /// Returns details for a specific connection.
    pub async fn get_connection(
        &self,
        gateway_uuid: &str,
        name: &str,
    ) -> Result<Connection, Error> {
        self.client
            .get(&format!(
                "/1.3/gateway/{gateway_uuid}/connections/{name}"
            ))
            .await
    }

    /// Creates a connection for a gateway.
    pub async fn create_connection(
        &self,
        gateway_uuid: &str,
        req: CreateConnectionRequest,
    ) -> Result<Connection, Error> {
        self.client
            .post(
                &format!("/1.3/gateway/{gateway_uuid}/connections"),
                &req,
            )
            .await
    }

    /// Deletes a connection.
    pub async fn delete_connection(
        &self,
        gateway_uuid: &str,
        name: &str,
    ) -> Result<(), Error> {
        self.client
            .delete(&format!(
                "/1.3/gateway/{gateway_uuid}/connections/{name}"
            ))
            .await
    }

    // ─── Tunnels ──────────────────────────────────────────────────────────────

    /// Returns a list of tunnels for a connection.
    pub async fn list_tunnels(
        &self,
        gateway_uuid: &str,
        connection_name: &str,
    ) -> Result<Vec<Tunnel>, Error> {
        self.client
            .get(&format!(
                "/1.3/gateway/{gateway_uuid}/connections/{connection_name}/tunnels"
            ))
            .await
    }

    /// Creates a tunnel for a connection.
    pub async fn create_tunnel(
        &self,
        gateway_uuid: &str,
        connection_name: &str,
        req: CreateTunnelRequest,
    ) -> Result<Tunnel, Error> {
        self.client
            .post(
                &format!(
                    "/1.3/gateway/{gateway_uuid}/connections/{connection_name}/tunnels"
                ),
                &req,
            )
            .await
    }

    /// Deletes a tunnel.
    pub async fn delete_tunnel(
        &self,
        gateway_uuid: &str,
        connection_name: &str,
        tunnel_name: &str,
    ) -> Result<(), Error> {
        self.client
            .delete(&format!(
                "/1.3/gateway/{gateway_uuid}/connections/{connection_name}/tunnels/{tunnel_name}"
            ))
            .await
    }

    // ─── Routes ───────────────────────────────────────────────────────────────

    /// Returns a list of routes for a gateway.
    pub async fn list_routes(
        &self,
        gateway_uuid: &str,
    ) -> Result<Vec<Route>, Error> {
        self.client
            .get(&format!("/1.3/gateway/{gateway_uuid}/routes"))
            .await
    }

    /// Creates a route for a gateway.
    pub async fn create_route(
        &self,
        gateway_uuid: &str,
        req: RouteInput,
    ) -> Result<Route, Error> {
        self.client
            .post(&format!("/1.3/gateway/{gateway_uuid}/routes"), &req)
            .await
    }

    /// Deletes a route from a gateway.
    pub async fn delete_route(
        &self,
        gateway_uuid: &str,
        route_name: &str,
    ) -> Result<(), Error> {
        self.client
            .delete(&format!(
                "/1.3/gateway/{gateway_uuid}/routes/{route_name}"
            ))
            .await
    }

    // ─── Labels ───────────────────────────────────────────────────────────────

    /// Returns a list of labels for a gateway.
    pub async fn list_labels(
        &self,
        gateway_uuid: &str,
    ) -> Result<Vec<crate::types::common::Label>, Error> {
        self.client
            .get(&format!("/1.3/gateway/{gateway_uuid}/labels"))
            .await
    }

    /// Creates a label on a gateway.
    pub async fn create_label(
        &self,
        gateway_uuid: &str,
        label: crate::types::common::Label,
    ) -> Result<crate::types::common::Label, Error> {
        self.client
            .post(
                &format!("/1.3/gateway/{gateway_uuid}/labels"),
                &label,
            )
            .await
    }

    /// Modifies a label on a gateway.
    pub async fn modify_label(
        &self,
        gateway_uuid: &str,
        key: &str,
        label: crate::types::common::Label,
    ) -> Result<crate::types::common::Label, Error> {
        self.client
            .patch(
                &format!("/1.3/gateway/{gateway_uuid}/labels/{key}"),
                &label,
            )
            .await
    }

    /// Deletes a label from a gateway.
    pub async fn delete_label(
        &self,
        gateway_uuid: &str,
        key: &str,
    ) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/gateway/{gateway_uuid}/labels/{key}"))
            .await
    }
}

#[cfg(test)]
mod tests;
