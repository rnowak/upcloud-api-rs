//! Managed Databases — `GET/POST/PATCH/DELETE /1.3/database` + sub-resources.

mod types;
pub use types::*;

use crate::client::Client;
use crate::error::Error;

// ─── Handler ──────────────────────────────────────────────────────────────────

pub struct DatabaseHandler<'a> {
    client: &'a Client,
}

impl<'a> DatabaseHandler<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    // ─── Service types ─────────────────────────────────────────────────────────

    /// Returns a list of available managed database service types.
    pub async fn list_types(&self) -> Result<Vec<DatabaseType>, Error> {
        self.client.get("/1.3/database/service-types").await
    }

    /// Returns available PostgreSQL extensions (global list).
    pub async fn list_pg_extensions(&self) -> Result<Vec<Extension>, Error> {
        self.client
            .get("/1.3/database/pg/available-extensions")
            .await
    }

    /// Returns a list of available database plans.
    pub async fn list_plans(&self) -> Result<Vec<ServicePlan>, Error> {
        self.client.get("/1.3/database/plans").await
    }

    // ─── Services ──────────────────────────────────────────────────────────────

    /// Returns a list of all managed database services.
    pub async fn list(&self) -> Result<Vec<Database>, Error> {
        self.client.get("/1.3/database").await
    }

    /// Returns details for a specific managed database.
    pub async fn get(&self, uuid: &str) -> Result<Database, Error> {
        self.client
            .get(&format!("/1.3/database/{uuid}"))
            .await
    }

    /// Creates a new managed database.
    pub async fn create(&self, req: CreateRequest) -> Result<Database, Error> {
        self.client.post("/1.3/database", &req).await
    }

    /// Modifies an existing managed database.
    pub async fn modify(
        &self,
        uuid: &str,
        req: ModifyRequest,
    ) -> Result<Database, Error> {
        self.client
            .patch(&format!("/1.3/database/{uuid}"), &req)
            .await
    }

    /// Deletes a managed database.
    pub async fn delete(&self, uuid: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/database/{uuid}"))
            .await
    }

    /// Starts a managed database.
    pub async fn start(&self, uuid: &str) -> Result<Database, Error> {
        self.client
            .post_empty(&format!("/1.3/database/{uuid}/start"))
            .await
    }

    /// Shuts down a managed database.
    pub async fn shutdown(&self, uuid: &str) -> Result<Database, Error> {
        self.client
            .post_empty(&format!("/1.3/database/{uuid}/shutdown"))
            .await
    }

    /// Returns logs for a managed database.
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
    pub async fn get_metrics(
        &self,
        uuid: &str,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .get(&format!("/1.3/database/{uuid}/metrics"))
            .await
    }

    /// Returns active sessions for a managed database.
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
    pub async fn get_query_statistics(
        &self,
        uuid: &str,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .get(&format!("/1.3/database/{uuid}/query-statistics"))
            .await
    }

    /// Returns current running queries.
    pub async fn get_current_queries(
        &self,
        uuid: &str,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .get(&format!("/1.3/database/{uuid}/current-queries"))
            .await
    }

    /// Terminates current running queries.
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
    pub async fn list_available_extensions(
        &self,
        uuid: &str,
    ) -> Result<Vec<Extension>, Error> {
        self.client
            .get(&format!("/1.3/database/{uuid}/available-extensions"))
            .await
    }

    // ─── Users ────────────────────────────────────────────────────────────────

    /// Returns a list of users for a managed database.
    pub async fn list_users(&self, uuid: &str) -> Result<Vec<User>, Error> {
        self.client
            .get(&format!("/1.3/database/{uuid}/users"))
            .await
    }

    /// Returns details for a specific database user.
    pub async fn get_user(&self, uuid: &str, username: &str) -> Result<User, Error> {
        self.client
            .get(&format!("/1.3/database/{uuid}/users/{username}"))
            .await
    }

    /// Creates a new database user.
    pub async fn create_user(
        &self,
        uuid: &str,
        req: CreateUserRequest,
    ) -> Result<User, Error> {
        self.client
            .post(&format!("/1.3/database/{uuid}/users"), &req)
            .await
    }

    /// Modifies a database user.
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
    pub async fn delete_user(&self, uuid: &str, username: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/database/{uuid}/users/{username}"))
            .await
    }

    /// Resets a database user's password.
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
    pub async fn list_logical_databases(
        &self,
        uuid: &str,
    ) -> Result<Vec<LogicalDatabase>, Error> {
        self.client
            .get(&format!("/1.3/database/{uuid}/databases"))
            .await
    }

    /// Returns details for a specific logical database.
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
    pub async fn delete_logical_database(
        &self,
        uuid: &str,
        dbname: &str,
    ) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/database/{uuid}/databases/{dbname}"))
            .await
    }

    /// Clones a managed database from a backup or point-in-time.
    pub async fn clone_database(
        &self,
        uuid: &str,
        req: &CloneRequest,
    ) -> Result<Database, Error> {
        self.client
            .post(&format!("/1.3/database/{uuid}/clone"), req)
            .await
    }

    /// Upgrades the version of a managed database.
    pub async fn upgrade_version(
        &self,
        uuid: &str,
        req: &UpgradeVersionRequest,
    ) -> Result<Database, Error> {
        self.client
            .post(&format!("/1.3/database/{uuid}/upgrade"), req)
            .await
    }
}

#[cfg(test)]
mod tests;
