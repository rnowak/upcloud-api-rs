# Managed Load Balancer

> Source: https://developers.upcloud.com/1.3/17-managed-loadbalancer

UpCloud Managed Load Balancer distributes incoming traffic across multiple backend servers, improving availability and scalability. The service supports TCP and HTTP modes, TLS termination via certificate bundles, advanced routing rules with matchers and actions, DNS-based dynamic members, health checks, sticky sessions, and per-service metrics.

---

## Plans

### List Plans

Returns a list of available load balancer plans.

#### Request

`GET /1.3/load-balancer/plans`

#### Query parameters

| Parameter | Accepted value | Default | Description |
|-----------|---------------|---------|-------------|
| `limit` | 0–100 | 10 | Number of entries to return. |
| `offset` | >= 0 | 0 | Offset for pagination. |

#### Response

```json
[
  {
    "name": "development",
    "per_server_max_sessions": 10000,
    "server_number": 1
  },
  {
    "name": "production-small",
    "per_server_max_sessions": 50000,
    "server_number": 2
  }
]
```

#### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `name` | String | Plan name identifier. |
| `per_server_max_sessions` | Integer | Maximum concurrent sessions per server. |
| `server_number` | Integer | Number of backend load balancer servers. |

---

### Get Plan Details

Returns details of a single load balancer plan.

#### Request

`GET /1.3/load-balancer/plans/{name}`

#### Response

```json
{
  "name": "production-small",
  "per_server_max_sessions": 50000,
  "server_number": 2
}
```

#### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `name` | String | Plan name identifier. |
| `per_server_max_sessions` | Integer | Maximum concurrent sessions per server. |
| `server_number` | Integer | Number of backend load balancer servers. |

---

## Services

### Create Service

Creates a new managed load balancer service.

#### Request

`POST /1.3/load-balancer`

```json
{
  "name": "my-lb-service",
  "plan": "production-small",
  "zone": "fi-hel1",
  "configured_status": "started",
  "networks": [
    {
      "name": "public-net",
      "type": "public",
      "family": "IPv4"
    },
    {
      "name": "private-net",
      "type": "private",
      "family": "IPv4",
      "uuid": "03000000-0000-4000-8000-000000000001"
    }
  ],
  "frontends": [
    {
      "name": "http-frontend",
      "mode": "http",
      "port": 80,
      "default_backend": "http-backend",
      "networks": [
        { "name": "public-net" }
      ]
    }
  ],
  "backends": [
    {
      "name": "http-backend",
      "members": [
        {
          "name": "web-server-1",
          "type": "static",
          "ip": "10.0.0.1",
          "port": 8080,
          "weight": 100,
          "max_sessions": 1000,
          "enabled": true
        }
      ]
    }
  ],
  "resolvers": [],
  "labels": [
    { "key": "env", "value": "production" }
  ]
}
```

#### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `name` | 1–64 chars, `^[a-zA-Z0-9_-]+$` | yes | Unique service name. |
| `plan` | Valid plan name | yes | Load balancer plan. |
| `zone` | Valid zone id | yes | Zone where the service is deployed. |
| `configured_status` | `started` / `stopped` | yes | Desired operational status. |
| `networks` | Array of 1–8 network objects | yes | Networks attached to the service. |
| `ip_addresses` | Array of IP address objects | no | Pre-assigned IP addresses. |
| `frontends` | Array of 0–100 frontend objects | no | Frontends for the service. |
| `backends` | Array of 0–100 backend objects | no | Backends for the service. |
| `resolvers` | Array of 0–100 resolver objects | no | DNS resolvers for the service. |
| `labels` | Array of 0–255 label objects | no | Key-value labels. |
| `maintenance_dow` | `monday`–`sunday` | no | Preferred maintenance day of the week. |
| `maintenance_time` | `HH:MM:SSZ` | no | Preferred maintenance time (UTC). |
| `network_uuid` | UUID string | no | **Deprecated.** Use `networks` instead. |

#### Response

`201 Created` — Returns the created service object.

---

### List Services

Lists all load balancer services in the account.

#### Request

`GET /1.3/load-balancer`

#### Query parameters

| Parameter | Accepted value | Default | Description |
|-----------|---------------|---------|-------------|
| `limit` | 0–100 | 10 | Number of entries to return. |
| `offset` | >= 0 | 0 | Offset for pagination. |
| `label` | Label selector string | — | Filter services by label. |

#### Response

```json
[
  {
    "backends": [],
    "configured_status": "started",
    "created_at": "2022-06-15T09:30:00Z",
    "dns_name": "lb-0a1b2c3d4e5f.upcloudlb.com",
    "frontends": [],
    "labels": [],
    "maintenance_dow": "monday",
    "maintenance_time": "03:00:00Z",
    "name": "my-lb-service",
    "networks": [],
    "operational_state": "running",
    "plan": "production-small",
    "resolvers": [],
    "updated_at": "2022-06-15T09:35:00Z",
    "uuid": "0a1b2c3d-4e5f-6789-abcd-ef0123456789",
    "zone": "fi-hel1"
  }
]
```

`200 OK`

---

### Get Service Details

Returns full details of a single load balancer service.

#### Request

`GET /1.3/load-balancer/{service_uuid}`

#### Response

```json
{
  "backends": [
    {
      "created_at": "2022-06-15T09:30:00Z",
      "members": [],
      "name": "http-backend",
      "properties": {
        "health_check_fall": 3,
        "health_check_interval": 10,
        "health_check_rise": 3,
        "health_check_tls_verify": false,
        "health_check_type": "tcp",
        "health_check_url": "/",
        "timeout_server": 10,
        "timeout_tunnel": 3600,
        "tls_enabled": false,
        "tls_use_system_ca": false,
        "tls_verify": false
      },
      "resolver": "",
      "tls_configs": [],
      "updated_at": "2022-06-15T09:30:00Z"
    }
  ],
  "configured_status": "started",
  "created_at": "2022-06-15T09:30:00Z",
  "dns_name": "lb-0a1b2c3d4e5f.upcloudlb.com",
  "frontends": [
    {
      "created_at": "2022-06-15T09:30:00Z",
      "default_backend": "http-backend",
      "mode": "http",
      "name": "http-frontend",
      "networks": [
        { "name": "public-net" }
      ],
      "port": 80,
      "properties": {
        "http2_enabled": false,
        "inbound_proxy_protocol": false,
        "timeout_client": 10
      },
      "rules": [],
      "tls_configs": [],
      "updated_at": "2022-06-15T09:30:00Z"
    }
  ],
  "labels": [],
  "maintenance_dow": "monday",
  "maintenance_time": "03:00:00Z",
  "name": "my-lb-service",
  "networks": [
    {
      "created_at": "2022-06-15T09:30:00Z",
      "dns_name": "lb-0a1b2c3d4e5f-public-net.upcloudlb.com",
      "family": "IPv4",
      "name": "public-net",
      "type": "public",
      "updated_at": "2022-06-15T09:30:00Z"
    }
  ],
  "operational_state": "running",
  "plan": "production-small",
  "resolvers": [],
  "updated_at": "2022-06-15T09:35:00Z",
  "uuid": "0a1b2c3d-4e5f-6789-abcd-ef0123456789",
  "zone": "fi-hel1"
}
```

`200 OK`

---

### Replace Service

Replaces an existing load balancer service. All specified top-level fields are replaced; omitted optional fields are cleared.

#### Request

`PUT /1.3/load-balancer/{service_uuid}`

```json
{
  "name": "my-lb-service-v2",
  "plan": "production-small",
  "configured_status": "started",
  "maintenance_dow": "tuesday",
  "maintenance_time": "04:00:00Z",
  "frontends": [],
  "backends": [],
  "resolvers": [],
  "labels": []
}
```

#### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `name` | 1–64 chars, `^[a-zA-Z0-9_-]+$` | yes | Service name. |
| `plan` | Valid plan name | yes | Load balancer plan. |
| `configured_status` | `started` / `stopped` | yes | Desired operational status. |
| `maintenance_dow` | `monday`–`sunday` | no | Preferred maintenance day. |
| `maintenance_time` | `HH:MM:SSZ` | no | Preferred maintenance time (UTC). |
| `ip_addresses` | Array of IP address objects | no | IP addresses. |
| `frontends` | Array of 0–100 frontend objects | no | Frontends. |
| `backends` | Array of 0–100 backend objects | no | Backends. |
| `resolvers` | Array of 0–100 resolver objects | no | DNS resolvers. |
| `labels` | Array of 0–255 label objects | no | Key-value labels. |

#### Response

`200 OK` — Returns the updated service object.

---

### Modify Service

Partially updates a load balancer service. Only provided fields are updated. Supports JSON Merge Patch semantics — send `null` to delete an optional field.

#### Request

`PATCH /1.3/load-balancer/{service_uuid}`

```json
{
  "name": "my-lb-service-renamed",
  "maintenance_dow": "wednesday"
}
```

All attributes are optional. Accepted attributes are the same as for [Replace Service](#replace-service).

#### Response

`200 OK` — Returns the updated service object.

---

### Delete Service

Deletes a load balancer service and all associated resources.

#### Request

`DELETE /1.3/load-balancer/{service_uuid}`

#### Response

`204 No Content`

---

## Networks

### Network Attributes

Network objects are provided when creating a service via the `networks` array.

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `name` | 1–64 chars, `^[a-zA-Z0-9_-]+$` | yes | Network name within the service. |
| `type` | `public` / `private` | yes | Network type. |
| `family` | `IPv4` | yes | Address family. |
| `uuid` | Valid SDN Private Network UUID | no | Required when `type` is `private`. |

### Modify Network

Modifies an existing network attached to a load balancer service.

#### Request

`PATCH /1.3/load-balancer/{service_uuid}/networks/{network_name}`

```json
{
  "name": "renamed-private-net"
}
```

#### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `name` | 1–64 chars, `^[a-zA-Z0-9_-]+$` | no | New network name. |

#### Response

`200 OK` — Returns the updated network object.

---

## IP Addresses

### List IP Addresses

Lists all IP addresses attached to a load balancer service.

#### Request

`GET /1.3/load-balancer/{service_uuid}/ip-addresses`

#### Response

```json
[
  {
    "address": "94.237.100.10",
    "created_at": "2022-06-15T09:30:00Z",
    "network_name": "public-net",
    "updated_at": "2022-06-15T09:30:00Z"
  }
]
```

`200 OK`

#### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `address` | String | IPv4 address. |
| `created_at` | String | ISO 8601 creation timestamp. |
| `network_name` | String | Name of the network the address belongs to. |
| `updated_at` | String | ISO 8601 last update timestamp. |

---

### Attach IP Address

Attaches an IP address to a load balancer service network.

#### Request

`POST /1.3/load-balancer/{service_uuid}/ip-addresses`

```json
{
  "address": "94.237.100.11",
  "network_name": "public-net"
}
```

#### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `address` | Valid IPv4 address | yes | IP address to attach. |
| `network_name` | Valid network name | yes | Target network name. |

#### Response

`201 Created` — Returns the created IP address object.

---

### Remove IP Address

Removes an IP address from a load balancer service.

#### Request

`DELETE /1.3/load-balancer/{service_uuid}/ip-addresses/{address}`

#### Response

`204 No Content`

---

## Frontends

### Create Frontend

Creates a new frontend for a load balancer service.

#### Request

`POST /1.3/load-balancer/{service_uuid}/frontends`

```json
{
  "name": "https-frontend",
  "mode": "http",
  "port": 443,
  "default_backend": "http-backend",
  "networks": [
    { "name": "public-net" }
  ],
  "rules": [],
  "tls_configs": [
    {
      "name": "my-tls",
      "certificate_bundle_uuid": "0a1b2c3d-1234-5678-abcd-ef0123456789"
    }
  ],
  "properties": {
    "timeout_client": 30,
    "http2_enabled": true,
    "inbound_proxy_protocol": false
  }
}
```

#### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `name` | 1–64 chars, `^[a-zA-Z0-9_-]+$` | yes | Frontend name. |
| `mode` | `tcp` / `http` | yes | Protocol mode. |
| `port` | 1–65535 | yes | Listening port. |
| `default_backend` | Valid backend name | yes | Default backend for unmatched traffic. |
| `networks` | Array of network name objects | yes | Networks to listen on. |
| `rules` | Array of 0–100 rule objects | no | Frontend rules. |
| `tls_configs` | Array of 0–1000 TLS config objects | no | TLS configurations. |
| `properties` | Frontend properties object | no | Frontend properties (see [Frontend Properties](#frontend-properties)). |

#### Response

`201 Created` — Returns the created frontend object.

---

### List Frontends

Lists all frontends of a load balancer service.

#### Request

`GET /1.3/load-balancer/{service_uuid}/frontends`

#### Response

```json
[
  {
    "created_at": "2022-06-15T09:30:00Z",
    "default_backend": "http-backend",
    "mode": "http",
    "name": "https-frontend",
    "networks": [
      { "name": "public-net" }
    ],
    "port": 443,
    "properties": {
      "http2_enabled": true,
      "inbound_proxy_protocol": false,
      "timeout_client": 30
    },
    "rules": [],
    "tls_configs": [],
    "updated_at": "2022-06-15T09:30:00Z"
  }
]
```

`200 OK`

---

### Get Frontend Details

Returns details of a single frontend.

#### Request

`GET /1.3/load-balancer/{service_uuid}/frontends/{frontend_name}`

#### Response

`200 OK` — Returns a frontend object (same structure as the list response element).

---

### Modify Frontend

Partially updates a frontend.

#### Request

`PATCH /1.3/load-balancer/{service_uuid}/frontends/{frontend_name}`

```json
{
  "port": 8443,
  "properties": {
    "timeout_client": 60
  }
}
```

All attributes are optional. Accepted attributes are the same as for [Create Frontend](#create-frontend).

#### Response

`200 OK` — Returns the updated frontend object.

---

### Delete Frontend

Deletes a frontend from a load balancer service.

#### Request

`DELETE /1.3/load-balancer/{service_uuid}/frontends/{frontend_name}`

#### Response

`204 No Content`

---

## Frontend TLS Configs

### Create Frontend TLS Config

Attaches a TLS certificate bundle to a frontend.

#### Request

`POST /1.3/load-balancer/{service_uuid}/frontends/{frontend_name}/tls-configs`

```json
{
  "name": "my-tls-config",
  "certificate_bundle_uuid": "0a1b2c3d-1234-5678-abcd-ef0123456789"
}
```

#### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `name` | 1–64 chars, `^[a-zA-Z0-9_-]+$` | yes | TLS config name. |
| `certificate_bundle_uuid` | Valid certificate bundle UUID | yes | Certificate bundle to use. |

#### Response

`201 Created` — Returns the created TLS config object.

---

### List Frontend TLS Configs

Lists all TLS configs attached to a frontend.

#### Request

`GET /1.3/load-balancer/{service_uuid}/frontends/{frontend_name}/tls-configs`

#### Response

```json
[
  {
    "certificate_bundle_uuid": "0a1b2c3d-1234-5678-abcd-ef0123456789",
    "created_at": "2022-06-15T09:30:00Z",
    "name": "my-tls-config",
    "updated_at": "2022-06-15T09:30:00Z"
  }
]
```

`200 OK`

---

### Get Frontend TLS Config Details

Returns details of a single frontend TLS config.

#### Request

`GET /1.3/load-balancer/{service_uuid}/frontends/{frontend_name}/tls-configs/{tls_config_name}`

#### Response

`200 OK` — Returns a TLS config object.

---

### Modify Frontend TLS Config

Partially updates a frontend TLS config.

#### Request

`PATCH /1.3/load-balancer/{service_uuid}/frontends/{frontend_name}/tls-configs/{tls_config_name}`

```json
{
  "certificate_bundle_uuid": "aaaabbbb-cccc-dddd-eeee-ffffffffffff"
}
```

#### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `name` | 1–64 chars | no | TLS config name. |
| `certificate_bundle_uuid` | Valid UUID | no | Certificate bundle UUID. |

#### Response

`200 OK` — Returns the updated TLS config object.

---

### Delete Frontend TLS Config

Removes a TLS config from a frontend.

#### Request

`DELETE /1.3/load-balancer/{service_uuid}/frontends/{frontend_name}/tls-configs/{tls_config_name}`

#### Response

`204 No Content`

---

## Frontend Rules

### Create Rule

Creates a new rule on a frontend.

#### Request

`POST /1.3/load-balancer/{service_uuid}/frontends/{frontend_name}/rules`

```json
{
  "name": "redirect-to-https",
  "priority": 10,
  "matching_condition": "and",
  "matchers": [
    {
      "type": "path",
      "match_path": {
        "method": "starts",
        "value": "/api"
      }
    }
  ],
  "actions": [
    {
      "type": "http_redirect",
      "action_http_redirect": {
        "scheme": "https",
        "location": ""
      }
    }
  ]
}
```

#### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `name` | 1–64 chars, `^[a-zA-Z0-9_-]+$` | yes | Rule name. |
| `priority` | 0–100 | yes | Evaluation priority (lower = first). |
| `matching_condition` | `and` / `or` | no | Logical operator for matchers (default: `and`). |
| `matchers` | Array of 0–40 matcher objects | yes | Conditions that trigger the rule (see [Rule Matchers](#rule-matchers)). |
| `actions` | Array of 1–40 action objects | yes | Actions to execute when matched (see [Rule Actions](#rule-actions)). |

#### Response

`201 Created` — Returns the created rule object.

---

### List Rules

Lists all rules on a frontend.

#### Request

`GET /1.3/load-balancer/{service_uuid}/frontends/{frontend_name}/rules`

#### Response

```json
[
  {
    "actions": [],
    "created_at": "2022-06-15T09:30:00Z",
    "matching_condition": "and",
    "matchers": [],
    "name": "redirect-to-https",
    "priority": 10,
    "updated_at": "2022-06-15T09:30:00Z"
  }
]
```

`200 OK`

---

### Get Rule Details

Returns details of a single rule.

#### Request

`GET /1.3/load-balancer/{service_uuid}/frontends/{frontend_name}/rules/{rule_name}`

#### Response

`200 OK` — Returns a rule object.

---

### Replace Rule

Replaces an existing rule entirely.

#### Request

`PUT /1.3/load-balancer/{service_uuid}/frontends/{frontend_name}/rules/{rule_name}`

```json
{
  "name": "redirect-to-https",
  "priority": 5,
  "matchers": [
    {
      "type": "host",
      "match_host": {
        "value": "example.com"
      }
    }
  ],
  "actions": [
    {
      "type": "http_redirect",
      "action_http_redirect": {
        "scheme": "https",
        "location": ""
      }
    }
  ]
}
```

#### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `name` | 1–64 chars | yes | Rule name. |
| `priority` | 0–100 | yes | Evaluation priority. |
| `matchers` | Array of 0–40 matcher objects | yes | Conditions. |
| `actions` | Array of 1–40 action objects | yes | Actions. |

#### Response

`200 OK` — Returns the updated rule object.

---

### Modify Rule

Partially updates a rule.

#### Request

`PATCH /1.3/load-balancer/{service_uuid}/frontends/{frontend_name}/rules/{rule_name}`

```json
{
  "priority": 20
}
```

All attributes are optional. Accepted attributes are the same as for [Create Rule](#create-rule).

#### Response

`200 OK` — Returns the updated rule object.

---

### Delete Rule

Deletes a rule from a frontend.

#### Request

`DELETE /1.3/load-balancer/{service_uuid}/frontends/{frontend_name}/rules/{rule_name}`

#### Response

`204 No Content`

---

## Backends

### Create Backend

Creates a new backend for a load balancer service.

#### Request

`POST /1.3/load-balancer/{service_uuid}/backends`

```json
{
  "name": "http-backend",
  "resolver": "my-resolver",
  "members": [
    {
      "name": "web-server-1",
      "type": "static",
      "ip": "10.0.0.1",
      "port": 8080,
      "weight": 100,
      "max_sessions": 1000,
      "enabled": true
    }
  ],
  "properties": {
    "timeout_server": 30,
    "health_check_type": "http",
    "health_check_url": "/health",
    "health_check_expected_status": 200
  },
  "tls_configs": []
}
```

#### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `name` | 1–64 chars, `^[a-zA-Z0-9_-]+$` | yes | Backend name. |
| `resolver` | Valid resolver name | no | DNS resolver for dynamic members. |
| `members` | Array of 0–100 member objects | yes | Backend members. |
| `properties` | Backend properties object | no | Backend properties (see [Backend Properties](#backend-properties)). |
| `tls_configs` | Array of 0–100 TLS config objects | no | TLS configurations for backend connections. |

#### Response

`201 Created` — Returns the created backend object.

---

### List Backends

Lists all backends of a load balancer service.

#### Request

`GET /1.3/load-balancer/{service_uuid}/backends`

#### Response

```json
[
  {
    "created_at": "2022-06-15T09:30:00Z",
    "members": [],
    "name": "http-backend",
    "properties": {
      "health_check_fall": 3,
      "health_check_interval": 10,
      "health_check_rise": 3,
      "health_check_type": "tcp",
      "timeout_server": 10,
      "timeout_tunnel": 3600
    },
    "resolver": "",
    "tls_configs": [],
    "updated_at": "2022-06-15T09:30:00Z"
  }
]
```

`200 OK`

---

### Get Backend Details

Returns details of a single backend.

#### Request

`GET /1.3/load-balancer/{service_uuid}/backends/{backend_name}`

#### Response

`200 OK` — Returns a backend object (same structure as the list response element).

---

### Modify Backend

Partially updates a backend.

#### Request

`PATCH /1.3/load-balancer/{service_uuid}/backends/{backend_name}`

```json
{
  "resolver": "new-resolver",
  "properties": {
    "timeout_server": 60,
    "health_check_type": "http",
    "health_check_url": "/healthz"
  }
}
```

All attributes are optional. Accepted attributes are the same as for [Create Backend](#create-backend).

#### Response

`200 OK` — Returns the updated backend object.

---

### Delete Backend

Deletes a backend from a load balancer service.

#### Request

`DELETE /1.3/load-balancer/{service_uuid}/backends/{backend_name}`

#### Response

`204 No Content`

---

## Backend Members

### Create Static Member

Adds a static member to a backend. Static members use an explicit IP address.

#### Request

`POST /1.3/load-balancer/{service_uuid}/backends/{backend_name}/members`

```json
{
  "name": "web-server-1",
  "type": "static",
  "ip": "10.0.0.1",
  "port": 8080,
  "weight": 100,
  "max_sessions": 1000,
  "enabled": true,
  "backup": false
}
```

#### Request attributes (static)

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `name` | 1–254 chars | yes | Member name. |
| `type` | `static` | yes | Member type. |
| `ip` | Valid IPv4 or IPv6 address | yes | Member IP address. |
| `port` | 1–65535 | yes | Member port. |
| `weight` | 0–100 | yes | Traffic weight (0 = no traffic). |
| `max_sessions` | 0–500000 | yes | Maximum concurrent sessions. |
| `enabled` | Boolean | yes | Whether the member accepts traffic. |
| `backup` | Boolean | no | If true, only used when all non-backup members are down. |

#### Response

`201 Created` — Returns the created member object.

---

### Create Dynamic Member

Adds a dynamic member to a backend. Dynamic members resolve their IP via DNS using the backend's resolver.

#### Request

`POST /1.3/load-balancer/{service_uuid}/backends/{backend_name}/members`

```json
{
  "name": "app.example.com",
  "type": "dynamic",
  "weight": 100,
  "max_sessions": 1000,
  "enabled": true
}
```

#### Request attributes (dynamic)

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `name` | FQDN, 1–254 chars | yes | Fully qualified domain name. |
| `type` | `dynamic` | yes | Member type. |
| `ip` | Valid IPv4 or IPv6 address | no | Optional override IP. |
| `port` | 1–65535 | no | Optional override port. |
| `weight` | 0–100 | yes | Traffic weight. |
| `max_sessions` | 0–500000 | yes | Maximum concurrent sessions. |
| `enabled` | Boolean | yes | Whether the member accepts traffic. |
| `backup` | Boolean | no | If true, acts as backup member. |

#### Response

`201 Created` — Returns the created member object.

---

### List Members

Lists all members of a backend.

#### Request

`GET /1.3/load-balancer/{service_uuid}/backends/{backend_name}/members`

#### Response

```json
[
  {
    "backup": false,
    "created_at": "2022-06-15T09:30:00Z",
    "enabled": true,
    "ip": "10.0.0.1",
    "max_sessions": 1000,
    "name": "web-server-1",
    "port": 8080,
    "type": "static",
    "updated_at": "2022-06-15T09:30:00Z",
    "weight": 100
  }
]
```

`200 OK`

---

### Get Member Details

Returns details of a single backend member.

#### Request

`GET /1.3/load-balancer/{service_uuid}/backends/{backend_name}/members/{member_name}`

#### Response

`200 OK` — Returns a member object.

---

### Modify Static Member

Partially updates a static member.

#### Request

`PATCH /1.3/load-balancer/{service_uuid}/backends/{backend_name}/members/{member_name}`

```json
{
  "ip": "10.0.0.2",
  "weight": 50,
  "enabled": false
}
```

All attributes are optional. Accepted attributes are the same as for [Create Static Member](#create-static-member) (excluding `type`).

#### Response

`200 OK` — Returns the updated member object.

---

### Modify Dynamic Member

Partially updates a dynamic member.

#### Request

`PATCH /1.3/load-balancer/{service_uuid}/backends/{backend_name}/members/{member_name}`

```json
{
  "weight": 50,
  "enabled": false
}
```

All attributes are optional. Accepted attributes are the same as for [Create Dynamic Member](#create-dynamic-member) (excluding `type`).

#### Response

`200 OK` — Returns the updated member object.

---

### Delete Member

Removes a member from a backend.

#### Request

`DELETE /1.3/load-balancer/{service_uuid}/backends/{backend_name}/members/{member_name}`

#### Response

`204 No Content`

---

## Backend TLS Configs

### Create Backend TLS Config

Attaches a TLS certificate bundle to a backend for backend-side TLS.

#### Request

`POST /1.3/load-balancer/{service_uuid}/backends/{backend_name}/tls-configs`

```json
{
  "name": "backend-tls",
  "certificate_bundle_uuid": "0a1b2c3d-1234-5678-abcd-ef0123456789"
}
```

#### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `name` | 1–64 chars, `^[a-zA-Z0-9_-]+$` | yes | TLS config name. |
| `certificate_bundle_uuid` | Valid certificate bundle UUID | yes | Certificate bundle to use. |

#### Response

`201 Created` — Returns the created TLS config object.

---

### List Backend TLS Configs

Lists all TLS configs attached to a backend.

#### Request

`GET /1.3/load-balancer/{service_uuid}/backends/{backend_name}/tls-configs`

#### Response

```json
[
  {
    "certificate_bundle_uuid": "0a1b2c3d-1234-5678-abcd-ef0123456789",
    "created_at": "2022-06-15T09:30:00Z",
    "name": "backend-tls",
    "updated_at": "2022-06-15T09:30:00Z"
  }
]
```

`200 OK`

---

### Get Backend TLS Config Details

Returns details of a single backend TLS config.

#### Request

`GET /1.3/load-balancer/{service_uuid}/backends/{backend_name}/tls-configs/{tls_config_name}`

#### Response

`200 OK` — Returns a TLS config object.

---

### Modify Backend TLS Config

Partially updates a backend TLS config.

#### Request

`PATCH /1.3/load-balancer/{service_uuid}/backends/{backend_name}/tls-configs/{tls_config_name}`

```json
{
  "certificate_bundle_uuid": "aaaabbbb-cccc-dddd-eeee-ffffffffffff"
}
```

#### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `name` | 1–64 chars | no | TLS config name. |
| `certificate_bundle_uuid` | Valid UUID | no | Certificate bundle UUID. |

#### Response

`200 OK` — Returns the updated TLS config object.

---

### Delete Backend TLS Config

Removes a TLS config from a backend.

#### Request

`DELETE /1.3/load-balancer/{service_uuid}/backends/{backend_name}/tls-configs/{tls_config_name}`

#### Response

`204 No Content`

---

## Resolvers

### Create Resolver

Creates a DNS resolver for a load balancer service. Resolvers are used by backends with dynamic members.

#### Request

`POST /1.3/load-balancer/{service_uuid}/resolvers`

```json
{
  "name": "my-resolver",
  "nameservers": ["94.237.127.9", "94.237.40.9"],
  "retries": 5,
  "timeout": 30,
  "timeout_retry": 10,
  "cache_valid": 3600,
  "cache_invalid": 30
}
```

#### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `name` | 1–64 chars, `^[a-zA-Z0-9_-]+$` | yes | Resolver name. |
| `nameservers` | Array of 1–10 IP addresses | yes | DNS nameserver addresses. |
| `retries` | 1–10 | yes | Number of retries on failure. |
| `timeout` | 1–60 (seconds) | yes | Timeout for DNS queries. |
| `timeout_retry` | 1–60 (seconds) | yes | Timeout between retries. |
| `cache_valid` | 1–86400 (seconds) | yes | TTL for valid DNS cache entries. |
| `cache_invalid` | 1–86400 (seconds) | yes | TTL for invalid/failed DNS cache entries. |

#### Response

`201 Created` — Returns the created resolver object.

---

### List Resolvers

Lists all resolvers of a load balancer service.

#### Request

`GET /1.3/load-balancer/{service_uuid}/resolvers`

#### Response

```json
[
  {
    "cache_invalid": 30,
    "cache_valid": 3600,
    "created_at": "2022-06-15T09:30:00Z",
    "name": "my-resolver",
    "nameservers": ["94.237.127.9", "94.237.40.9"],
    "retries": 5,
    "timeout": 30,
    "timeout_retry": 10,
    "updated_at": "2022-06-15T09:30:00Z"
  }
]
```

`200 OK`

---

### Get Resolver Details

Returns details of a single resolver.

#### Request

`GET /1.3/load-balancer/{service_uuid}/resolvers/{resolver_name}`

#### Response

`200 OK` — Returns a resolver object.

---

### Modify Resolver

Partially updates a resolver.

#### Request

`PATCH /1.3/load-balancer/{service_uuid}/resolvers/{resolver_name}`

```json
{
  "retries": 3,
  "timeout": 15
}
```

All attributes are optional. Accepted attributes are the same as for [Create Resolver](#create-resolver).

#### Response

`200 OK` — Returns the updated resolver object.

---

### Delete Resolver

Deletes a resolver from a load balancer service.

#### Request

`DELETE /1.3/load-balancer/{service_uuid}/resolvers/{resolver_name}`

#### Response

`204 No Content`

---

## Service Labels

### Create Service Label

Adds a label to a load balancer service.

#### Request

`POST /1.3/load-balancer/{service_uuid}/labels`

```json
{
  "key": "env",
  "value": "production"
}
```

#### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `key` | 2–32 chars | yes | Label key. |
| `value` | 0–255 chars | yes | Label value. |

#### Response

`201 Created` — Returns the created label object.

---

### List Service Labels

Lists all labels on a load balancer service.

#### Request

`GET /1.3/load-balancer/{service_uuid}/labels`

#### Response

```json
[
  {
    "key": "env",
    "value": "production"
  },
  {
    "key": "team",
    "value": "platform"
  }
]
```

`200 OK`

---

### Get Service Label Details

Returns details of a single service label.

#### Request

`GET /1.3/load-balancer/{service_uuid}/labels/{key}`

#### Response

```json
{
  "key": "env",
  "value": "production"
}
```

`200 OK`

---

### Modify Service Label

Partially updates a service label.

#### Request

`PATCH /1.3/load-balancer/{service_uuid}/labels/{key}`

```json
{
  "value": "staging"
}
```

#### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `key` | 2–32 chars | no | Label key. |
| `value` | 0–255 chars | no | Label value. |

#### Response

`200 OK` — Returns the updated label object.

---

### Delete Service Label

Removes a label from a load balancer service.

#### Request

`DELETE /1.3/load-balancer/{service_uuid}/labels/{key}`

#### Response

`204 No Content`

---

## Metrics

### Get Metrics

Returns an overview of metrics for the load balancer service.

#### Request

`GET /1.3/load-balancer/{service_uuid}/metrics`

#### Response

```json
{
  "frontends": [
    {
      "name": "https-frontend",
      "current_sessions": 42,
      "max_sessions": 50000,
      "total_sessions": 123456
    }
  ],
  "backends": [
    {
      "name": "http-backend",
      "current_sessions": 40,
      "members": [
        {
          "name": "web-server-1",
          "current_sessions": 20,
          "status": "up"
        }
      ]
    }
  ]
}
```

`200 OK`

---

### Get Frontend Metric Series

Returns time-series metric data for a specific frontend.

#### Request

`GET /1.3/load-balancer/{service_uuid}/metrics/series/frontends/{frontend_name}`

#### Query parameters

| Parameter | Accepted value | Description |
|-----------|---------------|-------------|
| `interval` | Duration string (e.g. `5m`, `1h`) | Aggregation interval. |
| `from` | ISO 8601 timestamp | Start of the time range. |
| `to` | ISO 8601 timestamp | End of the time range. |
| `sort` | `asc` / `desc` | Sort order by time. |

#### Response

```json
[
  {
    "timestamp": "2022-06-15T09:00:00Z",
    "current_sessions": 42,
    "request_rate": 150.5,
    "bytes_in": 1048576,
    "bytes_out": 5242880
  }
]
```

`200 OK`

---

### Get Combined Frontend Metric Series

Returns time-series metric data aggregated across all frontends.

#### Request

`GET /1.3/load-balancer/{service_uuid}/metrics/series/frontends`

#### Query parameters

Same as [Get Frontend Metric Series](#get-frontend-metric-series).

#### Response

`200 OK` — Returns an array of series data points (same structure as single frontend series).

---

### Get Backend Metric Series

Returns time-series metric data for a specific backend.

#### Request

`GET /1.3/load-balancer/{service_uuid}/metrics/series/backends/{backend_name}`

#### Query parameters

Same as [Get Frontend Metric Series](#get-frontend-metric-series).

#### Response

```json
[
  {
    "timestamp": "2022-06-15T09:00:00Z",
    "current_sessions": 40,
    "request_rate": 145.2,
    "bytes_in": 1000000,
    "bytes_out": 5000000
  }
]
```

`200 OK`

---

### Get Combined Backend Metric Series

Returns time-series metric data aggregated across all backends.

#### Request

`GET /1.3/load-balancer/{service_uuid}/metrics/series/backends`

#### Query parameters

Same as [Get Frontend Metric Series](#get-frontend-metric-series).

#### Response

`200 OK` — Returns an array of series data points.

---

### Get Backend Member Metric Series

Returns time-series metric data for a specific backend member.

#### Request

`GET /1.3/load-balancer/{service_uuid}/metrics/series/backends/{backend_name}/members/{member_name}`

#### Query parameters

Same as [Get Frontend Metric Series](#get-frontend-metric-series).

#### Response

```json
[
  {
    "timestamp": "2022-06-15T09:00:00Z",
    "current_sessions": 20,
    "request_rate": 72.0,
    "status": "up"
  }
]
```

`200 OK`

---

### Get Combined Backend Members Metric Series

Returns time-series metric data aggregated across all members of a backend.

#### Request

`GET /1.3/load-balancer/{service_uuid}/metrics/series/backends/{backend_name}/members`

#### Query parameters

Same as [Get Frontend Metric Series](#get-frontend-metric-series).

#### Response

`200 OK` — Returns an array of series data points.

---

## Certificate Bundles

### Create Manual Certificate Bundle

Creates a certificate bundle with manually provided certificate and private key.

#### Request

`POST /1.3/load-balancer/certificate-bundles`

```json
{
  "name": "my-manual-cert",
  "type": "manual",
  "certificate": "LS0tLS1CRUdJTi...",
  "intermediates": "LS0tLS1CRUdJTi...",
  "private_key": "LS0tLS1CRUdJTi...",
  "labels": [
    { "key": "env", "value": "production" }
  ]
}
```

#### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `name` | 1–255 chars | yes | Certificate bundle name. |
| `type` | `manual` | yes | Bundle type. |
| `certificate` | Base64-encoded PEM | yes | Server certificate. |
| `intermediates` | Base64-encoded PEM | no | Intermediate CA certificates. |
| `private_key` | Base64-encoded PEM | yes | Private key for the certificate. |
| `labels` | Array of label objects | no | Key-value labels. |

#### Response

`201 Created` — Returns the created certificate bundle object (private key is not returned).

---

### Create Dynamic Certificate Bundle

Creates a certificate bundle that is automatically provisioned via ACME (Let's Encrypt).

#### Request

`POST /1.3/load-balancer/certificate-bundles`

```json
{
  "name": "my-dynamic-cert",
  "type": "dynamic",
  "hostnames": ["example.com", "www.example.com"],
  "key_type": "rsa",
  "labels": []
}
```

#### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `name` | 1–255 chars | yes | Certificate bundle name. |
| `type` | `dynamic` | yes | Bundle type. |
| `hostnames` | Array of 1–100 hostnames | yes | Hostnames to include in the certificate. |
| `key_type` | `rsa` / `ecdsa` | yes | Key type for the generated certificate. |
| `labels` | Array of label objects | no | Key-value labels. |

#### Response

`201 Created` — Returns the created certificate bundle object.

---

### Create Authority Certificate Bundle

Creates a certificate bundle containing a Certificate Authority (CA) certificate for backend TLS verification.

#### Request

`POST /1.3/load-balancer/certificate-bundles`

```json
{
  "name": "my-ca-cert",
  "type": "authority",
  "certificate": "LS0tLS1CRUdJTi...",
  "labels": []
}
```

#### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `name` | 1–255 chars | yes | Certificate bundle name. |
| `type` | `authority` | yes | Bundle type. |
| `certificate` | Base64-encoded PEM | yes | CA certificate. |
| `labels` | Array of label objects | no | Key-value labels. |

#### Response

`201 Created` — Returns the created certificate bundle object.

---

### List Certificate Bundles

Lists all certificate bundles in the account.

#### Request

`GET /1.3/load-balancer/certificate-bundles`

#### Query parameters

| Parameter | Accepted value | Default | Description |
|-----------|---------------|---------|-------------|
| `limit` | 0–100 | 10 | Number of entries to return. |
| `offset` | >= 0 | 0 | Offset for pagination. |
| `filter_service_uuid` | Valid UUID | — | Filter by associated service. |
| `filter_name_or_hostname` | String | — | Filter by bundle name or hostname. |
| `filter_type` | `manual` / `dynamic` / `authority` | — | Filter by bundle type. |

#### Response

```json
[
  {
    "certificate": "LS0tLS1CRUdJTi...",
    "created_at": "2022-06-15T09:30:00Z",
    "hostnames": [],
    "intermediates": "LS0tLS1CRUdJTi...",
    "key_type": "",
    "labels": [],
    "name": "my-manual-cert",
    "not_after": "2023-06-15T09:30:00Z",
    "not_before": "2022-06-15T09:30:00Z",
    "operational_state": "",
    "type": "manual",
    "updated_at": "2022-06-15T09:30:00Z",
    "uuid": "0a1b2c3d-1234-5678-abcd-ef0123456789"
  }
]
```

`200 OK`

---

### Get Certificate Bundle Details

Returns details of a single certificate bundle.

#### Request

`GET /1.3/load-balancer/certificate-bundles/{certificate_bundle_uuid}`

#### Response

`200 OK` — Returns a certificate bundle object.

---

### Modify Manual Certificate Bundle

Partially updates a manual certificate bundle.

#### Request

`PATCH /1.3/load-balancer/certificate-bundles/{certificate_bundle_uuid}`

```json
{
  "name": "my-manual-cert-updated",
  "certificate": "LS0tLS1CRUdJTi...",
  "intermediates": "LS0tLS1CRUdJTi...",
  "private_key": "LS0tLS1CRUdJTi..."
}
```

#### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `name` | 1–255 chars | no | Bundle name. |
| `certificate` | Base64-encoded PEM | no | Server certificate. |
| `intermediates` | Base64-encoded PEM | no | Intermediate CA certificates. |
| `private_key` | Base64-encoded PEM | no | Private key. |
| `labels` | Array of label objects | no | Key-value labels. |

#### Response

`200 OK` — Returns the updated certificate bundle object.

---

### Modify Dynamic Certificate Bundle

Partially updates a dynamic certificate bundle.

#### Request

`PATCH /1.3/load-balancer/certificate-bundles/{certificate_bundle_uuid}`

```json
{
  "name": "my-dynamic-cert-updated",
  "hostnames": ["new.example.com"]
}
```

#### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `name` | 1–255 chars | no | Bundle name. |
| `hostnames` | Array of 1–100 hostnames | no | Hostnames for the certificate. |
| `labels` | Array of label objects | no | Key-value labels. |

#### Response

`200 OK` — Returns the updated certificate bundle object.

---

### Modify Authority Certificate Bundle

Partially updates an authority certificate bundle.

#### Request

`PATCH /1.3/load-balancer/certificate-bundles/{certificate_bundle_uuid}`

```json
{
  "name": "my-ca-cert-updated",
  "certificate": "LS0tLS1CRUdJTi..."
}
```

#### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `name` | 1–255 chars | no | Bundle name. |
| `certificate` | Base64-encoded PEM | no | CA certificate. |
| `labels` | Array of label objects | no | Key-value labels. |

#### Response

`200 OK` — Returns the updated certificate bundle object.

---

### Delete Certificate Bundle

Deletes a certificate bundle.

#### Request

`DELETE /1.3/load-balancer/certificate-bundles/{certificate_bundle_uuid}`

#### Response

`204 No Content`

---

## Certificate Bundle Labels

### Create Certificate Bundle Label

Adds a label to a certificate bundle.

#### Request

`POST /1.3/load-balancer/certificate-bundles/{certificate_bundle_uuid}/labels`

```json
{
  "key": "env",
  "value": "production"
}
```

#### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `key` | 2–32 chars | yes | Label key. |
| `value` | 0–255 chars | yes | Label value. |

#### Response

`201 Created` — Returns the created label object.

---

### List Certificate Bundle Labels

Lists all labels on a certificate bundle.

#### Request

`GET /1.3/load-balancer/certificate-bundles/{certificate_bundle_uuid}/labels`

#### Response

```json
[
  {
    "key": "env",
    "value": "production"
  }
]
```

`200 OK`

---

### Get Certificate Bundle Label Details

Returns details of a single certificate bundle label.

#### Request

`GET /1.3/load-balancer/certificate-bundles/{certificate_bundle_uuid}/labels/{key}`

#### Response

```json
{
  "key": "env",
  "value": "production"
}
```

`200 OK`

---

### Modify Certificate Bundle Label

Partially updates a certificate bundle label.

#### Request

`PATCH /1.3/load-balancer/certificate-bundles/{certificate_bundle_uuid}/labels/{key}`

```json
{
  "value": "staging"
}
```

#### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `key` | 2–32 chars | no | Label key. |
| `value` | 0–255 chars | no | Label value. |

#### Response

`200 OK` — Returns the updated label object.

---

### Delete Certificate Bundle Label

Removes a label from a certificate bundle.

#### Request

`DELETE /1.3/load-balancer/certificate-bundles/{certificate_bundle_uuid}/labels/{key}`

#### Response

`204 No Content`

---

## DNS Challenge

### Get DNS Challenge Domain

Returns the DNS challenge domain used for ACME DNS-01 validation when provisioning dynamic certificate bundles.

#### Request

`GET /1.3/load-balancer/certificate-bundles/dns-challenge-domain`

#### Response

```json
{
  "domain": "_acme-challenge.xxxxx.upcloudlb.com"
}
```

`200 OK`

#### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `domain` | String | The CNAME target for `_acme-challenge` DNS records. |

---

## Rule Matchers

Rule matchers define conditions that incoming requests must satisfy for a rule to be applied. Multiple matchers can be combined using the `matching_condition` field (`and` / `or`).

| Matcher type | Description | Frontend mode |
|-------------|-------------|---------------|
| `body_size` | Match by request body size. | `http` |
| `cookie` | Match by cookie name/value. | `http` |
| `header` | Match by request header. | `http` |
| `host` | Match by `Host` header. | `http` |
| `http_method` | Match by HTTP method (GET, POST, etc.). | `http` |
| `http_status` | Match by response HTTP status code. | `http` |
| `path` | Match by URL path. | `http` |
| `request_header` | Match by request header (alias for `header`). | `http` |
| `response_header` | Match by response header. | `http` |
| `src_ip` | Match by source IP address. | `tcp`, `http` |
| `src_port` | Match by source port. | `tcp`, `http` |
| `url` | Match by full URL. | `http` |
| `url_query` | Match by URL query string. | `http` |
| `url_param` | Match by URL query parameter. | `http` |
| `num_members_up` | Match by number of healthy backend members. | `tcp`, `http` |

### Matcher object structure

```json
{
  "type": "path",
  "match_path": {
    "method": "starts",
    "value": "/api"
  }
}
```

The inner match object key is `match_{type}` (e.g. `match_path`, `match_host`, `match_src_ip`). Common match methods include: `exact`, `starts`, `ends`, `contains`, `regex`, `exists`, `greater`, `less`, `range`, `inverse`.

---

## Rule Actions

Rule actions define what happens when a rule's matchers are satisfied.

| Action type | Description | Frontend mode |
|------------|-------------|---------------|
| `use_backend` | Route traffic to a specified backend. | `tcp`, `http` |
| `tcp_reject` | Reject the TCP connection. | `tcp`, `http` |
| `http_return` | Return a static HTTP response. | `http` |
| `http_redirect` | Redirect the request to another URL. | `http` |
| `http_rewrite_path` | Rewrite the URL path. | `http` |
| `http_rewrite_uri` | Rewrite the full URI. | `http` |
| `set_forwarded_headers` | Set `X-Forwarded-*` headers. | `http` |
| `set_request_header` | Set or modify a request header before forwarding. | `http` |
| `set_response_header` | Set or modify a response header before returning. | `http` |

### Action object examples

**use_backend:**
```json
{
  "type": "use_backend",
  "action_use_backend": {
    "backend": "api-backend"
  }
}
```

**http_redirect:**
```json
{
  "type": "http_redirect",
  "action_http_redirect": {
    "scheme": "https",
    "location": ""
  }
}
```

**http_return:**
```json
{
  "type": "http_return",
  "action_http_return": {
    "status": 503,
    "content_type": "text/plain",
    "payload": "Service unavailable"
  }
}
```

**set_request_header:**
```json
{
  "type": "set_request_header",
  "action_set_request_header": {
    "header": "X-Custom-Header",
    "value": "my-value"
  }
}
```

**http_rewrite_path:**
```json
{
  "type": "http_rewrite_path",
  "action_http_rewrite_path": {
    "path": "/v2%[path]"
  }
}
```

---

## Frontend Properties

Frontend properties control timeouts and protocol settings on the frontend.

| Property | Accepted value | Default | Description |
|----------|---------------|---------|-------------|
| `timeout_client` | 1–86400 (seconds) | 10 | Client connection timeout. |
| `inbound_proxy_protocol` | Boolean | `false` | Enable PROXY protocol for inbound connections. |
| `http2_enabled` | Boolean | `false` | Enable HTTP/2 support. |

---

## Backend Properties

Backend properties control health checks, timeouts, TLS, sticky sessions, and protocol settings.

| Property | Accepted value | Default | Description |
|----------|---------------|---------|-------------|
| `timeout_server` | 1–86400 (seconds) | 10 | Server connection timeout. |
| `timeout_tunnel` | 1–3024000 (seconds) | 3600 | Tunnel (WebSocket/long-poll) timeout. |
| `health_check_type` | `tcp` / `http` | `tcp` | Health check protocol. |
| `health_check_interval` | 1–86400 (seconds) | 10 | Interval between health checks. |
| `health_check_fall` | 1–100 | 3 | Number of consecutive failures to mark member as down. |
| `health_check_rise` | 1–100 | 3 | Number of consecutive successes to mark member as up. |
| `health_check_url` | 1–255 chars | `/` | URL path for HTTP health checks. |
| `health_check_tls_verify` | Boolean | `false` | Verify TLS certificate during health checks. |
| `health_check_expected_status` | 100–599 | 200 | Expected HTTP status code for a healthy response. |
| `health_check_on_down` | `""` / `shutdown-sessions` | `""` | Action when member goes down. `shutdown-sessions` terminates active sessions. |
| `sticky_session_cookie_name` | String | — | Cookie name for sticky sessions. When set, enables cookie-based session persistence. |
| `sticky_session_cookie_attributes` | String | — | Additional cookie attributes (e.g. `SameSite=None; Secure`). |
| `outbound_proxy_protocol` | `v1` / `v2` / `null` | `null` | PROXY protocol version for outbound connections to members. |
| `tls_enabled` | Boolean | `false` | Enable TLS for backend connections. |
| `tls_verify` | Boolean | `false` | Verify backend server TLS certificates. |
| `tls_use_system_ca` | Boolean | `false` | Use system CA store for TLS verification. |
| `http2_enabled` | Boolean | `false` | Enable HTTP/2 for backend connections. |

---

## Service Configured Status

| Status | Description |
|--------|-------------|
| `started` | The load balancer service is configured to run. |
| `stopped` | The load balancer service is configured to be stopped. |

---

## Service Operational State

The operational state reflects the current lifecycle phase of the service.

| State | Description |
|-------|-------------|
| `pending` | Service creation request has been accepted. |
| `setup-agent` | Provisioning the management agent. |
| `setup-server` | Provisioning backend servers. |
| `setup-network` | Configuring network interfaces. |
| `setup-lb` | Configuring load balancer software. |
| `setup-dns` | Configuring DNS records. |
| `checkup` | Running post-setup verification. |
| `running` | Service is operational and handling traffic. |
| `delete-dns` | Removing DNS records during deletion. |
| `delete-network` | Removing network configuration during deletion. |
| `delete-server` | Removing backend servers during deletion. |
| `delete-service` | Finalising service deletion. |

---

## Dynamic Certificate Bundle Operational State

| State | Description |
|-------|-------------|
| `idle` | No pending certificate operations. |
| `pending` | Certificate request has been submitted. |
| `setup-challenge` | ACME DNS-01 challenge is being set up. |
| `complete-challenge` | ACME challenge has been completed; certificate is being issued. |
