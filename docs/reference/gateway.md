# Network Gateways

> Source: https://developers.upcloud.com/1.3/19-network-gateways

Network gateways connect SDN Private Networks to external IP networks. Gateways have VPN and NAT functionalities that can be enabled either separately or at the same time. The VPN functionality offers a way to establish encrypted connections using site-to-site IPSec VPN connections between SDN Private Networks and remote networks. The NAT gateway functionality enables outbound Internet connections from Cloud Servers in SDN Private Networks.

The network gateway service is attached to an SDN router. It can be utilized by one or more SDN private networks attached to the same router.

### Available features

| Feature | Description |
|---------|-------------|
| `nat` | NAT gateway — Network Address Translation service that lets cloud servers in SDN private networks connect to the Internet through the public IP assigned to the network gateway service. |
| `vpn` | VPN gateway — Virtual Private Network service used to establish an encrypted network connection when using public networks. |

> **Note:** VPN MSS value is set as 1350 bytes and MTU as 1400 bytes.

---

## List Plans

Returns a list of available gateway plans.

### Request

`GET /1.3/gateway/plans`

### Response

```json
[
  {
    "name": "advanced",
    "per_gateway_bandwidth_mbps": 10000,
    "per_gateway_max_connections": 100000,
    "server_number": 2,
    "supported_features": ["nat", "vpn"],
    "vpn_tunnel_amount": 10
  },
  {
    "name": "development",
    "per_gateway_bandwidth_mbps": 10,
    "per_gateway_max_connections": 10000,
    "server_number": 1,
    "supported_features": ["nat"],
    "vpn_tunnel_amount": 0
  },
  {
    "name": "production",
    "per_gateway_bandwidth_mbps": 1000,
    "per_gateway_max_connections": 50000,
    "server_number": 2,
    "supported_features": ["nat", "vpn"],
    "vpn_tunnel_amount": 2
  },
  {
    "name": "standard",
    "per_gateway_bandwidth_mbps": 500,
    "per_gateway_max_connections": 20000,
    "server_number": 2,
    "supported_features": ["nat"],
    "vpn_tunnel_amount": 0
  }
]
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `name` | String | Plan name identifier. |
| `per_gateway_bandwidth_mbps` | Integer | Maximum bandwidth in Mbps. |
| `per_gateway_max_connections` | Integer | Maximum number of concurrent connections. |
| `server_number` | Integer | Number of backend servers. |
| `supported_features` | Array of String | Features available on this plan (`nat`, `vpn`). |
| `vpn_tunnel_amount` | Integer | Number of VPN tunnels allowed. |

---

## Get Plan Details

Returns gateway plan details by given `{name}`.

### Request

`GET /1.3/gateway/plans/{name}`

### Response

```json
{
  "name": "advanced",
  "per_gateway_bandwidth_mbps": 10000,
  "per_gateway_max_connections": 100000,
  "server_number": 2,
  "supported_features": ["nat", "vpn"],
  "vpn_tunnel_amount": 10
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `name` | String | Plan name identifier. |
| `per_gateway_bandwidth_mbps` | Integer | Maximum bandwidth in Mbps. |
| `per_gateway_max_connections` | Integer | Maximum number of concurrent connections. |
| `server_number` | Integer | Number of backend servers. |
| `supported_features` | Array of String | Features available on this plan (`nat`, `vpn`). |
| `vpn_tunnel_amount` | Integer | Number of VPN tunnels allowed. |

---

## List Services

Returns a list of network gateway services.

### Request

`GET /1.3/gateway`

### Request attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `limit` | Integer (0–100) | Number of entries to receive at most. Default: `10`. |
| `offset` | Integer (≥ 0) | Offset for retrieved results. Default: `0`. |
| `label` | String | Filter by label. |

### Response

```json
[
  {
    "addresses": [
      {
        "address": "100.127.0.241",
        "name": "public-ip-1"
      }
    ],
    "configured_status": "started",
    "created_at": "2022-12-01T09:04:08.529138Z",
    "features": ["nat", "vpn"],
    "connections": [
      {
        "uuid": "1085d477-8d8f-4c97-9bef-731933187538",
        "name": "example-connection",
        "type": "ipsec",
        "local_routes": [
          {
            "name": "upcloud-example-route",
            "type": "static",
            "static_network": "10.0.0.0/24"
          }
        ],
        "remote_routes": [
          {
            "name": "remote-example-route",
            "type": "static",
            "static_network": "10.0.1.0/24"
          }
        ],
        "tunnels": [
          {
            "local_address": { "name": "public-ip-1" },
            "name": "example-tunnel-1",
            "uuid": "1085d477-8d8f-4c97-9bef-731933187777",
            "remote_address": { "address": "100.10.0.111" },
            "ipsec": {
              "authentication": { "authentication": "psk" },
              "child_rekey_time": 1440,
              "dpd_delay": 30,
              "dpd_timeout": 120,
              "ike_lifetime": 86400,
              "phase1_algorithms": ["aes128gcm128", "aes256gcm128"],
              "phase1_dh_group_numbers": [14, 16, 18, 19, 20, 21],
              "phase1_integrity_algorithms": ["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"],
              "phase2_algorithms": ["aes128gcm128", "aes256gcm128"],
              "phase2_dh_group_numbers": [14, 16, 18, 19, 20, 21],
              "phase2_integrity_algorithms": ["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"],
              "rekey_time": 14400
            },
            "tunnel_healthy": true,
            "tunnel_up": true,
            "tunnel_internal_ip": "169.254.17.1",
            "internal_peer_ping_interval": 0,
            "operational_state": "established",
            "created_at": "2022-12-01T09:04:08.529138Z",
            "updated_at": "2022-12-01T19:04:08.529138Z"
          }
        ],
        "created_at": "2022-12-01T09:04:08.529138Z",
        "updated_at": "2022-12-01T19:04:08.529138Z"
      }
    ],
    "name": "example-gateway",
    "operational_state": "running",
    "plan": "production",
    "routers": [
      {
        "created_at": "2022-12-01T09:04:08.529138Z",
        "uuid": "1085d477-8d8f-4c97-9bef-731933187538"
      }
    ],
    "labels": [
      { "key": "env", "value": "testing" }
    ],
    "automatic_tunnel_internal_ip_allocation": true,
    "updated_at": "2022-12-01T19:04:08.529138Z",
    "uuid": "10c153e0-12e4-4dea-8748-4f34850ff55d",
    "zone": "fi-hel1"
  }
]
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `uuid` | UUID | Unique identifier for the gateway. |
| `name` | String | Name of the gateway service (unique within account). |
| `zone` | String | Zone in which the service is hosted (e.g. `fi-hel1`). |
| `plan` | String | Plan name (e.g. `production`). |
| `features` | Array of String | Enabled gateway features (`nat`, `vpn`). |
| `configured_status` | String | Service status managed by the customer (`started` / `stopped`). |
| `operational_state` | String | Current operational lifecycle state managed by the system. |
| `routers` | Array of Object | Attached routers. |
| `addresses` | Array of Object | Gateway public IP addresses. |
| `connections` | Array of Object | VPN connections (only for VPN gateways). |
| `labels` | Array of Object | Key-value labels attached to the gateway. |
| `automatic_tunnel_internal_ip_allocation` | Boolean | Whether internal tunnel IPs are automatically allocated. |
| `created_at` | String (datetime) | Creation timestamp. |
| `updated_at` | String (datetime) | Last update timestamp. |

---

## Get Service Details

Returns network gateway service by given `{service_uuid}`.

### Request

`GET /1.3/gateway/{service_uuid}`

### Response

```json
{
  "addresses": [
    {
      "address": "100.127.0.241",
      "name": "public-ip-1"
    }
  ],
  "configured_status": "started",
  "created_at": "2022-12-01T09:04:08.529138Z",
  "features": ["nat", "vpn"],
  "connections": [
    {
      "name": "example-connection",
      "uuid": "1085d477-8d8f-4c97-9bef-731933187538",
      "type": "ipsec",
      "local_routes": [
        {
          "name": "upcloud-example-route",
          "type": "static",
          "static_network": "10.0.0.0/24"
        }
      ],
      "remote_routes": [
        {
          "name": "remote-example-route",
          "type": "static",
          "static_network": "10.0.1.0/24"
        }
      ],
      "tunnels": [
        {
          "local_address": { "name": "public-ip-1" },
          "name": "example-tunnel-1",
          "uuid": "1085d477-8d8f-4c97-9bef-731933187777",
          "remote_address": { "address": "100.10.0.111" },
          "ipsec": {
            "authentication": { "authentication": "psk" },
            "child_rekey_time": 1440,
            "dpd_delay": 30,
            "dpd_timeout": 120,
            "ike_lifetime": 86400,
            "phase1_algorithms": ["aes128gcm128", "aes256gcm128"],
            "phase1_dh_group_numbers": [14, 16, 18, 19, 20, 21],
            "phase1_integrity_algorithms": ["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"],
            "phase2_algorithms": ["aes128gcm128", "aes256gcm128"],
            "phase2_dh_group_numbers": [14, 16, 18, 19, 20, 21],
            "phase2_integrity_algorithms": ["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"],
            "rekey_time": 14400
          },
          "tunnel_healthy": true,
          "tunnel_up": true,
          "tunnel_internal_ip": "169.254.17.1",
          "internal_peer_ping_interval": 0,
          "operational_state": "established",
          "created_at": "2022-12-01T09:04:08.529138Z",
          "updated_at": "2022-12-01T19:04:08.529138Z"
        }
      ],
      "created_at": "2022-12-01T09:04:08.529138Z",
      "updated_at": "2022-12-01T19:04:08.529138Z"
    }
  ],
  "name": "example-gateway",
  "operational_state": "running",
  "plan": "production",
  "routers": [
    {
      "created_at": "2022-12-01T09:04:08.529138Z",
      "uuid": "1085d477-8d8f-4c97-9bef-731933187538"
    }
  ],
  "labels": [
    { "key": "env", "value": "testing" }
  ],
  "automatic_tunnel_internal_ip_allocation": true,
  "updated_at": "2022-12-01T19:04:08.529138Z",
  "uuid": "10c153e0-12e4-4dea-8748-4f34850ff55d",
  "zone": "fi-hel1"
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `uuid` | UUID | Unique identifier for the gateway. |
| `name` | String | Name of the gateway service (unique within account). |
| `zone` | String | Zone in which the service is hosted (e.g. `fi-hel1`). |
| `plan` | String | Plan name (e.g. `production`). |
| `features` | Array of String | Enabled gateway features (`nat`, `vpn`). |
| `configured_status` | String | Service status managed by the customer (`started` / `stopped`). |
| `operational_state` | String | Current operational lifecycle state managed by the system. |
| `routers` | Array of Object | Attached routers. |
| `addresses` | Array of Object | Gateway public IP addresses. |
| `connections` | Array of Object | VPN connections (only for VPN gateways). |
| `labels` | Array of Object | Key-value labels attached to the gateway. |
| `automatic_tunnel_internal_ip_allocation` | Boolean | Whether internal tunnel IPs are automatically allocated. |
| `created_at` | String (datetime) | Creation timestamp. |
| `updated_at` | String (datetime) | Last update timestamp. |

---

## Create Service

Creates a new network gateway service.

### Request

`POST /1.3/gateway`

```json
{
  "name": "example-gateway",
  "zone": "fi-hel1",
  "features": ["nat", "vpn"],
  "plan": "production",
  "routers": [
    { "uuid": "1085d477-8d8f-4c97-9bef-731933187538" }
  ],
  "addresses": [
    { "name": "my-public-ip" }
  ],
  "connections": [
    {
      "name": "example-connection",
      "type": "ipsec",
      "local_routes": [
        {
          "name": "upcloud-example-route",
          "type": "static",
          "static_network": "10.0.0.0/24"
        }
      ],
      "remote_routes": [
        {
          "name": "remote-example-route",
          "type": "static",
          "static_network": "10.0.1.0/24"
        }
      ],
      "tunnels": [
        {
          "name": "example-tunnel-1",
          "local_address": { "name": "public-ip-1" },
          "remote_address": { "address": "100.10.0.111" },
          "ipsec": {
            "authentication": {
              "authentication": "psk",
              "psk": "{your PSK}"
            }
          },
          "tunnel_internal_ip": "169.254.17.1",
          "internal_peer_ping_interval": 5
        }
      ]
    }
  ],
  "labels": [
    { "key": "env", "value": "testing" }
  ],
  "configured_status": "started",
  "automatic_tunnel_internal_ip_allocation": false
}
```

### Request attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `name` | String (1–64 chars, `^[a-zA-Z0-9_-]+$`) | **Required.** Name of the service (unique within account). |
| `features` | Array of String | **Required.** Gateway features to enable (`nat`, `vpn`). |
| `zone` | String | **Required.** Zone identifier (e.g. `fi-hel1`). Only public zones supported. |
| `plan` | String | Plan name (e.g. `production`). Default plan only supports NAT. |
| `routers` | Array of Object | **Required.** Attached routers (currently only one supported). |
| `configured_status` | String | **Required.** Desired initial status (`started` / `stopped`). |
| `connections` | Array of Object | Initial VPN connections. |
| `addresses` | Array of Object | Gateway public IP addresses (currently only one supported). |
| `labels` | Array of Object | Key-value labels. |
| `automatic_tunnel_internal_ip_allocation` | Boolean | Automatically assign tunnel internal IPs (defaults to `true` on creation). |

### Response

`HTTP/1.1 201 Created`

---

## Replace Service

Replaces an existing network gateway service by given `{service_uuid}`.

### Request

`PUT /1.3/gateway/{service_uuid}`

```json
{
  "name": "example-gateway",
  "configured_status": "started"
}
```

### Request attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `name` | String (1–64 chars, `^[a-zA-Z0-9_-]+$`) | **Required.** Name of the service (unique within account). |
| `configured_status` | String | **Required.** Service status (`started` / `stopped`). |
| `labels` | Array of Object | Key-value labels. |

### Response

`HTTP/1.1 200 OK`

---

## Modify Service

Modifies an existing network gateway service by given `{service_uuid}`.

### Request

`PATCH /1.3/gateway/{service_uuid}`

```json
{
  "name": "example-gateway",
  "configured_status": "started",
  "labels": [
    { "key": "env", "value": "testing" }
  ]
}
```

### Request attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `name` | String (1–64 chars, `^[a-zA-Z0-9_-]+$`) | Name of the service (unique within account). |
| `configured_status` | String | Service status (`started` / `stopped`). |
| `labels` | Array of Object | Key-value labels. |
| `features` | Array of String | Gateway features to enable (`nat`, `vpn`). |
| `plan` | String | Plan name (e.g. `production`). |
| `connections` | Array of Object | VPN connections. |
| `automatic_tunnel_internal_ip_allocation` | Boolean | Automatically assign tunnel internal IPs. |

### Response

`HTTP/1.1 200 OK`

---

## Delete Service

Deletes an existing network gateway service by given `{service_uuid}`.

### Request

`DELETE /1.3/gateway/{service_uuid}`

### Response

`HTTP/1.1 204 No Content`

---

## Create Service Label

Creates a new label on a gateway by given `{service_uuid}`.

### Request

`POST /1.3/gateway/{service_uuid}/labels`

```json
{
  "key": "foo",
  "value": "bar"
}
```

### Request attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `key` | String (2–32 printable ASCII, must not start with `_`) | **Required.** Label identifier (unique within a service). |
| `value` | String (0–255 chars) | **Required.** Label value. |

### Response

`HTTP/1.1 201 Created`

---

## List Service Labels

Returns a list of available service labels by given `{service_uuid}`.

### Request

`GET /1.3/gateway/{service_uuid}/labels`

### Response

```json
[
  { "key": "env", "value": "staging" },
  { "key": "foo", "value": "bar" }
]
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `key` | String | Label identifier. |
| `value` | String | Label value. |

---

## Get Service Label Details

Returns label details by given `{service_uuid}` and `{key}`.

### Request

`GET /1.3/gateway/{service_uuid}/labels/{key}`

### Response

```json
{
  "key": "env",
  "value": "staging"
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `key` | String | Label identifier. |
| `value` | String | Label value. |

---

## Modify Service Label

Modifies existing label by given `{service_uuid}` and `{key}`.

### Request

`PATCH /1.3/gateway/{service_uuid}/labels/{key}`

```json
{
  "key": "env",
  "value": "production"
}
```

### Request attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `key` | String (2–32 printable ASCII, must not start with `_`) | Label identifier (unique within a service). |
| `value` | String (0–255 chars) or `null` | Label value. |

### Response

`HTTP/1.1 200 OK`

---

## Delete Service Label

Deletes existing label by given `{service_uuid}` and `{key}`.

### Request

`DELETE /1.3/gateway/{service_uuid}/labels/{key}`

### Response

`HTTP/1.1 204 No Content`

---

## List Connections

Returns a list of network gateway connections.

### Request

`GET /1.3/gateway/{service_uuid}/connections`

### Response

```json
[
  {
    "name": "example-connection",
    "uuid": "1085d477-8d8f-4c97-9bef-731933187538",
    "type": "ipsec",
    "local_routes": [
      {
        "name": "upcloud-example-route",
        "type": "static",
        "static_network": "10.0.0.0/24"
      }
    ],
    "remote_routes": [
      {
        "name": "remote-example-route",
        "type": "static",
        "static_network": "10.0.1.0/24"
      }
    ],
    "tunnels": [
      {
        "local_address": { "name": "public-ip-1" },
        "name": "example-tunnel-1",
        "uuid": "1085d477-8d8f-4c97-9bef-731933187777",
        "remote_address": { "address": "100.10.0.111" },
        "ipsec": {
          "authentication": { "authentication": "psk" },
          "child_rekey_time": 1440,
          "dpd_delay": 30,
          "dpd_timeout": 120,
          "ike_lifetime": 86400,
          "phase1_algorithms": ["aes128gcm128", "aes256gcm128"],
          "phase1_dh_group_numbers": [14, 16, 18, 19, 20, 21],
          "phase1_integrity_algorithms": ["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"],
          "phase2_algorithms": ["aes128gcm128", "aes256gcm128"],
          "phase2_dh_group_numbers": [14, 16, 18, 19, 20, 21],
          "phase2_integrity_algorithms": ["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"],
          "rekey_time": 14400
        },
        "tunnel_healthy": true,
        "tunnel_up": true,
        "tunnel_internal_ip": "169.254.17.1",
        "internal_peer_ping_interval": 0,
        "operational_state": "established",
        "created_at": "2022-12-01T09:04:08.529138Z",
        "updated_at": "2022-12-01T19:04:08.529138Z"
      }
    ],
    "created_at": "2022-12-01T09:04:08.529138Z",
    "updated_at": "2022-12-01T19:04:08.529138Z"
  }
]
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `uuid` | UUID | Unique identifier for the connection. |
| `name` | String | Connection name (unique within the gateway). |
| `type` | String | Connection type (currently only `ipsec`). |
| `local_routes` | Array of Object | Local (UpCloud side) routes. |
| `remote_routes` | Array of Object | Remote peer routes. |
| `tunnels` | Array of Object | VPN tunnels within this connection. |
| `created_at` | String (datetime) | Creation timestamp. |
| `updated_at` | String (datetime) | Last update timestamp. |

---

## Get Connection Details

Returns network gateway connection for `{service_uuid}` gateway by given `{connection_uuid}`.

### Request

`GET /1.3/gateway/{service_uuid}/connections/{connection_uuid}`

### Response

```json
{
  "name": "example-connection",
  "uuid": "1085d477-8d8f-4c97-9bef-731933187538",
  "type": "ipsec",
  "local_routes": [
    {
      "name": "upcloud-example-route",
      "type": "static",
      "static_network": "10.0.0.0/24"
    }
  ],
  "remote_routes": [
    {
      "name": "remote-example-route",
      "type": "static",
      "static_network": "10.0.1.0/24"
    }
  ],
  "tunnels": [
    {
      "local_address": { "name": "public-ip-1" },
      "name": "example-tunnel-1",
      "uuid": "1085d477-8d8f-4c97-9bef-731933187777",
      "remote_address": { "address": "100.10.0.111" },
      "ipsec": {
        "authentication": { "authentication": "psk" },
        "child_rekey_time": 1440,
        "dpd_delay": 30,
        "dpd_timeout": 120,
        "ike_lifetime": 86400,
        "phase1_algorithms": ["aes128gcm128", "aes256gcm128"],
        "phase1_dh_group_numbers": [14, 16, 18, 19, 20, 21],
        "phase1_integrity_algorithms": ["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"],
        "phase2_algorithms": ["aes128gcm128", "aes256gcm128"],
        "phase2_dh_group_numbers": [14, 16, 18, 19, 20, 21],
        "phase2_integrity_algorithms": ["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"],
        "rekey_time": 14400
      },
      "tunnel_healthy": true,
      "tunnel_up": true,
      "tunnel_internal_ip": "169.254.17.1",
      "internal_peer_ping_interval": 0,
      "operational_state": "established",
      "created_at": "2022-12-01T09:04:08.529138Z",
      "updated_at": "2022-12-01T19:04:08.529138Z"
    }
  ],
  "created_at": "2022-12-01T09:04:08.529138Z",
  "updated_at": "2022-12-01T19:04:08.529138Z"
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `uuid` | UUID | Unique identifier for the connection. |
| `name` | String | Connection name (unique within the gateway). |
| `type` | String | Connection type (currently only `ipsec`). |
| `local_routes` | Array of Object | Local (UpCloud side) routes. |
| `remote_routes` | Array of Object | Remote peer routes. |
| `tunnels` | Array of Object | VPN tunnels within this connection. |
| `created_at` | String (datetime) | Creation timestamp. |
| `updated_at` | String (datetime) | Last update timestamp. |

---

## Create Connection

Creates a new connection to network gateway.

### Request

`POST /1.3/gateway/{service_uuid}/connections`

```json
{
  "name": "example-connection",
  "type": "ipsec",
  "local_routes": [
    {
      "name": "upcloud-example-route",
      "type": "static",
      "static_network": "10.0.0.0/24"
    }
  ],
  "remote_routes": [
    {
      "name": "remote-example-route",
      "type": "static",
      "static_network": "10.0.1.0/24"
    }
  ],
  "tunnels": [
    {
      "name": "example-tunnel-1",
      "local_address": { "name": "public-ip-1" },
      "remote_address": { "address": "100.10.0.111" },
      "ipsec": {
        "authentication": {
          "authentication": "psk",
          "psk": "{your PSK}"
        }
      },
      "tunnel_internal_ip": "169.254.17.1",
      "internal_peer_ping_interval": 0
    }
  ]
}
```

### Request attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `uuid` | UUID | Optional UUID for the connection. |
| `name` | String (1–64 chars, `^[a-zA-Z0-9_-]+$`) | **Required.** Connection name (unique within gateway). |
| `type` | String | **Required.** Connection type (currently only `ipsec`). |
| `local_routes` | Array of Object | Local (UpCloud side) routes. |
| `remote_routes` | Array of Object | Remote peer routes. |
| `tunnels` | Array of Object | VPN tunnels. |

> **Note:** At least one of `local_routes`, `remote_routes`, or `tunnels` must be provided.

### Response

`HTTP/1.1 201 Created`

---

## Modify Connection

Modifies network gateway connection for `{service_uuid}` gateway by given `{connection_uuid}`.

### Request

`PATCH /1.3/gateway/{service_uuid}/connections/{connection_uuid}`

```json
{
  "local_routes": [
    {
      "name": "modified-routes",
      "type": "static",
      "static_network": "10.0.22.0/24"
    }
  ]
}
```

### Request attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `local_routes` | Array of Object | Local (UpCloud side) routes. |
| `remote_routes` | Array of Object | Remote peer routes. |
| `tunnels` | Array of Object | VPN tunnels. |

### Response

`HTTP/1.1 200 OK`

---

## Delete Connection

Deletes network gateway connection for `{service_uuid}` with name `{connection_uuid}`.

### Request

`DELETE /1.3/gateway/{service_uuid}/connections/{connection_uuid}`

### Response

`HTTP/1.1 204 No Content`

---

## List Tunnels

Returns a list of network gateway connection tunnels.

### Request

`GET /1.3/gateway/{service_uuid}/connections/{connection_uuid}/tunnels`

### Response

```json
[
  {
    "local_address": { "name": "public-ip-1" },
    "name": "example-tunnel-1",
    "uuid": "1085d477-8d8f-4c97-9bef-731933187538",
    "remote_address": { "address": "100.10.0.111" },
    "ipsec": {
      "authentication": { "authentication": "psk" },
      "child_rekey_time": 1440,
      "dpd_delay": 30,
      "dpd_timeout": 120,
      "ike_lifetime": 86400,
      "phase1_algorithms": ["aes128gcm128", "aes256gcm128"],
      "phase1_dh_group_numbers": [14, 16, 18, 19, 20, 21],
      "phase1_integrity_algorithms": ["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"],
      "phase2_algorithms": ["aes128gcm128", "aes256gcm128"],
      "phase2_dh_group_numbers": [14, 16, 18, 19, 20, 21],
      "phase2_integrity_algorithms": ["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"],
      "rekey_time": 14400
    },
    "tunnel_healthy": true,
    "tunnel_up": true,
    "tunnel_internal_ip": "169.254.17.1",
    "internal_peer_ping_interval": 0,
    "operational_state": "established",
    "created_at": "2022-12-01T09:04:08.529138Z",
    "updated_at": "2022-12-01T19:04:08.529138Z"
  }
]
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `uuid` | UUID | Unique identifier for the tunnel. |
| `name` | String | Tunnel name (unique within gateway). |
| `local_address` | Object | Local (gateway-side) tunnel endpoint address. |
| `remote_address` | Object | Remote (peer-side) tunnel endpoint address. |
| `ipsec` | Object | IPSec tunnel configuration. |
| `operational_state` | String | Current tunnel lifecycle state. |
| `tunnel_up` | Boolean | `true` if an IPSec SA has been established. |
| `tunnel_healthy` | Boolean | `true` if no bad log messages in last 5 minutes. |
| `tunnel_internal_ip` | String | Internal link-local IP address assigned to the tunnel. |
| `internal_peer_ping_interval` | Integer | Interval (seconds) between internal peer ping probes. |
| `created_at` | String (datetime) | Creation timestamp. |
| `updated_at` | String (datetime) | Last update timestamp. |

---

## Get Tunnel Details

Returns network gateway connection tunnel by given `{tunnel_uuid}`.

### Request

`GET /1.3/gateway/{service_uuid}/connections/{connection_uuid}/tunnels/{tunnel_uuid}`

### Response

```json
{
  "local_address": { "name": "public-ip-1" },
  "name": "example-tunnel-1",
  "uuid": "1085d477-8d8f-4c97-9bef-731933187538",
  "remote_address": { "address": "100.10.0.111" },
  "ipsec": {
    "authentication": { "authentication": "psk" },
    "child_rekey_time": 1440,
    "dpd_delay": 30,
    "dpd_timeout": 120,
    "ike_lifetime": 86400,
    "phase1_algorithms": ["aes128gcm128", "aes256gcm128"],
    "phase1_dh_group_numbers": [14, 16, 18, 19, 20, 21],
    "phase1_integrity_algorithms": ["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"],
    "phase2_algorithms": ["aes128gcm128", "aes256gcm128"],
    "phase2_dh_group_numbers": [14, 16, 18, 19, 20, 21],
    "phase2_integrity_algorithms": ["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"],
    "rekey_time": 14400
  },
  "tunnel_healthy": true,
  "tunnel_up": true,
  "tunnel_internal_ip": "169.254.17.1",
  "internal_peer_ping_interval": 0,
  "operational_state": "established",
  "created_at": "2022-12-01T09:04:08.529138Z",
  "updated_at": "2022-12-01T19:04:08.529138Z"
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `uuid` | UUID | Unique identifier for the tunnel. |
| `name` | String | Tunnel name (unique within gateway). |
| `local_address` | Object | Local (gateway-side) tunnel endpoint address. |
| `remote_address` | Object | Remote (peer-side) tunnel endpoint address. |
| `ipsec` | Object | IPSec tunnel configuration. |
| `operational_state` | String | Current tunnel lifecycle state. |
| `tunnel_up` | Boolean | `true` if an IPSec SA has been established. |
| `tunnel_healthy` | Boolean | `true` if no bad log messages in last 5 minutes. |
| `tunnel_internal_ip` | String | Internal link-local IP address assigned to the tunnel. |
| `internal_peer_ping_interval` | Integer | Interval (seconds) between internal peer ping probes. |
| `created_at` | String (datetime) | Creation timestamp. |
| `updated_at` | String (datetime) | Last update timestamp. |

---

## Create Tunnel

Creates a new tunnel to network gateway connection.

### Request

`POST /1.3/gateway/{service_uuid}/connections/{connection_uuid}/tunnels`

```json
{
  "name": "example-tunnel-2",
  "local_address": { "name": "public-ip-1" },
  "remote_address": { "address": "100.10.0.111" },
  "ipsec": {
    "authentication": {
      "authentication": "psk",
      "psk": "{your PSK}"
    }
  },
  "tunnel_internal_ip": "169.254.17.1",
  "internal_peer_ping_interval": 0
}
```

### Request attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `uuid` | UUID | Optional UUID for the tunnel. |
| `name` | String (1–64 chars, `^[a-zA-Z0-9_-]+$`) | **Required.** Tunnel name (unique within gateway). |
| `local_address` | Object | **Required.** Local address name referencing a gateway address. |
| `remote_address` | Object | **Required.** Remote public IP address of the peer. |
| `ipsec` | Object | **Required.** IPSec configuration. |
| `tunnel_internal_ip` | String | Tunnel link-local address (inside tunnel IPv4). Default: `""`. |
| `internal_peer_ping_interval` | Integer | Peer ping interval in seconds (0 = disabled, ≥ 5 to enable). Default: `0`. |

### Response

`HTTP/1.1 201 Created`

---

## Modify Tunnel

Modifies network gateway connection tunnel for `{tunnel_uuid}`.

### Request

`PATCH /1.3/gateway/{service_uuid}/connections/{connection_uuid}/tunnels/{tunnel_uuid}`

```json
{
  "remote_address": { "address": "100.10.0.222" }
}
```

### Request attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `name` | String (1–64 chars, `^[a-zA-Z0-9_-]+$`) | Tunnel name (unique within gateway). |
| `local_address` | Object | Local address name referencing a gateway address. |
| `remote_address` | Object | Remote public IP address of the peer. |
| `ipsec` | Object | IPSec configuration. |
| `tunnel_internal_ip` | String | Tunnel link-local address for the pinger. |
| `internal_peer_ping_interval` | Integer | Tunnel peer ping interval in seconds. |

### Response

`HTTP/1.1 200 OK`

---

## Delete Tunnel

Deletes a tunnel under network gateway connection `{connection_uuid}` with uuid `{tunnel_uuid}`.

### Request

`DELETE /1.3/gateway/{service_uuid}/connections/{connection_uuid}/tunnels/{tunnel_uuid}`

### Response

`HTTP/1.1 204 No Content`

---

## Routes

Routes are used for `local_routes` (UpCloud-side networks) and `remote_routes` (remote peer networks). Only `static` routes are supported currently.

### Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `type` | String | **Required.** Routing type (only `static` currently). |
| `static_network` | String | **Required.** Destination prefix (valid IPv4 CIDR). |
| `name` | String (1–64 chars, `^[a-zA-Z0-9_-]+$`) | **Required.** Unique route name. |

---

## Local Address

Public (UpCloud) endpoint IP address of a tunnel.

### Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `name` | String | **Required.** Name of a gateway address (see Addresses). |

---

## Remote Address

Remote public IP address of the tunnel (the other IPSec endpoint).

### Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `address` | String | **Required.** Remote peer IPv4 address. Must be a global non-private unicast IP. |

---

## IPSec

Tunnel IPSec configuration object.

### Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `authentication` | Object | **Required** (on create). IPSec authentication object. |
| `phase1_algorithms` | Array of String | Phase 1 proposal algorithms. Default: `[aes128, aes256, aes128gcm128, aes256gcm128]`. |
| `phase2_algorithms` | Array of String | Phase 2 SA algorithms. Default: `[aes128, aes256, aes128gcm128, aes256gcm128]`. |
| `phase1_integrity_algorithms` | Array of String | Phase 1 integrity algorithms. Default: `[sha256, sha384, sha512]`. |
| `phase2_integrity_algorithms` | Array of String | Phase 2 integrity algorithms. Default: `[sha256, sha384, sha512]`. |
| `phase1_dh_group_numbers` | Array of Integer | Phase 1 Diffie-Hellman group numbers. Default: `[14, 16, 18, 19, 20, 21]`. |
| `phase2_dh_group_numbers` | Array of Integer | Phase 2 Diffie-Hellman group numbers. Default: `[14, 16, 18, 19, 20, 21]`. |
| `child_rekey_time` | Integer (0–2147483647) | IKE child SA rekey time in seconds. Default: `1440`. |
| `rekey_time` | Integer (0–2147483647) | IKE SA rekey time in seconds. Default: `14400`. |
| `dpd_delay` | Integer (0–2147483647) | Dead peer detection delay in seconds. Default: `30`. |
| `dpd_timeout` | Integer (0–2147483647) | Dead peer detection timeout in seconds. Default: `120`. |
| `ike_lifetime` | Integer (0–2147483647) | Maximum IKE SA lifetime in seconds. Default: `86400`. |

---

## IPSec Authentication

Tunnel IPSec authentication object.

### Attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `authentication` | String | **Required.** Authentication type (currently only `psk`). |
| `psk` | String (8–64 chars, `^[a-zA-Z1-9_.][a-zA-Z0-9_.]+$`) | Pre-shared key. Required on create, optional on modify. Write-only (cannot be read after creation). |

---

## Get Metrics

Returns metrics by given `{service_uuid}`.

### Request

`GET /1.3/gateway/{service_uuid}/metrics`

### Response

```json
{
  "gateways": [
    {
      "active_connections": 15,
      "created_at": "2024-01-11T13:16:05.161672Z",
      "name": "gateway1",
      "total_accepted_connections": 8773,
      "total_rejected_sessions": 3,
      "updated_at": "2024-01-17T10:25:13.393453Z"
    }
  ],
  "ipsec_metrics": {
    "ike_sas": [
      {
        "child_sas": [
          {
            "name": "example-connection/example-tunnel-1",
            "tunnel_id": 213,
            "unique_id": "389",
            "state": "installed",
            "spi_in": "c3d20761",
            "spi_out": "cec8f1bc",
            "bytes_in": 25537422,
            "bytes_out": 645540,
            "packets_in": 20353,
            "packets_out": 7685,
            "rekey_time": 1214,
            "life_time": 1445,
            "install_time": 139,
            "local_traffic_selectors": ["0.0.0.0/0", "::/0"],
            "remote_traffic_selectors": ["0.0.0.0/0", "::/0"],
            "created_at": "2024-01-11T13:19:10.077746Z",
            "updated_at": "2024-01-17T10:25:13.393453Z"
          }
        ],
        "created_at": "2024-01-11T13:18:55.056675Z",
        "established": 9286,
        "initator": true,
        "local_host": "100.127.0.241",
        "local_id": "100.127.0.241",
        "name": "example-connection/example-tunnel-1",
        "reauth_time": 0,
        "rekey_time": 4483,
        "remote_host": "100.10.0.111",
        "remote_id": "100.10.0.111",
        "operational_state": "established",
        "internal_state": "established",
        "tunnel_id": 213,
        "unique_id": "149",
        "updated_at": "2024-01-17T10:25:13.393453Z",
        "version": 2,
        "heuristic_state": {
          "tunnel_up": true,
          "tunnel_healthy": true,
          "last_down_message": "received AUTHENTICATION_FAILED notify error",
          "updated_at": "2024-01-17T10:25:13.393453Z",
          "last_down_message_updated_at": "2024-01-17T06:46:18.938719Z",
          "up_events": 5,
          "down_events": 4,
          "log_message_bad_events": 1
        }
      }
    ]
  }
}
```

### Response attributes

**`gateways` attributes:**

| Attribute | Type | Description |
|-----------|------|-------------|
| `name` | String | Name of the gateway. |
| `active_connections` | Integer | Current number of connections through the gateway. |
| `total_accepted_connections` | Integer | Total number of accepted connections. |
| `total_rejected_connections` | Integer | Total number of rejected connections. |
| `created_at` | String (datetime) | When first metrics sample was collected. |
| `updated_at` | String (datetime) | When latest metrics sample was collected. |

**`ike_sas` attributes:**

| Attribute | Type | Description |
|-----------|------|-------------|
| `name` | String | Name constructed from connection and tunnel names. |
| `tunnel_id` | Integer | Internal tunnel ID. |
| `unique_id` | Integer | Internal unique ID. |
| `operational_state` | String | SA state (same values as tunnel `operational_state`). |
| `internal_state` | String | Internal SA state. |
| `reauth_time` | Integer | Remaining time until next IKE reauthentication (seconds). |
| `rekey_time` | Integer | Remaining time until next IKE rekey (seconds). |
| `established` | Integer | Seconds this SA has been operating. |
| `initator` | Boolean | `true` if this endpoint established the SA. |
| `local_host` | String | Local endpoint IP address. |
| `local_id` | String | Internal identifier of the local endpoint. |
| `remote_host` | String | Remote endpoint IP address. |
| `remote_id` | String | Internal identifier of the remote endpoint. |
| `version` | Integer | IKE version. |
| `created_at` | String (datetime) | When first metrics sample was collected. |
| `updated_at` | String (datetime) | When latest metrics sample was collected. |

**`child_sas` attributes:**

| Attribute | Type | Description |
|-----------|------|-------------|
| `name` | String | Name constructed from connection and tunnel names. |
| `tunnel_id` | Integer | Internal tunnel ID. |
| `unique_id` | Integer | Internal unique ID. |
| `state` | String | Child SA state. |
| `spi_in` | String | Inbound IKE Security Parameter Index (8 hex digits). |
| `spi_out` | String | Outbound IKE Security Parameter Index (8 hex digits). |
| `bytes_in` | Integer | Total bytes received through IKE Child SA. |
| `bytes_out` | Integer | Total bytes sent through IKE Child SA. |
| `packets_in` | Integer | Total packets received through IKE Child SA. |
| `packets_out` | Integer | Total packets sent through IKE Child SA. |
| `rekey_time` | Integer | Remaining time until next IKE rekey (seconds). |
| `life_time` | Integer | Remaining SA lifetime (seconds). |
| `install_time` | Integer | Seconds since SA was installed. |
| `local_traffic_selectors` | Array of String | Local traffic selectors. |
| `remote_traffic_selectors` | Array of String | Remote traffic selectors. |
| `created_at` | String (datetime) | When first metrics sample was collected. |
| `updated_at` | String (datetime) | When latest metrics sample was collected. |

**`heuristic_state` attributes:**

| Attribute | Type | Description |
|-----------|------|-------------|
| `tunnel_up` | Boolean | `true` if SA is established and tunnel is up. |
| `tunnel_healthy` | Boolean | `true` if no bad log messages seen in past 5 minutes. |
| `last_down_message` | String | Last log message indicating the tunnel is unhealthy. |
| `last_down_message_updated_at` | String (datetime) | When `last_down_message` was last updated. |
| `up_events` | Integer | Number of tunnel up events. |
| `down_events` | Integer | Number of tunnel down events. |
| `log_message_bad_events` | Integer | Number of bad log messages. |
| `updated_at` | String (datetime) | When `heuristic_state` was last changed. |
