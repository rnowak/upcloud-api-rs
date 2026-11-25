//! Networks, Routers, and Network Peerings.
//!
//! All documented on the Networks page (13-networks).

mod types;

pub use types::*;

use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::Error;

/// Handler for network, router, and peering operations.
pub struct NetworkHandler<'a> {
    client: &'a Client,
}

impl<'a> NetworkHandler<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    // ─── Networks ─────────────────────────────────────────────────────────────

    /// Returns a list of all networks.
    ///
    /// `GET /1.3/network`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/13-networks/#list-all-networks)
    pub async fn list(&self) -> Result<Vec<Network>, Error> {
        let resp: NetworkListResponse = self.client.get("/1.3/network").await?;
        Ok(resp.networks.network)
    }

    /// Returns a list of networks in a specific zone.
    ///
    /// `GET /1.3/network?zone={zone}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/13-networks/#list-networks-within-zone)
    pub async fn list_in_zone(&self, zone: &str) -> Result<Vec<Network>, Error> {
        let resp: NetworkListResponse = self
            .client
            .get_with_query("/1.3/network", &[("zone", zone)])
            .await?;
        Ok(resp.networks.network)
    }

    /// Returns details for a specific network.
    ///
    /// `GET /1.3/network/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/13-networks/#get-network-details)
    pub async fn get(&self, uuid: &str) -> Result<Network, Error> {
        let resp: NetworkResponse = self.client.get(&format!("/1.3/network/{uuid}")).await?;
        Ok(resp.network)
    }

    /// Creates a new private network.
    ///
    /// `POST /1.3/network`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/13-networks/#create-sdn-private-network)
    pub async fn create(&self, req: CreateNetworkRequest) -> Result<Network, Error> {
        #[derive(Serialize)]
        struct Body {
            network: CreateNetworkRequest,
        }
        let resp: NetworkResponse = self
            .client
            .post("/1.3/network", &Body { network: req })
            .await?;
        Ok(resp.network)
    }

    /// Modifies an existing network.
    ///
    /// `PUT /1.3/network/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/13-networks/#modify-network-details)
    pub async fn modify(&self, uuid: &str, req: ModifyNetworkRequest) -> Result<Network, Error> {
        #[derive(Serialize)]
        struct Body {
            network: ModifyNetworkRequest,
        }
        let resp: NetworkResponse = self
            .client
            .put(&format!("/1.3/network/{uuid}"), &Body { network: req })
            .await?;
        Ok(resp.network)
    }

    /// Deletes a network.
    ///
    /// `DELETE /1.3/network/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/13-networks/#delete-network)
    pub async fn delete(&self, uuid: &str) -> Result<(), Error> {
        self.client.delete(&format!("/1.3/network/{uuid}")).await
    }

    /// Attaches a router to a network.
    ///
    /// `PUT /1.3/network/{network_uuid}` with `{"network": {"router": "<router_uuid>"}}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/13-networks/#modify-network-details)
    pub async fn attach_router(
        &self,
        network_uuid: &str,
        router_uuid: &str,
    ) -> Result<Network, Error> {
        #[derive(Serialize)]
        struct Inner<'a> {
            router: &'a str,
        }
        #[derive(Serialize)]
        struct Body<'a> {
            network: Inner<'a>,
        }
        let resp: NetworkResponse = self
            .client
            .put(
                &format!("/1.3/network/{network_uuid}"),
                &Body {
                    network: Inner {
                        router: router_uuid,
                    },
                },
            )
            .await?;
        Ok(resp.network)
    }

    /// Detaches the router from a network.
    ///
    /// `PUT /1.3/network/{network_uuid}` with `{"network": {"router": null}}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/13-networks/#modify-network-details)
    pub async fn detach_router(&self, network_uuid: &str) -> Result<Network, Error> {
        let body = serde_json::json!({"network": {"router": null}});
        let resp: NetworkResponse = self
            .client
            .put(&format!("/1.3/network/{network_uuid}"), &body)
            .await?;
        Ok(resp.network)
    }

    // ─── Server interfaces ────────────────────────────────────────────────────

    /// Returns the networking configuration for a server.
    ///
    /// `GET /1.3/server/{server_uuid}/networking`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/13-networks/#list-server-networks)
    pub async fn get_server_networking(&self, server_uuid: &str) -> Result<Network, Error> {
        let resp: NetworkResponse = self
            .client
            .get(&format!("/1.3/server/{server_uuid}/networking"))
            .await?;
        Ok(resp.network)
    }

    /// Creates a network interface on a server.
    ///
    /// `POST /1.3/server/{server_uuid}/networking/interface`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/13-networks/#list-server-networks)
    pub async fn create_interface(
        &self,
        server_uuid: &str,
        req: CreateInterfaceRequest,
    ) -> Result<Network, Error> {
        #[derive(Serialize)]
        struct Body {
            interface: CreateInterfaceRequest,
        }
        let resp: NetworkResponse = self
            .client
            .post(
                &format!("/1.3/server/{server_uuid}/networking/interface"),
                &Body { interface: req },
            )
            .await?;
        Ok(resp.network)
    }

    /// Modifies a network interface on a server.
    ///
    /// `PUT /1.3/server/{server_uuid}/networking/interface/{index}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/13-networks/#list-server-networks)
    pub async fn modify_interface(
        &self,
        server_uuid: &str,
        index: u32,
        req: ModifyInterfaceRequest,
    ) -> Result<Network, Error> {
        #[derive(Serialize)]
        struct Body {
            interface: ModifyInterfaceRequest,
        }
        let resp: NetworkResponse = self
            .client
            .put(
                &format!("/1.3/server/{server_uuid}/networking/interface/{index}"),
                &Body { interface: req },
            )
            .await?;
        Ok(resp.network)
    }

    /// Deletes a network interface from a server.
    ///
    /// `DELETE /1.3/server/{server_uuid}/networking/interface/{index}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/13-networks/#list-server-networks)
    pub async fn delete_interface(&self, server_uuid: &str, index: u32) -> Result<(), Error> {
        self.client
            .delete(&format!(
                "/1.3/server/{server_uuid}/networking/interface/{index}"
            ))
            .await
    }

    /// Assigns an IP address to a network interface.
    ///
    /// `POST /1.3/server/{server_uuid}/networking/interface/{index}/ip-address`
    ///
    /// When `force` is `true`, appends `?force=yes` to the request URL.
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/13-networks/#list-server-networks)
    pub async fn assign_interface_ip(
        &self,
        server_uuid: &str,
        index: u32,
        req: AssignInterfaceIpRequest,
        force: bool,
    ) -> Result<(), Error> {
        #[derive(Serialize)]
        struct Body {
            ip_address: AssignInterfaceIpRequest,
        }
        let mut url = format!("/1.3/server/{server_uuid}/networking/interface/{index}/ip-address");
        if force {
            url.push_str("?force=yes");
        }
        self.client
            .post_no_response(&url, &Body { ip_address: req })
            .await
    }

    /// Removes an IP address from a network interface.
    ///
    /// `DELETE /1.3/server/{server_uuid}/networking/interface/{index}/ip-address/{address}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/13-networks/#list-server-networks)
    pub async fn delete_interface_ip(
        &self,
        server_uuid: &str,
        index: u32,
        address: &str,
    ) -> Result<(), Error> {
        self.client
            .delete(&format!(
                "/1.3/server/{server_uuid}/networking/interface/{index}/ip-address/{address}"
            ))
            .await
    }

    // ─── Routers ──────────────────────────────────────────────────────────────

    /// Returns a list of all routers.
    ///
    /// `GET /1.3/router`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/13-networks/#list-routers)
    pub async fn list_routers(&self) -> Result<Vec<Router>, Error> {
        let resp: RouterListResponse = self.client.get("/1.3/router").await?;
        Ok(resp.routers.router)
    }

    /// Returns details for a specific router.
    ///
    /// `GET /1.3/router/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/13-networks/#get-router-details)
    pub async fn get_router(&self, uuid: &str) -> Result<Router, Error> {
        let resp: RouterResponse = self.client.get(&format!("/1.3/router/{uuid}")).await?;
        Ok(resp.router)
    }

    /// Creates a new router.
    ///
    /// `POST /1.3/router`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/13-networks/#create-router)
    pub async fn create_router(&self, req: CreateRouterRequest) -> Result<Router, Error> {
        #[derive(Serialize)]
        struct Body {
            router: CreateRouterRequest,
        }
        let resp: RouterResponse = self
            .client
            .post("/1.3/router", &Body { router: req })
            .await?;
        Ok(resp.router)
    }

    /// Modifies an existing router.
    ///
    /// `PATCH /1.3/router/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/13-networks/#modify-router)
    pub async fn modify_router(
        &self,
        uuid: &str,
        req: ModifyRouterRequest,
    ) -> Result<Router, Error> {
        #[derive(Serialize)]
        struct Body {
            router: ModifyRouterRequest,
        }
        let resp: RouterResponse = self
            .client
            .patch(&format!("/1.3/router/{uuid}"), &Body { router: req })
            .await?;
        Ok(resp.router)
    }

    /// Deletes a router.
    ///
    /// `DELETE /1.3/router/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/13-networks/#delete-router)
    pub async fn delete_router(&self, uuid: &str) -> Result<(), Error> {
        self.client.delete(&format!("/1.3/router/{uuid}")).await
    }

    // ─── Network Peerings ─────────────────────────────────────────────────────

    /// Returns a list of all network peerings.
    ///
    /// `GET /1.3/network-peering`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/13-networks/#list-network-peerings)
    pub async fn list_peerings(&self) -> Result<Vec<NetworkPeering>, Error> {
        let resp: PeeringListResponse = self.client.get("/1.3/network-peering").await?;
        Ok(resp.network_peerings)
    }

    /// Returns details for a specific network peering.
    ///
    /// `GET /1.3/network-peering/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/13-networks/#get-network-peering-details)
    pub async fn get_peering(&self, uuid: &str) -> Result<NetworkPeering, Error> {
        let resp: PeeringResponse = self
            .client
            .get(&format!("/1.3/network-peering/{uuid}"))
            .await?;
        Ok(resp.network_peering)
    }

    /// Creates a new network peering.
    ///
    /// `POST /1.3/network-peering`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/13-networks/#create-network-peering)
    pub async fn create_peering(&self, req: CreatePeeringRequest) -> Result<NetworkPeering, Error> {
        #[derive(Serialize)]
        struct Body {
            network_peering: CreatePeeringRequest,
        }
        let resp: PeeringResponse = self
            .client
            .post(
                "/1.3/network-peering",
                &Body {
                    network_peering: req,
                },
            )
            .await?;
        Ok(resp.network_peering)
    }

    /// Modifies an existing network peering.
    ///
    /// `PATCH /1.3/network-peering/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/13-networks/#modify-network-peering)
    pub async fn modify_peering(
        &self,
        uuid: &str,
        req: ModifyPeeringRequest,
    ) -> Result<NetworkPeering, Error> {
        #[derive(Serialize)]
        struct Body {
            network_peering: ModifyPeeringRequest,
        }
        let resp: PeeringResponse = self
            .client
            .patch(
                &format!("/1.3/network-peering/{uuid}"),
                &Body {
                    network_peering: req,
                },
            )
            .await?;
        Ok(resp.network_peering)
    }

    /// Deletes a network peering.
    ///
    /// `DELETE /1.3/network-peering/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/13-networks/#delete-network-peering)
    pub async fn delete_peering(&self, uuid: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/network-peering/{uuid}"))
            .await
    }
}

// ─── Response wrappers (crate-private) ────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct NetworkListResponse {
    networks: NetworkWrapper,
}

#[derive(Debug, Deserialize)]
struct NetworkWrapper {
    network: Vec<Network>,
}

#[derive(Debug, Deserialize)]
struct NetworkResponse {
    network: Network,
}

#[derive(Debug, Deserialize)]
struct RouterListResponse {
    routers: RouterWrapper,
}

#[derive(Debug, Deserialize)]
struct RouterWrapper {
    router: Vec<Router>,
}

#[derive(Debug, Deserialize)]
struct RouterResponse {
    router: Router,
}

#[derive(Debug, Deserialize)]
struct PeeringListResponse {
    network_peerings: Vec<NetworkPeering>,
}

#[derive(Debug, Deserialize)]
struct PeeringResponse {
    network_peering: NetworkPeering,
}

#[cfg(test)]
mod tests;
