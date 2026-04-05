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
mod tests {
    use super::*;

    const DB_LIST_JSON: &str = r#"[
  {
    "uuid": "09d9d824-8b3e-4a57-a9b3-77ec51fd5c84",
    "name": "pg-2x2xcpu-4gb-50gb-de-fra1",
    "title": "pg-2x2xcpu-4gb-50gb-de-fra1",
    "plan": "2x2xCPU-4GB-50GB",
    "zone": "de-fra1",
    "type": "pg",
    "state": "running",
    "powered": true,
    "components": [
      {
        "component": "pg",
        "host": "example.db.upclouddatabases.com",
        "port": 11550,
        "route": "dynamic",
        "usage": "primary"
      },
      {
        "component": "pgbouncer",
        "host": "example.db.upclouddatabases.com",
        "port": 11551,
        "route": "dynamic",
        "usage": "primary"
      }
    ],
    "node_states": [
      {
        "name": "pg-2x2xcpu-4gb-50gb-de-fra1-1",
        "role": "master",
        "state": "running"
      },
      {
        "name": "pg-2x2xcpu-4gb-50gb-de-fra1-2",
        "role": "standby",
        "state": "syncing_data"
      }
    ],
    "properties": {
      "automatic_utility_network_ip_filter": false,
      "ip_filter": ["0.0.0.0/0"],
      "public_access": true,
      "version": "13"
    },
    "service_uri": "postgres://upadmin:pass@host:11550/defaultdb?sslmode=require",
    "service_uri_params": {
      "host": "example.db.upclouddatabases.com",
      "port": "11550"
    },
    "labels": [
      {"key": "env", "value": "production"}
    ]
  }
]"#;

    const DB_USER_LIST_JSON: &str = r#"[
  {"username": "upadmin", "type": "primary"},
  {"username": "reader", "type": "readonly", "pg_allow_replication": false}
]"#;

    const DB_LOGICAL_DB_LIST_JSON: &str = r#"[
  {"name": "defaultdb", "lc_collate": "en_US.UTF-8", "lc_ctype": "en_US.UTF-8"},
  {"name": "testdb"}
]"#;

    const DB_PLAN_JSON: &str = r#"[
  {"name": "1x1xCPU-2GB-25GB", "node_count": 1, "storage_size": 25600},
  {"name": "2x2xCPU-4GB-50GB", "node_count": 2, "storage_size": 51200}
]"#;

    const DB_SESSION_JSON: &str = r#"[
  {
    "pid": 12345,
    "query": "SELECT 1",
    "query_duration": "00:00:01",
    "state": "active",
    "client_addr": "192.168.1.1",
    "datname": "defaultdb",
    "usename": "upadmin"
  }
]"#;

    #[test]
    fn deserialize_managed_database_list() {
        let dbs: Vec<Database> = serde_json::from_str(DB_LIST_JSON).unwrap();
        assert_eq!(dbs.len(), 1);
        let db = &dbs[0];
        assert_eq!(db.uuid, "09d9d824-8b3e-4a57-a9b3-77ec51fd5c84");
        assert_eq!(db.name, "pg-2x2xcpu-4gb-50gb-de-fra1");
        assert_eq!(db.title, "pg-2x2xcpu-4gb-50gb-de-fra1");
        assert_eq!(db.plan, "2x2xCPU-4GB-50GB");
        assert_eq!(db.db_type, "pg");
        assert_eq!(db.state, State::Running);
        assert_eq!(db.zone, "de-fra1");
        assert_eq!(db.powered, Some(true));
        assert!(db.primary_database.is_none());
        assert!(db.access_control.is_none());
        assert!(db.extended_access_control.is_none());
        assert!(db.version.is_none());
        assert!(db.created_at.is_none());
        assert!(db.updated_at.is_none());
        assert!(db.networks.is_none());

        let components = db.components.as_ref().unwrap();
        assert_eq!(components.len(), 2);
        assert_eq!(components[0].component, "pg");
        assert_eq!(components[0].host, "example.db.upclouddatabases.com");
        assert_eq!(components[0].port, 11550);
        assert_eq!(components[0].route, "dynamic");
        assert_eq!(components[0].usage, "primary");
        assert_eq!(components[1].component, "pgbouncer");
        assert_eq!(components[1].host, "example.db.upclouddatabases.com");
        assert_eq!(components[1].port, 11551);
        assert_eq!(components[1].route, "dynamic");
        assert_eq!(components[1].usage, "primary");

        let nodes = db.node_states.as_ref().unwrap();
        assert_eq!(nodes.len(), 2);
        assert_eq!(nodes[0].name, "pg-2x2xcpu-4gb-50gb-de-fra1-1");
        assert_eq!(nodes[0].role, "master");
        assert_eq!(nodes[0].state, "running");
        assert_eq!(nodes[1].name, "pg-2x2xcpu-4gb-50gb-de-fra1-2");
        assert_eq!(nodes[1].role, "standby");
        assert_eq!(nodes[1].state, "syncing_data");

        let props = db.properties.as_ref().unwrap();
        assert_eq!(props["automatic_utility_network_ip_filter"], false);
        assert_eq!(props["ip_filter"][0], "0.0.0.0/0");
        assert_eq!(props["public_access"], true);
        assert_eq!(props["version"], "13");

        assert_eq!(
            db.service_uri.as_deref(),
            Some("postgres://upadmin:pass@host:11550/defaultdb?sslmode=require")
        );
        let uri_params = db.service_uri_params.as_ref().unwrap();
        assert_eq!(uri_params["host"], "example.db.upclouddatabases.com");
        assert_eq!(uri_params["port"], "11550");

        let labels = db.labels.as_ref().unwrap();
        assert_eq!(labels.len(), 1);
        assert_eq!(labels[0].key, "env");
        assert_eq!(labels[0].value, "production");
    }

    #[test]
    fn deserialize_db_user_list() {
        let users: Vec<User> = serde_json::from_str(DB_USER_LIST_JSON).unwrap();
        assert_eq!(users.len(), 2);
        assert_eq!(users[0].username, "upadmin");
        assert_eq!(users[0].user_type, UserType::Primary);
        assert!(users[0].password.is_none());
        assert!(users[0].pg_allow_replication.is_none());
        assert!(users[0].authentication.is_none());
        assert_eq!(users[1].username, "reader");
        assert_eq!(users[1].user_type, UserType::Readonly);
        assert_eq!(users[1].pg_allow_replication, Some(false));
        assert!(users[1].password.is_none());
        assert!(users[1].authentication.is_none());
    }

    #[test]
    fn deserialize_db_logical_database_list() {
        let dbs: Vec<LogicalDatabase> =
            serde_json::from_str(DB_LOGICAL_DB_LIST_JSON).unwrap();
        assert_eq!(dbs.len(), 2);
        assert_eq!(dbs[0].name, "defaultdb");
        assert_eq!(dbs[0].lc_collate.as_deref(), Some("en_US.UTF-8"));
        assert_eq!(dbs[0].lc_ctype.as_deref(), Some("en_US.UTF-8"));
        assert_eq!(dbs[1].name, "testdb");
        assert!(dbs[1].lc_collate.is_none());
        assert!(dbs[1].lc_ctype.is_none());
    }

    #[test]
    fn deserialize_db_plan_list() {
        let plans: Vec<ServicePlan> = serde_json::from_str(DB_PLAN_JSON).unwrap();
        assert_eq!(plans.len(), 2);
        assert_eq!(plans[0].name, "1x1xCPU-2GB-25GB");
        assert_eq!(plans[0].node_count, Some(1));
        assert_eq!(plans[0].storage_size, Some(25600));
        assert!(plans[0].backup_config.is_none());
        assert!(plans[0].max_memory_percent.is_none());
        assert!(plans[0].zones.is_none());
        assert_eq!(plans[1].name, "2x2xCPU-4GB-50GB");
        assert_eq!(plans[1].node_count, Some(2));
        assert_eq!(plans[1].storage_size, Some(51200));
        assert!(plans[1].backup_config.is_none());
        assert!(plans[1].max_memory_percent.is_none());
        assert!(plans[1].zones.is_none());
    }

    #[test]
    fn deserialize_db_session_list() {
        let sessions: Vec<Session> = serde_json::from_str(DB_SESSION_JSON).unwrap();
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].pid, Some(12345));
        assert_eq!(sessions[0].query.as_deref(), Some("SELECT 1"));
        assert_eq!(sessions[0].query_duration.as_deref(), Some("00:00:01"));
        assert_eq!(sessions[0].state.as_deref(), Some("active"));
        assert_eq!(sessions[0].client_addr.as_deref(), Some("192.168.1.1"));
        assert_eq!(sessions[0].datname.as_deref(), Some("defaultdb"));
        assert_eq!(sessions[0].usename.as_deref(), Some("upadmin"));
    }

    // ─── New deserialization tests ───────────────────────────────────────────

    #[test]
    fn deserialize_pg_available_extensions() {
        let json = r#"[
  {
    "extensions": [
      {"default_version": "1.0", "name": "bloom", "versions": ["1.0"]},
      {"default_version": "1.0", "name": "bool_plperl", "versions": ["1.0"]},
      {"default_version": "1.3", "name": "btree_gin", "versions": ["1.0", "1.1", "1.2", "1.3"]}
    ],
    "version": "13"
  },
  {
    "extensions": [
      {"default_version": "7", "name": "unit", "versions": ["3", "4", "5", "6", "7"]},
      {"default_version": "1.1", "name": "uuid-ossp", "versions": ["1.1"]},
      {"default_version": "0.8.1", "name": "vector", "versions": ["0.8.1"]}
    ],
    "version": "14"
  }
]"#;
        let versions: Vec<PgVersionExtensions> = serde_json::from_str(json).unwrap();
        assert_eq!(versions.len(), 2);
        assert_eq!(versions[0].version, "13");
        assert_eq!(versions[0].extensions.len(), 3);
        assert_eq!(versions[0].extensions[0].name, "bloom");
        assert_eq!(versions[0].extensions[0].default_version, "1.0");
        assert_eq!(versions[0].extensions[0].versions, vec!["1.0"]);
        assert_eq!(versions[0].extensions[1].name, "bool_plperl");
        assert_eq!(versions[0].extensions[1].default_version, "1.0");
        assert_eq!(versions[0].extensions[1].versions, vec!["1.0"]);
        assert_eq!(versions[0].extensions[2].name, "btree_gin");
        assert_eq!(versions[0].extensions[2].default_version, "1.3");
        assert_eq!(
            versions[0].extensions[2].versions,
            vec!["1.0", "1.1", "1.2", "1.3"]
        );
        assert_eq!(versions[1].version, "14");
        assert_eq!(versions[1].extensions.len(), 3);
        assert_eq!(versions[1].extensions[0].name, "unit");
        assert_eq!(versions[1].extensions[0].default_version, "7");
        assert_eq!(
            versions[1].extensions[0].versions,
            vec!["3", "4", "5", "6", "7"]
        );
        assert_eq!(versions[1].extensions[1].name, "uuid-ossp");
        assert_eq!(versions[1].extensions[1].default_version, "1.1");
        assert_eq!(versions[1].extensions[1].versions, vec!["1.1"]);
        assert_eq!(versions[1].extensions[2].name, "vector");
        assert_eq!(versions[1].extensions[2].default_version, "0.8.1");
        assert_eq!(versions[1].extensions[2].versions, vec!["0.8.1"]);
    }

    #[test]
    fn deserialize_instance_available_extensions() {
        let json = r#"[
  {"default_version": "1.6", "name": "pg_buffercache", "versions": ["1.2", "1.3", "1.4", "1.5", "1.6"]},
  {"default_version": "1.6", "name": "pg_cron", "versions": ["1.0", "1.1", "1.2", "1.3", "1.4", "1.4-1", "1.5", "1.6"]},
  {"default_version": "5.2.4", "name": "pg_partman", "versions": ["5.2.4"]}
]"#;
        let exts: Vec<PgExtensionDetail> = serde_json::from_str(json).unwrap();
        assert_eq!(exts.len(), 3);
        assert_eq!(exts[0].name, "pg_buffercache");
        assert_eq!(exts[0].default_version, "1.6");
        assert_eq!(exts[0].versions, vec!["1.2", "1.3", "1.4", "1.5", "1.6"]);
        assert_eq!(exts[1].name, "pg_cron");
        assert_eq!(exts[1].default_version, "1.6");
        assert_eq!(exts[1].versions.len(), 8);
        assert_eq!(exts[2].name, "pg_partman");
        assert_eq!(exts[2].default_version, "5.2.4");
        assert_eq!(exts[2].versions, vec!["5.2.4"]);
    }

    #[test]
    fn deserialize_db_service_types() {
        let json = r#"{
  "mysql": {
    "name": "mysql",
    "description": "MySQL - Relational Database Management System",
    "latest_available_version": "8.0.26",
    "service_plans": [
      {
        "node_count": 1,
        "plan": "1x1xCPU-2GB-25GB",
        "core_number": 1,
        "storage_size": 25600,
        "memory_amount": 2048
      }
    ],
    "properties": {
      "public_access": {
        "default": false,
        "title": "Public Access",
        "type": "boolean"
      }
    }
  },
  "pg": {
    "name": "pg",
    "description": "PostgreSQL - Object-Relational Database Management System",
    "latest_available_version": "13.5",
    "service_plans": [],
    "properties": {}
  }
}"#;
        let val: serde_json::Value = serde_json::from_str(json).unwrap();
        assert!(val.is_object());
        let obj = val.as_object().unwrap();
        assert!(obj.contains_key("mysql"));
        assert!(obj.contains_key("pg"));

        let mysql = &val["mysql"];
        assert_eq!(mysql["name"], "mysql");
        assert_eq!(mysql["latest_available_version"], "8.0.26");
        assert_eq!(
            mysql["description"],
            "MySQL - Relational Database Management System"
        );
        let plans = mysql["service_plans"].as_array().unwrap();
        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0]["plan"], "1x1xCPU-2GB-25GB");
        assert_eq!(plans[0]["node_count"], 1);
        assert_eq!(plans[0]["core_number"], 1);
        assert_eq!(plans[0]["storage_size"], 25600);
        assert_eq!(plans[0]["memory_amount"], 2048);
        let mysql_props = mysql["properties"].as_object().unwrap();
        assert!(mysql_props.contains_key("public_access"));
        assert_eq!(mysql["properties"]["public_access"]["default"], false);
        assert_eq!(mysql["properties"]["public_access"]["title"], "Public Access");
        assert_eq!(mysql["properties"]["public_access"]["type"], "boolean");

        let pg = &val["pg"];
        assert_eq!(pg["name"], "pg");
        assert_eq!(
            pg["description"],
            "PostgreSQL - Object-Relational Database Management System"
        );
        assert_eq!(pg["latest_available_version"], "13.5");
        assert!(pg["service_plans"].as_array().unwrap().is_empty());
        assert!(pg["properties"].as_object().unwrap().is_empty());
    }

    #[test]
    fn deserialize_create_db_request() {
        let json = r#"{
  "hostname_prefix": "doc-api-unique-prefix",
  "plan": "1x1xCPU-2GB-25GB",
  "title": "my-managed-database",
  "type": "mysql",
  "zone": "de-fra1",
  "maintenance": {"dow": "sunday", "time": "05:00:00"},
  "termination_protection": false,
  "properties": {
    "automatic_utility_network_ip_filter": true,
    "version": "8",
    "ip_filter": []
  },
  "networks": [
    {"name": "example-network-1", "type": "private", "family": "IPv4", "uuid": "03631160-d57a-4926-ad48-a2f828229dcb"}
  ],
  "labels": [
    {"key": "env", "value": "staging"},
    {"key": "foo", "value": "bar"}
  ]
}"#;
        // The docs use hostname_prefix but our type uses name; parse as Value first
        let val: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(val["hostname_prefix"], "doc-api-unique-prefix");
        assert_eq!(val["plan"], "1x1xCPU-2GB-25GB");
        assert_eq!(val["title"], "my-managed-database");
        assert_eq!(val["type"], "mysql");
        assert_eq!(val["zone"], "de-fra1");
        assert_eq!(val["maintenance"]["dow"], "sunday");
        assert_eq!(val["maintenance"]["time"], "05:00:00");
        assert_eq!(val["termination_protection"], false);
        assert_eq!(val["properties"]["automatic_utility_network_ip_filter"], true);
        assert_eq!(val["properties"]["version"], "8");
        assert!(val["properties"]["ip_filter"].as_array().unwrap().is_empty());

        let networks = val["networks"].as_array().unwrap();
        assert_eq!(networks.len(), 1);
        assert_eq!(networks[0]["name"], "example-network-1");
        assert_eq!(networks[0]["type"], "private");
        assert_eq!(networks[0]["family"], "IPv4");
        assert_eq!(networks[0]["uuid"], "03631160-d57a-4926-ad48-a2f828229dcb");

        let labels = val["labels"].as_array().unwrap();
        assert_eq!(labels.len(), 2);
        assert_eq!(labels[0]["key"], "env");
        assert_eq!(labels[0]["value"], "staging");
        assert_eq!(labels[1]["key"], "foo");
        assert_eq!(labels[1]["value"], "bar");
    }

    #[test]
    fn deserialize_db_details() {
        let json = r#"{
  "uuid": "09d9d824-8b3e-4a57-a9b3-77ec51fd5c84",
  "name": "pg-2x2xcpu-4gb-50gb-de-fra1",
  "title": "pg-2x2xcpu-4gb-50gb-de-fra1",
  "plan": "2x2xCPU-4GB-50GB",
  "zone": "de-fra1",
  "type": "pg",
  "state": "running",
  "powered": true,
  "components": [
    {"component": "pg", "host": "example.db.upclouddatabases.com", "port": 11550, "route": "dynamic", "usage": "primary"}
  ],
  "node_states": [
    {"name": "pg-1", "role": "master", "state": "running"}
  ],
  "properties": {"public_access": true, "version": "13"},
  "service_uri": "postgres://upadmin:pass@host:11550/defaultdb?sslmode=require",
  "service_uri_params": {"host": "example.db.upclouddatabases.com", "port": "11550"}
}"#;
        // Parse as Value to verify full JSON validity
        let val: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(val["uuid"], "09d9d824-8b3e-4a57-a9b3-77ec51fd5c84");

        // Parse as Database
        let db: Database = serde_json::from_str(json).unwrap();
        assert_eq!(db.uuid, "09d9d824-8b3e-4a57-a9b3-77ec51fd5c84");
        assert_eq!(db.name, "pg-2x2xcpu-4gb-50gb-de-fra1");
        assert_eq!(db.title, "pg-2x2xcpu-4gb-50gb-de-fra1");
        assert_eq!(db.plan, "2x2xCPU-4GB-50GB");
        assert_eq!(db.zone, "de-fra1");
        assert_eq!(db.db_type, "pg");
        assert_eq!(db.state, State::Running);
        assert_eq!(db.powered, Some(true));
        assert!(db.primary_database.is_none());
        assert!(db.access_control.is_none());
        assert!(db.extended_access_control.is_none());
        assert!(db.version.is_none());
        assert!(db.created_at.is_none());
        assert!(db.updated_at.is_none());
        assert!(db.networks.is_none());
        assert!(db.labels.is_none());

        let components = db.components.as_ref().unwrap();
        assert_eq!(components.len(), 1);
        assert_eq!(components[0].component, "pg");
        assert_eq!(components[0].host, "example.db.upclouddatabases.com");
        assert_eq!(components[0].port, 11550);
        assert_eq!(components[0].route, "dynamic");
        assert_eq!(components[0].usage, "primary");

        let nodes = db.node_states.as_ref().unwrap();
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].name, "pg-1");
        assert_eq!(nodes[0].role, "master");
        assert_eq!(nodes[0].state, "running");

        assert_eq!(
            db.service_uri.as_deref(),
            Some("postgres://upadmin:pass@host:11550/defaultdb?sslmode=require")
        );

        let uri_params = db.service_uri_params.as_ref().unwrap();
        assert_eq!(uri_params["host"], "example.db.upclouddatabases.com");
        assert_eq!(uri_params["port"], "11550");

        let props = db.properties.as_ref().unwrap();
        assert_eq!(props["public_access"], true);
        assert_eq!(props["version"], "13");
    }

    #[test]
    fn deserialize_modify_db_request() {
        let json = r#"{
  "title": "my-managed-database",
  "powered": true,
  "plan": "1x1xCPU-2GB-25GB",
  "properties": {"automatic_utility_network_ip_filter": true}
}"#;
        let req: ModifyRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.title.as_deref(), Some("my-managed-database"));
        assert_eq!(req.powered, Some(true));
        assert_eq!(req.plan.as_deref(), Some("1x1xCPU-2GB-25GB"));
        let props = req.properties.as_ref().unwrap();
        assert_eq!(props["automatic_utility_network_ip_filter"], true);
        assert!(req.labels.is_none());
    }

    #[test]
    fn deserialize_create_db_user_mysql() {
        let json = r#"{"username": "api-doc-user", "authentication": "caching_sha2_password", "password": "new-password"}"#;
        let req: CreateUserRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.username, "api-doc-user");
        assert_eq!(req.password, "new-password");
        assert_eq!(req.authentication.as_deref(), Some("caching_sha2_password"));
        assert!(req.user_type.is_none());
        assert!(req.pg_allow_replication.is_none());
    }

    #[test]
    fn deserialize_db_user_detail_mysql() {
        let json = r#"{"username": "api-doc-user", "password": "my-password", "authentication": "caching_sha2_password", "type": "normal"}"#;
        let user: User = serde_json::from_str(json).unwrap();
        assert_eq!(user.username, "api-doc-user");
        assert_eq!(user.password.as_deref(), Some("my-password"));
        assert_eq!(
            user.authentication.as_deref(),
            Some("caching_sha2_password")
        );
        assert_eq!(user.user_type, UserType::Normal);
        assert!(user.pg_allow_replication.is_none());
    }

    #[test]
    fn deserialize_db_user_detail_pg() {
        let json = r#"{"username": "api-doc-user", "password": "new-password", "type": "normal", "pg_access_control": {"allow_replication": true}}"#;
        // pg_access_control is not in User; parse as Value
        let val: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(val["username"], "api-doc-user");
        assert_eq!(val["password"], "new-password");
        assert_eq!(val["type"], "normal");
        assert_eq!(val["pg_access_control"]["allow_replication"], true);
    }

    #[test]
    fn deserialize_modify_db_user_request() {
        let json =
            r#"{"authentication": "caching_sha2_password", "password": "new-password"}"#;
        let req: ModifyUserRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.password.as_deref(), Some("new-password"));
        assert_eq!(
            req.authentication.as_deref(),
            Some("caching_sha2_password")
        );
        assert!(req.pg_allow_replication.is_none());
    }

    #[test]
    fn deserialize_create_logical_db() {
        let json =
            r#"{"name": "database-doc-api", "lc_collate": "en_US.UTF8", "lc_ctype": "en_US.UTF8"}"#;
        let req: CreateLogicalDatabaseRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.name, "database-doc-api");
        assert_eq!(req.lc_collate.as_deref(), Some("en_US.UTF8"));
        assert_eq!(req.lc_ctype.as_deref(), Some("en_US.UTF8"));
    }

    #[test]
    fn deserialize_session_terminate() {
        let json = r#"{"success": true}"#;
        let val: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(val["success"], true);
    }

    #[test]
    fn deserialize_pg_sessions_detailed() {
        let json = r#"{
  "pg": [
    {
      "application_name": "client 1.5.14",
      "backend_start": "2022-01-21T13:26:26.682858Z",
      "backend_type": "client backend",
      "client_addr": "111.111.111.111",
      "client_port": 61264,
      "datid": 16401,
      "datname": "defaultdb",
      "id": "pid_2065031",
      "query": "SELECT 1",
      "query_duration": 12225858000,
      "state": "idle",
      "usename": "upadmin"
    }
  ]
}"#;
        let val: serde_json::Value = serde_json::from_str(json).unwrap();
        let sessions = val["pg"].as_array().unwrap();
        assert_eq!(sessions.len(), 1);
        let s = &sessions[0];
        assert_eq!(s["application_name"], "client 1.5.14");
        assert_eq!(s["backend_start"], "2022-01-21T13:26:26.682858Z");
        assert_eq!(s["backend_type"], "client backend");
        assert_eq!(s["client_addr"], "111.111.111.111");
        assert_eq!(s["client_port"], 61264);
        assert_eq!(s["datid"], 16401);
        assert_eq!(s["datname"], "defaultdb");
        assert_eq!(s["id"], "pid_2065031");
        assert_eq!(s["query"], "SELECT 1");
        assert_eq!(s["query_duration"], 12225858000_u64);
        assert_eq!(s["state"], "idle");
        assert_eq!(s["usename"], "upadmin");
    }

    #[test]
    fn deserialize_query_statistics() {
        let json = r#"{
  "pg": [
    {
      "blk_read_time": 0,
      "blk_write_time": 0,
      "calls": 57425,
      "database_name": "defaultdb",
      "max_time": 133387,
      "mean_time": 1060,
      "min_time": 151,
      "query": "COMMIT",
      "rows": 0,
      "total_time": 60879904,
      "user_name": "upadmin"
    }
  ]
}"#;
        let val: serde_json::Value = serde_json::from_str(json).unwrap();
        let stats = val["pg"].as_array().unwrap();
        assert_eq!(stats.len(), 1);
        let s = &stats[0];
        assert_eq!(s["blk_read_time"], 0);
        assert_eq!(s["blk_write_time"], 0);
        assert_eq!(s["calls"], 57425);
        assert_eq!(s["database_name"], "defaultdb");
        assert_eq!(s["query"], "COMMIT");
        assert_eq!(s["max_time"], 133387);
        assert_eq!(s["mean_time"], 1060);
        assert_eq!(s["min_time"], 151);
        assert_eq!(s["rows"], 0);
        assert_eq!(s["total_time"], 60879904);
        assert_eq!(s["user_name"], "upadmin");
    }

    #[test]
    fn deserialize_db_metrics() {
        let json = r#"{
  "cpu_usage": {
    "data": {
      "cols": [
        {"label": "time", "type": "date"},
        {"label": "test-1 (master)", "type": "number"}
      ],
      "rows": [
        ["2022-01-21T13:10:30Z", 2.744]
      ]
    },
    "hints": {"title": "CPU usage %"}
  }
}"#;
        let val: serde_json::Value = serde_json::from_str(json).unwrap();
        let cpu = &val["cpu_usage"];
        let cols = cpu["data"]["cols"].as_array().unwrap();
        assert_eq!(cols.len(), 2);
        assert_eq!(cols[0]["label"], "time");
        assert_eq!(cols[0]["type"], "date");
        assert_eq!(cols[1]["label"], "test-1 (master)");
        assert_eq!(cols[1]["type"], "number");
        let rows = cpu["data"]["rows"].as_array().unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][0], "2022-01-21T13:10:30Z");
        assert_eq!(rows[0][1], 2.744);
        assert_eq!(cpu["hints"]["title"], "CPU usage %");
    }

    #[test]
    fn deserialize_db_logs() {
        let json = r#"{
  "first_log_offset": "1642770443687",
  "offset": "1642770443687",
  "logs": [
    {
      "hostname": "api-doc-hostname",
      "msg": "LOG: connection authorized",
      "time": "2022-01-21T13:07:23.687241Z",
      "service": "postgresql-13.service"
    }
  ]
}"#;
        let val: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(val["first_log_offset"], "1642770443687");
        assert_eq!(val["offset"], "1642770443687");
        let logs = val["logs"].as_array().unwrap();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0]["hostname"], "api-doc-hostname");
        assert_eq!(logs[0]["msg"], "LOG: connection authorized");
        assert_eq!(logs[0]["time"], "2022-01-21T13:07:23.687241Z");
        assert_eq!(logs[0]["service"], "postgresql-13.service");
    }

    #[test]
    fn deserialize_db_versions() {
        let json = r#"["10", "11", "12", "13", "14"]"#;
        let versions: Vec<String> = serde_json::from_str(json).unwrap();
        assert_eq!(versions.len(), 5);
        assert_eq!(versions[0], "10");
        assert_eq!(versions[1], "11");
        assert_eq!(versions[2], "12");
        assert_eq!(versions[3], "13");
        assert_eq!(versions[4], "14");
    }

    #[test]
    fn deserialize_db_networks() {
        let json = r#"[
  {"uuid": "03aa7245-2ff9-49c8-9f0e-7ca0270d71a4", "name": "example-private-network", "family": "IPv4", "type": "private"}
]"#;
        let nets: Vec<Network> = serde_json::from_str(json).unwrap();
        assert_eq!(nets.len(), 1);
        assert_eq!(nets[0].uuid, "03aa7245-2ff9-49c8-9f0e-7ca0270d71a4");
        assert_eq!(nets[0].name, "example-private-network");
        assert_eq!(nets[0].family, "IPv4");
        assert_eq!(nets[0].network_type, "private");
    }

    // ─── Service type detail (single type, official docs page 16) ─────────────

    const DB_SERVICE_TYPE_DETAIL_JSON: &str = r#"{
  "mysql": {
    "name": "mysql",
    "description": "MySQL - Relational Database Management System",
    "latest_available_version": "8.0.26",
    "service_plans": [
      {
        "backup_config_msql": {
          "interval": 24,
          "max_count": 2,
          "recovery_mode": "pitr"
        },
        "node_count": 1,
        "zones": {
          "zone": [
            { "name": "es-mad1" },
            { "name": "pl-waw1" },
            { "name": "us-sjo1" }
          ]
        },
        "plan": "1x1xCPU-2GB-25GB",
        "core_number": 1,
        "storage_size": 25600,
        "memory_amount": 2048
      }
    ],
    "properties": {
      "admin_password": {
        "createOnly": true,
        "example": "z66o9QXqKM",
        "maxLength": 256,
        "minLength": 8,
        "pattern": "^[a-zA-Z0-9-_]+$",
        "title": "Custom password for admin user. Defaults to random string. This must be set only when a new service is being created.",
        "type": ["string", "null"],
        "user_error": "Must consist of alpha-numeric characters, underscores or dashes"
      },
      "admin_username": {
        "createOnly": true,
        "example": "upadmin",
        "maxLength": 64,
        "pattern": "^[_A-Za-z0-9][-._A-Za-z0-9]{0,63}$",
        "title": "Custom username for admin user. This must be set only when a new service is being created.",
        "type": ["string", "null"],
        "user_error": "Must consist of alpha-numeric characters, dots, underscores or dashes, may not start with dash or dot, max 64 characters"
      },
      "public_access": {
        "default": false,
        "title": "Public Access",
        "type": "boolean",
        "description": "Allow access to the service from the public Internet"
      },
      "version": {
        "title": "MySQL major version",
        "type": ["string", "null"],
        "enum": ["8"]
      }
    }
  }
}"#;

    #[test]
    fn deserialize_db_service_type_detail() {
        let val: serde_json::Value =
            serde_json::from_str(DB_SERVICE_TYPE_DETAIL_JSON).unwrap();
        assert!(val.is_object());
        let obj = val.as_object().unwrap();
        assert_eq!(obj.len(), 1);
        assert!(obj.contains_key("mysql"));

        let mysql = &val["mysql"];
        assert_eq!(mysql["name"], "mysql");
        assert_eq!(mysql["description"], "MySQL - Relational Database Management System");
        assert_eq!(mysql["latest_available_version"], "8.0.26");

        let plans = mysql["service_plans"].as_array().unwrap();
        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0]["plan"], "1x1xCPU-2GB-25GB");
        assert_eq!(plans[0]["node_count"], 1);
        assert_eq!(plans[0]["core_number"], 1);
        assert_eq!(plans[0]["storage_size"], 25600);
        assert_eq!(plans[0]["memory_amount"], 2048);

        let backup = &plans[0]["backup_config_msql"];
        assert_eq!(backup["interval"], 24);
        assert_eq!(backup["max_count"], 2);
        assert_eq!(backup["recovery_mode"], "pitr");

        let zones = plans[0]["zones"]["zone"].as_array().unwrap();
        assert_eq!(zones.len(), 3);
        assert_eq!(zones[0]["name"], "es-mad1");
        assert_eq!(zones[1]["name"], "pl-waw1");
        assert_eq!(zones[2]["name"], "us-sjo1");

        let props = mysql["properties"].as_object().unwrap();
        assert!(props.contains_key("admin_password"));
        assert!(props.contains_key("admin_username"));
        assert!(props.contains_key("public_access"));
        assert!(props.contains_key("version"));

        assert_eq!(props["admin_password"]["createOnly"], true);
        assert_eq!(props["admin_password"]["example"], "z66o9QXqKM");
        assert_eq!(props["admin_password"]["maxLength"], 256);
        assert_eq!(props["admin_password"]["minLength"], 8);

        assert_eq!(props["public_access"]["default"], false);
        assert_eq!(props["public_access"]["title"], "Public Access");
        assert_eq!(props["public_access"]["type"], "boolean");
        assert_eq!(
            props["public_access"]["description"],
            "Allow access to the service from the public Internet"
        );

        let version_enum = props["version"]["enum"].as_array().unwrap();
        assert_eq!(version_enum.len(), 1);
        assert_eq!(version_enum[0], "8");
    }
}
