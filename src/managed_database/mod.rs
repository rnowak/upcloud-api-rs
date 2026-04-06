//! Managed Databases — `GET/POST/PATCH/DELETE /1.3/database` + sub-resources.

mod types;
pub use types::*;

use crate::client::Client;
use crate::error::Error;

// ─── Handler ──────────────────────────────────────────────────────────────────

/// Handler for managed database operations.
pub struct DatabaseHandler<'a> {
    client: &'a Client,
}

impl<'a> DatabaseHandler<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    // ─── Service types ─────────────────────────────────────────────────────────

    /// Returns a list of available managed database service types.
    ///
    /// `GET /1.3/database/service-types`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#get-managed-database-service-types)
    pub async fn list_types(&self) -> Result<Vec<DatabaseType>, Error> {
        self.client.get("/1.3/database/service-types").await
    }

    /// Returns available PostgreSQL extensions (global list).
    ///
    /// `GET /1.3/database/pg/available-extensions`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#get-managed-database-service-types)
    pub async fn list_pg_extensions(&self) -> Result<Vec<Extension>, Error> {
        self.client
            .get("/1.3/database/pg/available-extensions")
            .await
    }

    /// Returns a list of available database plans.
    ///
    /// `GET /1.3/database/plans`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#get-managed-database-plans)
    pub async fn list_plans(&self) -> Result<Vec<ServicePlan>, Error> {
        self.client.get("/1.3/database/plans").await
    }

    // ─── Services ──────────────────────────────────────────────────────────────

    /// Returns a list of all managed database services.
    ///
    /// `GET /1.3/database`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#list-managed-databases)
    pub async fn list(&self) -> Result<Vec<Database>, Error> {
        self.client.get("/1.3/database").await
    }

    /// Returns details for a specific managed database.
    ///
    /// `GET /1.3/database/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#get-managed-database-details)
    pub async fn get(&self, uuid: &str) -> Result<Database, Error> {
        self.client.get(&format!("/1.3/database/{uuid}")).await
    }

    /// Creates a new managed database.
    ///
    /// `POST /1.3/database`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#create-managed-database)
    pub async fn create(&self, req: CreateRequest) -> Result<Database, Error> {
        self.client.post("/1.3/database", &req).await
    }

    /// Modifies an existing managed database.
    ///
    /// `PATCH /1.3/database/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#modify-managed-database)
    pub async fn modify(&self, uuid: &str, req: ModifyRequest) -> Result<Database, Error> {
        self.client
            .patch(&format!("/1.3/database/{uuid}"), &req)
            .await
    }

    /// Deletes a managed database.
    ///
    /// `DELETE /1.3/database/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#delete-managed-database)
    pub async fn delete(&self, uuid: &str) -> Result<(), Error> {
        self.client.delete(&format!("/1.3/database/{uuid}")).await
    }

    /// Starts a managed database.
    ///
    /// `POST /1.3/database/{uuid}/start`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#start-managed-database)
    pub async fn start(&self, uuid: &str) -> Result<Database, Error> {
        self.client
            .post_empty(&format!("/1.3/database/{uuid}/start"))
            .await
    }

    /// Shuts down a managed database.
    ///
    /// `POST /1.3/database/{uuid}/shutdown`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#shutdown-managed-database)
    pub async fn shutdown(&self, uuid: &str) -> Result<Database, Error> {
        self.client
            .post_empty(&format!("/1.3/database/{uuid}/shutdown"))
            .await
    }

    /// Returns logs for a managed database.
    ///
    /// `GET /1.3/database/{uuid}/logs`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#get-managed-database-logs)
    pub async fn get_logs(
        &self,
        uuid: &str,
        params: GetLogsRequest,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .get_with_query(&format!("/1.3/database/{uuid}/logs"), &params)
            .await
    }

    /// Returns metrics for a managed database.
    ///
    /// `GET /1.3/database/{uuid}/metrics`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#get-managed-database-metrics)
    pub async fn get_metrics(&self, uuid: &str) -> Result<serde_json::Value, Error> {
        self.client
            .get(&format!("/1.3/database/{uuid}/metrics"))
            .await
    }

    /// Returns active sessions for a managed database.
    ///
    /// `GET /1.3/database/{uuid}/sessions`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#get-managed-database-sessions)
    pub async fn list_sessions(
        &self,
        uuid: &str,
        params: GetSessionsRequest,
    ) -> Result<Vec<Session>, Error> {
        self.client
            .get_with_query(&format!("/1.3/database/{uuid}/sessions"), &params)
            .await
    }

    /// Returns query statistics for a managed database.
    ///
    /// `GET /1.3/database/{uuid}/query-statistics`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#get-managed-database-query-statistics)
    pub async fn get_query_statistics(&self, uuid: &str) -> Result<serde_json::Value, Error> {
        self.client
            .get(&format!("/1.3/database/{uuid}/query-statistics"))
            .await
    }

    /// Returns current running queries.
    ///
    /// `GET /1.3/database/{uuid}/current-queries`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#get-managed-database-current-queries)
    pub async fn get_current_queries(&self, uuid: &str) -> Result<serde_json::Value, Error> {
        self.client
            .get(&format!("/1.3/database/{uuid}/current-queries"))
            .await
    }

    /// Terminates current running queries.
    ///
    /// `DELETE /1.3/database/{uuid}/current-queries`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#cancel-managed-database-current-queries)
    pub async fn terminate_queries(
        &self,
        uuid: &str,
        req: TerminateQueriesRequest,
    ) -> Result<(), Error> {
        self.client
            .delete_with_body(&format!("/1.3/database/{uuid}/current-queries"), &req)
            .await
    }

    /// Returns available PostgreSQL extensions for a specific database instance.
    ///
    /// `GET /1.3/database/{uuid}/available-extensions`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#get-managed-database-service-types)
    pub async fn list_available_extensions(&self, uuid: &str) -> Result<Vec<Extension>, Error> {
        self.client
            .get(&format!("/1.3/database/{uuid}/available-extensions"))
            .await
    }

    // ─── Users ────────────────────────────────────────────────────────────────

    /// Returns a list of users for a managed database.
    ///
    /// `GET /1.3/database/{uuid}/users`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#list-users)
    pub async fn list_users(&self, uuid: &str) -> Result<Vec<User>, Error> {
        self.client
            .get(&format!("/1.3/database/{uuid}/users"))
            .await
    }

    /// Returns details for a specific database user.
    ///
    /// `GET /1.3/database/{uuid}/users/{username}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#get-user-details)
    pub async fn get_user(&self, uuid: &str, username: &str) -> Result<User, Error> {
        self.client
            .get(&format!("/1.3/database/{uuid}/users/{username}"))
            .await
    }

    /// Creates a new database user.
    ///
    /// `POST /1.3/database/{uuid}/users`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#create-user)
    pub async fn create_user(&self, uuid: &str, req: CreateUserRequest) -> Result<User, Error> {
        self.client
            .post(&format!("/1.3/database/{uuid}/users"), &req)
            .await
    }

    /// Modifies a database user.
    ///
    /// `PATCH /1.3/database/{uuid}/users/{username}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#modify-user)
    pub async fn modify_user(
        &self,
        uuid: &str,
        username: &str,
        req: ModifyUserRequest,
    ) -> Result<User, Error> {
        self.client
            .patch(&format!("/1.3/database/{uuid}/users/{username}"), &req)
            .await
    }

    /// Deletes a database user.
    ///
    /// `DELETE /1.3/database/{uuid}/users/{username}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#delete-user)
    pub async fn delete_user(&self, uuid: &str, username: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/database/{uuid}/users/{username}"))
            .await
    }

    /// Resets a database user's password.
    ///
    /// `PATCH /1.3/database/{uuid}/users/{username}/access-control`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#modify-user)
    pub async fn reset_user_password(
        &self,
        uuid: &str,
        username: &str,
        req: ResetUserPasswordRequest,
    ) -> Result<User, Error> {
        self.client
            .patch(
                &format!("/1.3/database/{uuid}/users/{username}/access-control"),
                &req,
            )
            .await
    }

    // ─── Logical Databases ────────────────────────────────────────────────────

    /// Returns a list of logical databases within a managed database service.
    ///
    /// `GET /1.3/database/{uuid}/databases`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#list-logical-databases)
    pub async fn list_logical_databases(&self, uuid: &str) -> Result<Vec<LogicalDatabase>, Error> {
        self.client
            .get(&format!("/1.3/database/{uuid}/databases"))
            .await
    }

    /// Returns details for a specific logical database.
    ///
    /// `GET /1.3/database/{uuid}/databases/{dbname}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#get-logical-database-details)
    pub async fn get_logical_database(
        &self,
        uuid: &str,
        dbname: &str,
    ) -> Result<LogicalDatabase, Error> {
        self.client
            .get(&format!("/1.3/database/{uuid}/databases/{dbname}"))
            .await
    }

    /// Creates a new logical database.
    ///
    /// `POST /1.3/database/{uuid}/databases`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#create-logical-database-mysql-postgresql)
    pub async fn create_logical_database(
        &self,
        uuid: &str,
        req: CreateLogicalDatabaseRequest,
    ) -> Result<LogicalDatabase, Error> {
        self.client
            .post(&format!("/1.3/database/{uuid}/databases"), &req)
            .await
    }

    /// Deletes a logical database.
    ///
    /// `DELETE /1.3/database/{uuid}/databases/{dbname}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#delete-logical-database-mysql-postgresql)
    pub async fn delete_logical_database(&self, uuid: &str, dbname: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/database/{uuid}/databases/{dbname}"))
            .await
    }

    /// Clones a managed database from a backup or point-in-time.
    ///
    /// `POST /1.3/database/{uuid}/clone`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#clone-managed-database)
    pub async fn clone_database(&self, uuid: &str, req: &CloneRequest) -> Result<Database, Error> {
        self.client
            .post(&format!("/1.3/database/{uuid}/clone"), req)
            .await
    }

    /// Upgrades the version of a managed database.
    ///
    /// `POST /1.3/database/{uuid}/upgrade`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#get-managed-database-versions)
    pub async fn upgrade_version(
        &self,
        uuid: &str,
        req: &UpgradeVersionRequest,
    ) -> Result<Database, Error> {
        self.client
            .post(&format!("/1.3/database/{uuid}/upgrade"), req)
            .await
    }

    // ─── OpenSearch Access Control ────────────────────────────────────────────

    /// Returns access control settings for a managed database.
    ///
    /// `GET /1.3/database/{uuid}/access-control`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#get-managed-database-access-control)
    pub async fn get_access_control(&self, uuid: &str) -> Result<OpenSearchAccessControl, Error> {
        self.client
            .get(&format!("/1.3/database/{uuid}/access-control"))
            .await
    }

    /// Modifies access control settings for a managed database.
    ///
    /// `PUT /1.3/database/{uuid}/access-control`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#modify-managed-database-access-control)
    pub async fn modify_access_control(
        &self,
        uuid: &str,
        req: ModifyAccessControlRequest,
    ) -> Result<OpenSearchAccessControl, Error> {
        self.client
            .put(&format!("/1.3/database/{uuid}/access-control"), &req)
            .await
    }

    /// Modifies user-level access control for a managed database user.
    ///
    /// `PUT /1.3/database/{uuid}/users/{username}/access-control`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#modify-managed-database-user-access-control)
    pub async fn modify_user_access_control(
        &self,
        uuid: &str,
        username: &str,
        req: ModifyUserAccessControlRequest,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .put(
                &format!("/1.3/database/{uuid}/users/{username}/access-control"),
                &req,
            )
            .await
    }

    // ─── OpenSearch Index Management ──────────────────────────────────────────

    /// Returns a list of indices for a managed OpenSearch database.
    ///
    /// `GET /1.3/database/{uuid}/indices`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#get-managed-database-indices)
    pub async fn list_indices(&self, uuid: &str) -> Result<Vec<OpenSearchIndex>, Error> {
        self.client
            .get(&format!("/1.3/database/{uuid}/indices"))
            .await
    }

    /// Deletes an index from a managed OpenSearch database.
    ///
    /// `DELETE /1.3/database/{uuid}/indices/{index_name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#delete-managed-database-index)
    pub async fn delete_index(&self, uuid: &str, index_name: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/database/{uuid}/indices/{index_name}"))
            .await
    }

    // ─── Session Management ───────────────────────────────────────────────────

    /// Cancels or terminates a session in a managed database.
    ///
    /// `DELETE /1.3/database/{uuid}/sessions/{pid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/16-managed-database/#cancel-managed-database-session)
    pub async fn cancel_session(&self, uuid: &str, pid: i64) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/database/{uuid}/sessions/{pid}"))
            .await
    }

    // ─── State Polling ────────────────────────────────────────────────────────

    /// Polls until the database reaches the desired state or the timeout expires.
    pub async fn wait_for_state(
        &self,
        uuid: &str,
        desired_state: State,
        timeout: std::time::Duration,
    ) -> Result<Database, Error> {
        let start = std::time::Instant::now();
        loop {
            let details = self.get(uuid).await?;
            if details.state == desired_state {
                return Ok(details);
            }
            if start.elapsed() >= timeout {
                return Err(Error::InvalidConfig(format!(
                    "timeout waiting for database {} to reach state {:?}",
                    uuid, desired_state
                )));
            }
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    }
}

#[cfg(test)]
mod tests;
