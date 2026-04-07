//! Network Gateways — `GET/POST/PATCH/DELETE /1.3/gateway` + connections, tunnels, routes.

mod types;
pub use types::*;

use crate::client::Client;
use crate::error::Error;
use crate::types::PageParams;

use std::time::{Duration, Instant};
use tokio::time::sleep;

// ─── Handler ──────────────────────────────────────────────────────────────────

/// Handler for network gateway operations.
pub struct GatewayHandler<'a> {
    client: &'a Client,
}

impl<'a> GatewayHandler<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    // ─── Plans ────────────────────────────────────────────────────────────────

    /// Returns a list of available gateway plans.
    ///
    /// `GET /1.3/gateway/plans`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/19-network-gateways/#list-plans)
    pub async fn list_plans(&self) -> Result<Vec<Plan>, Error> {
        self.client.get("/1.3/gateway/plans").await
    }

    /// Returns details for a specific gateway plan.
    ///
    /// `GET /1.3/gateway/plans/{name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/19-network-gateways/#get-plan-details)
    pub async fn get_plan(&self, name: &str) -> Result<Plan, Error> {
        self.client.get(&format!("/1.3/gateway/plans/{name}")).await
    }

    // ─── Gateways ─────────────────────────────────────────────────────────────

    /// Returns a list of all gateways.
    ///
    /// `GET /1.3/gateway`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/19-network-gateways/#list-services)
    pub async fn list(&self, params: PageParams) -> Result<Vec<Gateway>, Error> {
        self.client.get_with_query("/1.3/gateway", &params).await
    }

    /// Returns details for a specific gateway.
    ///
    /// `GET /1.3/gateway/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/19-network-gateways/#get-service-details)
    pub async fn get(&self, uuid: &str) -> Result<Gateway, Error> {
        self.client.get(&format!("/1.3/gateway/{uuid}")).await
    }

    /// Creates a new gateway.
    ///
    /// `POST /1.3/gateway`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/19-network-gateways/#create-service)
    pub async fn create(&self, req: CreateRequest) -> Result<Gateway, Error> {
        self.client.post("/1.3/gateway", &req).await
    }

    /// Modifies an existing gateway.
    ///
    /// `PATCH /1.3/gateway/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/19-network-gateways/#modify-service)
    pub async fn modify(&self, uuid: &str, req: ModifyRequest) -> Result<Gateway, Error> {
        self.client
            .patch(&format!("/1.3/gateway/{uuid}"), &req)
            .await
    }

    /// Deletes a gateway.
    ///
    /// `DELETE /1.3/gateway/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/19-network-gateways/#delete-service)
    pub async fn delete(&self, uuid: &str) -> Result<(), Error> {
        self.client.delete(&format!("/1.3/gateway/{uuid}")).await
    }

    // ─── Connections ──────────────────────────────────────────────────────────

    /// Returns a list of connections for a gateway.
    ///
    /// `GET /1.3/gateway/{gateway_uuid}/connections`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/19-network-gateways/#list-connections)
    pub async fn list_connections(&self, gateway_uuid: &str) -> Result<Vec<Connection>, Error> {
        self.client
            .get(&format!("/1.3/gateway/{gateway_uuid}/connections"))
            .await
    }

    /// Returns details for a specific connection.
    ///
    /// `GET /1.3/gateway/{gateway_uuid}/connections/{name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/19-network-gateways/#get-connection-details)
    pub async fn get_connection(
        &self,
        gateway_uuid: &str,
        name: &str,
    ) -> Result<Connection, Error> {
        self.client
            .get(&format!("/1.3/gateway/{gateway_uuid}/connections/{name}"))
            .await
    }

    /// Creates a connection for a gateway.
    ///
    /// `POST /1.3/gateway/{gateway_uuid}/connections`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/19-network-gateways/#create-connection)
    pub async fn create_connection(
        &self,
        gateway_uuid: &str,
        req: CreateConnectionRequest,
    ) -> Result<Connection, Error> {
        self.client
            .post(&format!("/1.3/gateway/{gateway_uuid}/connections"), &req)
            .await
    }

    /// Deletes a connection.
    ///
    /// `DELETE /1.3/gateway/{gateway_uuid}/connections/{name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/19-network-gateways/#delete-connection)
    pub async fn delete_connection(&self, gateway_uuid: &str, name: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/gateway/{gateway_uuid}/connections/{name}"))
            .await
    }

    /// Modifies an existing connection.
    ///
    /// `PATCH /1.3/gateway/{gateway_uuid}/connections/{connection_name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/19-network-gateways/#modify-connection)
    pub async fn modify_connection(
        &self,
        gateway_uuid: &str,
        connection_name: &str,
        req: ModifyConnectionRequest,
    ) -> Result<Connection, Error> {
        self.client
            .patch(
                &format!("/1.3/gateway/{gateway_uuid}/connections/{connection_name}"),
                &req,
            )
            .await
    }

    // ─── Tunnels ──────────────────────────────────────────────────────────────

    /// Returns a list of tunnels for a connection.
    ///
    /// `GET /1.3/gateway/{gateway_uuid}/connections/{connection_name}/tunnels`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/19-network-gateways/#list-tunnels)
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
    ///
    /// `POST /1.3/gateway/{gateway_uuid}/connections/{connection_name}/tunnels`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/19-network-gateways/#create-tunnel)
    pub async fn create_tunnel(
        &self,
        gateway_uuid: &str,
        connection_name: &str,
        req: CreateTunnelRequest,
    ) -> Result<Tunnel, Error> {
        self.client
            .post(
                &format!("/1.3/gateway/{gateway_uuid}/connections/{connection_name}/tunnels"),
                &req,
            )
            .await
    }

    /// Deletes a tunnel.
    ///
    /// `DELETE /1.3/gateway/{gateway_uuid}/connections/{connection_name}/tunnels/{tunnel_name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/19-network-gateways/#delete-tunnel)
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

    /// Returns details for a specific tunnel.
    ///
    /// `GET /1.3/gateway/{gateway_uuid}/connections/{connection_name}/tunnels/{tunnel_name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/19-network-gateways/#get-tunnel-details)
    pub async fn get_tunnel(
        &self,
        gateway_uuid: &str,
        connection_name: &str,
        tunnel_name: &str,
    ) -> Result<Tunnel, Error> {
        self.client
            .get(&format!(
                "/1.3/gateway/{gateway_uuid}/connections/{connection_name}/tunnels/{tunnel_name}"
            ))
            .await
    }

    /// Modifies an existing tunnel.
    ///
    /// `PATCH /1.3/gateway/{gateway_uuid}/connections/{connection_name}/tunnels/{tunnel_name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/19-network-gateways/#modify-tunnel)
    pub async fn modify_tunnel(
        &self,
        gateway_uuid: &str,
        connection_name: &str,
        tunnel_name: &str,
        req: ModifyTunnelRequest,
    ) -> Result<Tunnel, Error> {
        self.client
            .patch(
                &format!(
                    "/1.3/gateway/{gateway_uuid}/connections/{connection_name}/tunnels/{tunnel_name}"
                ),
                &req,
            )
            .await
    }

    // ─── Routes ───────────────────────────────────────────────────────────────

    /// Returns a list of routes for a gateway.
    ///
    /// `GET /1.3/gateway/{gateway_uuid}/routes`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/19-network-gateways/)
    pub async fn list_routes(&self, gateway_uuid: &str) -> Result<Vec<Route>, Error> {
        self.client
            .get(&format!("/1.3/gateway/{gateway_uuid}/routes"))
            .await
    }

    /// Creates a route for a gateway.
    ///
    /// `POST /1.3/gateway/{gateway_uuid}/routes`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/19-network-gateways/)
    pub async fn create_route(&self, gateway_uuid: &str, req: RouteInput) -> Result<Route, Error> {
        self.client
            .post(&format!("/1.3/gateway/{gateway_uuid}/routes"), &req)
            .await
    }

    /// Deletes a route from a gateway.
    ///
    /// `DELETE /1.3/gateway/{gateway_uuid}/routes/{route_name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/19-network-gateways/)
    pub async fn delete_route(&self, gateway_uuid: &str, route_name: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/gateway/{gateway_uuid}/routes/{route_name}"))
            .await
    }

    // ─── Labels ───────────────────────────────────────────────────────────────

    /// Returns a list of labels for a gateway.
    ///
    /// `GET /1.3/gateway/{gateway_uuid}/labels`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/19-network-gateways/)
    pub async fn list_labels(
        &self,
        gateway_uuid: &str,
    ) -> Result<Vec<crate::types::Label>, Error> {
        self.client
            .get(&format!("/1.3/gateway/{gateway_uuid}/labels"))
            .await
    }

    /// Creates a label on a gateway.
    ///
    /// `POST /1.3/gateway/{gateway_uuid}/labels`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/19-network-gateways/)
    pub async fn create_label(
        &self,
        gateway_uuid: &str,
        label: crate::types::Label,
    ) -> Result<crate::types::Label, Error> {
        self.client
            .post(&format!("/1.3/gateway/{gateway_uuid}/labels"), &label)
            .await
    }

    /// Modifies a label on a gateway.
    ///
    /// `PATCH /1.3/gateway/{gateway_uuid}/labels/{key}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/19-network-gateways/)
    pub async fn modify_label(
        &self,
        gateway_uuid: &str,
        key: &str,
        label: crate::types::Label,
    ) -> Result<crate::types::Label, Error> {
        self.client
            .patch(&format!("/1.3/gateway/{gateway_uuid}/labels/{key}"), &label)
            .await
    }

    /// Deletes a label from a gateway.
    ///
    /// `DELETE /1.3/gateway/{gateway_uuid}/labels/{key}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/19-network-gateways/)
    pub async fn delete_label(&self, gateway_uuid: &str, key: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/gateway/{gateway_uuid}/labels/{key}"))
            .await
    }

    // ─── Polling helpers ──────────────────────────────────────────────────────

    /// Polls until a gateway reaches the desired operational state or the timeout expires.
    pub async fn wait_for_state(
        &self,
        uuid: &str,
        desired_state: OperationalState,
        timeout: Duration,
    ) -> Result<Gateway, Error> {
        let start = Instant::now();
        loop {
            let gw = self.get(uuid).await?;
            if gw.operational_state == Some(desired_state) {
                return Ok(gw);
            }
            if start.elapsed() >= timeout {
                return Err(Error::InvalidConfig(format!(
                    "timeout waiting for gateway {} to reach state {:?}",
                    uuid, desired_state
                )));
            }
            sleep(Duration::from_secs(5)).await;
        }
    }
}

#[cfg(test)]
mod tests;
