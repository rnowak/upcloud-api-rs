# File Storage

> Source: https://developers.upcloud.com/1.3/25-file-storage

File Storage provides storage shares over the Network File System (NFS) protocol. It is designed for workloads that require file-level access with a standard POSIX interface.

The API follows [JSON Merge Patch (RFC 7386)](https://www.rfc-editor.org/rfc/rfc7386) semantics. When modifying nested structures the payload should contain the entire array (`PUT` within `PATCH`). `null` indicates property deletion; absent properties are left unchanged.

---

## Service endpoints

### Create service

Creates a new file storage service.

```
POST /1.3/file-storage
```

#### Request body

```json
{
  "name": "test-name",
  "zone": "fi-hel2",
  "configured_status": "started",
  "size_gib": 250,
  "encrypted": false,
  "networks": [
    {
      "uuid": "03720d07-80a9-41ac-90b0-68a932109339",
      "name": "net",
      "family": "IPv4",
      "ip_address": "192.168.147.120"
    }
  ],
  "shares": [
    {
      "name": "data",
      "path": "/data",
      "acl": [
        {
          "name": "read",
          "target": "*",
          "permission": "ro"
        }
      ]
    },
    {
      "name": "proj",
      "path": "/project/data",
      "acl": [
        {
          "name": "write",
          "target": "192.168.147.0/24",
          "permission": "rw"
        }
      ]
    }
  ],
  "labels": [
    {
      "key": "environment",
      "value": "production"
    }
  ]
}
```

#### Attributes

| Attribute | Accepted value | Required | Description |
|---|---|---|---|
| `name` | 1-64 characters, pattern `^[a-zA-Z0-9_-]+$` | yes | Unique name within the customer account. |
| `zone` | A valid zone identifier | yes | Zone in which the service will be hosted, e.g. `fi-hel2`. |
| `configured_status` | `started` / `stopped` | yes | Service status managed by the customer. |
| `size_gib` | 250–25000 | yes | Size of the file storage in GiB. |
| `encrypted` | `true` / `false` | no | Enables AES-256-GCM encryption at rest. Disabled by default. Immutable after creation. |
| `shares` | Array of 1–255 share objects | no | Initial shares. See [Create share](#create-share). |
| `networks` | Array of 0–1 network objects | no | Attached networks. Must reside in the file storage zone. See [Create network](#create-network). |
| `labels` | Array of 0–255 label objects | no | Labels for service filtering. See [Labels](#create-label). |

#### Responses

| HTTP status | Error code | Description |
|---|---|---|
| `201 Created` | — | Service created. |
| `400 Bad Request` | `INVALID_REQUEST` | Validation error. |
| `402 Payment Required` | `INSUFFICIENT_CREDITS` | Not enough credits. |

---

### List services

Returns a list of file storage services.

```
GET /1.3/file-storage
```

#### Query parameters

| Parameter | Accepted value | Default | Required | Description |
|---|---|---|---|---|
| `limit` | 0–100 | 10 | no | Maximum number of entries to return. |
| `offset` | ≥ 0 | 0 | no | Offset for retrieved results. |
| `sort` | `created_at`, `-created_at` | `-created_at` | no | Order of returned results. |

#### Response `200 OK`

```json
[
  {
    "configured_status": "started",
    "created_at": "2025-08-01T10:40:04.140473Z",
    "encrypted": false,
    "labels": [
      {
        "key": "environment",
        "value": "production"
      }
    ],
    "name": "test2",
    "networks": [
      {
        "family": "IPv4",
        "ip_address": "10.10.10.50",
        "name": "net",
        "uuid": "0391f203-6eef-4a69-bfcc-fffffbdfdc32"
      }
    ],
    "operational_state": "running",
    "shares": [
      {
        "acl": [
          {
            "name": "read",
            "permission": "ro",
            "target": "*"
          }
        ],
        "name": "data",
        "path": "/data"
      },
      {
        "acl": [
          {
            "name": "write",
            "permission": "rw",
            "target": "10.10.10.0/24"
          }
        ],
        "name": "proj",
        "path": "/project/data"
      }
    ],
    "size_gib": 250,
    "state_messages": [],
    "updated_at": "2025-08-03T19:23:52.350216Z",
    "uuid": "17871d62-6964-4336-b68d-4a85a8ee10fe",
    "zone": "fi-hel2"
  }
]
```

---

### Get service details

Returns file storage service details by given service UUID.

```
GET /1.3/file-storage/{service_uuid}
```

#### Response `200 OK`

```json
{
  "configured_status": "started",
  "created_at": "2025-08-01T10:40:04.140473Z",
  "encrypted": false,
  "labels": [
    {
      "key": "environment",
      "value": "production"
    }
  ],
  "name": "test2",
  "networks": [
    {
      "family": "IPv4",
      "ip_address": "10.10.10.50",
      "name": "net",
      "uuid": "0391f203-6eef-4a69-bfcc-fffffbdfdc32"
    }
  ],
  "operational_state": "running",
  "shares": [
    {
      "acl": [
        {
          "name": "read",
          "permission": "ro",
          "target": "*"
        }
      ],
      "name": "data",
      "path": "/data"
    },
    {
      "acl": [
        {
          "name": "write",
          "permission": "rw",
          "target": "10.10.10.0/24"
        }
      ],
      "name": "proj",
      "path": "/project/data"
    }
  ],
  "size_gib": 250,
  "state_messages": [],
  "updated_at": "2025-08-03T19:23:52.350216Z",
  "uuid": "17871d62-6964-4336-b68d-4a85a8ee10fe",
  "zone": "fi-hel2"
}
```

---

### Replace service

Replaces an existing file storage service (full update).

```
PUT /1.3/file-storage/{service_uuid}
```

#### Request body

```json
{
  "name": "test-name",
  "configured_status": "started",
  "size_gib": 250,
  "networks": [
    {
      "uuid": "03720d07-80a9-41ac-90b0-68a932109339",
      "name": "net",
      "family": "IPv4",
      "ip_address": "192.168.147.120"
    }
  ],
  "labels": [
    {
      "key": "environment",
      "value": "production"
    }
  ]
}
```

#### Attributes

| Attribute | Accepted value | Required | Description |
|---|---|---|---|
| `name` | 1-64 characters, pattern `^[a-zA-Z0-9_-]+$` | yes | Unique name within the customer account. |
| `configured_status` | `started` / `stopped` | yes | Service status managed by the customer. |
| `size_gib` | 250–25000 | yes | Size in GiB. Can be expanded but not reduced. |
| `networks` | Array of 0–1 network objects | no | Attached networks. Must reside in the file storage zone. |
| `labels` | Array of 0–255 label objects | no | Labels for service filtering. |

#### Responses

| HTTP status | Error code | Description |
|---|---|---|
| `200 OK` | — | Service replaced. |
| `400 Bad Request` | `INVALID_REQUEST` | Validation error. |
| `402 Payment Required` | `INSUFFICIENT_CREDITS` | Not enough credits. |
| `404 Not Found` | `RESOURCE_NOT_FOUND` | Service not found. |

---

### Modify service

Partially modifies an existing file storage service (JSON Merge Patch).

```
PATCH /1.3/file-storage/{service_uuid}
```

#### Request body

```json
{
  "name": "test-name",
  "configured_status": "started",
  "size_gib": 250,
  "networks": [
    {
      "uuid": "03720d07-80a9-41ac-90b0-68a932109339",
      "name": "net",
      "family": "IPv4",
      "ip_address": "192.168.147.120"
    }
  ]
}
```

#### Attributes

| Attribute | Accepted value | Required | Description |
|---|---|---|---|
| `name` | 1-64 characters, pattern `^[a-zA-Z0-9_-]+$` | no | Unique name within the customer account. |
| `configured_status` | `started` / `stopped` | no | Service status managed by the customer. |
| `size_gib` | 250–25000 | no | Size in GiB. Can be expanded but not reduced. |
| `networks` | Array of 0–1 network objects | no | Attached networks. Must reside in the file storage zone. |
| `labels` | Array of 0–255 label objects | no | Labels for service filtering. |

#### Responses

| HTTP status | Error code | Description |
|---|---|---|
| `200 OK` | — | Service modified. |
| `400 Bad Request` | `INVALID_REQUEST` | Validation error. |
| `402 Payment Required` | `INSUFFICIENT_CREDITS` | Not enough credits. |
| `404 Not Found` | `RESOURCE_NOT_FOUND` | Service not found. |

---

### Delete service

Deletes an existing file storage service.

```
DELETE /1.3/file-storage/{service_uuid}
```

#### Responses

| HTTP status | Error code | Description |
|---|---|---|
| `204 No Content` | — | Service deleted. |
| `404 Not Found` | `RESOURCE_NOT_FOUND` | Service not found. |

---

## Network endpoints

### Create network

Creates a new network attachment on a file storage service.

```
POST /1.3/file-storage/{service_uuid}/networks
```

#### Request body

```json
{
  "uuid": "03720d07-80a9-41ac-90b0-68a932109339",
  "name": "test-name",
  "family": "IPv4",
  "ip_address": "10.20.30.40"
}
```

#### Attributes

| Attribute | Accepted value | Required | Description |
|---|---|---|---|
| `name` | 1-64 characters, pattern `^[a-zA-Z0-9_-]+$` | yes | Unique name within the service. |
| `uuid` | A valid network UUID | yes | Private network UUID. |
| `family` | `IPv4` | yes | Network family. IPv6 is not currently supported. |
| `ip_address` | IPv4 address | no | IP address of the service. If omitted, a free address is taken from the private network pool. |

#### Responses

| HTTP status | Error code | Description |
|---|---|---|
| `201 Created` | — | Network created. |
| `400 Bad Request` | `INVALID_REQUEST` | Validation error. |
| `402 Payment Required` | `INSUFFICIENT_CREDITS` | Not enough credits. |

---

### List networks

Returns a list of networks attached to a file storage service.

```
GET /1.3/file-storage/{service_uuid}/networks
```

#### Response `200 OK`

```json
[
  {
    "family": "IPv4",
    "ip_address": "192.168.147.250",
    "name": "test-name",
    "uuid": "03720d07-80a9-41ac-90b0-68a932109339"
  }
]
```

---

### Get network details

Returns details of a single network by name.

```
GET /1.3/file-storage/{service_uuid}/networks/{network_name}
```

#### Response `200 OK`

```json
{
  "family": "IPv4",
  "ip_address": "192.168.147.250",
  "name": "test-name",
  "uuid": "03720d07-80a9-41ac-90b0-68a932109339"
}
```

---

### Modify network

Partially modifies an existing network (JSON Merge Patch).

```
PATCH /1.3/file-storage/{service_uuid}/networks/{network_name}
```

#### Request body

```json
{
  "uuid": "03720d07-80a9-41ac-90b0-68a932109339",
  "name": "test-name",
  "family": "IPv4",
  "ip_address": "10.20.30.40"
}
```

#### Attributes

| Attribute | Accepted value | Required | Description |
|---|---|---|---|
| `name` | 1-64 characters, pattern `^[a-zA-Z0-9_-]+$` | no | Unique name within the service. |
| `uuid` | A valid network UUID | no | Private network UUID. |
| `family` | `IPv4` | no | Network family. IPv6 is not currently supported. |
| `ip_address` | IPv4 address | no | IP address of the service. If omitted, a free address is taken from the private network pool. |

#### Responses

| HTTP status | Error code | Description |
|---|---|---|
| `200 OK` | — | Network modified. |
| `400 Bad Request` | `INVALID_REQUEST` | Validation error. |
| `402 Payment Required` | `INSUFFICIENT_CREDITS` | Not enough credits. |

---

### Delete network

Deletes an existing network from a file storage service.

```
DELETE /1.3/file-storage/{service_uuid}/networks/{network_name}
```

#### Responses

| HTTP status | Error code | Description |
|---|---|---|
| `204 No Content` | — | Network deleted. |
| `404 Not Found` | `RESOURCE_NOT_FOUND` | Resource not found. |

---

## Share endpoints

Each `share` object maps directly to an NFS export: the share `path` becomes the exported directory and each `acl` entry becomes a client → permission rule.

> **Nested share precedence:** A parent share's ACL may take precedence over a child share's ACL when the parent rule matches the same client with equal or broader scope. The effective permission for a client is determined by the most permissive share ACL, not simply by the deepest path.

### Create share

Creates a new NFS share on a file storage service.

```
POST /1.3/file-storage/{service_uuid}/shares
```

#### Request body

```json
{
  "name": "data",
  "path": "/data",
  "acl": [
    {
      "name": "read",
      "target": "192.168.147.0/24",
      "permission": "ro"
    }
  ]
}
```

#### Attributes

| Attribute | Accepted value | Required | Description |
|---|---|---|---|
| `name` | 1-128 characters, pattern `^[a-zA-Z0-9_-]+$` | yes | Unique name within the service. |
| `path` | 1-512 characters, valid absolute path `^/[a-zA-Z0-9/_-]*` | yes | Immutable once created. Directory lifecycle is system-managed. **Deleting the share permanently removes the directory and all content.** |
| `acl` | Array of 0–50 ACL objects | yes | Access control list. Empty ACL means no access. Nested shares remain visible within parent ACLs. |

#### Responses

| HTTP status | Error code | Description |
|---|---|---|
| `201 Created` | — | Share created. |
| `400 Bad Request` | `INVALID_REQUEST` | Validation error. |
| `402 Payment Required` | `INSUFFICIENT_CREDITS` | Not enough credits. |

---

### List shares

Returns a list of shares on a file storage service.

```
GET /1.3/file-storage/{service_uuid}/shares
```

#### Response `200 OK`

```json
[
  {
    "name": "proj",
    "path": "/project/data",
    "acl": [
      {
        "name": "write",
        "target": "*",
        "permission": "rw"
      }
    ]
  },
  {
    "name": "logs",
    "path": "/project/logs",
    "deleting": true,
    "acl": [
      {
        "name": "read",
        "target": "192.168.147.0/24",
        "permission": "ro"
      }
    ]
  }
]
```

#### Response attributes

| Attribute | Description |
|---|---|
| `name` | Unique name within the service. |
| `path` | Absolute path of the share. |
| `acl` | List of ACL objects. |
| `deleting` | Indicates the share is being deleted. |

---

### Get share details

Returns a single share by name.

```
GET /1.3/file-storage/{service_uuid}/shares/{share_name}
```

#### Response `200 OK`

```json
{
  "acl": [
    {
      "name": "write",
      "permission": "rw",
      "target": "*"
    }
  ],
  "name": "proj",
  "path": "/project/data"
}
```

---

### Modify share

Partially modifies an existing share (JSON Merge Patch).

```
PATCH /1.3/file-storage/{service_uuid}/shares/{share_name}
```

#### Request body

```json
{
  "name": "test-name",
  "acl": [
    {
      "name": "read-only",
      "target": "192.168.25.0/24",
      "permission": "ro"
    }
  ]
}
```

#### Attributes

| Attribute | Accepted value | Required | Description |
|---|---|---|---|
| `name` | 1-128 characters, pattern `^[a-zA-Z0-9_-]+$` | no | Unique name within the service. |
| `acl` | Array of 0–50 ACL objects | no | Access control list. Empty ACL means no access. |

#### Responses

| HTTP status | Error code | Description |
|---|---|---|
| `200 OK` | — | Share modified. |
| `400 Bad Request` | `INVALID_REQUEST` | Validation error. |
| `402 Payment Required` | `INSUFFICIENT_CREDITS` | Not enough credits. |

---

### Delete share

Deletes an existing share. **The directory and all content are permanently deleted.**

```
DELETE /1.3/file-storage/{service_uuid}/shares/{share_name}
```

#### Responses

| HTTP status | Error code | Description |
|---|---|---|
| `204 No Content` | — | Share deleted. |
| `404 Not Found` | `RESOURCE_NOT_FOUND` | Resource not found. |

---

## ACL endpoints

### Create ACL

Creates a new ACL entry on a share.

```
POST /1.3/file-storage/{service_uuid}/shares/{share_name}/acl
```

#### Request body

```json
{
  "name": "read-only",
  "target": "192.168.147.0/24",
  "permission": "ro"
}
```

#### Attributes

| Attribute | Accepted value | Required | Description |
|---|---|---|---|
| `name` | 1-64 characters, pattern `^[a-zA-Z0-9_-]+$` | yes | Unique name within the share. |
| `target` | IP address, CIDR, or `*` | yes | Client allowed to access the share. |
| `permission` | `ro` / `rw` | yes | Read-only or read-write access. |

#### Responses

| HTTP status | Error code | Description |
|---|---|---|
| `201 Created` | — | ACL created. |
| `400 Bad Request` | `INVALID_REQUEST` | Validation error. |
| `402 Payment Required` | `INSUFFICIENT_CREDITS` | Not enough credits. |

---

### List ACLs

Returns a list of ACL entries on a share.

```
GET /1.3/file-storage/{service_uuid}/shares/{share_name}/acl
```

#### Response `200 OK`

```json
[
  {
    "name": "read-only",
    "target": "192.168.147.0/24",
    "permission": "rw"
  },
  {
    "name": "guest",
    "target": "192.168.150.0/24",
    "permission": "ro"
  }
]
```

---

### Get ACL details

Returns a single ACL entry by name.

```
GET /1.3/file-storage/{service_uuid}/shares/{share_name}/acl/{acl_name}
```

#### Response `200 OK`

```json
{
  "name": "read-only",
  "permission": "rw",
  "target": "*"
}
```

---

### Modify ACL

Partially modifies an existing ACL entry (JSON Merge Patch).

```
PATCH /1.3/file-storage/{service_uuid}/shares/{share_name}/acl/{acl_name}
```

#### Request body

```json
{
  "name": "read-only",
  "target": "192.168.25.0/24",
  "permission": "ro"
}
```

#### Attributes

| Attribute | Accepted value | Required | Description |
|---|---|---|---|
| `name` | 1-64 characters, pattern `^[a-zA-Z0-9_-]+$` | no | Unique name within the share. |
| `target` | IP address, CIDR, or `*` | no | Client allowed to access the share. |
| `permission` | `ro` / `rw` | no | Read-only or read-write access. |

#### Responses

| HTTP status | Error code | Description |
|---|---|---|
| `200 OK` | — | ACL modified. |
| `400 Bad Request` | `INVALID_REQUEST` | Validation error. |
| `402 Payment Required` | `INSUFFICIENT_CREDITS` | Not enough credits. |

---

### Delete ACL

Deletes an existing ACL entry.

```
DELETE /1.3/file-storage/{service_uuid}/shares/{share_name}/acl/{acl_name}
```

#### Responses

| HTTP status | Error code | Description |
|---|---|---|
| `204 No Content` | — | ACL deleted. |
| `404 Not Found` | `RESOURCE_NOT_FOUND` | Resource not found. |

---

## Label endpoints

### Create label

Creates a new label on a file storage service. Labels are used for service filtering.

```
POST /1.3/file-storage/{service_uuid}/labels
```

#### Label filtering examples

- Exact match: `GET /1.3/file-storage?label=env%3Dstaging`
- Existence: `GET /1.3/file-storage?label=env`
- Multiple: `GET /1.3/file-storage?label=env&label=foo%3Dbar`

#### Request body

```json
{
  "key": "foo",
  "value": "bar"
}
```

#### Attributes

| Attribute | Accepted value | Required | Description |
|---|---|---|---|
| `key` | 2-32 printable ASCII characters (0x20-0x7E), must not start with `_` | yes | Label identifier. Unique within a service. |
| `value` | 0-255 characters, pattern `\A[\p{L}\p{N}\p{P}\p{S}\p{M}\p{Z}]*\z` | yes | Label value. |

#### Responses

| HTTP status | Error code | Description |
|---|---|---|
| `201 Created` | — | Label created. |
| `400 Bad Request` | `INVALID_REQUEST` | Validation error. |
| `402 Payment Required` | `INSUFFICIENT_CREDITS` | Not enough credits. |

---

### List labels

Returns a list of labels on a file storage service.

```
GET /1.3/file-storage/{service_uuid}/labels
```

#### Response `200 OK`

```json
[
  {
    "key": "env",
    "value": "staging"
  },
  {
    "key": "foo",
    "value": "bar"
  }
]
```

---

### Get label details

Returns a single label by key.

```
GET /1.3/file-storage/{service_uuid}/labels/{key}
```

#### Response `200 OK`

```json
{
  "key": "env",
  "value": "staging"
}
```

---

### Modify label

Partially modifies an existing label (JSON Merge Patch).

```
PATCH /1.3/file-storage/{service_uuid}/labels/{key}
```

#### Request body

```json
{
  "key": "env",
  "value": "production"
}
```

#### Attributes

| Attribute | Accepted value | Required | Description |
|---|---|---|---|
| `key` | 2-32 printable ASCII characters (0x20-0x7E), must not start with `_` | no | Label identifier. Unique within a service. |
| `value` | 0-255 characters, pattern `\A[\p{L}\p{N}\p{P}\p{S}\p{M}\p{Z}]*\z` / `null` | no | Label value. Use `null` to delete. |

#### Responses

| HTTP status | Error code | Description |
|---|---|---|
| `200 OK` | — | Label modified. |

---

### Delete label

Deletes an existing label.

```
DELETE /1.3/file-storage/{service_uuid}/labels/{key}
```

#### Responses

| HTTP status | Error code | Description |
|---|---|---|
| `204 No Content` | — | Label deleted. |
| `404 Not Found` | `RESOURCE_NOT_FOUND` | Resource not found. |

---

## Current service state

Returns the effective current state of a service. Newly created resources (e.g. shares) may not appear here until they are fully applied on the underlying storage. Returns `404` if the service has never reached `running` state.

```
GET /1.3/file-storage/{service_uuid}/current-state
```

#### Response `200 OK`

```json
{
  "configured_status": "started",
  "created_at": "2025-08-01T10:40:04.140473Z",
  "labels": [
    {
      "key": "environment",
      "value": "production"
    }
  ],
  "name": "test2",
  "networks": [
    {
      "family": "IPv4",
      "ip_address": "10.10.10.50",
      "name": "net",
      "uuid": "0391f203-6eef-4a69-bfcc-fffffbdfdc32"
    }
  ],
  "operational_state": "running",
  "shares": [
    {
      "acl": [
        {
          "name": "read",
          "permission": "ro",
          "target": "*"
        }
      ],
      "name": "data",
      "path": "/data"
    },
    {
      "acl": [
        {
          "name": "write",
          "permission": "rw",
          "target": "10.10.10.0/24"
        }
      ],
      "name": "proj",
      "path": "/project/data"
    }
  ],
  "size_gib": 250,
  "state_messages": [],
  "updated_at": "2025-08-03T19:23:52.350216Z",
  "uuid": "17871d62-6964-4336-b68d-4a85a8ee10fe",
  "zone": "fi-hel2"
}
```

---

## Service configured status

| Status | Description |
|---|---|
| `started` | The service is running. |
| `stopped` | The service is stopped. |

## Service operational state

| State | Description |
|---|---|
| `pending` | Newly created service or reconfiguration started. |
| `setup-network` | Configuring network. |
| `setup-instance` | Configuring service instance. |
| `setup-checkup` | Verifying the service is correctly configured and running. |
| `running` | Service is up and running. |
| `stopped` | Service is down. |
| `delete-instance` | Removing service instance. |
| `delete-network` | Reconfiguring network. |
| `deleting` | Deleting the service. |
