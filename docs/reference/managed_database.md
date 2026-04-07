# Managed Databases

> Source: https://developers.upcloud.com/1.3/16-managed-database

A Managed Database is a database-as-a-service which lets customers use a database without the need to set up a server or the database software. Supported types: **PostgreSQL** (`pg`), **MySQL** (`mysql`), **Valkey** (`valkey`), and **OpenSearch** (`opensearch`).

## Managed Database States

| State | Description |
|-------|-------------|
| `pending` | Newly created service or started reconfiguration. |
| `setup-network` | Configuring network. |
| `check-network` | Checking network configuration. |
| `setup-peer` | Configuring SDN network peerings. |
| `setup-service` | Service is being created or reconfigured. |
| `check-peer` | Checking SDN network peerings. |
| `rebuilding` | Service creation in progress. |
| `rebalancing` | Service is being upgraded or migrated. |
| `error` | Service encountered an error requiring user action. |
| `running` | Service is up and running. |
| `stopped` | Service is stopped. |
| `cleanup-service` | Cleaning up service resources. |
| `cleanup-network` | Cleaning up network resources. |
| `delete-service` | Deleting the service. |

---

## Service Types & Plans

### List Managed Database Types

Returns a list of available database types with plans, zones, versions, and configuration properties.

#### Request

```
GET /1.3/database/service-types HTTP/1.1
```

#### Response

```http
HTTP/1.1 200 OK
```

```json
{
  "mysql": {
    "name": "mysql",
    "description": "MySQL - Relational Database Management System",
    "latest_available_version": "8.0.26",
    "service_plans": [
      {
        "backup_config_mysql": {
          "interval": 24,
          "max_count": 2,
          "recovery_mode": "pitr"
        },
        "node_count": 1,
        "zones": {
          "zone": [
            { "name": "es-mad1" },
            { "name": "de-fra1" }
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
        "type": ["string", "null"],
        "maxLength": 256,
        "minLength": 8
      },
      "public_access": {
        "default": false,
        "type": "boolean"
      },
      "version": {
        "type": ["string", "null"],
        "enum": ["8"]
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
```

#### Response Attributes

| Attribute | Accepted value | Description |
|-----------|---------------|-------------|
| `name` | string | Database type identifier. |
| `description` | string | Human-readable description. |
| `latest_available_version` | string | Latest available software version. |
| `service_plans` | array | Available plans with node counts, zones, and backup configs. |
| `service_plans[].plan` | string | Plan identifier. |
| `service_plans[].node_count` | integer | Number of nodes in the plan. |
| `service_plans[].core_number` | integer | Number of CPU cores. |
| `service_plans[].storage_size` | integer | Storage size in MB. |
| `service_plans[].memory_amount` | integer | Memory in MB. |
| `service_plans[].zones` | object | Available zones for this plan. |
| `properties` | object | Database type-specific configuration properties. |

---

### Get Managed Database Type Details

Returns database type details by given `{database_type}`.

#### Request

```
GET /1.3/database/service-types/{database_type} HTTP/1.1
```

#### Response

```http
HTTP/1.1 200 OK
```

```json
{
  "mysql": {
    "name": "mysql",
    "description": "MySQL - Relational Database Management System",
    "latest_available_version": "8.0.26",
    "service_plans": [],
    "properties": {}
  }
}
```

Response format is identical to a single entry from [List Managed Database Types](#list-managed-database-types).

---

### List All PostgreSQL Extensions by Version

Returns all available extensions for each supported PostgreSQL version.

#### Request

```
GET /1.3/database/pg/available-extensions HTTP/1.1
```

#### Response

```http
HTTP/1.1 200 OK
```

```json
[
  {
    "extensions": [
      {
        "default_version": "1.0",
        "name": "bloom",
        "versions": ["1.0"]
      },
      {
        "default_version": "1.3",
        "name": "btree_gin",
        "versions": ["1.0", "1.1", "1.2", "1.3"]
      }
    ],
    "version": "13"
  },
  {
    "extensions": [
      {
        "default_version": "0.8.1",
        "name": "vector",
        "versions": ["0.8.1"]
      }
    ],
    "version": "14"
  }
]
```

#### Response Attributes

| Attribute | Accepted value | Description |
|-----------|---------------|-------------|
| `version` | string | PostgreSQL major version number. |
| `extensions` | array | List of available extensions for this version. |
| `extensions[].name` | string | Name of the extension. |
| `extensions[].default_version` | string | Default version installed when enabling the extension. |
| `extensions[].versions` | array | All available versions of the extension. |

---

### List Extensions for a Database Instance

Returns available extensions for a specific PostgreSQL database instance.

#### Request

```
GET /1.3/database/{uuid}/available-extensions HTTP/1.1
```

#### Response

```http
HTTP/1.1 200 OK
```

```json
[
  {
    "default_version": "1.6",
    "name": "pg_buffercache",
    "versions": ["1.2", "1.3", "1.4", "1.5", "1.6"]
  },
  {
    "default_version": "1.6",
    "name": "pg_cron",
    "versions": ["1.0", "1.1", "1.2", "1.3", "1.4", "1.5", "1.6"]
  }
]
```

#### Response Attributes

| Attribute | Accepted value | Description |
|-----------|---------------|-------------|
| `name` | string | Name of the extension. |
| `default_version` | string | Default version installed when enabling the extension. |
| `versions` | array | All available versions of the extension. |

---

## Service CRUD

### List Managed Databases

Returns a list of all available Managed Database services. Supports label-based filtering.

#### Request

```
GET /1.3/database HTTP/1.1
```

#### Query Parameters

| Attribute | Accepted value | Default value | Required | Description |
|-----------|---------------|---------------|----------|-------------|
| `limit` | 0–100 | 10 | no | Number of entries to receive at most. |
| `offset` | >= 0 | 0 | no | Offset for retrieved results. |
| `label` | string | — | no | Filter by label (e.g. `label=env%3Dstaging`, `label=env`). |

#### Response

```http
HTTP/1.1 200 OK
```

```json
[
  {
    "backups": [
      {
        "backup_name": "backup-2022-01-06T12:36:06.739000Z",
        "backup_time": "2022-01-06T12:36:06.739000Z",
        "data_size": 36464640
      }
    ],
    "components": [
      {
        "component": "pg",
        "host": "pg-2x2xcpu-4gb-50gb-de-fra1-example.db.upclouddatabases.com",
        "port": 11550,
        "route": "dynamic",
        "usage": "primary"
      }
    ],
    "connection_pools": [],
    "create_time": "2022-01-04T09:36:14Z",
    "maintenance": {
      "dow": "sunday",
      "time": "05:00:00",
      "pending_updates": []
    },
    "name": "pg-2x2xcpu-4gb-50gb-de-fra1",
    "node_count": 2,
    "node_states": [
      {
        "name": "pg-2x2xcpu-4gb-50gb-de-fra1-1",
        "role": "master",
        "state": "running",
        "progress_updates": []
      }
    ],
    "plan": "2x2xCPU-4GB-50GB",
    "additional_disk_space_gib": 100,
    "powered": true,
    "metadata": {
      "max_connections": 200,
      "pg_version": "13.5",
      "write_block_threshold_exceeded": false
    },
    "properties": {
      "automatic_utility_network_ip_filter": false,
      "ip_filter": ["0.0.0.0/0"],
      "public_access": true,
      "version": "13"
    },
    "uuid": "09d9d824-8b3e-4a57-a9b3-77ec51fd5c84",
    "state": "running",
    "title": "pg-2x2xcpu-4gb-50gb-de-fra1",
    "type": "pg",
    "update_time": "2022-01-04T09:36:24Z",
    "service_uri": "postgres://upadmin:pass@host:11550/defaultdb?sslmode=require",
    "service_uri_params": {
      "dbname": "defaultdb",
      "host": "host.db.upclouddatabases.com",
      "password": "pass",
      "port": "11550",
      "ssl_mode": "require",
      "user": "upadmin"
    },
    "users": [
      { "username": "upadmin", "type": "primary" }
    ],
    "termination_protection": false,
    "zone": "de-fra1"
  }
]
```

---

### Get Managed Database Details

Returns details of a Managed Database service by its `{uuid}`.

#### Request

```
GET /1.3/database/{uuid} HTTP/1.1
```

#### Response

```http
HTTP/1.1 200 OK
```

Response body is a single service object with the same structure as entries in [List Managed Databases](#list-managed-databases).

#### Response Attributes

| Attribute | Accepted value | Description |
|-----------|---------------|-------------|
| `uuid` | UUID string | Unique identifier of the service. |
| `type` | `pg` / `mysql` / `valkey` / `opensearch` | Database type. |
| `title` | string | Service name. |
| `name` | string | Auto-generated service name. |
| `plan` | string | Current plan identifier. |
| `additional_disk_space_gib` | integer | Additional disk space in GiB. |
| `zone` | string | Zone identifier. |
| `state` | string | Current service state. |
| `powered` | boolean | Whether the service is powered on. |
| `create_time` | ISO 8601 timestamp | When the service was created. |
| `update_time` | ISO 8601 timestamp | When the service was last updated. |
| `node_count` | integer | Number of nodes. |
| `node_states` | array | State of each node (name, role, state, progress_updates). |
| `backups` | array | Available backups (backup_name, backup_time, data_size). |
| `components` | array | Connection endpoints (component, host, port, route, usage). |
| `connection_pools` | array | PostgreSQL connection pools. |
| `maintenance` | object | Maintenance schedule (dow, time, pending_updates). |
| `metadata` | object | Type-specific metadata. |
| `properties` | object | Type-specific configuration properties. |
| `service_uri` | string | Connection URI. |
| `service_uri_params` | object | Parsed connection URI parameters. |
| `users` | array | Database users. |
| `termination_protection` | boolean | Whether termination protection is enabled. |
| `labels` | array | Service labels. |

---

### Get Managed Database Versions

Returns available versions for a Managed Database service by its `{uuid}`.

#### Request

```
GET /1.3/database/{uuid}/versions HTTP/1.1
```

#### Response

```http
HTTP/1.1 200 OK
```

```json
["10", "11", "12", "13", "14"]
```

---

### Create Managed Database

Creates a new Managed Database instance. New databases are started by default. An initial backup is created automatically on startup.

#### Request

```
POST /1.3/database HTTP/1.1
```

```json
{
  "hostname_prefix": "my-db-prefix",
  "plan": "1x1xCPU-2GB-25GB",
  "additional_disk_space_gib": 10,
  "title": "my-managed-database",
  "type": "pg",
  "zone": "de-fra1",
  "maintenance": {
    "dow": "sunday",
    "time": "05:00:00"
  },
  "termination_protection": false,
  "properties": {
    "automatic_utility_network_ip_filter": true,
    "version": "13",
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
    { "key": "env", "value": "staging" }
  ]
}
```

#### Request Attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `hostname_prefix` | 3–30 characters | yes | Unique prefix for the service hostname. |
| `title` | 3–256 characters | yes | Service display name. |
| `plan` | A valid plan identifier | yes | Plan for the service (see `service_plans` in type details). |
| `additional_disk_space_gib` | 10 GiB increments, up to 4× plan storage | no | Additional disk space in GiB. |
| `zone` | A valid zone identifier | yes | Zone where the service will be hosted. |
| `type` | `mysql` / `pg` / `valkey` / `opensearch` | yes | Database type. |
| `maintenance.dow` | Weekday name, lowercase | no | Maintenance day of week (random if not set). |
| `maintenance.time` | `HH:MM:SS` (UTC) | no | Maintenance time (random if not set). |
| `properties` | object | yes | Type-specific configuration properties. |
| `termination_protection` | boolean | no | Prevent service from being powered off or deleted. |
| `networks` | array of 1–8 network objects | no | SDN networks to attach. |
| `labels` | array of 0–255 label objects / `null` | no | Labels for service filtering. |

#### Response

```http
HTTP/1.1 200 OK
```

Response body contains the created service object.

#### Create Read-Only Replica

Read-only replicas are supported for MySQL and PostgreSQL (1-node plans only). Add `service_integrations` to the creation request:

```json
{
  "hostname_prefix": "my-replica",
  "plan": "1x1xCPU-2GB-25GB",
  "title": "my-read-replica",
  "type": "pg",
  "zone": "de-fra1",
  "properties": {},
  "service_integrations": [
    {
      "type": "read_replica",
      "source_uuid": "09d11daa-4dbf-425d-81a0-bba0ea09db02"
    }
  ]
}
```

#### Service Integration Attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `type` | `read_replica` | yes | Type of the service integration. |
| `source_uuid` | A valid Managed Database UUID | yes | UUID of the primary service. |

#### Error Responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 409 Conflict | `SERVICE_EXISTS` | A service with the supplied hostname_prefix already exists. |
| 400 Bad Request | `VALIDATION_ERROR` | Must provide zone, plan, type, and hostname_prefix. |
| 400 Bad Request | `VALIDATION_ERROR` | hostname_prefix too short (min 3) or too long (max 30). |
| 400 Bad Request | `VALIDATION_ERROR` | Unrecognized service type, plan, or zone. |
| 400 Bad Request | `SERVICE_ERROR` | Invalid property value. |

---

### Modify Managed Database

Modifies a Managed Database service by its `{uuid}`.

#### Request

```
PATCH /1.3/database/{uuid} HTTP/1.1
```

```json
{
  "title": "my-managed-database",
  "powered": true,
  "plan": "2x2xCPU-4GB-50GB",
  "additional_disk_space_gib": 100,
  "zone": "de-fra1",
  "maintenance": {
    "dow": "sunday",
    "time": "12:00:00"
  },
  "termination_protection": false,
  "properties": {
    "automatic_utility_network_ip_filter": true,
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
    { "key": "env", "value": "production" }
  ]
}
```

#### Request Attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `title` | 3–256 characters | no | Service display name. |
| `powered` | `true` / `false` | no | Power on/off the service. |
| `plan` | A valid plan identifier | no | Upgrade plan. |
| `additional_disk_space_gib` | 10 GiB increments, up to 4× plan storage | no | Additional disk space in GiB. |
| `zone` | A valid zone identifier | no | Migrate to a different zone. |
| `maintenance.dow` | Weekday name, lowercase | no | Maintenance day of week. |
| `maintenance.time` | `HH:MM:SS` (UTC) | no | Maintenance time. |
| `properties` | object | no | Type-specific configuration properties. |
| `termination_protection` | boolean | no | Prevent service from being powered off or deleted. |
| `networks` | array of 1–8 network objects | no | SDN networks to attach. |
| `labels` | array of 0–255 label objects / `null` | no | Labels for service filtering. |

#### Response

```http
HTTP/1.1 200 OK
```

#### Error Responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 Bad Request | `SERVICE_ERROR` | Service is protected against termination and shutdown. |
| 409 Conflict | `SERVICE_EXISTS` | A service with the supplied hostname_prefix already exists. |
| 400 Bad Request | `VALIDATION_ERROR` | Invalid or missing required fields. |
| 400 Bad Request | `SERVICE_ERROR` | Invalid property value. |

---

### Delete Managed Database

Deletes a Managed Database. All data including backups will be erased.

#### Request

```
DELETE /1.3/database/{uuid} HTTP/1.1
```

#### Response

```http
HTTP/1.1 204 No Content
```

#### Error Responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 Bad Request | `SERVICE_ERROR` | Service is protected against termination and shutdown. |
| 404 Not Found | `SERVICE_NOT_FOUND` | Service not found. |

---

## Networks

### Attach SDN Network

Attaches an SDN network to a Managed Database service.

#### Request

```
POST /1.3/database/{uuid}/networks HTTP/1.1
```

```json
{
  "name": "example-private-network",
  "uuid": "03bec0ad-85c3-459e-824d-710f8f24f740",
  "type": "private",
  "family": "IPv4"
}
```

#### Request Attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `name` | 1–64 characters, `^[a-zA-Z0-9_-]+$` | yes | Network name, unique within the service. |
| `uuid` | A valid network UUID | yes | Private network UUID. |
| `type` | `private` | yes | Network type. |
| `family` | `IPv4` | yes | Network family (IPv6 not currently supported). |

#### Response

```http
HTTP/1.1 201 Created
```

#### Error Responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 402 Payment Required | `INSUFFICIENT_CREDITS` | Account does not have enough credits. |
| 400 Bad Request | `INVALID_REQUEST` | Validation error. |

---

### List Attached SDN Networks

Returns attached SDN networks for a service.

#### Request

```
GET /1.3/database/{uuid}/networks HTTP/1.1
```

#### Response

```http
HTTP/1.1 200 OK
```

```json
[
  {
    "uuid": "03aa7245-2ff9-49c8-9f0e-7ca0270d71a4",
    "name": "example-private-network",
    "family": "IPv4",
    "type": "private"
  }
]
```

---

### Get Attached SDN Network Details

Returns details of an attached SDN network by `{network_name}`.

#### Request

```
GET /1.3/database/{uuid}/networks/{network_name} HTTP/1.1
```

#### Response

```http
HTTP/1.1 200 OK
```

```json
{
  "uuid": "03aa7245-2ff9-49c8-9f0e-7ca0270d71a4",
  "name": "example-private-network",
  "family": "IPv4",
  "type": "private"
}
```

---

### Detach SDN Network

Detaches an SDN network from a service.

#### Request

```
DELETE /1.3/database/{uuid}/networks/{network_name} HTTP/1.1
```

#### Response

```http
HTTP/1.1 204 No Content
```

#### Error Responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 404 Not Found | `RESOURCE_NOT_FOUND` | Resource not found. |

---

### Get Peering Status

Returns network peering status for a service using SDN private networks.

#### Request

```
GET /1.3/database/{uuid}/peering-status HTTP/1.1
```

#### Response

```http
HTTP/1.1 200 OK
```

```json
{
  "create_time": "2023-12-05T09:54:04Z",
  "update_time": "2023-12-05T09:54:09Z",
  "state": "active",
  "network_peerings": [
    {
      "create_time": "2023-12-05T09:54:15Z",
      "update_time": "2023-12-05T09:56:13Z",
      "network_peer": "03196597-b972-4815-ba3a-85bb18365ced",
      "state": "active",
      "state_info": "The peering is in active state"
    }
  ]
}
```

#### Response Attributes

| Attribute | Accepted value | Description |
|-----------|---------------|-------------|
| `state` | string | Overall peering state. |
| `network_peerings` | array | List of individual peering statuses. |
| `network_peerings[].network_peer` | UUID | Peer network UUID. |
| `network_peerings[].state` | string | State of this peering. |
| `network_peerings[].state_info` | string | Human-readable state description. |

---

## Tasks & Migration

### Get Task Details

Returns task details for a service by `{uuid}` and task `{id}`.

#### Request

```
GET /1.3/database/{uuid}/tasks/{id} HTTP/1.1
```

#### Response

```http
HTTP/1.1 200 OK
```

```json
{
  "create_time": "2022-10-04T12:21:42Z",
  "operation": "mysql_migration_check",
  "id": "9b39f10d-e356-437b-b907-d6d5574721b2",
  "result_codes": [],
  "result": "All pre-checks passed successfully, preferred migration method will be [Replication]",
  "success": true
}
```

#### Response Attributes

| Attribute | Accepted value | Description |
|-----------|---------------|-------------|
| `id` | UUID | Task identifier. |
| `operation` | string | Operation name (e.g. `pg_migration_check`, `pg_upgrade_check`). |
| `create_time` | ISO 8601 timestamp | When the task was created. |
| `result` | string | Task result message. |
| `result_codes` | array | Result codes, if any. |
| `success` | boolean | Whether the task succeeded. |

---

### Create Task — Migration Check

Creates a migration check task for MySQL or PostgreSQL.

#### Request

```
POST /1.3/database/{uuid}/tasks HTTP/1.1
```

```json
{
  "migration_check": {
    "source_service_uri": "postgresql://user:pass@host:5432/dbname",
    "method": "dump",
    "ignore_dbs": "db_name_1,db_name_2"
  },
  "operation": "migration_check"
}
```

#### Request Attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `operation` | `migration_check` | yes | Operation name. |
| `migration_check` | object | yes | Embedded migration check fields. |
| `migration_check.source_service_uri` | A valid database connection string | yes | Source database connection URI. |
| `migration_check.method` | `dump` / `replication` | no | Migration method. |
| `migration_check.ignore_dbs` | Comma-separated database names | no | Databases to ignore (MySQL only). |

#### Response

```http
HTTP/1.1 200 OK
```

```json
{
  "create_time": "2022-09-22T07:22:15Z",
  "operation": "pg_migration_check",
  "id": "b32bb2e0-30dd-41b1-bb83-3de6806546a5",
  "operation_type": "async"
}
```

---

### Create Task — Upgrade Check

Creates an upgrade check task for PostgreSQL.

#### Request

```
POST /1.3/database/{uuid}/tasks HTTP/1.1
```

```json
{
  "target_version": "14",
  "operation": "upgrade_check"
}
```

#### Request Attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `operation` | `upgrade_check` | yes | Operation name. |
| `target_version` | Major version number as string | yes | Target version to check compatibility for. |

#### Response

```http
HTTP/1.1 200 OK
```

```json
{
  "create_time": "2022-09-20T12:59:30Z",
  "result": "ok *Clusters are compatible*",
  "operation": "pg_upgrade_check",
  "success": true,
  "source_pg_version": "12",
  "target_pg_version": "14",
  "id": "96d1f15f-38c6-48b0-87ff-f62c2029338d",
  "operation_type": "sync"
}
```

---

### Get Migration Status

Returns migration progress for a service.

#### Request

```
GET /1.3/database/{uuid}/migration-status HTTP/1.1
```

#### Response

```http
HTTP/1.1 200 OK
```

```json
{
  "method": "replication",
  "seconds_behind_master": 0,
  "source_active": true,
  "status": "done",
  "databases": [
    {
      "dbname": "defaultdb",
      "method": "replication",
      "status": "syncing"
    }
  ]
}
```

#### Response Attributes

| Attribute | Accepted value | Description |
|-----------|---------------|-------------|
| `method` | `dump` / `replication` | Migration method being used. |
| `seconds_behind_master` | integer | Replication lag in seconds. |
| `source_active` | boolean | Whether the source is still active. |
| `status` | string | Overall migration status. |
| `databases` | array | Per-database migration status. |

---

### Create External Migration

Creates a migration from an external database via `PATCH` on the service, setting `properties.migration`.

#### Request

```
PATCH /1.3/database/{uuid} HTTP/1.1
```

```json
{
  "properties": {
    "migration": {
      "dbname": "source_db_name",
      "host": "100.237.30.9",
      "method": "replication",
      "ignore_dbs": "db_name_1,db_name_2",
      "password": "source_password",
      "port": 3306,
      "ssl": true,
      "username": "source_username"
    }
  }
}
```

#### Migration Attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `migration` | object / `null` | yes | Migration configuration (set to `null` to cancel). |
| `host` | A valid hostname | yes | Source database hostname. |
| `username` | string | yes | Source database username. |
| `password` | string | yes | Source database password. |
| `dbname` | string | no | Source database name for initial connection. |
| `ignore_dbs` | Comma-separated database names | no | Databases to ignore (MySQL only). |
| `method` | `dump` / `replication` | no | Force migration method (MySQL and Valkey only). |
| `port` | 1–65535 | yes | Source database port. |
| `ssl` | `true` / `false` | yes | Whether to use SSL for the source connection. |

#### Response

```http
HTTP/1.1 200 OK
```

---

## Upgrade & Maintenance

### Upgrade Managed Database

Upgrades a PostgreSQL or OpenSearch service to a newer major version. Downgrades are not permitted.

#### Request

```
POST /1.3/database/{uuid}/upgrade HTTP/1.1
```

```json
{
  "target_version": "14"
}
```

#### Request Attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `target_version` | Version number as string | yes | Target major version for the upgrade. |

#### Response

```http
HTTP/1.1 200 OK
```

#### Error Responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 Bad Request | `SERVICE_ERROR` | Major version downgrade is not possible. |

---

### Start Maintenance

Starts pending maintenance updates for a service.

#### Request

```
PUT /1.3/database/{uuid}/start-maintenance HTTP/1.1
```

#### Response

| HTTP status | Description |
|-------------|-------------|
| 200 OK | Maintenance started. |
| 200 OK | Service is up to date, maintenance not required. |

---

## Clone

### Clone Managed Database

Creates a clone of a service. Supports point-in-time recovery, plan changes, and zone changes.

#### Request

```
POST /1.3/database/{uuid}/clone HTTP/1.1
```

```json
{
  "title": "new-title",
  "hostname_prefix": "new-hostname-prefix",
  "clone_time": "2022-01-21T12:21:00Z",
  "plan": "1x1xCPU-2GB-25GB",
  "zone": "fi-hel1",
  "maintenance": {
    "dow": "sunday",
    "time": "12:00:00"
  },
  "properties": {
    "automatic_utility_network_ip_filter": false,
    "version": "13",
    "public_access": true,
    "ip_filter": ["0.0.0.0/0"]
  }
}
```

#### Request Attributes

| Attribute | Accepted value | Default value | Required | Description |
|-----------|---------------|---------------|----------|-------------|
| `title` | 3–256 characters | — | yes | Title for the cloned service. |
| `hostname_prefix` | 3–30 characters | — | yes | Unique hostname prefix for the clone. |
| `clone_time` | RFC 3339 timestamp (UTC) | Latest point in time | no | Point-in-time for PG/MySQL clones. |
| `backup_name` | A valid backup name | Latest backup | no | Backup to clone from (Valkey only). |
| `plan` | A valid plan identifier | Inherited | no | Plan for the cloned service. |
| `zone` | A valid zone identifier | Inherited | no | Zone for the cloned service. |
| `maintenance.dow` | Weekday name, lowercase | Inherited | no | Maintenance day of week. |
| `maintenance.time` | `HH:MM:SS` (UTC) | Inherited | no | Maintenance time. |
| `properties` | object | Inherited | no | Type-specific configuration properties. |

#### Response

```http
HTTP/1.1 200 OK
```

#### Error Responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 409 Conflict | `SERVICE_EXISTS` | hostname_prefix already exists. |
| 400 Bad Request | `VALIDATION_ERROR` | Missing or invalid fields. |
| 400 Bad Request | `SERVICE_ERROR` | clone_time beyond oldest backup range. |
| 400 Bad Request | `SERVICE_ERROR` | Requested backup_name does not exist. |
| 400 Bad Request | `INVALID_REQUEST` | Invalid clone_time format. |

---

## Logs, Sessions, Query Stats, Metrics

### List Logs

Returns system-level logs for a service.

#### Request

```
GET /1.3/database/{uuid}/logs?limit={limit}&offset={offset}&order={order} HTTP/1.1
```

#### Query Parameters

| Attribute | Accepted value | Default value | Required | Description |
|-----------|---------------|---------------|----------|-------------|
| `limit` | 1–500 | 100 | no | Maximum number of log entries. |
| `offset` | 1–128 characters | — | no | Opaque offset identifier. |
| `order` | `desc` / `asc` | `desc` | no | Sort order. |

#### Response

```http
HTTP/1.1 200 OK
```

```json
{
  "first_log_offset": "1642770443687",
  "offset": "1642770443687",
  "logs": [
    {
      "hostname": "pg-2x2xcpu-4gb-50gb-de-fra1-1",
      "msg": "[10-1] pid=3328089,user=postgres,db=defaultdb LOG: connection authorized",
      "time": "2022-01-21T13:07:23.687241Z",
      "service": "postgresql-13.service"
    }
  ]
}
```

---

### List Sessions

Returns current session details for PG, MySQL, or Valkey services.

#### Request

```
GET /1.3/database/{uuid}/sessions?limit={limit}&offset={offset}&order={order} HTTP/1.1
```

#### Query Parameters

| Attribute | Accepted value | Default value | Required | Description |
|-----------|---------------|---------------|----------|-------------|
| `limit` | 1–5000 | 10 | no | Maximum number of entries. |
| `offset` | >= 0 | — | no | Offset for results. |
| `order` | Variable and direction, e.g. `"pid:desc"` | `query_duration:desc` | no | Sort by session variable. |

#### Response — PostgreSQL

```json
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
      "query": "SELECT ...",
      "query_duration": 12225858000,
      "query_start": "2022-01-21T13:26:28.63132Z",
      "state": "idle",
      "usename": "upadmin",
      "wait_event": "ClientRead",
      "wait_event_type": "Client"
    }
  ]
}
```

#### Response — MySQL

```json
{
  "mysql": [
    {
      "application_name": "",
      "client_addr": "111.111.111.111:63244",
      "datname": "defaultdb",
      "id": "pid_23325",
      "query": "SELECT ...",
      "query_duration": 0,
      "state": "active",
      "usename": "upadmin"
    }
  ]
}
```

#### Response — Valkey

```json
{
  "valkey": [
    {
      "active_channel_subscriptions": 0,
      "active_database": "",
      "client_addr": "[::1]:39956",
      "connection_age": 2079483000000000,
      "connection_idle": 3000000000,
      "flags": [],
      "flags_raw": "N",
      "id": "15",
      "multi_exec_commands": -1,
      "application_name": "",
      "output_buffer": 0,
      "output_buffer_memory": 0,
      "output_list_length": 0,
      "query": "info",
      "query_buffer": 0,
      "query_buffer_free": 0
    }
  ]
}
```

---

### Terminate Session

Terminates a session or kills a running query.

#### Request

```
DELETE /1.3/database/{uuid}/sessions/{pid}?terminate={terminate} HTTP/1.1
```

#### Query Parameters

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `terminate` | `true` / `false` | no | Request immediate termination instead of soft cancel. |

#### Response

```http
HTTP/1.1 200 OK
```

```json
{
  "success": true
}
```

---

### List Query Statistics

Returns query statistics for MySQL or PostgreSQL services.

#### Request

```
GET /1.3/database/{uuid}/query-statistics?limit={limit}&offset={offset}&order={order} HTTP/1.1
```

#### Query Parameters

| Attribute | Accepted value | Default value | Required | Description |
|-----------|---------------|---------------|----------|-------------|
| `limit` | 1–5000 | 10 | no | Maximum number of entries. |
| `offset` | >= 0 | — | no | Offset for results. |
| `order` | Variable:direction (e.g. `"calls:desc"`) | PG: `calls:desc`, MySQL: `count_star:desc` | no | Sort order. |

#### Response — PostgreSQL

```json
{
  "pg": [
    {
      "blk_read_time": 0,
      "blk_write_time": 0,
      "calls": 57425,
      "database_name": "defaultdb",
      "local_blks_dirtied": 0,
      "local_blks_hit": 0,
      "local_blks_read": 0,
      "local_blks_written": 0,
      "max_time": 133387,
      "mean_time": 1060,
      "min_time": 151,
      "query": "COMMIT",
      "rows": 0,
      "shared_blks_dirtied": 0,
      "shared_blks_hit": 0,
      "shared_blks_read": 0,
      "shared_blks_written": 0,
      "stddev_time": 0,
      "temp_blks_read": 0,
      "temp_blks_written": 0,
      "total_time": 60879904,
      "user_name": "upadmin"
    }
  ]
}
```

#### PostgreSQL Query Stats Attributes

| Attribute | Description |
|-----------|-------------|
| `blk_read_time` | Total time spent reading blocks (ns). |
| `blk_write_time` | Total time spent writing blocks (ns). |
| `calls` | Number of times executed. |
| `database_name` | Database name. |
| `local_blks_dirtied` | Total local blocks dirtied. |
| `local_blks_hit` | Total local block cache hits. |
| `local_blks_read` | Total local blocks read. |
| `local_blks_written` | Total local blocks written. |
| `max_time` | Maximum time spent (ns). |
| `mean_time` | Mean time spent (ns). |
| `min_time` | Minimum time spent (ns). |
| `query` | Representative query text. |
| `rows` | Total rows retrieved or affected. |
| `shared_blks_dirtied` | Total shared blocks dirtied. |
| `shared_blks_hit` | Total shared block cache hits. |
| `shared_blks_read` | Total shared blocks read. |
| `shared_blks_written` | Total shared blocks written. |
| `stddev_time` | Standard deviation of time spent (ns). |
| `temp_blks_read` | Total temp blocks read. |
| `temp_blks_written` | Total temp blocks written. |
| `total_time` | Total time spent (ns). |
| `user_name` | User that executed the statement. |

#### MySQL Query Stats Attributes

| Attribute | Description |
|-----------|-------------|
| `avg_timer_wait` | Average wait time (ns). |
| `count_star` | Number of summarized events. |
| `digest` | Statement digest MD5 hash. |
| `digest_text` | Normalized statement digest text. |
| `first_seen` | When the digest was first seen. |
| `last_seen` | When the digest was last seen. |
| `max_timer_wait` | Maximum wait time (ns). |
| `min_timer_wait` | Minimum wait time (ns). |
| `quantile_95` | 95th percentile latency (ns). |
| `quantile_99` | 99th percentile latency (ns). |
| `quantile_999` | 99.9th percentile latency (ns). |
| `query_sample_text` | A sample SQL statement producing this digest. |
| `query_sample_timer_wait` | Wait time for the sample statement (ns). |
| `schema_name` | Schema name. |
| `sum_rows_affected` | Sum of rows affected. |
| `sum_rows_sent` | Sum of rows returned. |
| `sum_timer_wait` | Total wait time (ns). |

---

### List Metrics

Returns time-series metrics for a service.

#### Request

```
GET /1.3/database/{uuid}/metrics?period={period} HTTP/1.1
```

#### Query Parameters

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `period` | `hour` / `day` / `week` / `month` / `year` | yes | Metrics time period. |

#### Response

```http
HTTP/1.1 200 OK
```

```json
{
  "cpu_usage": {
    "data": {
      "cols": [
        { "label": "time", "type": "date" },
        { "label": "node-1 (master)", "type": "number" }
      ],
      "rows": [
        ["2022-01-21T13:10:30Z", 2.74],
        ["2022-01-21T13:11:00Z", 3.07]
      ]
    },
    "hints": { "title": "CPU usage %" }
  },
  "disk_usage": {
    "data": { "cols": [], "rows": [] },
    "hints": { "title": "Disk space usage %" }
  },
  "diskio_reads": {
    "data": { "cols": [], "rows": [] },
    "hints": { "title": "Disk iops (reads)" }
  },
  "diskio_writes": {
    "data": { "cols": [], "rows": [] },
    "hints": { "title": "Disk iops (writes)" }
  },
  "load_average": {
    "data": { "cols": [], "rows": [] },
    "hints": { "title": "Load average (5 min)" }
  },
  "mem_usage": {
    "data": { "cols": [], "rows": [] },
    "hints": { "title": "Memory usage %" }
  },
  "net_receive": {
    "data": { "cols": [], "rows": [] },
    "hints": { "title": "Network receive (bytes/s)" }
  },
  "net_send": {
    "data": { "cols": [], "rows": [] },
    "hints": { "title": "Network transmit (bytes/s)" }
  }
}
```

#### Available Metric Keys

| Key | Description |
|-----|-------------|
| `cpu_usage` | CPU usage percentage per node. |
| `disk_usage` | Disk space usage percentage per node. |
| `diskio_reads` | Disk I/O reads per node. |
| `diskio_writes` | Disk I/O writes per node. |
| `load_average` | 5-minute load average per node. |
| `mem_usage` | Memory usage percentage per node. |
| `net_receive` | Network receive in bytes/s per node. |
| `net_send` | Network transmit in bytes/s per node. |

---

## Logical Databases

### List Logical Databases

Returns logical databases for a MySQL or PostgreSQL service.

#### Request

```
GET /1.3/database/{uuid}/databases HTTP/1.1
```

#### Response

```http
HTTP/1.1 200 OK
```

```json
[
  {
    "name": "defaultdb",
    "lc_collate": "en_US.UTF-8",
    "lc_ctype": "en_US.UTF-8"
  },
  {
    "name": "mydb",
    "lc_collate": "en_US.UTF-8",
    "lc_ctype": "en_US.UTF-8"
  }
]
```

---

### Create Logical Database

Creates a new logical database for a MySQL or PostgreSQL service.

#### Request

```
POST /1.3/database/{uuid}/databases HTTP/1.1
```

```json
{
  "name": "my-database",
  "lc_collate": "en_US.UTF8",
  "lc_ctype": "en_US.UTF8"
}
```

#### Request Attributes

| Attribute | Accepted value | Default value | Required | Description |
|-----------|---------------|---------------|----------|-------------|
| `name` | 1–40 characters | — | yes | Database name. |
| `lc_collate` | 1–128 characters | `en_US.UTF-8` | no | PostgreSQL only, string sort order. |
| `lc_ctype` | 1–128 characters | `en_US.UTF-8` | no | PostgreSQL only, character classification. |

#### Response

```http
HTTP/1.1 200 OK
```

#### Error Responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 409 Conflict | `DB_EXISTS` | Database with same name already exists. |
| 400 Bad Request | `SERVICE_ERROR` | Invalid lc_collate or lc_ctype value. |
| 400 Bad Request | `VALIDATION_ERROR` | Invalid database name format. |

---

### Delete Logical Database

Deletes a logical database by `{database_name}`.

#### Request

```
DELETE /1.3/database/{uuid}/databases/{database_name} HTTP/1.1
```

#### Response

```http
HTTP/1.1 204 No Content
```

#### Error Responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 404 Not Found | `DB_NOT_FOUND` | Logical database not found. |

---

## Users

### List Users

Returns database users for a service.

#### Request

```
GET /1.3/database/{uuid}/users HTTP/1.1
```

#### Response — MySQL

```json
[
  {
    "username": "upadmin",
    "password": "my-password",
    "authentication": "caching_sha2_password",
    "type": "primary"
  }
]
```

#### Response — PostgreSQL

```json
[
  {
    "username": "api-doc-user",
    "password": "my-password",
    "type": "normal",
    "pg_access_control": {
      "allow_replication": true
    }
  }
]
```

#### Response — Valkey

```json
[
  {
    "username": "api-doc-user",
    "password": "my-password",
    "type": "regular",
    "valkey_access_control": {
      "categories": ["+@set"],
      "channels": ["*"],
      "commands": ["+set"],
      "keys": ["key_*"]
    }
  }
]
```

#### Response — OpenSearch

```json
[
  {
    "username": "api-doc-user",
    "password": "my-password",
    "opensearch_access_control": {
      "rules": [
        { "index": "index_1", "permission": "read" },
        { "index": "index_2", "permission": "write" }
      ]
    }
  }
]
```

---

### Get User Details

Returns user details by `{username}`.

#### Request

```
GET /1.3/database/{uuid}/users/{username} HTTP/1.1
```

#### Response

```http
HTTP/1.1 200 OK
```

```json
{
  "username": "api-doc-user",
  "password": "my-password",
  "authentication": "caching_sha2_password",
  "type": "normal"
}
```

---

### Create User

Creates a new database user. Request body varies by database type.

#### Request

```
POST /1.3/database/{uuid}/users HTTP/1.1
```

#### MySQL

```json
{
  "username": "api-doc-user",
  "authentication": "caching_sha2_password",
  "password": "new-password"
}
```

#### PostgreSQL

```json
{
  "username": "api-doc-user",
  "password": "new-password",
  "pg_access_control": {
    "allow_replication": true
  }
}
```

#### Valkey

```json
{
  "username": "api-doc-user",
  "password": "new-password",
  "valkey_access_control": {
    "categories": ["+@set"],
    "channels": ["*"],
    "commands": ["+set"],
    "keys": ["key_*"]
  }
}
```

#### OpenSearch

```json
{
  "username": "api-doc-user",
  "password": "new-password",
  "opensearch_access_control": {
    "rules": [
      { "index": "index_1", "permission": "read" },
      { "index": "index_2", "permission": "write" }
    ]
  }
}
```

#### Request Attributes

| Attribute | Accepted value | Default value | Required | Description |
|-----------|---------------|---------------|----------|-------------|
| `username` | 1–64 characters | — | yes | Username for the new user. |
| `password` | 8–256 characters | Randomly generated | no | User password. |
| `authentication` | `caching_sha2_password` / `mysql_native_password` | `caching_sha2_password` | no | MySQL only, authentication type. |
| `pg_access_control` | object | — | no | PostgreSQL access control. |
| `valkey_access_control` | object | — | no | Valkey access control. |
| `opensearch_access_control` | object | — | no | OpenSearch access control. |

#### PostgreSQL Access Control Attributes

| Attribute | Accepted value | Default value | Required | Description |
|-----------|---------------|---------------|----------|-------------|
| `allow_replication` | boolean | `true` | no | Grant replication privilege. |

#### Valkey Access Control Attributes

| Attribute | Accepted value | Default value | Required | Description |
|-----------|---------------|---------------|----------|-------------|
| `categories` | array of strings | — | no | Access control for command categories. |
| `channels` | array of strings | `["*"]` | no | Access control for Pub/Sub channels. |
| `commands` | array of strings | — | no | Access control for commands. |
| `keys` | array of strings | — | no | Access control for keys. |

#### OpenSearch Access Control Attributes

| Attribute | Accepted value | Default value | Required | Description |
|-----------|---------------|---------------|----------|-------------|
| `rules` | array of rule objects | `[]` | no | Access control rules with `index` and `permission`. |
| `rules[].index` | Index name / pattern / Top-level API | — | no | Index name, pattern, or top-level API. |
| `rules[].permission` | `deny` / `admin` / `readwrite` / `read` / `write` | — | no | Permission level. |

#### Response

```http
HTTP/1.1 200 OK
```

Response body contains the created user object.

#### Error Responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 409 Conflict | `SERVICE_ERROR` | Username already exists. |
| 400 Bad Request | `VALIDATION_ERROR` | Password too short (min 8 characters). |
| 400 Bad Request | `VALIDATION_ERROR` | Invalid username format. |
| 400 Bad Request | `VALIDATION_ERROR` | Invalid authentication type (MySQL only). |

---

### Modify User

Modifies user password or authentication type.

#### Request

```
PATCH /1.3/database/{uuid}/users/{username} HTTP/1.1
```

```json
{
  "authentication": "caching_sha2_password",
  "password": "new-password"
}
```

#### Request Attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `password` | 8–256 characters | no | New user password. |
| `authentication` | `caching_sha2_password` / `mysql_native_password` | no | MySQL only, authentication type. |

#### Response

```http
HTTP/1.1 200 OK
```

#### Error Responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 Bad Request | `VALIDATION_ERROR` | Password too short (min 8 characters). |
| 400 Bad Request | `VALIDATION_ERROR` | Invalid authentication type (MySQL only). |

---

### Modify User Access Control

Modifies user access control settings (PG, Valkey, OpenSearch).

#### Request

```
PATCH /1.3/database/{uuid}/users/{username}/access-control HTTP/1.1
```

#### PostgreSQL

```json
{
  "pg_access_control": {
    "allow_replication": true
  }
}
```

#### Valkey

```json
{
  "valkey_access_control": {
    "categories": [],
    "channels": ["*"],
    "commands": [],
    "keys": []
  }
}
```

#### OpenSearch

```json
{
  "opensearch_access_control": {
    "rules": []
  }
}
```

#### Request Attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `pg_access_control` | object | no | PostgreSQL access control (see Create User). |
| `valkey_access_control` | object | no | Valkey access control (see Create User). |
| `opensearch_access_control` | object | no | OpenSearch access control (see Create User). |

#### Response

```http
HTTP/1.1 200 OK
```

#### Error Responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 Bad Request | `VALIDATION_ERROR` | Must provide required fields in the access control payload. |

---

### Delete User

Deletes a user by `{username}`.

#### Request

```
DELETE /1.3/database/{uuid}/users/{username} HTTP/1.1
```

#### Response

```http
HTTP/1.1 204 No Content
```

#### Error Responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 404 Not Found | `USER_NOT_FOUND` | User not found. |

---

## Connection Pools (PostgreSQL)

### List Connection Pools

Returns connection pools for a PostgreSQL service.

#### Request

```
GET /1.3/database/{uuid}/connection-pools HTTP/1.1
```

#### Response

```http
HTTP/1.1 200 OK
```

```json
[
  {
    "connection_uri": "postgres://upadmin:pass@host:11551/pool-1?sslmode=require",
    "database": "defaultdb",
    "pool_mode": "session",
    "pool_name": "pool-1",
    "pool_size": 50,
    "username": "upadmin"
  },
  {
    "connection_uri": "postgres://host:11551/pool-2?sslmode=require",
    "database": "defaultdb",
    "pool_mode": "transaction",
    "pool_name": "pool-2",
    "pool_size": 10,
    "username": ""
  }
]
```

---

### Get Connection Pool Details

Returns details for a specific connection pool by `{pool_name}`.

#### Request

```
GET /1.3/database/{uuid}/connection-pools/{pool_name} HTTP/1.1
```

#### Response

```http
HTTP/1.1 200 OK
```

```json
{
  "connection_uri": "postgres://upadmin:pass@host:11551/pool-1?sslmode=require",
  "database": "defaultdb",
  "pool_mode": "session",
  "pool_name": "pool-1",
  "pool_size": 50,
  "username": "upadmin"
}
```

---

### Create Connection Pool

Creates a connection pool for a PostgreSQL service.

#### Request

```
POST /1.3/database/{uuid}/connection-pools HTTP/1.1
```

```json
{
  "database": "defaultdb",
  "pool_mode": "transaction",
  "pool_name": "pool-1",
  "pool_size": 10,
  "username": "upadmin"
}
```

#### Request Attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `database` | 1–40 characters | yes | Logical database name. |
| `pool_mode` | `session` / `transaction` / `statement` | yes | Connection pool mode. |
| `pool_name` | 1–63 characters | yes | Pool name. |
| `pool_size` | 1–10000 | yes | Pool size (number of connections). |
| `username` | 1–64 characters | no | User for the pool (all users if not set). |

#### Response

```http
HTTP/1.1 200 OK
```

```json
{
  "connection_uri": "postgres://host:11551/pool-1?sslmode=require",
  "database": "defaultdb",
  "pool_mode": "transaction",
  "pool_name": "pool-1",
  "pool_size": 10,
  "username": "upadmin"
}
```

#### Error Responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 409 Conflict | `SERVICE_ERROR` | Pool with same name already exists. |
| 400 Bad Request | `SERVICE_ERROR` | Invalid pool_name format. |
| 409 Conflict | `SERVICE_ERROR` | Total pool connections exceed database limit. |
| 400 Bad Request | `SERVICE_ERROR` | Invalid pool_mode. |
| 403 Forbidden | `SERVICE_ERROR` | Database or username does not exist. |

---

### Modify Connection Pool

Modifies a connection pool by `{pool_name}`.

#### Request

```
PATCH /1.3/database/{uuid}/connection-pools/{pool_name} HTTP/1.1
```

```json
{
  "database": "defaultdb",
  "pool_mode": "transaction",
  "pool_size": 10,
  "username": "upadmin"
}
```

#### Request Attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `database` | 1–40 characters | no | Logical database name. |
| `pool_mode` | `session` / `transaction` / `statement` | no | Connection pool mode. |
| `pool_size` | 1–10000 | no | Pool size. |
| `username` | 1–64 characters | no | User for the pool. |

#### Response

```http
HTTP/1.1 200 OK
```

#### Error Responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 409 Conflict | `SERVICE_ERROR` | Total pool connections exceed database limit. |
| 400 Bad Request | `SERVICE_ERROR` | Invalid pool_mode. |
| 403 Forbidden | `SERVICE_ERROR` | Database or username does not exist. |

---

### Delete Connection Pool

Deletes a connection pool by `{pool_name}`.

#### Request

```
DELETE /1.3/database/{uuid}/connection-pools/{pool_name} HTTP/1.1
```

#### Response

```http
HTTP/1.1 204 No Content
```

#### Error Responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 404 Not Found | `CONNECTION_POOL_NOT_FOUND` | Connection pool not found. |

---

## Access Control & Security (OpenSearch)

### Get Access Controls

Returns access control settings for an OpenSearch service.

#### Request

```
GET /1.3/database/{uuid}/access-control HTTP/1.1
```

#### Response

```http
HTTP/1.1 200 OK
```

```json
{
  "access_control": false,
  "extended_access_control": false
}
```

#### Response Attributes

| Attribute | Accepted value | Description |
|-----------|---------------|-------------|
| `access_control` | boolean | Whether user access control is enforced. |
| `extended_access_control` | boolean | Whether top-level `_mget`, `_msearch`, `_bulk` APIs are granted. |

---

### Modify Access Controls

Modifies access control settings for an OpenSearch service.

#### Request

```
PATCH /1.3/database/{uuid}/access-control HTTP/1.1
```

```json
{
  "access_control": true,
  "extended_access_control": false
}
```

#### Request Attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `access_control` | `true` / `false` | no | Enable/disable user access control enforcement. |
| `extended_access_control` | `true` / `false` | no | Grant access to top-level bulk APIs. |

#### Response

```http
HTTP/1.1 200 OK
```

---

### Get Security Management Status

Returns security management status for an OpenSearch service.

#### Request

```
GET /1.3/database/{uuid}/security HTTP/1.1
```

#### Response

```http
HTTP/1.1 200 OK
```

```json
{
  "security_plugin_admin_enabled": true,
  "security_plugin_available": true,
  "security_plugin_enabled": true
}
```

#### Response Attributes

| Attribute | Accepted value | Description |
|-----------|---------------|-------------|
| `security_plugin_admin_enabled` | boolean | Whether the security admin is defined. |
| `security_plugin_available` | boolean | Whether security is available for the service. |
| `security_plugin_enabled` | boolean | Whether security is enabled for the service. |

---

### Enable Security Management Admin

Enables the security management admin. This action is irreversible — user management moves to the OpenSearch Security Dashboard/API.

#### Request

```
POST /1.3/database/{uuid}/security/admin HTTP/1.1
```

```json
{
  "admin_password": "d.9aB_23Q"
}
```

#### Request Attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `admin_password` | 8–256 characters | yes | Security admin password. |

#### Response

```http
HTTP/1.1 200 OK
```

```json
{
  "security_plugin_admin_enabled": true,
  "security_plugin_available": true,
  "security_plugin_enabled": true
}
```

---

### Modify Security Management Admin Password

Changes the security management admin password.

#### Request

```
PUT /1.3/database/{uuid}/security/admin HTTP/1.1
```

```json
{
  "admin_password": "d.9aB_23Q",
  "new_password": "d.9aB_43Q"
}
```

#### Request Attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `admin_password` | 8–256 characters | yes | Current admin password. |
| `new_password` | 8–256 characters | yes | New admin password. |

#### Response

```http
HTTP/1.1 200 OK
```

```json
{
  "security_plugin_admin_enabled": true,
  "security_plugin_available": true,
  "security_plugin_enabled": true
}
```

---

## Indices (OpenSearch)

### List Indices

Returns indices for an OpenSearch service.

#### Request

```
GET /1.3/database/{uuid}/indices HTTP/1.1
```

#### Response

```http
HTTP/1.1 200 OK
```

```json
[
  {
    "create_time": "2023-05-11T13:24:10.623Z",
    "docs": 1,
    "health": "green",
    "index_name": ".kibana_1",
    "number_of_replicas": 0,
    "number_of_shards": 1,
    "read_only_allow_delete": false,
    "size": 5313,
    "status": "open"
  },
  {
    "create_time": "2023-05-11T13:24:10.623Z",
    "docs": 1,
    "health": "green",
    "index_name": "index_1",
    "number_of_replicas": 0,
    "number_of_shards": 1,
    "read_only_allow_delete": false,
    "size": 5313,
    "status": "open"
  }
]
```

#### Response Attributes

| Attribute | Accepted value | Description |
|-----------|---------------|-------------|
| `create_time` | ISO 8601 timestamp | When the index was created. |
| `docs` | integer | Number of documents in the index. |
| `health` | `green` / `yellow` / `red` | Health status of the index. |
| `index_name` | string | Name of the index. |
| `number_of_replicas` | integer | Number of replicas. |
| `number_of_shards` | integer | Number of shards. |
| `read_only_allow_delete` | boolean | Whether the index is in read-only-allow-delete state. |
| `size` | integer | Size of the index in bytes. |
| `status` | `open` / `closed` | Index status. |

---

### Delete Index

Deletes an index by `{index_name}`.

#### Request

```
DELETE /1.3/database/{uuid}/indices/{index_name} HTTP/1.1
```

#### Response

```http
HTTP/1.1 204 No Content
```

---

## Alerts

### List All Alerts

Returns all alerts across all Managed Database services.

#### Request

```
GET /1.3/database/alerts HTTP/1.1
```

#### Response

```http
HTTP/1.1 200 OK
```

```json
[
  {
    "create_time": "2024-03-18T20:08:54.55884Z",
    "event": "user_alert_os_too_many_shards",
    "uuid": "090b1f30-00b9-463b-7edf-a6393ec684fd",
    "title": "opensearch-1x2xcpu-2",
    "service_type": "opensearch",
    "severity": "warning"
  },
  {
    "create_time": "2024-03-16T18:11:55.14855Z",
    "event": "pg-disk-full",
    "uuid": "090b1f60-10b9-463b-1edf-a6398ec571fd",
    "title": "pg-1x2xcpu-2",
    "service_type": "pg",
    "severity": "critical"
  }
]
```

#### Response Attributes

| Attribute | Accepted value | Description |
|-----------|---------------|-------------|
| `create_time` | ISO 8601 timestamp | When the alert was created. |
| `event` | string | Alert event identifier. |
| `uuid` | UUID | Service UUID associated with the alert. |
| `title` | string | Service title. |
| `service_type` | `pg` / `mysql` / `opensearch` / `valkey` | Database type. |
| `severity` | `warning` / `critical` | Alert severity. |

---

### List Service Alerts

Returns alerts for a specific service.

#### Request

```
GET /1.3/database/{uuid}/alerts HTTP/1.1
```

#### Response

```http
HTTP/1.1 200 OK
```

```json
[
  {
    "create_time": "2024-03-18T12:18:54.55855Z",
    "event": "pg-disk-full",
    "service_type": "pg",
    "severity": "critical"
  }
]
```

---

## Labels

### Create Label

Creates a new label on a service.

#### Request

```
POST /1.3/database/{service_uuid}/labels HTTP/1.1
```

```json
{
  "key": "env",
  "value": "staging"
}
```

#### Request Attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `key` | 2–32 printable ASCII characters, must not start with `_` | yes | Label identifier (unique within a service). |
| `value` | 0–255 characters | yes | Label value. |

#### Response

```http
HTTP/1.1 201 Created
```

#### Error Responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 402 Payment Required | `INSUFFICIENT_CREDITS` | Account does not have enough credits. |
| 400 Bad Request | `INVALID_REQUEST` | Validation error. |

---

### List Labels

Returns all labels for a service.

#### Request

```
GET /1.3/database/{service_uuid}/labels HTTP/1.1
```

#### Response

```http
HTTP/1.1 200 OK
```

```json
[
  { "key": "env", "value": "staging" },
  { "key": "foo", "value": "bar" }
]
```

---

### Get Label Details

Returns a specific label by `{key}`.

#### Request

```
GET /1.3/database/{service_uuid}/labels/{key} HTTP/1.1
```

#### Response

```http
HTTP/1.1 200 OK
```

```json
{
  "key": "env",
  "value": "staging"
}
```

---

### Modify Label

Modifies an existing label by `{key}`.

#### Request

```
PATCH /1.3/database/{service_uuid}/labels/{key} HTTP/1.1
```

```json
{
  "key": "env",
  "value": "production"
}
```

#### Request Attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `key` | 2–32 printable ASCII characters, must not start with `_` | no | Updated label key. |
| `value` | 0–255 characters / `null` | no | Updated label value. |

#### Response

```http
HTTP/1.1 200 OK
```

---

### Delete Label

Deletes a label by `{key}`.

#### Request

```
DELETE /1.3/database/{service_uuid}/labels/{key} HTTP/1.1
```

#### Response

```http
HTTP/1.1 204 No Content
```

#### Error Responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 404 Not Found | `RESOURCE_NOT_FOUND` | Resource not found. |

---

## Service Integrations

### List Integrations

Returns integrations for a service.

#### Request

```
GET /1.3/database/{service_uuid}/integrations HTTP/1.1
```

#### Response

```http
HTTP/1.1 200 OK
```

```json
[
  {
    "uuid": "7398fbb0-e5ea-4bfa-9f31-bdf500988684",
    "type": "logs",
    "source_name": "my-postgres-service-1",
    "source_uuid": "09c5db1e-9273-4d31-9fd0-a9d71f861597",
    "destination_name": "my-opensearch-service-1",
    "destination_uuid": "098c5dfa-2616-4821-aaaa-8748c6a23111",
    "state": "starting",
    "node_states": [],
    "properties": {
      "opensearch_index_days_max": 30,
      "opensearch_index_prefix": "mysql_logs"
    }
  },
  {
    "uuid": "7398fbb0-e5ea-4bfa-9f31-bdf500988348",
    "type": "rsyslog",
    "source_name": "my-mysql-service-1",
    "source_uuid": "09c5db1e-9273-4d31-9fd0-a9d71f834f90",
    "destination_name": "my-rsyslog-endpoint-1",
    "destination_uuid": "20e86f48-2dcd-4a43-813d-3532a2112c1d",
    "state": "running",
    "node_states": [
      { "name": "mysql-1", "status": "running", "errors": [] }
    ],
    "properties": {}
  }
]
```

#### Response Attributes

| Attribute | Accepted value | Description |
|-----------|---------------|-------------|
| `uuid` | UUID | Integration identifier. |
| `type` | `logs` / `rsyslog` / `read_replica` | Integration type. |
| `source_name` | string | Source service name. |
| `source_uuid` | UUID | Source service UUID. |
| `destination_name` | string | Destination service or endpoint name. |
| `destination_uuid` | UUID | Destination service or endpoint UUID. |
| `state` | string | Integration state. |
| `node_states` | array | Per-node integration state. |
| `properties` | object | Integration properties. |

---

### Get Integration Details

Returns details for a specific integration.

#### Request

```
GET /1.3/database/{service_uuid}/integrations/{integration_uuid} HTTP/1.1
```

#### Response

```http
HTTP/1.1 200 OK
```

```json
{
  "uuid": "7398fbb0-e5ea-4bfa-9f31-bdf500988184",
  "type": "logs",
  "source_name": "my-mysql-service-1",
  "source_uuid": "09c5db1e-9273-4d31-9fd0-a9d71f861587",
  "destination_name": "my-opensearch-service-1",
  "destination_uuid": "098c5dfa-2616-4821-aaaa-8748c6a23111",
  "state": "running",
  "node_states": [
    { "name": "mysql-1", "status": "running", "errors": [] }
  ],
  "properties": {
    "opensearch_index_days_max": 30,
    "opensearch_index_prefix": "mysql_logs"
  }
}
```

---

### Create Integration

Creates a new service integration.

#### Request

```
POST /1.3/database/{service_uuid}/integrations HTTP/1.1
```

```json
{
  "type": "logs",
  "destination_uuid": "09e86f48-2dcd-4a43-843d-3532ab112c1d",
  "properties": {
    "opensearch_index_days_max": 30,
    "opensearch_index_prefix": "mysql_logs"
  }
}
```

#### Request Attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `type` | `logs` / `rsyslog` | yes | Integration type. |
| `destination_uuid` | UUID | yes | Destination OpenSearch service or integration endpoint UUID. |
| `properties` | object | no | Integration properties. |

#### OpenSearch Logs Properties

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `opensearch_index_days_max` | 1–10000 | no | Maximum days to keep logs in the index. |
| `opensearch_index_prefix` | 1–1024 characters | no | Prefix for the index name. |

#### Response

```http
HTTP/1.1 201 Created
```

#### Error Responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 Bad Request | `INVALID_REQUEST` | Validation error. |

---

### Modify Integration

Modifies an existing integration.

#### Request

```
PATCH /1.3/database/{service_uuid}/integrations/{integration_uuid} HTTP/1.1
```

```json
{
  "properties": {
    "opensearch_index_days_max": 15,
    "opensearch_index_prefix": "modified_mysql_logs"
  }
}
```

#### Request Attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `properties` | object | yes | Updated integration properties. |

#### Response

```http
HTTP/1.1 200 OK
```

---

### Delete Integration

Deletes an integration.

#### Request

```
DELETE /1.3/database/{service_uuid}/integrations/{integration_uuid} HTTP/1.1
```

#### Response

```http
HTTP/1.1 204 No Content
```

#### Error Responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 404 Not Found | `RESOURCE_NOT_FOUND` | Resource not found. |

---

## Integration Endpoints

### List Integration Endpoints

Returns all integration endpoints.

#### Request

```
GET /1.3/database/integration-endpoints HTTP/1.1
```

#### Response

```http
HTTP/1.1 200 OK
```

```json
[
  {
    "uuid": "20e86f48-2dcd-4a43-843d-3532ab112c1d",
    "name": "rsyslog",
    "type": "rsyslog",
    "properties": {
      "format": "rfc5424",
      "max_message_size": 8192,
      "port": 514,
      "sd": "",
      "server": "94.237.95.164",
      "tls": false
    }
  },
  {
    "uuid": "f522fb75-3869-4356-881d-11bff2739321",
    "name": "my-rsyslog-1",
    "type": "rsyslog",
    "properties": {
      "format": "custom",
      "logline": "{\"message\":\"%msg%\",\"hostname\":\"%HOSTNAME%\"}",
      "port": 514,
      "server": "1.1.1.1",
      "tls": false
    }
  },
  {
    "uuid": "02febe91-86a0-4592-87a0-67b0bd3115d1",
    "name": "my-rsyslog-2",
    "type": "rsyslog",
    "properties": {
      "ca": "-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----",
      "cert": "-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----",
      "key": "-----BEGIN RSA PRIVATE KEY-----\n...\n-----END RSA PRIVATE KEY-----",
      "format": "rfc5424",
      "max_message_size": 8192,
      "port": 514,
      "sd": "",
      "server": "94.237.95.164",
      "tls": true
    }
  }
]
```

---

### Get Integration Endpoint Details

Returns details for a specific endpoint.

#### Request

```
GET /1.3/database/integration-endpoints/{integration_uuid} HTTP/1.1
```

#### Response

```http
HTTP/1.1 200 OK
```

```json
{
  "uuid": "20e86f48-2dcd-4a43-843d-3532ab112c1d",
  "name": "my-rsyslog-1",
  "type": "rsyslog",
  "properties": {
    "format": "rfc5424",
    "max_message_size": 8192,
    "port": 514,
    "sd": "",
    "server": "2.2.2.2",
    "tls": false
  }
}
```

---

### Create Integration Endpoint

Creates a new integration endpoint.

#### Request

```
POST /1.3/database/integration-endpoints HTTP/1.1
```

```json
{
  "name": "my-rsyslog-1",
  "type": "rsyslog",
  "properties": {
    "format": "rfc5424",
    "max_message_size": 8192,
    "port": 514,
    "server": "1.1.1.1",
    "tls": false
  }
}
```

#### Request Attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `name` | 1–64 characters | yes | Endpoint name. |
| `type` | `rsyslog` | yes | Endpoint type (currently only rsyslog). |
| `properties` | object | yes | Endpoint properties. |

#### Rsyslog Properties Attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `server` | IPv4 / IPv6 / hostname | yes | Server address. |
| `port` | 1–65535 | yes | Port number. |
| `format` | `rfc5424` / `rfc3164` / `custom` | yes | Message format (`custom` requires `logline`). |
| `tls` | `true` / `false` | yes | Whether to use TLS encryption. |
| `max_message_size` | 2048–2147483647 | no | Maximum message size in bytes. |
| `sd` | 0–1024 characters | no | Structured data block for rfc5424. |
| `logline` | 0–1024 characters | no | Custom log line template using `%tag%` syntax. |
| `ca` | string (PEM) | no | CA certificate for TLS. |
| `cert` | string (PEM) | no | Client certificate for TLS. |
| `key` | string (PEM) | no | Client key for TLS. |

#### Supported Custom Format Tags

| Tag | Description |
|-----|-------------|
| `%HOSTNAME%` | The hostname of the server. |
| `%app-name%` | The application name. |
| `%msg%` | The log message. |
| `%msgid%` | The unique identifier. |
| `%pri%` | The priority level. |
| `%procid%` | The process ID. |
| `%structured-data%` | Additional data fields. |
| `%timestamp%` | The default timestamp. |
| `%timestamp:::date-rfc3339%` | Timestamp in RFC 3339 format. |

#### Response

```http
HTTP/1.1 201 Created
```

#### Error Responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 Bad Request | `INVALID_REQUEST` | Validation error. |

---

### Modify Integration Endpoint

Modifies an existing endpoint.

#### Request

```
PATCH /1.3/database/integration-endpoints/{integration_uuid} HTTP/1.1
```

```json
{
  "properties": {
    "server": "3.3.3.3",
    "port": 514,
    "tls": true
  }
}
```

#### Request Attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `properties` | object | yes | Updated endpoint properties. |

#### Rsyslog Properties Attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `server` | IPv4 / IPv6 / hostname | yes | Server address. |
| `port` | 1–65535 | yes | Port number. |
| `format` | `rfc5424` / `rfc3164` / `custom` | yes | Message format. |
| `tls` | `true` / `false` | yes | Whether to use TLS. |
| `max_message_size` | 2048–2147483647 | no | Maximum message size. |
| `sd` | 0–1024 characters | no | Structured data block. |
| `logline` | 0–1024 characters | no | Custom log line template. |

#### Response

```http
HTTP/1.1 200 OK
```

#### Error Responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 Bad Request | `INVALID_REQUEST` | Validation error. |

---

### Delete Integration Endpoint

Deletes an endpoint.

#### Request

```
DELETE /1.3/database/integration-endpoints/{integration_uuid} HTTP/1.1
```

#### Response

```http
HTTP/1.1 204 No Content
```

#### Error Responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 404 Not Found | `RESOURCE_NOT_FOUND` | Resource not found. |

---

## Versions

### List All Service Versions

Returns all available database service versions with lifecycle information.

#### Request

```
GET /1.3/database/versions HTTP/1.1
```

#### Response

```http
HTTP/1.1 200 OK
```

```json
[
  {
    "major_version": "8",
    "service_type": "mysql",
    "state": "available",
    "end_of_life_time": null,
    "availability_end_time": null,
    "availability_start_time": "2019-05-10T00:00:00Z",
    "termination_time": null,
    "upgrade_to_service_type": null,
    "upgrade_to_version": null,
    "upstream_end_of_life_time": null
  },
  {
    "major_version": "17",
    "service_type": "pg",
    "state": "available",
    "end_of_life_time": "2029-11-08T00:00:00Z",
    "availability_end_time": "2029-05-08T00:00:00Z",
    "availability_start_time": "2024-12-09T00:00:00Z",
    "termination_time": null,
    "upgrade_to_service_type": null,
    "upgrade_to_version": null,
    "upstream_end_of_life_time": "2029-11-08T00:00:00Z"
  },
  {
    "major_version": "1",
    "service_type": "opensearch",
    "state": "available",
    "end_of_life_time": null,
    "availability_end_time": null,
    "availability_start_time": "2021-09-21T00:00:00Z",
    "termination_time": null,
    "upgrade_to_service_type": "opensearch",
    "upgrade_to_version": "2",
    "upstream_end_of_life_time": null
  }
]
```

#### Response Attributes

| Attribute | Accepted value | Description |
|-----------|---------------|-------------|
| `major_version` | string | Major version number. |
| `service_type` | `mysql` / `pg` / `opensearch` / `valkey` | Database type. |
| `state` | `available` / `preview` | Availability state. |
| `end_of_life_time` | ISO 8601 timestamp / `null` | When the version reaches end of life. |
| `availability_end_time` | ISO 8601 timestamp / `null` | When the version stops being available for new services. |
| `availability_start_time` | ISO 8601 timestamp | When the version became available. |
| `termination_time` | ISO 8601 timestamp / `null` | When existing services on this version will be terminated. |
| `upgrade_to_service_type` | string / `null` | Recommended service type to upgrade to. |
| `upgrade_to_version` | string / `null` | Recommended version to upgrade to. |
| `upstream_end_of_life_time` | ISO 8601 timestamp / `null` | When the upstream project marks this version EOL. |
