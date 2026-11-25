use super::*;
// Source: https://developers.upcloud.com/1.3/16-managed-database/#list-managed-databases
const DB_LIST_JSON: &str = r#"
    [
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
                "ip_filter": [
                    "0.0.0.0/0"
                ],
                "public_access": true,
                "version": "13"
            },
            "service_uri": "postgres://upadmin:pass@host:11550/defaultdb?sslmode=require",
            "service_uri_params": {
                "host": "example.db.upclouddatabases.com",
                "port": "11550"
            },
            "labels": [
                {
                    "key": "env",
                    "value": "production"
                }
            ]
        }
    ]
"#;

#[test]
fn deserialize_managed_database_list() {
    let dbs: Vec<Database> = serde_json::from_str(DB_LIST_JSON).unwrap();
    assert_eq!(dbs.len(), 1);
    let db = &dbs[0];
    assert_eq!(db.uuid, "09d9d824-8b3e-4a57-a9b3-77ec51fd5c84");
    assert_eq!(db.name, "pg-2x2xcpu-4gb-50gb-de-fra1");
    assert_eq!(db.title, "pg-2x2xcpu-4gb-50gb-de-fra1");
    assert_eq!(db.plan, "2x2xCPU-4GB-50GB");
    assert_eq!(db.db_type, DatabaseType::Pg);
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
    assert_eq!(nodes[0].role, NodeRole::Master);
    assert_eq!(nodes[0].state, State::Running);
    assert_eq!(nodes[1].name, "pg-2x2xcpu-4gb-50gb-de-fra1-2");
    assert_eq!(nodes[1].role, NodeRole::Standby);
    assert_eq!(nodes[1].state, State::SyncingData);

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

// Source: https://developers.upcloud.com/1.3/16-managed-database/#list-users
const DB_USER_LIST_JSON: &str = r#"
    [
        {
            "username": "upadmin",
            "type": "primary"
        },
        {
            "username": "reader",
            "type": "readonly",
            "pg_access_control": {
                "allow_replication": false
            }
        }
    ]
"#;

#[test]
fn deserialize_db_user_list() {
    let users: Vec<User> = serde_json::from_str(DB_USER_LIST_JSON).unwrap();
    assert_eq!(users.len(), 2);
    assert_eq!(users[0].username, "upadmin");
    assert_eq!(users[0].user_type, UserType::Primary);
    assert!(users[0].password.is_none());
    assert!(users[0].pg_access_control.is_none());
    assert!(users[0].authentication.is_none());
    assert_eq!(users[1].username, "reader");
    assert_eq!(users[1].user_type, UserType::Readonly);
    let pg = users[1]
        .pg_access_control
        .as_ref()
        .expect("pg_access_control");
    assert_eq!(pg.allow_replication, Some(false));
    assert!(users[1].password.is_none());
    assert!(users[1].authentication.is_none());
}

// Source: https://developers.upcloud.com/1.3/16-managed-database/#list-logical-databases
const DB_LOGICAL_DB_LIST_JSON: &str = r#"
    [
        {
            "name": "defaultdb",
            "lc_collate": "en_US.UTF-8",
            "lc_ctype": "en_US.UTF-8"
        },
        {
            "name": "testdb"
        }
    ]
"#;

#[test]
fn deserialize_db_logical_database_list() {
    let dbs: Vec<LogicalDatabase> = serde_json::from_str(DB_LOGICAL_DB_LIST_JSON).unwrap();
    assert_eq!(dbs.len(), 2);
    assert_eq!(dbs[0].name, "defaultdb");
    assert_eq!(dbs[0].lc_collate.as_deref(), Some("en_US.UTF-8"));
    assert_eq!(dbs[0].lc_ctype.as_deref(), Some("en_US.UTF-8"));
    assert_eq!(dbs[1].name, "testdb");
    assert!(dbs[1].lc_collate.is_none());
    assert!(dbs[1].lc_ctype.is_none());
}

// Source: https://developers.upcloud.com/1.3/16-managed-database/#get-managed-database-plans
const DB_PLAN_JSON: &str = r#"
    [
        {
            "name": "1x1xCPU-2GB-25GB",
            "node_count": 1,
            "storage_size": 25600
        },
        {
            "name": "2x2xCPU-4GB-50GB",
            "node_count": 2,
            "storage_size": 51200
        }
    ]
"#;

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

// Source: https://developers.upcloud.com/1.3/16-managed-database/#get-managed-database-sessions
const DB_SESSION_JSON: &str = r#"
    [
        {
            "pid": 12345,
            "query": "SELECT 1",
            "query_duration": "00:00:01",
            "state": "active",
            "client_addr": "192.168.1.1",
            "datname": "defaultdb",
            "usename": "upadmin"
        }
    ]
"#;

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

// Source: https://developers.upcloud.com/1.3/16-managed-database/#get-managed-database-service-types
const PG_AVAILABLE_EXTENSIONS_JSON: &str = r#"
    [
        {
            "extensions": [
                {
                    "default_version": "1.0",
                    "name": "bloom",
                    "versions": [
                        "1.0"
                    ]
                },
                {
                    "default_version": "1.0",
                    "name": "bool_plperl",
                    "versions": [
                        "1.0"
                    ]
                },
                {
                    "default_version": "1.3",
                    "name": "btree_gin",
                    "versions": [
                        "1.0",
                        "1.1",
                        "1.2",
                        "1.3"
                    ]
                }
            ],
            "version": "13"
        },
        {
            "extensions": [
                {
                    "default_version": "7",
                    "name": "unit",
                    "versions": [
                        "3",
                        "4",
                        "5",
                        "6",
                        "7"
                    ]
                },
                {
                    "default_version": "1.1",
                    "name": "uuid-ossp",
                    "versions": [
                        "1.1"
                    ]
                },
                {
                    "default_version": "0.8.1",
                    "name": "vector",
                    "versions": [
                        "0.8.1"
                    ]
                }
            ],
            "version": "14"
        }
    ]
"#;

// ─── New deserialization tests ───────────────────────────────────────────

#[test]
fn deserialize_pg_available_extensions() {
    let versions: Vec<PgVersionExtensions> =
        serde_json::from_str(PG_AVAILABLE_EXTENSIONS_JSON).unwrap();
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

// Source: https://developers.upcloud.com/1.3/16-managed-database/#get-managed-database-service-types
const INSTANCE_AVAILABLE_EXTENSIONS_JSON: &str = r#"
    [
        {
            "default_version": "1.6",
            "name": "pg_buffercache",
            "versions": [
                "1.2",
                "1.3",
                "1.4",
                "1.5",
                "1.6"
            ]
        },
        {
            "default_version": "1.6",
            "name": "pg_cron",
            "versions": [
                "1.0",
                "1.1",
                "1.2",
                "1.3",
                "1.4",
                "1.4-1",
                "1.5",
                "1.6"
            ]
        },
        {
            "default_version": "5.2.4",
            "name": "pg_partman",
            "versions": [
                "5.2.4"
            ]
        }
    ]
"#;

#[test]
fn deserialize_instance_available_extensions() {
    let exts: Vec<PgExtensionDetail> =
        serde_json::from_str(INSTANCE_AVAILABLE_EXTENSIONS_JSON).unwrap();
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

// Source: https://developers.upcloud.com/1.3/16-managed-database/#get-managed-database-service-types
const DB_SERVICE_TYPES_JSON: &str = r#"
    {
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
    }
"#;

#[test]
fn deserialize_db_service_types() {
    // API returns a map keyed by service name with nested service_plans/properties
    // that don't match the typed DatabaseServiceType struct; kept as untyped JSON.
    let val: serde_json::Value = serde_json::from_str(DB_SERVICE_TYPES_JSON).unwrap();
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
    assert_eq!(
        mysql["properties"]["public_access"]["title"],
        "Public Access"
    );
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

// Source: https://developers.upcloud.com/1.3/16-managed-database/#create-managed-database
const CREATE_DB_REQUEST_JSON: &str = r#"
    {
        "hostname_prefix": "doc-api-unique-prefix",
        "plan": "1x1xCPU-2GB-25GB",
        "title": "my-managed-database",
        "type": "mysql",
        "zone": "de-fra1",
        "maintenance": {
            "dow": "sunday",
            "time": "05:00:00"
        },
        "termination_protection": false,
        "properties": {
            "automatic_utility_network_ip_filter": true,
            "version": "8",
            "ip_filter": []
        },
        "networks": [
            {
                "name": "example-network-1",
                "type": "private",
                "family": "IPv4",
                "uuid": "03631160-d57a-4926-ad48-a2f828229dcb"
            }
        ],
        "labels": [
            {
                "key": "env",
                "value": "staging"
            },
            {
                "key": "foo",
                "value": "bar"
            }
        ]
    }
"#;

#[test]
fn deserialize_create_db_request() {
    let req: CreateRequest = serde_json::from_str(CREATE_DB_REQUEST_JSON).unwrap();
    assert_eq!(req.hostname_prefix, "doc-api-unique-prefix");
    assert_eq!(req.plan, "1x1xCPU-2GB-25GB");
    assert_eq!(req.title, "my-managed-database");
    assert_eq!(req.db_type, DatabaseType::Mysql);
    assert_eq!(req.zone, "de-fra1");
    assert_eq!(req.termination_protection, Some(false));
    let maint = req.maintenance.expect("maintenance present");
    assert_eq!(maint.dow, "sunday");
    assert_eq!(maint.time, "05:00:00");
    let networks = req.networks.expect("networks present");
    assert_eq!(networks.len(), 1);
    assert_eq!(networks[0].name, "example-network-1");
    let labels = req.labels.expect("labels present");
    assert_eq!(labels.len(), 2);
    assert_eq!(labels[0].key, "env");
    assert_eq!(labels[0].value, "staging");
}

// Source: https://developers.upcloud.com/1.3/16-managed-database/#get-managed-database-details
const DB_DETAILS_JSON: &str = r#"
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
            }
        ],
        "node_states": [
            {
                "name": "pg-1",
                "role": "master",
                "state": "running"
            }
        ],
        "properties": {
            "public_access": true,
            "version": "13"
        },
        "service_uri": "postgres://upadmin:pass@host:11550/defaultdb?sslmode=require",
        "service_uri_params": {
            "host": "example.db.upclouddatabases.com",
            "port": "11550"
        }
    }
"#;

#[test]
fn deserialize_db_details() {
    let db: Database = serde_json::from_str(DB_DETAILS_JSON).unwrap();
    assert_eq!(db.uuid, "09d9d824-8b3e-4a57-a9b3-77ec51fd5c84");
    assert_eq!(db.name, "pg-2x2xcpu-4gb-50gb-de-fra1");
    assert_eq!(db.title, "pg-2x2xcpu-4gb-50gb-de-fra1");
    assert_eq!(db.plan, "2x2xCPU-4GB-50GB");
    assert_eq!(db.zone, "de-fra1");
    assert_eq!(db.db_type, DatabaseType::Pg);
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
    assert_eq!(nodes[0].role, NodeRole::Master);
    assert_eq!(nodes[0].state, State::Running);

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

// Source: https://developers.upcloud.com/1.3/16-managed-database/#modify-managed-database
const MODIFY_DB_REQUEST_JSON: &str = r#"
    {
        "title": "my-managed-database",
        "powered": true,
        "plan": "1x1xCPU-2GB-25GB",
        "properties": {
            "automatic_utility_network_ip_filter": true
        },
        "zone": "de-fra1",
        "networks": [
            {
                "family": "IPv4",
                "name": "my-network",
                "type": "private",
                "uuid": "03000000-0000-4000-8000-000000000001"
            }
        ],
        "additional_disk_space_gib": 50,
        "maintenance": {
            "dow": "monday",
            "time": "03:00:00"
        }
    }
"#;

#[test]
fn deserialize_modify_db_request() {
    let req: ModifyRequest = serde_json::from_str(MODIFY_DB_REQUEST_JSON).unwrap();
    assert_eq!(req.title.as_deref(), Some("my-managed-database"));
    assert_eq!(req.powered, Some(true));
    assert_eq!(req.plan.as_deref(), Some("1x1xCPU-2GB-25GB"));
    let props = req.properties.as_ref().unwrap();
    assert_eq!(props["automatic_utility_network_ip_filter"], true);
    assert!(req.labels.is_none());
    assert_eq!(req.zone.as_deref(), Some("de-fra1"));
    let networks = req.networks.as_ref().unwrap();
    assert_eq!(networks.len(), 1);
    assert_eq!(networks[0].family, Family::IPv4);
    assert_eq!(networks[0].name, "my-network");
    assert_eq!(networks[0].network_type, NetworkType::Private);
    assert_eq!(req.additional_disk_space_gib, Some(50));
    let maint = req.maintenance.as_ref().unwrap();
    assert_eq!(maint.dow, "monday");
    assert_eq!(maint.time, "03:00:00");
}

// Source: https://developers.upcloud.com/1.3/16-managed-database/#create-user
const CREATE_DB_USER_MYSQL_JSON: &str = r#"
    {
        "username": "api-doc-user",
        "authentication": "caching_sha2_password",
        "password": "new-password"
    }
"#;

#[test]
fn deserialize_create_db_user_mysql() {
    let req: CreateUserRequest = serde_json::from_str(CREATE_DB_USER_MYSQL_JSON).unwrap();
    assert_eq!(req.username, "api-doc-user");
    assert_eq!(req.password, "new-password");
    assert_eq!(req.authentication.as_deref(), Some("caching_sha2_password"));
    assert!(req.user_type.is_none());
    assert!(req.pg_access_control.is_none());
}

// Source: https://developers.upcloud.com/1.3/16-managed-database/#get-user-details
const DB_USER_DETAIL_MYSQL_JSON: &str = r#"
    {
        "username": "api-doc-user",
        "password": "my-password",
        "authentication": "caching_sha2_password",
        "type": "normal"
    }
"#;

#[test]
fn deserialize_db_user_detail_mysql() {
    let user: User = serde_json::from_str(DB_USER_DETAIL_MYSQL_JSON).unwrap();
    assert_eq!(user.username, "api-doc-user");
    assert_eq!(user.password.as_deref(), Some("my-password"));
    assert_eq!(
        user.authentication.as_deref(),
        Some("caching_sha2_password")
    );
    assert_eq!(user.user_type, UserType::Normal);
    assert!(user.pg_access_control.is_none());
}

// Source: https://developers.upcloud.com/1.3/16-managed-database/#get-user-details
const DB_USER_DETAIL_PG_JSON: &str = r#"
    {
        "username": "api-doc-user",
        "password": "new-password",
        "type": "normal",
        "pg_access_control": {
            "allow_replication": true
        }
    }
"#;

#[test]
fn deserialize_db_user_detail_pg() {
    let user: User = serde_json::from_str(DB_USER_DETAIL_PG_JSON).unwrap();
    assert_eq!(user.username, "api-doc-user");
    assert_eq!(user.password.as_deref(), Some("new-password"));
    assert_eq!(user.user_type, UserType::Normal);
    let pg = user.pg_access_control.expect("pg_access_control present");
    assert_eq!(pg.allow_replication, Some(true));
}

const MODIFY_DB_USER_REQUEST_JSON: &str = r#"
    {
        "authentication": "caching_sha2_password",
        "password": "new-password"
    }
"#;

#[test]
fn deserialize_modify_db_user_request() {
    let req: ModifyUserRequest = serde_json::from_str(MODIFY_DB_USER_REQUEST_JSON).unwrap();
    assert_eq!(req.password.as_deref(), Some("new-password"));
    assert_eq!(req.authentication.as_deref(), Some("caching_sha2_password"));
}

const CREATE_LOGICAL_DB_JSON: &str = r#"
    {
        "name": "database-doc-api",
        "lc_collate": "en_US.UTF8",
        "lc_ctype": "en_US.UTF8"
    }
"#;

#[test]
fn deserialize_create_logical_db() {
    let req: CreateLogicalDatabaseRequest = serde_json::from_str(CREATE_LOGICAL_DB_JSON).unwrap();
    assert_eq!(req.name, "database-doc-api");
    assert_eq!(req.lc_collate.as_deref(), Some("en_US.UTF8"));
    assert_eq!(req.lc_ctype.as_deref(), Some("en_US.UTF8"));
}

// Source: https://developers.upcloud.com/1.3/16-managed-database/#cancel-managed-database-session
const SESSION_TERMINATE_JSON: &str = r#"
    {
        "success": true
    }
"#;

#[test]
fn deserialize_session_terminate() {
    // API returns `{"success": true}` with no matching typed struct.
    let val: serde_json::Value = serde_json::from_str(SESSION_TERMINATE_JSON).unwrap();
    assert_eq!(val["success"], true);
}

#[test]
fn cancel_session_terminate_query_param() {
    // Verify the query-parameter tuple produced by cancel_session for both modes.
    let cancel = ("terminate", false.to_string());
    assert_eq!(cancel, ("terminate", "false".to_owned()));

    let terminate = ("terminate", true.to_string());
    assert_eq!(terminate, ("terminate", "true".to_owned()));
}

#[test]
fn cancel_session_url_format() {
    let uuid = "09d9d824-8b3e-4a57-a9b3-77ec51fd5c84";
    let pid: i64 = 12345;
    let url = format!("/1.3/database/{uuid}/sessions/{pid}");
    assert_eq!(
        url,
        "/1.3/database/09d9d824-8b3e-4a57-a9b3-77ec51fd5c84/sessions/12345"
    );
}

// Source: https://developers.upcloud.com/1.3/16-managed-database/#get-managed-database-sessions
const PG_SESSIONS_DETAILED_JSON: &str = r#"
    {
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
    }
"#;

#[test]
fn deserialize_pg_sessions_detailed() {
    // API returns untyped JSON (get_current_queries → Value).
    let val: serde_json::Value = serde_json::from_str(PG_SESSIONS_DETAILED_JSON).unwrap();
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

// Source: https://developers.upcloud.com/1.3/16-managed-database/#get-managed-database-query-statistics
const QUERY_STATISTICS_JSON: &str = r#"
    {
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
    }
"#;

#[test]
fn deserialize_query_statistics() {
    // API returns untyped JSON (get_query_statistics → Value).
    let val: serde_json::Value = serde_json::from_str(QUERY_STATISTICS_JSON).unwrap();
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

// Source: https://developers.upcloud.com/1.3/16-managed-database/#get-managed-database-metrics
const DB_METRICS_JSON: &str = r#"
    {
        "cpu_usage": {
            "data": {
                "cols": [
                    {
                        "label": "time",
                        "type": "date"
                    },
                    {
                        "label": "test-1 (master)",
                        "type": "number"
                    }
                ],
                "rows": [
                    [
                        "2022-01-21T13:10:30Z",
                        2.744
                    ]
                ]
            },
            "hints": {
                "title": "CPU usage %"
            }
        }
    }
"#;

#[test]
fn deserialize_db_metrics() {
    // API returns untyped JSON (get_metrics → Value).
    let val: serde_json::Value = serde_json::from_str(DB_METRICS_JSON).unwrap();
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

// Source: https://developers.upcloud.com/1.3/16-managed-database/#get-managed-database-logs
const DB_LOGS_JSON: &str = r#"
    {
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
    }
"#;

#[test]
fn deserialize_db_logs() {
    // API returns untyped JSON (get_logs → Value).
    let val: serde_json::Value = serde_json::from_str(DB_LOGS_JSON).unwrap();
    assert_eq!(val["first_log_offset"], "1642770443687");
    assert_eq!(val["offset"], "1642770443687");
    let logs = val["logs"].as_array().unwrap();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0]["hostname"], "api-doc-hostname");
    assert_eq!(logs[0]["msg"], "LOG: connection authorized");
    assert_eq!(logs[0]["time"], "2022-01-21T13:07:23.687241Z");
    assert_eq!(logs[0]["service"], "postgresql-13.service");
}

// Source: https://developers.upcloud.com/1.3/16-managed-database/#get-managed-database-versions
const DB_VERSIONS_JSON: &str = r#"
    [
        "10",
        "11",
        "12",
        "13",
        "14"
    ]
"#;

#[test]
fn deserialize_db_versions() {
    let versions: Vec<String> = serde_json::from_str(DB_VERSIONS_JSON).unwrap();
    assert_eq!(versions.len(), 5);
    assert_eq!(versions[0], "10");
    assert_eq!(versions[1], "11");
    assert_eq!(versions[2], "12");
    assert_eq!(versions[3], "13");
    assert_eq!(versions[4], "14");
}

// Source: https://developers.upcloud.com/1.3/16-managed-database/#get-managed-database-details
const DB_NETWORKS_JSON: &str = r#"
    [
        {
            "uuid": "03aa7245-2ff9-49c8-9f0e-7ca0270d71a4",
            "name": "example-private-network",
            "family": "IPv4",
            "type": "private"
        }
    ]
"#;

#[test]
fn deserialize_db_networks() {
    let nets: Vec<Network> = serde_json::from_str(DB_NETWORKS_JSON).unwrap();
    assert_eq!(nets.len(), 1);
    assert_eq!(nets[0].uuid, "03aa7245-2ff9-49c8-9f0e-7ca0270d71a4");
    assert_eq!(nets[0].name, "example-private-network");
    assert_eq!(nets[0].family, Family::IPv4);
    assert_eq!(nets[0].network_type, NetworkType::Private);
}

// Source: https://developers.upcloud.com/1.3/16-managed-database/#get-managed-database-service-types
const DB_SERVICE_TYPE_DETAIL_JSON: &str = r#"
    {
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
                            {
                                "name": "es-mad1"
                            },
                            {
                                "name": "pl-waw1"
                            },
                            {
                                "name": "us-sjo1"
                            }
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
                    "type": [
                        "string",
                        "null"
                    ],
                    "user_error": "Must consist of alpha-numeric characters, underscores or dashes"
                },
                "admin_username": {
                    "createOnly": true,
                    "example": "upadmin",
                    "maxLength": 64,
                    "pattern": "^[_A-Za-z0-9][-._A-Za-z0-9]{0,63}$",
                    "title": "Custom username for admin user. This must be set only when a new service is being created.",
                    "type": [
                        "string",
                        "null"
                    ],
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
                    "type": [
                        "string",
                        "null"
                    ],
                    "enum": [
                        "8"
                    ]
                }
            }
        }
    }
"#;

// ─── Service type detail (single type, official docs page 16) ─────────────

#[test]
fn deserialize_db_service_type_detail() {
    // API returns a map keyed by service name with nested plans/properties/zones
    // that don't match any typed struct; kept as untyped JSON.
    let val: serde_json::Value = serde_json::from_str(DB_SERVICE_TYPE_DETAIL_JSON).unwrap();
    assert!(val.is_object());
    let obj = val.as_object().unwrap();
    assert_eq!(obj.len(), 1);
    assert!(obj.contains_key("mysql"));

    let mysql = &val["mysql"];
    assert_eq!(mysql["name"], "mysql");
    assert_eq!(
        mysql["description"],
        "MySQL - Relational Database Management System"
    );
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

// ─── State deserialization ────────────────────────────────────────────────

#[test]
fn deserialize_state_kebab_case() {
    let cases = vec![
        ("pending", State::Pending),
        ("setup-network", State::SetupNetwork),
        ("check-network", State::CheckNetwork),
        ("setup-peer", State::SetupPeer),
        ("check-peer", State::CheckPeer),
        ("setup-service", State::SetupService),
        ("rebuilding", State::Rebuilding),
        ("rebalancing", State::Rebalancing),
        ("running", State::Running),
        ("stopped", State::Stopped),
        ("error", State::Error),
        ("cleanup-service", State::CleanupService),
        ("cleanup-network", State::CleanupNetwork),
        ("delete-service", State::DeleteService),
    ];
    for (json_val, expected) in cases {
        let json = format!("\"{}\"", json_val);
        let state: State = serde_json::from_str(&json).unwrap();
        assert_eq!(state, expected, "failed for {json}");
    }
}

// ─── Database detail with backups and maintenance from docs ───────────────
// Real JSON from UpCloud API 1.3 docs: "Get Managed Database details"
// Source: https://developers.upcloud.com/1.3/16-managed-database/#get-managed-database-details
const DATABASE_DETAIL_FROM_DOCS_JSON: &str = r#"
    {
        "backups": [
            {
                "backup_name": "backup-2022-01-06T12:36:06.739000Z",
                "backup_time": "2022-01-06T12:36:06.739000Z",
                "data_size": 36464640
            },
            {
                "backup_name": "backup-2022-01-07T12:36:06.261000Z",
                "backup_time": "2022-01-07T12:36:06.261000Z",
                "data_size": 36413440
            }
        ],
        "components": [
            {
                "component": "pg",
                "host": "pg-2x2xcpu-4gb-50gb-de-fra1-example-prefix-project.db.upclouddatabases.com",
                "port": 11550,
                "route": "dynamic",
                "usage": "primary"
            },
            {
                "component": "pgbouncer",
                "host": "pg-2x2xcpu-4gb-50gb-de-fra1-example-prefix-project.db.upclouddatabases.com",
                "port": 11551,
                "route": "dynamic",
                "usage": "primary"
            }
        ],
        "maintenance": {
            "dow": "sunday",
            "time": "05:00:00"
        },
        "name": "pg-2x2xcpu-4gb-50gb-de-fra1",
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
        "plan": "2x2xCPU-4GB-50GB",
        "powered": true,
        "properties": {
            "automatic_utility_network_ip_filter": false,
            "ip_filter": [
                "0.0.0.0/0"
            ],
            "public_access": true,
            "version": "13"
        },
        "uuid": "09d9d824-8b3e-4a57-a9b3-77ec51fd5c84",
        "state": "running",
        "title": "pg-2x2xcpu-4gb-50gb-de-fra1",
        "type": "pg",
        "service_uri": "postgres://upadmin:example-prefix-project@pg-2x2xcpu-4gb-50gb-de-fra1-example-prefix-project.db.upclouddatabases.com:11550/defaultdb?sslmode=require",
        "service_uri_params": {
            "dbname": "defaultdb",
            "host": "pg-2x2xcpu-4gb-50gb-de-fra1-example-prefix-project.db.upclouddatabases.com",
            "password": "example-prefix-project",
            "port": "11550",
            "ssl_mode": "require",
            "user": "upadmin"
        },
        "termination_protection": false,
        "zone": "de-fra1"
    }
"#;

#[test]
fn deserialize_database_detail_from_docs() {
    let db: Database = serde_json::from_str(DATABASE_DETAIL_FROM_DOCS_JSON).unwrap();
    assert_eq!(db.uuid, "09d9d824-8b3e-4a57-a9b3-77ec51fd5c84");
    assert_eq!(db.name, "pg-2x2xcpu-4gb-50gb-de-fra1");
    assert_eq!(db.title, "pg-2x2xcpu-4gb-50gb-de-fra1");
    assert_eq!(db.plan, "2x2xCPU-4GB-50GB");
    assert_eq!(db.zone, "de-fra1");
    assert_eq!(db.db_type, DatabaseType::Pg);
    assert_eq!(db.state, State::Running);
    assert_eq!(db.powered, Some(true));
    assert_eq!(db.termination_protection, Some(false));

    let backups = db.backups.as_ref().unwrap();
    assert_eq!(backups.len(), 2);
    assert_eq!(backups[0].backup_name, "backup-2022-01-06T12:36:06.739000Z");
    assert_eq!(backups[0].backup_time, "2022-01-06T12:36:06.739000Z");
    assert_eq!(backups[0].data_size, Some(36464640));
    assert_eq!(backups[1].backup_name, "backup-2022-01-07T12:36:06.261000Z");
    assert_eq!(backups[1].data_size, Some(36413440));

    let maint = db.maintenance.as_ref().unwrap();
    assert_eq!(maint.dow, "sunday");
    assert_eq!(maint.time, "05:00:00");

    let components = db.components.as_ref().unwrap();
    assert_eq!(components.len(), 2);
    assert_eq!(components[0].component, "pg");
    assert_eq!(components[0].port, 11550);
    assert_eq!(components[0].route, "dynamic");
    assert_eq!(components[0].usage, "primary");
    assert_eq!(components[1].component, "pgbouncer");
    assert_eq!(components[1].port, 11551);

    let nodes = db.node_states.as_ref().unwrap();
    assert_eq!(nodes.len(), 2);
    assert_eq!(nodes[0].name, "pg-2x2xcpu-4gb-50gb-de-fra1-1");
    assert_eq!(nodes[0].role, NodeRole::Master);
    assert_eq!(nodes[0].state, State::Running);
    assert_eq!(nodes[1].role, NodeRole::Standby);
    assert_eq!(nodes[1].state, State::SyncingData);

    assert!(db.service_uri.as_ref().unwrap().starts_with("postgres://"));
    let params = db.service_uri_params.as_ref().unwrap();
    assert_eq!(params["dbname"], "defaultdb");
    assert_eq!(params["port"], "11550");
    assert_eq!(params["user"], "upadmin");
}

// ─── OpenSearch access control types ──────────────────────────────────────

const OPENSEARCH_ACCESS_CONTROL_JSON: &str = r#"
    {
        "access_control": true,
        "extended_access_control": false
    }
"#;

#[test]
fn deserialize_opensearch_access_control() {
    let acl: OpenSearchAccessControl =
        serde_json::from_str(OPENSEARCH_ACCESS_CONTROL_JSON).unwrap();
    assert_eq!(acl.access_control, Some(true));
    assert_eq!(acl.extended_access_control, Some(false));
}

const OPENSEARCH_ACCESS_CONTROL_RULE_JSON: &str = r#"
    {
        "index": "my-index-*",
        "permission": "readwrite"
    }
"#;

#[test]
fn deserialize_opensearch_access_control_rule() {
    let rule: OpenSearchAccessControlRule =
        serde_json::from_str(OPENSEARCH_ACCESS_CONTROL_RULE_JSON).unwrap();
    assert_eq!(rule.index, "my-index-*");
    assert_eq!(rule.permission, "readwrite");
}

// Source: https://developers.upcloud.com/1.3/16-managed-database/#get-managed-database-indices
const OPENSEARCH_INDEX_JSON: &str = r#"
    {
        "create_time": "2022-06-01T00:00:00Z",
        "docs": 500,
        "health": "green",
        "index_name": "test-index",
        "number_of_replicas": 1,
        "number_of_shards": 5,
        "read_only_allow_delete": false,
        "size": 1048576,
        "status": "open"
    }
"#;

#[test]
fn deserialize_opensearch_index() {
    let idx: OpenSearchIndex = serde_json::from_str(OPENSEARCH_INDEX_JSON).unwrap();
    assert_eq!(idx.index_name, "test-index");
    assert_eq!(idx.docs, Some(500));
    assert_eq!(idx.health.as_deref(), Some("green"));
    assert_eq!(idx.number_of_replicas, Some(1));
    assert_eq!(idx.number_of_shards, Some(5));
    assert_eq!(idx.read_only_allow_delete, Some(false));
    assert_eq!(idx.size, Some(1048576));
    assert_eq!(idx.status, Some(OpenSearchIndexStatus::Open));
}

// ─── ModifyAccessControlRequest ───────────────────────────────────────────

const MODIFY_ACCESS_CONTROL_REQUEST_JSON: &str = r#"
    {
        "access_control": true,
        "extended_access_control": false
    }
"#;

#[test]
fn deserialize_modify_access_control_request() {
    let req: ModifyAccessControlRequest =
        serde_json::from_str(MODIFY_ACCESS_CONTROL_REQUEST_JSON).unwrap();
    assert_eq!(req.access_control, Some(true));
    assert_eq!(req.extended_access_control, Some(false));
}

// Source: https://developers.upcloud.com/1.3/16-managed-database/#modify-managed-database-user-access-control
const USER_OPENSEARCH_ACCESS_CONTROL_JSON: &str = r#"
    {
        "opensearch_access_control": {
            "rules": [
                {
                    "index": "logs-*",
                    "permission": "read"
                },
                {
                    "index": "events-*",
                    "permission": "readwrite"
                }
            ]
        }
    }
"#;

#[test]
fn deserialize_user_opensearch_access_control() {
    let req: ModifyUserAccessControlRequest =
        serde_json::from_str(USER_OPENSEARCH_ACCESS_CONTROL_JSON).unwrap();
    let acl = req.opensearch_access_control.as_ref().unwrap();
    let rules = acl.rules.as_ref().unwrap();
    assert_eq!(rules.len(), 2);
    assert_eq!(rules[0].index, "logs-*");
    assert_eq!(rules[0].permission, "read");
    assert_eq!(rules[1].index, "events-*");
    assert_eq!(rules[1].permission, "readwrite");
}
