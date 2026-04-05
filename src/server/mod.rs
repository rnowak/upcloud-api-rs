//! Servers and Server Groups.
//!
//! Servers: `GET/POST/PUT/DELETE /1.3/server` + actions.
//! Server Groups: `GET/POST/PUT/DELETE /1.3/server-group`.

mod types;
pub mod firewall;

pub use types::*;

use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use tokio::time::sleep;

use crate::client::Client;
use crate::error::Error;
use self::firewall::{Rule, CreateRuleRequest};

/// Handler for server operations.
pub struct ServerHandler<'a> {
    client: &'a Client,
}

impl<'a> ServerHandler<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    // ─── Servers ──────────────────────────────────────────────────────────────

    /// Returns a list of all servers, optionally filtered, ordered, and paginated.
    ///
    /// Pass `ListParams::default()` to list all servers without filtering.
    ///
    /// `GET /1.3/server`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/8-servers/#list-servers)
    pub async fn list(&self, params: &ListParams) -> Result<Vec<Server>, Error> {
        let pairs = params.to_query_pairs();
        let resp: ServerListResponse = if pairs.is_empty() {
            self.client.get("/1.3/server").await?
        } else {
            self.client
                .get_with_query_pairs("/1.3/server", &pairs)
                .await?
        };
        Ok(resp.servers.server)
    }

    /// Returns details for a specific server.
    ///
    /// `GET /1.3/server/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/8-servers/#get-server-details)
    pub async fn get(&self, uuid: &str) -> Result<ServerDetails, Error> {
        let resp: ServerResponse = self.client.get(&format!("/1.3/server/{uuid}")).await?;
        Ok(resp.server)
    }

    /// Creates a new server.
    ///
    /// `POST /1.3/server`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/8-servers/#create-server)
    pub async fn create(&self, req: CreateRequest) -> Result<ServerDetails, Error> {
        #[derive(Serialize)]
        struct Body {
            server: CreateRequest,
        }
        let resp: ServerResponse = self
            .client
            .post("/1.3/server", &Body { server: req })
            .await?;
        Ok(resp.server)
    }

    /// Modifies an existing server.
    ///
    /// `PUT /1.3/server/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/8-servers/#modify-server)
    pub async fn modify(
        &self,
        uuid: &str,
        req: ModifyRequest,
    ) -> Result<ServerDetails, Error> {
        #[derive(Serialize)]
        struct Body {
            server: ModifyRequest,
        }
        let resp: ServerResponse = self
            .client
            .put(&format!("/1.3/server/{uuid}"), &Body { server: req })
            .await?;
        Ok(resp.server)
    }

    /// Deletes a server.
    ///
    /// `DELETE /1.3/server/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/8-servers/#delete-server)
    pub async fn delete(&self, uuid: &str) -> Result<(), Error> {
        self.client.delete(&format!("/1.3/server/{uuid}")).await
    }

    /// Deletes a server and its attached storage devices.
    ///
    /// `DELETE /1.3/server/{uuid}?storages=1`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/8-servers/#delete-server)
    pub async fn delete_with_storages(
        &self,
        uuid: &str,
        backups: DeleteStorageBackupsMode,
    ) -> Result<(), Error> {
        let req = self
            .client
            .request(
                reqwest::Method::DELETE,
                &format!("/1.3/server/{uuid}"),
            )
            .query(&[("storages", "1"), ("backups", &backups.to_string())]);
        self.client.send_empty(req).await
    }

    // ─── Actions ──────────────────────────────────────────────────────────────

    /// Starts a stopped server.
    ///
    /// `POST /1.3/server/{uuid}/start`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/8-servers/#start-server)
    pub async fn start(&self, uuid: &str) -> Result<ServerDetails, Error> {
        let resp: ServerResponse = self
            .client
            .post_empty(&format!("/1.3/server/{uuid}/start"))
            .await?;
        Ok(resp.server)
    }

    /// Relocates a server to a different zone.
    ///
    /// `POST /1.3/server/{uuid}/relocate`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/8-servers/#relocate-server)
    pub async fn relocate(
        &self,
        uuid: &str,
        req: &RelocateRequest,
    ) -> Result<ServerDetails, Error> {
        let resp: ServerResponse = self
            .client
            .post(&format!("/1.3/server/{uuid}/relocate"), req)
            .await?;
        Ok(resp.server)
    }

    /// Stops a running server.
    ///
    /// `POST /1.3/server/{uuid}/stop`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/8-servers/#stop-server)
    pub async fn stop(&self, uuid: &str, req: StopRequest) -> Result<ServerDetails, Error> {
        #[derive(Serialize)]
        struct Body {
            stop_server: StopRequest,
        }
        let resp: ServerResponse = self
            .client
            .post(
                &format!("/1.3/server/{uuid}/stop"),
                &Body { stop_server: req },
            )
            .await?;
        Ok(resp.server)
    }

    /// Restarts a running server.
    ///
    /// `POST /1.3/server/{uuid}/restart`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/8-servers/#restart-server)
    pub async fn restart(
        &self,
        uuid: &str,
        req: RestartRequest,
    ) -> Result<ServerDetails, Error> {
        #[derive(Serialize)]
        struct Body {
            restart_server: RestartRequest,
        }
        let resp: ServerResponse = self
            .client
            .post(
                &format!("/1.3/server/{uuid}/restart"),
                &Body {
                    restart_server: req,
                },
            )
            .await?;
        Ok(resp.server)
    }

    /// Cancels an ongoing operation on a server.
    ///
    /// `POST /1.3/server/{uuid}/cancel`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/8-servers/#cancel-server-operation)
    pub async fn cancel_operation(&self, uuid: &str) -> Result<(), Error> {
        self.client
            .post_no_response_empty(&format!("/1.3/server/{uuid}/cancel"))
            .await
    }

    /// Rebuilds a server from a template (re-installs the OS).
    ///
    /// `POST /1.3/server/{uuid}/rebuild`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/8-servers/#rebuild-server)
    pub async fn rebuild(
        &self,
        uuid: &str,
        req: RebuildRequest,
    ) -> Result<ServerDetails, Error> {
        #[derive(Serialize)]
        struct Body {
            server_rebuild: RebuildRequest,
        }
        let resp: ServerResponse = self
            .client
            .post(
                &format!("/1.3/server/{uuid}/rebuild"),
                &Body {
                    server_rebuild: req,
                },
            )
            .await?;
        Ok(resp.server)
    }

    // ─── Storage devices ──────────────────────────────────────────────────────

    /// Attaches a storage device to a server.
    ///
    /// `POST /1.3/server/{uuid}/storage/attach`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/9-storages/#attach-storage)
    pub async fn attach_storage(
        &self,
        server_uuid: &str,
        req: AttachStorageRequest,
    ) -> Result<ServerDetails, Error> {
        #[derive(Serialize)]
        struct Body {
            storage_device: AttachStorageRequest,
        }
        let resp: ServerResponse = self
            .client
            .post(
                &format!("/1.3/server/{server_uuid}/storage/attach"),
                &Body {
                    storage_device: req,
                },
            )
            .await?;
        Ok(resp.server)
    }

    /// Detaches a storage device from a server.
    ///
    /// `POST /1.3/server/{uuid}/storage/detach`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/9-storages/#detach-storage)
    pub async fn detach_storage(
        &self,
        server_uuid: &str,
        req: DetachStorageRequest,
    ) -> Result<ServerDetails, Error> {
        #[derive(Serialize)]
        struct Body {
            storage_device: DetachStorageRequest,
        }
        let resp: ServerResponse = self
            .client
            .post(
                &format!("/1.3/server/{server_uuid}/storage/detach"),
                &Body {
                    storage_device: req,
                },
            )
            .await?;
        Ok(resp.server)
    }

    /// Loads a CDROM into the server's virtual optical drive.
    ///
    /// `POST /1.3/server/{uuid}/cdrom/load`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/9-storages/#load-cd-rom)
    pub async fn load_cdrom(
        &self,
        server_uuid: &str,
        req: LoadCdromRequest,
    ) -> Result<ServerDetails, Error> {
        #[derive(Serialize)]
        struct Body {
            storage_device: LoadCdromRequest,
        }
        let resp: ServerResponse = self
            .client
            .post(
                &format!("/1.3/server/{server_uuid}/cdrom/load"),
                &Body {
                    storage_device: req,
                },
            )
            .await?;
        Ok(resp.server)
    }

    /// Ejects the CDROM from the server's virtual optical drive.
    ///
    /// `POST /1.3/server/{uuid}/cdrom/eject`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/9-storages/#eject-cd-rom)
    pub async fn eject_cdrom(&self, server_uuid: &str) -> Result<ServerDetails, Error> {
        let resp: ServerResponse = self
            .client
            .post_empty(&format!("/1.3/server/{server_uuid}/cdrom/eject"))
            .await?;
        Ok(resp.server)
    }

    // ─── GPUs ─────────────────────────────────────────────────────────────────

    /// Returns GPU information for a server.
    ///
    /// `GET /1.3/server/{uuid}/gpus`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/8-servers/#get-server-gpus)
    pub async fn list_gpus(&self, server_uuid: &str) -> Result<Vec<GpuInfo>, Error> {
        self.client
            .get(&format!("/1.3/server/{server_uuid}/gpus"))
            .await
    }

    // ─── Firewall rules ───────────────────────────────────────────────────────

    /// Returns all firewall rules for a server.
    ///
    /// `GET /1.3/server/{uuid}/firewall_rule`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/11-firewall/#list-firewall-rules)
    pub async fn list_firewall_rules(
        &self,
        server_uuid: &str,
    ) -> Result<Vec<Rule>, Error> {
        let resp: FirewallRuleListResponse = self
            .client
            .get(&format!("/1.3/server/{server_uuid}/firewall_rule"))
            .await?;
        Ok(resp.firewall_rules.firewall_rule)
    }

    /// Creates a firewall rule for a server.
    ///
    /// `POST /1.3/server/{uuid}/firewall_rule`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/11-firewall/#create-firewall-rule)
    pub async fn create_firewall_rule(
        &self,
        server_uuid: &str,
        req: CreateRuleRequest,
    ) -> Result<Rule, Error> {
        #[derive(Serialize)]
        struct Body {
            firewall_rule: CreateRuleRequest,
        }
        let resp: FirewallRuleResponse = self
            .client
            .post(
                &format!("/1.3/server/{server_uuid}/firewall_rule"),
                &Body { firewall_rule: req },
            )
            .await?;
        Ok(resp.firewall_rule)
    }

    /// Returns details for a specific firewall rule.
    ///
    /// `GET /1.3/server/{uuid}/firewall_rule/{position}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/11-firewall/#get-firewall-rule-details)
    pub async fn get_firewall_rule(
        &self,
        server_uuid: &str,
        position: u32,
    ) -> Result<Rule, Error> {
        let resp: FirewallRuleResponse = self
            .client
            .get(&format!(
                "/1.3/server/{server_uuid}/firewall_rule/{position}"
            ))
            .await?;
        Ok(resp.firewall_rule)
    }

    /// Deletes a firewall rule from a server.
    ///
    /// `DELETE /1.3/server/{uuid}/firewall_rule/{position}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/11-firewall/#remove-firewall-rule)
    pub async fn delete_firewall_rule(
        &self,
        server_uuid: &str,
        position: u32,
    ) -> Result<(), Error> {
        self.client
            .delete(&format!(
                "/1.3/server/{server_uuid}/firewall_rule/{position}"
            ))
            .await
    }

    /// Replaces all firewall rules for a server.
    ///
    /// `PUT /1.3/server/{uuid}/firewall_rule`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/11-firewall/#create-multiple-firewall-rules)
    pub async fn create_firewall_rules(
        &self,
        server_uuid: &str,
        rules: &[CreateRuleRequest],
    ) -> Result<Vec<Rule>, Error> {
        #[derive(Serialize)]
        struct Inner<'a> {
            firewall_rule: &'a [CreateRuleRequest],
        }
        #[derive(Serialize)]
        struct Body<'a> {
            firewall_rules: Inner<'a>,
        }
        let resp: FirewallRuleListResponse = self
            .client
            .put(
                &format!("/1.3/server/{server_uuid}/firewall_rule"),
                &Body {
                    firewall_rules: Inner {
                        firewall_rule: rules,
                    },
                },
            )
            .await?;
        Ok(resp.firewall_rules.firewall_rule)
    }

    // ─── Server Groups ────────────────────────────────────────────────────────

    /// Returns a list of all server groups.
    ///
    /// `GET /1.3/server-group`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/8-servers/#list-all-server-groups)
    pub async fn list_groups(&self) -> Result<Vec<Group>, Error> {
        let resp: ServerGroupListResponse = self.client.get("/1.3/server-group").await?;
        Ok(resp.server_groups.server_group)
    }

    /// Returns details for a specific server group.
    ///
    /// `GET /1.3/server-group/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/8-servers/#show-server-group-details)
    pub async fn get_group(&self, uuid: &str) -> Result<Group, Error> {
        let resp: ServerGroupResponse = self
            .client
            .get(&format!("/1.3/server-group/{uuid}"))
            .await?;
        Ok(resp.server_group)
    }

    /// Creates a new server group.
    ///
    /// `POST /1.3/server-group`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/8-servers/#create-a-new-server-group)
    pub async fn create_group(
        &self,
        req: CreateGroupRequest,
    ) -> Result<Group, Error> {
        #[derive(Serialize)]
        struct Body {
            server_group: CreateGroupRequest,
        }
        let resp: ServerGroupResponse = self
            .client
            .post("/1.3/server-group", &Body { server_group: req })
            .await?;
        Ok(resp.server_group)
    }

    /// Modifies an existing server group.
    ///
    /// `PUT /1.3/server-group/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/8-servers/#modify-server-group)
    pub async fn modify_group(
        &self,
        uuid: &str,
        req: ModifyGroupRequest,
    ) -> Result<Group, Error> {
        #[derive(Serialize)]
        struct Body {
            server_group: ModifyGroupRequest,
        }
        let resp: ServerGroupResponse = self
            .client
            .put(
                &format!("/1.3/server-group/{uuid}"),
                &Body { server_group: req },
            )
            .await?;
        Ok(resp.server_group)
    }

    /// Deletes a server group.
    ///
    /// `DELETE /1.3/server-group/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/8-servers/#delete-server-group)
    pub async fn delete_group(&self, uuid: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/server-group/{uuid}"))
            .await
    }

    // ─── Polling helpers ──────────────────────────────────────────────────────

    /// Polls until a server reaches the desired state or the timeout expires.
    pub async fn wait_for_state(
        &self,
        uuid: &str,
        desired_state: State,
        timeout: Duration,
    ) -> Result<ServerDetails, Error> {
        let start = Instant::now();
        loop {
            let details = self.get(uuid).await?;
            if details.server.state == desired_state {
                return Ok(details);
            }
            if start.elapsed() >= timeout {
                return Err(Error::InvalidConfig(format!(
                    "timeout waiting for server {} to reach state {:?}",
                    uuid, desired_state
                )));
            }
            sleep(Duration::from_secs(5)).await;
        }
    }
}

// ─── Response wrappers (crate-private) ────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct ServerListResponse {
    servers: ServerListWrapper,
}

#[derive(Debug, Deserialize)]
struct ServerListWrapper {
    server: Vec<Server>,
}

#[derive(Debug, Deserialize)]
struct ServerResponse {
    server: ServerDetails,
}

#[derive(Debug, Deserialize)]
struct FirewallRuleListResponse {
    firewall_rules: FirewallRuleWrapper,
}

#[derive(Debug, Deserialize)]
struct FirewallRuleWrapper {
    firewall_rule: Vec<Rule>,
}

#[derive(Debug, Deserialize)]
struct FirewallRuleResponse {
    firewall_rule: Rule,
}

#[derive(Debug, Deserialize)]
struct ServerGroupListResponse {
    server_groups: ServerGroupListWrapper,
}

#[derive(Debug, Deserialize)]
struct ServerGroupListWrapper {
    server_group: Vec<Group>,
}

#[derive(Debug, Deserialize)]
struct ServerGroupResponse {
    server_group: Group,
}

#[cfg(test)]
mod tests;
