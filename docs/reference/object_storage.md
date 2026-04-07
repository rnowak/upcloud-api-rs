# Managed Object Storage

> Source: https://developers.upcloud.com/1.3/21-managed-object-storage

This document provides a comprehensive API reference for the UpCloud Managed Object Storage API. The base URL for all endpoints is `https://api.upcloud.com`.

---

## List regions

List all available object storage regions.

### Request

```http
GET /1.3/object-storage-2/regions
```

### Response

**HTTP 200 OK**

```json
[
  {
    "name": "europe-1",
    "primary_zone": "fi-hel1",
    "zones": [
      { "name": "fi-hel1" },
      { "name": "de-fra1" }
    ]
  }
]
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| name | string | Name of the region. |
| primary_zone | string | The primary availability zone of the region. |
| zones | array | List of availability zones in the region. |
| zones[].name | string | Name of the availability zone. |

---

## Get region details

Get details of a specific object storage region.

### Request

```http
GET /1.3/object-storage-2/regions/{name}
```

### Response

**HTTP 200 OK**

```json
{
  "name": "europe-1",
  "primary_zone": "fi-hel1",
  "zones": [
    { "name": "fi-hel1" },
    { "name": "de-fra1" }
  ]
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| name | string | Name of the region. |
| primary_zone | string | The primary availability zone of the region. |
| zones | array | List of availability zones in the region. |
| zones[].name | string | Name of the availability zone. |

---

## Create service

Create a new Managed Object Storage service.

### Request

```http
POST /1.3/object-storage-2
```

```json
{
  "name": "example-service",
  "region": "example",
  "configured_status": "started",
  "termination_protection": false,
  "networks": [
    {
      "name": "example-public-network",
      "type": "public",
      "family": "IPv4"
    },
    {
      "name": "example-private-network",
      "uuid": "03aa7245-2ff9-49c8-9f0e-7ca0270d71a4",
      "type": "private",
      "family": "IPv4"
    }
  ]
}
```

### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| name | 1–64 characters, pattern `^[a-zA-Z0-9_-]+$` | Yes | Name of the service. Must be unique within the account. |
| region | Valid region ID | Yes | Region where the service will be created. |
| configured_status | `started` or `stopped` | Yes | The desired status of the service. |
| termination_protection | Boolean | No | When enabled, the service cannot be deleted. |
| networks | Array of 0–8 network objects | No | Networks to attach to the service. |
| labels | Array of 0–255 label objects | No | Labels to assign to the service. |

### Response

**HTTP 201 Created**

Returns the created service object.

---

## List services

List all Managed Object Storage services.

### Request

```http
GET /1.3/object-storage-2
```

### Request attributes

Query parameters:

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| limit | 0–100 (default `10`) | No | Maximum number of services to return. |
| offset | >= 0 (default `0`) | No | Offset for pagination. |
| sort | `created_at`, `service_name`, `operational_state`, `region_name`; prefix with `-` for descending (default `-created_at`) | No | Sort order for results. |

### Response

**HTTP 200 OK**

```json
[
  {
    "configured_status": "started",
    "created_at": "2023-01-01T00:00:00Z",
    "custom_domains": [],
    "endpoints": [],
    "labels": [],
    "name": "example-service",
    "networks": [],
    "operational_state": "running",
    "region": "europe-1",
    "state_messages": [],
    "static_websites": [],
    "termination_protection": false,
    "updated_at": "2023-01-01T00:00:00Z",
    "usage": {
      "total_objects": 0,
      "total_size_bytes": 0
    },
    "uuid": "0a1b2c3d-4e5f-6a7b-8c9d-0e1f2a3b4c5d"
  }
]
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| configured_status | string | The configured status of the service (`started` or `stopped`). |
| created_at | string | ISO 8601 timestamp of when the service was created. |
| custom_domains | array | List of custom domains attached to the service. |
| endpoints | array | List of service endpoints. |
| labels | array | Labels assigned to the service. |
| name | string | Name of the service. |
| networks | array | Networks attached to the service. |
| operational_state | string | Current operational state of the service. |
| region | string | Region of the service. |
| state_messages | array | Messages about the current state of the service. |
| static_websites | array | Static website configurations for the service. |
| termination_protection | boolean | Whether termination protection is enabled. |
| updated_at | string | ISO 8601 timestamp of the last update. |
| usage | object | Usage statistics for the service. |
| usage.total_objects | integer | Total number of objects stored. |
| usage.total_size_bytes | integer | Total size of all objects in bytes. |
| uuid | string | UUID of the service. |

---

## Get service details

Get full details of a specific Managed Object Storage service, including users, access keys, and policies.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}
```

### Response

**HTTP 200 OK**

```json
{
  "configured_status": "started",
  "created_at": "2023-01-01T00:00:00Z",
  "custom_domains": [],
  "endpoints": [],
  "labels": [],
  "name": "example-service",
  "networks": [],
  "operational_state": "running",
  "region": "europe-1",
  "state_messages": [],
  "static_websites": [],
  "termination_protection": false,
  "updated_at": "2023-01-01T00:00:00Z",
  "usage": {
    "total_objects": 0,
    "total_size_bytes": 0
  },
  "users": [
    {
      "access_keys": [],
      "arn": "urn:ecs:iam::example-service:user/example_user",
      "policies": [],
      "username": "example_user"
    }
  ],
  "uuid": "0a1b2c3d-4e5f-6a7b-8c9d-0e1f2a3b4c5d"
}
```

### Response attributes

Same as [List services](#list-services) response, plus:

| Attribute | Type | Description |
|-----------|------|-------------|
| users | array | List of users associated with the service. |
| users[].access_keys | array | Access keys for the user. |
| users[].arn | string | ARN of the user. |
| users[].policies | array | Policies attached to the user. |
| users[].username | string | Username of the user. |

---

## Replace service

Replace an existing Managed Object Storage service configuration entirely.

### Request

```http
PUT /1.3/object-storage-2/{service_uuid}
```

```json
{
  "name": "example-service",
  "configured_status": "started",
  "termination_protection": false,
  "networks": [
    {
      "name": "example-public-network",
      "type": "public",
      "family": "IPv4"
    }
  ],
  "labels": []
}
```

### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| name | 1–64 characters, pattern `^[a-zA-Z0-9_-]+$` | Yes | Name of the service. |
| configured_status | `started` or `stopped` | Yes | The desired status of the service. |
| termination_protection | Boolean | No | Whether termination protection is enabled. |
| networks | Array of network objects | No | Networks to attach to the service. |
| labels | Array of label objects | No | Labels to assign to the service. |

### Response

**HTTP 200 OK**

Returns the updated service object.

---

## Modify service

Partially modify an existing Managed Object Storage service using JSON Merge Patch. Setting an attribute to `null` will delete it.

### Request

```http
PATCH /1.3/object-storage-2/{service_uuid}
```

```json
{
  "configured_status": "stopped"
}
```

### Request attributes

All attributes are optional. Uses JSON Merge Patch semantics (setting a value to `null` deletes the field).

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| name | 1–64 characters, pattern `^[a-zA-Z0-9_-]+$` | No | Name of the service. |
| configured_status | `started` or `stopped` | No | The desired status of the service. |
| termination_protection | Boolean | No | Whether termination protection is enabled. |
| networks | Array of network objects | No | Networks to attach to the service. |
| labels | Array of label objects | No | Labels to assign to the service. |

### Response

**HTTP 200 OK**

Returns the updated service object.

---

## Delete service

Delete a Managed Object Storage service.

### Request

```http
DELETE /1.3/object-storage-2/{service_uuid}
```

### Request attributes

Query parameters:

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| force | Boolean (default `false`) | No | Force deletion even if termination protection is enabled. |

### Response

**HTTP 204 No Content**

---

## Create bucket

Create a new bucket in a Managed Object Storage service.

### Request

```http
POST /1.3/object-storage-2/{service_uuid}/buckets
```

```json
{
  "name": "my-bucket-1"
}
```

### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| name | 1–254 characters | Yes | Name of the bucket. |

### Response

**HTTP 201 Created**

```json
{
  "name": "my-bucket-1",
  "total_objects": 0,
  "total_size_bytes": 0
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| name | string | Name of the bucket. |
| total_objects | integer | Total number of objects in the bucket. |
| total_size_bytes | integer | Total size of objects in the bucket in bytes. |

---

## Delete bucket

Delete a bucket from a Managed Object Storage service.

### Request

```http
DELETE /1.3/object-storage-2/{service_uuid}/buckets/{bucket_name}
```

### Response

**HTTP 204 No Content**

---

## Get service metrics

Get aggregated metrics for a Managed Object Storage service.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/metrics
```

### Response

**HTTP 200 OK**

```json
{
  "total_objects": 1287,
  "total_size_bytes": 48277239
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| total_objects | integer | Total number of objects in the service. |
| total_size_bytes | integer | Total size of all objects in bytes. |

---

## List bucket metrics

List metrics for all buckets in a Managed Object Storage service.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/buckets
```

### Request attributes

Query parameters:

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| limit | Integer | No | Maximum number of results to return. |
| offset | Integer | No | Offset for pagination. |

### Response

**HTTP 200 OK**

```json
[
  {
    "name": "my-bucket-1",
    "total_objects": 500,
    "total_size_bytes": 20480000
  },
  {
    "name": "my-bucket-2",
    "total_objects": 787,
    "total_size_bytes": 27797239
  }
]
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| name | string | Name of the bucket. |
| total_objects | integer | Total number of objects in the bucket. |
| total_size_bytes | integer | Total size of objects in the bucket in bytes. |

---

## List service metrics series

List time-series metrics for a Managed Object Storage service.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/metrics/series
```

### Request attributes

Query parameters:

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| from | RFC 3339 timestamp | No | Start of the time range. |
| to | RFC 3339 timestamp | No | End of the time range. |
| interval | `1h` to `24h` | No | Aggregation interval. |
| sort | String | No | Sort order for results. |
| limit | Integer | No | Maximum number of results to return. |
| offset | Integer | No | Offset for pagination. |

### Response

**HTTP 200 OK**

```json
[
  {
    "bytes_created": 1024,
    "bytes_deleted": 512,
    "bytes_received": 2048,
    "bytes_total": 48277239,
    "bytes_transmitted": 4096,
    "end_at": "2023-01-01T01:00:00Z",
    "objects_created": 10,
    "objects_deleted": 5,
    "objects_total": 1287,
    "start_at": "2023-01-01T00:00:00Z"
  }
]
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| bytes_created | integer | Bytes created during the interval. |
| bytes_deleted | integer | Bytes deleted during the interval. |
| bytes_received | integer | Bytes received during the interval. |
| bytes_total | integer | Total bytes at the end of the interval. |
| bytes_transmitted | integer | Bytes transmitted during the interval. |
| end_at | string | End of the interval (RFC 3339). |
| objects_created | integer | Objects created during the interval. |
| objects_deleted | integer | Objects deleted during the interval. |
| objects_total | integer | Total objects at the end of the interval. |
| start_at | string | Start of the interval (RFC 3339). |

---

## Create network

Attach a network to a Managed Object Storage service.

### Request

```http
POST /1.3/object-storage-2/{service_uuid}/networks
```

Public network example:

```json
{
  "name": "example-public-network",
  "type": "public",
  "family": "IPv4"
}
```

Private network example:

```json
{
  "name": "example-private-network",
  "uuid": "03aa7245-2ff9-49c8-9f0e-7ca0270d71a4",
  "type": "private",
  "family": "IPv4"
}
```

### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| name | 1–64 characters | Yes | Name of the network. |
| uuid | Valid UUID | Yes (for private networks) | UUID of the private network to attach. |
| type | `public` or `private` | Yes | Type of the network. |
| family | `IPv4` | Yes | Address family of the network. |

### Response

**HTTP 201 Created**

Returns the created network object.

---

## List networks

List all networks attached to a Managed Object Storage service.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/networks
```

### Response

**HTTP 200 OK**

```json
[
  {
    "name": "example-public-network",
    "type": "public",
    "family": "IPv4"
  },
  {
    "name": "example-private-network",
    "uuid": "03aa7245-2ff9-49c8-9f0e-7ca0270d71a4",
    "type": "private",
    "family": "IPv4"
  }
]
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| name | string | Name of the network. |
| uuid | string | UUID of the private network (only for private networks). |
| type | string | Type of the network (`public` or `private`). |
| family | string | Address family of the network. |

---

## Get network details

Get details of a specific network attached to a Managed Object Storage service.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/networks/{network_name}
```

### Response

**HTTP 200 OK**

```json
{
  "name": "example-public-network",
  "type": "public",
  "family": "IPv4"
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| name | string | Name of the network. |
| uuid | string | UUID of the private network (only for private networks). |
| type | string | Type of the network (`public` or `private`). |
| family | string | Address family of the network. |

---

## Delete network

Remove a network from a Managed Object Storage service.

### Request

```http
DELETE /1.3/object-storage-2/{service_uuid}/networks/{network_name}
```

### Response

**HTTP 204 No Content**

---

## List groups

List all IAM groups in a Managed Object Storage service.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/groups
```

### Response

**HTTP 200 OK**

```json
[
  {
    "arn": "urn:ecs:iam::example-service:group/example-group",
    "created_at": "2023-01-01T00:00:00Z",
    "name": "example-group"
  }
]
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| arn | string | ARN of the group. |
| created_at | string | ISO 8601 timestamp of when the group was created. |
| name | string | Name of the group. |

---

## Get group

Get details of a specific IAM group.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/groups/{group_name}
```

### Response

**HTTP 200 OK**

```json
{
  "arn": "urn:ecs:iam::example-service:group/example-group",
  "created_at": "2023-01-01T00:00:00Z",
  "name": "example-group"
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| arn | string | ARN of the group. |
| created_at | string | ISO 8601 timestamp of when the group was created. |
| name | string | Name of the group. |

---

## Create group

Create a new IAM group in a Managed Object Storage service.

### Request

```http
POST /1.3/object-storage-2/{service_uuid}/groups
```

```json
{
  "name": "new-group"
}
```

### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| name | String | Yes | Name of the group. |

### Response

**HTTP 201 Created**

```json
{
  "arn": "urn:ecs:iam::example-service:group/new-group",
  "created_at": "2023-01-01T00:00:00Z",
  "name": "new-group"
}
```

---

## Attach user to group

Attach a user to an IAM group.

### Request

```http
POST /1.3/object-storage-2/{service_uuid}/groups/{group_name}/users/{username}
```

### Response

**HTTP 204 No Content**

---

## Remove user from group

Remove a user from an IAM group.

### Request

```http
DELETE /1.3/object-storage-2/{service_uuid}/groups/{group_name}/users/{username}
```

### Response

**HTTP 204 No Content**

---

## Delete group

Delete an IAM group from a Managed Object Storage service.

### Request

```http
DELETE /1.3/object-storage-2/{service_uuid}/groups/{group_name}
```

### Response

**HTTP 204 No Content**

---

## List group inline policies

List all inline policies attached to an IAM group.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/groups/{group_name}/inline-policies
```

### Response

**HTTP 200 OK**

```json
[
  {
    "document": "{\"Version\":\"2012-10-17\",\"Statement\":[...]}",
    "name": "example-inline-policy"
  }
]
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| document | string | The JSON policy document. |
| name | string | Name of the inline policy. |

---

## Get group inline policy

Get a specific inline policy attached to an IAM group.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/groups/{group_name}/inline-policies/{policy_name}
```

### Response

**HTTP 200 OK**

```json
{
  "document": "{\"Version\":\"2012-10-17\",\"Statement\":[...]}",
  "name": "example-inline-policy"
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| document | string | The JSON policy document. |
| name | string | Name of the inline policy. |

---

## Create group inline policy

Create an inline policy for an IAM group.

### Request

```http
POST /1.3/object-storage-2/{service_uuid}/groups/{group_name}/inline-policies
```

```json
{
  "name": "example-inline-policy",
  "document": "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Effect\":\"Allow\",\"Action\":[\"s3:*\"],\"Resource\":[\"*\"]}]}"
}
```

### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| name | String | Yes | Name of the inline policy. |
| document | String (JSON policy document) | Yes | The policy document. |

### Response

**HTTP 201 Created**

Returns the created inline policy object.

---

## Delete group inline policy

Delete an inline policy from an IAM group.

### Request

```http
DELETE /1.3/object-storage-2/{service_uuid}/groups/{group_name}/inline-policies/{policy_name}
```

### Response

**HTTP 204 No Content**

---

## List IAM group policies

List all managed IAM policies attached to a group.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/groups/{group_name}/policies
```

### Response

**HTTP 200 OK**

```json
[
  {
    "arn": "urn:ecs:iam::example-service:policy/example-policy",
    "name": "example-policy"
  }
]
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| arn | string | ARN of the policy. |
| name | string | Name of the policy. |

---

## Attach IAM group policy

Attach a managed IAM policy to a group.

### Request

```http
POST /1.3/object-storage-2/{service_uuid}/groups/{group_name}/policies
```

```json
{
  "name": "policy-name"
}
```

### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| name | String | Yes | Name of the policy to attach. |

### Response

**HTTP 204 No Content**

---

## Detach IAM group policy

Detach a managed IAM policy from a group.

### Request

```http
DELETE /1.3/object-storage-2/{service_uuid}/groups/{group_name}/policies/{policy_name}
```

### Response

**HTTP 204 No Content**

---

## List roles

List all IAM roles in a Managed Object Storage service.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/roles
```

### Response

**HTTP 200 OK**

```json
[
  {
    "arn": "urn:ecs:iam::example-service:role/example-role",
    "assume_role_policy_document": "...",
    "created_at": "2023-01-01T00:00:00Z",
    "description": "An example role",
    "inline_policies": [],
    "max_session_duration": 3600,
    "name": "example-role",
    "permissions_boundary": "",
    "policies": [],
    "tags": []
  }
]
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| arn | string | ARN of the role. |
| assume_role_policy_document | string | The trust policy document for the role. |
| created_at | string | ISO 8601 timestamp of when the role was created. |
| description | string | Description of the role. |
| inline_policies | array | Inline policies attached to the role. |
| max_session_duration | integer | Maximum session duration in seconds. |
| name | string | Name of the role. |
| permissions_boundary | string | Permissions boundary policy for the role. |
| policies | array | Managed policies attached to the role. |
| tags | array | Tags assigned to the role. |

---

## Get role

Get details of a specific IAM role.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/roles/{role_name}
```

### Response

**HTTP 200 OK**

```json
{
  "arn": "urn:ecs:iam::example-service:role/example-role",
  "assume_role_policy_document": "...",
  "created_at": "2023-01-01T00:00:00Z",
  "description": "An example role",
  "inline_policies": [],
  "max_session_duration": 3600,
  "name": "example-role",
  "permissions_boundary": "",
  "policies": [],
  "tags": []
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| arn | string | ARN of the role. |
| assume_role_policy_document | string | The trust policy document for the role. |
| created_at | string | ISO 8601 timestamp of when the role was created. |
| description | string | Description of the role. |
| inline_policies | array | Inline policies attached to the role. |
| max_session_duration | integer | Maximum session duration in seconds. |
| name | string | Name of the role. |
| permissions_boundary | string | Permissions boundary policy for the role. |
| policies | array | Managed policies attached to the role. |
| tags | array | Tags assigned to the role. |

---

## Create role

Create a new IAM role in a Managed Object Storage service.

### Request

```http
POST /1.3/object-storage-2/{service_uuid}/roles
```

```json
{
  "name": "example-role",
  "assume_role_policy_document": "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Effect\":\"Allow\",\"Principal\":{\"AWS\":\"*\"},\"Action\":\"sts:AssumeRole\"}]}",
  "description": "An example role",
  "max_session_duration": 3600,
  "permissions_boundary": "example-boundary-policy",
  "tags": [
    { "key": "environment", "value": "production" }
  ]
}
```

### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| name | 1–64 characters | Yes | Name of the role. |
| assume_role_policy_document | String, max 131072 characters | Yes | The trust policy document for the role. |
| description | 1–1000 characters | No | Description of the role. |
| max_session_duration | 900–43200 (default `3600`) | No | Maximum session duration in seconds. |
| permissions_boundary | 1–128 characters | No | Permissions boundary policy name. |
| tags | Array, max 50 items | No | Tags to assign to the role. |

### Response

**HTTP 201 Created**

Returns the created role object.

---

## Update role

Partially update an existing IAM role.

### Request

```http
PATCH /1.3/object-storage-2/{service_uuid}/roles/{role_name}
```

```json
{
  "description": "Updated description",
  "max_session_duration": 7200
}
```

### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| description | 1–1000 characters | No | Updated description of the role. |
| max_session_duration | 900–43200 | No | Updated maximum session duration in seconds. |

### Response

**HTTP 200 OK**

Returns the updated role object.

---

## Delete role

Delete an IAM role from a Managed Object Storage service.

### Request

```http
DELETE /1.3/object-storage-2/{service_uuid}/role/{role_name}
```

### Response

**HTTP 204 No Content**

---

## List attached role policies

List all managed IAM policies attached to a role.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/roles/{role_name}/policies
```

### Response

**HTTP 200 OK**

```json
[
  {
    "policy_name": "example-policy",
    "policy_arn": "urn:ecs:iam::example-service:policy/example-policy"
  }
]
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| policy_name | string | Name of the attached policy. |
| policy_arn | string | ARN of the attached policy. |

---

## Attach policy to role

Attach a managed IAM policy to a role.

### Request

```http
POST /1.3/object-storage-2/{service_uuid}/roles/{role_name}/policies
```

```json
{
  "name": "test-policy"
}
```

### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| name | String | Yes | Name of the policy to attach. |

### Response

**HTTP 204 No Content**

---

## Detach policy from role

Detach a managed IAM policy from a role.

### Request

```http
DELETE /1.3/object-storage-2/{service_uuid}/roles/{role_name}/policies/{policy_name}
```

### Response

**HTTP 204 No Content**

---

## List role inline policies

List all inline policies attached to an IAM role.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/roles/{role_name}/inline-policies
```

### Response

**HTTP 200 OK**

```json
[
  {
    "document": "{\"Version\":\"2012-10-17\",\"Statement\":[...]}",
    "name": "example-inline-policy"
  }
]
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| document | string | The JSON policy document. |
| name | string | Name of the inline policy. |

---

## Get role inline policy

Get a specific inline policy attached to an IAM role.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/roles/{role_name}/inline-policies/{policy_name}
```

### Response

**HTTP 200 OK**

```json
{
  "document": "{\"Version\":\"2012-10-17\",\"Statement\":[...]}",
  "name": "example-inline-policy"
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| document | string | The JSON policy document. |
| name | string | Name of the inline policy. |

---

## Create role inline policy

Create an inline policy for an IAM role.

### Request

```http
POST /1.3/object-storage-2/{service_uuid}/roles/{role_name}/inline-policies
```

```json
{
  "name": "example-inline-policy",
  "document": "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Effect\":\"Allow\",\"Action\":[\"s3:*\"],\"Resource\":[\"*\"]}]}"
}
```

### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| name | String | Yes | Name of the inline policy. |
| document | String (JSON policy document) | Yes | The policy document. |

### Response

**HTTP 201 Created**

Returns the created inline policy object.

---

## Delete role inline policy

Delete an inline policy from an IAM role.

### Request

```http
DELETE /1.3/object-storage-2/{service_uuid}/roles/{role_name}/inline-policies/{policy_name}
```

### Response

**HTTP 204 No Content**

---

## Create role permissions boundary

Set the permissions boundary for an IAM role.

### Request

```http
PUT /1.3/object-storage-2/{service_uuid}/roles/{role_name}/permissions-boundary
```

```json
{
  "policy_name": "sample-policy"
}
```

### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| policy_name | String | Yes | Name of the policy to use as the permissions boundary. |

### Response

**HTTP 200 OK**

```json
{
  "policy_name": "sample-policy"
}
```

---

## Delete role permissions boundary

Remove the permissions boundary from an IAM role.

### Request

```http
DELETE /1.3/object-storage-2/{service_uuid}/roles/{role_name}/permissions-boundary
```

### Response

**HTTP 204 No Content**

---

## List role tags

List all tags assigned to an IAM role.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/roles/{role_name}/tags
```

### Response

**HTTP 200 OK**

```json
[
  {
    "key": "environment",
    "value": "production"
  }
]
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| key | string | Tag key. |
| value | string | Tag value. |

---

## Replace role tags

Replace all tags on an IAM role.

### Request

```http
PUT /1.3/object-storage-2/{service_uuid}/roles/{role_name}/tags
```

```json
{
  "key": "key",
  "value": "val"
}
```

### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| key | String | Yes | Tag key. |
| value | String | Yes | Tag value. |

### Response

**HTTP 200 OK**

Returns the updated tag object.

---

## Delete role tag

Delete a specific tag from an IAM role.

### Request

```http
DELETE /1.3/object-storage-2/{service_uuid}/roles/{role_name}/tags/{tag_key}
```

### Response

**HTTP 204 No Content**

---

## Assume role policy

Set the assume role policy document for an IAM role.

### Request

```http
PUT /1.3/object-storage-2/{service_uuid}/roles/{role_name}/assume-role-policy
```

```json
{
  "document": "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Effect\":\"Allow\",\"Principal\":{\"AWS\":\"*\"},\"Action\":\"sts:AssumeRole\"}]}"
}
```

### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| document | String (JSON policy document) | Yes | The trust policy document. |

### Response

**HTTP 200 OK**

Returns the updated assume role policy.

---

## Create policy

Create a new managed IAM policy in a Managed Object Storage service.

### Request

```http
POST /1.3/object-storage-2/{service_uuid}/policies
```

```json
{
  "name": "example-policy",
  "description": "An example policy",
  "document": "%7B%22Version%22%3A%222012-10-17%22%2C%22Statement%22%3A%5B%7B%22Effect%22%3A%22Allow%22%2C%22Action%22%3A%5B%22s3%3A%2A%22%5D%2C%22Resource%22%3A%5B%22%2A%22%5D%7D%5D%7D"
}
```

### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| name | 1–128 characters, pattern `[\w+=,.@-]+` | Yes | Name of the policy. |
| description | 1–1000 characters | No | Description of the policy. |
| document | 1–6144 characters, URL-encoded | Yes | The policy document (URL-encoded JSON). |

### Response

**HTTP 201 Created**

Returns the created policy object.

---

## List policies

List all managed IAM policies in a Managed Object Storage service.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/policies
```

### Response

**HTTP 200 OK**

```json
[
  {
    "arn": "urn:ecs:iam::example-service:policy/example-policy",
    "attachment_count": 1,
    "created_at": "2023-01-01T00:00:00Z",
    "default_version_id": "v1",
    "description": "An example policy",
    "name": "example-policy",
    "system": false,
    "updated_at": "2023-01-01T00:00:00Z"
  }
]
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| arn | string | ARN of the policy. |
| attachment_count | integer | Number of entities the policy is attached to. |
| created_at | string | ISO 8601 timestamp of when the policy was created. |
| default_version_id | string | ID of the default policy version. |
| description | string | Description of the policy. |
| name | string | Name of the policy. |
| system | boolean | Whether this is a system-managed policy. |
| updated_at | string | ISO 8601 timestamp of the last update. |

---

## Get policy details

Get details of a specific managed IAM policy, including its document.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/policies/{policy_name}
```

### Response

**HTTP 200 OK**

```json
{
  "arn": "urn:ecs:iam::example-service:policy/example-policy",
  "attachment_count": 1,
  "created_at": "2023-01-01T00:00:00Z",
  "default_version_id": "v1",
  "description": "An example policy",
  "document": "{\"Version\":\"2012-10-17\",\"Statement\":[...]}",
  "name": "example-policy",
  "system": false,
  "updated_at": "2023-01-01T00:00:00Z"
}
```

### Response attributes

Same as [List policies](#list-policies) response, plus:

| Attribute | Type | Description |
|-----------|------|-------------|
| document | string | The JSON policy document. |

---

## Delete policy

Delete a managed IAM policy from a Managed Object Storage service.

### Request

```http
DELETE /1.3/object-storage-2/{service_uuid}/policies/{policy_name}
```

### Response

**HTTP 204 No Content**

---

## Create policy version

Create a new version of an existing managed IAM policy.

### Request

```http
POST /1.3/object-storage-2/{service_uuid}/policies/{policy_name}/versions
```

```json
{
  "document": "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Effect\":\"Allow\",\"Action\":[\"s3:GetObject\"],\"Resource\":[\"*\"]}]}",
  "is_default": true
}
```

### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| document | String (JSON policy document) | Yes | The policy document for the new version. |
| is_default | Boolean | No | Whether this version should become the default. |

### Response

**HTTP 201 Created**

```json
{
  "create_date": "2023-01-01T00:00:00Z",
  "document": "{\"Version\":\"2012-10-17\",\"Statement\":[...]}",
  "is_default": true,
  "version_id": "v2"
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| create_date | string | ISO 8601 timestamp of when the version was created. |
| document | string | The JSON policy document. |
| is_default | boolean | Whether this is the default version. |
| version_id | string | Version identifier. |

---

## List policy versions

List all versions of a managed IAM policy.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/policies/{policy_name}/versions
```

### Response

**HTTP 200 OK**

```json
[
  {
    "create_date": "2023-01-01T00:00:00Z",
    "document": "{\"Version\":\"2012-10-17\",\"Statement\":[...]}",
    "is_default": true,
    "version_id": "v1"
  }
]
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| create_date | string | ISO 8601 timestamp of when the version was created. |
| document | string | The JSON policy document. |
| is_default | boolean | Whether this is the default version. |
| version_id | string | Version identifier. |

---

## Get policy version

Get a specific version of a managed IAM policy.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/policies/{policy_name}/versions/{policy_version}
```

### Response

**HTTP 200 OK**

```json
{
  "create_date": "2023-01-01T00:00:00Z",
  "document": "{\"Version\":\"2012-10-17\",\"Statement\":[...]}",
  "is_default": true,
  "version_id": "v1"
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| create_date | string | ISO 8601 timestamp of when the version was created. |
| document | string | The JSON policy document. |
| is_default | boolean | Whether this is the default version. |
| version_id | string | Version identifier. |

---

## Delete policy version

Delete a specific version of a managed IAM policy.

### Request

```http
DELETE /1.3/object-storage-2/{service_uuid}/policies/{policy_name}/versions/{policy_version}
```

### Response

**HTTP 204 No Content**

---

## Create user

Create a new user in a Managed Object Storage service.

### Request

```http
POST /1.3/object-storage-2/{service_uuid}/users
```

```json
{
  "username": "example_user"
}
```

### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| username | 1–64 characters, pattern `[\w+=,.@-]+` | Yes | Username for the new user. |

### Response

**HTTP 201 Created**

```json
{
  "access_keys": [],
  "arn": "urn:ecs:iam::example-service:user/example_user",
  "created_at": "2023-01-01T00:00:00Z",
  "policies": [],
  "username": "example_user"
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| access_keys | array | Access keys for the user. |
| arn | string | ARN of the user. |
| created_at | string | ISO 8601 timestamp of when the user was created. |
| policies | array | Policies attached to the user. |
| username | string | Username of the user. |

---

## List users

List all users in a Managed Object Storage service.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/users
```

### Response

**HTTP 200 OK**

```json
[
  {
    "access_keys": [],
    "arn": "urn:ecs:iam::example-service:user/example_user",
    "created_at": "2023-01-01T00:00:00Z",
    "policies": [],
    "username": "example_user"
  }
]
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| access_keys | array | Access keys for the user. |
| arn | string | ARN of the user. |
| created_at | string | ISO 8601 timestamp of when the user was created. |
| policies | array | Policies attached to the user. |
| username | string | Username of the user. |

---

## Get user details

Get details of a specific user in a Managed Object Storage service.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/users/{username}
```

### Response

**HTTP 200 OK**

```json
{
  "access_keys": [],
  "arn": "urn:ecs:iam::example-service:user/example_user",
  "created_at": "2023-01-01T00:00:00Z",
  "policies": [],
  "username": "example_user"
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| access_keys | array | Access keys for the user. |
| arn | string | ARN of the user. |
| created_at | string | ISO 8601 timestamp of when the user was created. |
| policies | array | Policies attached to the user. |
| username | string | Username of the user. |

---

## Delete user

Delete a user from a Managed Object Storage service.

### Request

```http
DELETE /1.3/object-storage-2/{service_uuid}/users/{username}
```

### Response

**HTTP 204 No Content**

---

## List user tags

List all tags assigned to a user.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/users/{username}/tags
```

### Response

**HTTP 200 OK**

```json
[
  {
    "key": "department",
    "value": "engineering"
  }
]
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| key | string | Tag key. |
| value | string | Tag value. |

---

## Replace user tags

Replace all tags on a user.

### Request

```http
PUT /1.3/object-storage-2/{service_uuid}/users/{username}/tags
```

```json
{
  "key": "key",
  "value": "val"
}
```

### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| key | String | Yes | Tag key. |
| value | String | Yes | Tag value. |

### Response

**HTTP 200 OK**

Returns the updated tag object.

---

## Delete user tag

Delete a specific tag from a user.

### Request

```http
DELETE /1.3/object-storage-2/{service_uuid}/users/{username}/tags/{tag_key}
```

### Response

**HTTP 204 No Content**

---

## Create user permissions boundary

Set the permissions boundary for a user.

### Request

```http
PUT /1.3/object-storage-2/{service_uuid}/users/{username}/permissions-boundary
```

```json
{
  "policy_name": "sample-policy"
}
```

### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| policy_name | String | Yes | Name of the policy to use as the permissions boundary. |

### Response

**HTTP 200 OK**

Returns the permissions boundary object.

---

## Delete user permissions boundary

Remove the permissions boundary from a user.

### Request

```http
DELETE /1.3/object-storage-2/{service_uuid}/users/{username}/permissions-boundary
```

### Response

**HTTP 204 No Content**

---

## List user inline policies

List all inline policies attached to a user.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/users/{username}/inline-policies
```

### Response

**HTTP 200 OK**

```json
[
  {
    "document": "{\"Version\":\"2012-10-17\",\"Statement\":[...]}",
    "name": "example-inline-policy"
  }
]
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| document | string | The JSON policy document. |
| name | string | Name of the inline policy. |

---

## Get user inline policy

Get a specific inline policy attached to a user.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/users/{username}/inline-policies/{policy_name}
```

### Response

**HTTP 200 OK**

```json
{
  "document": "{\"Version\":\"2012-10-17\",\"Statement\":[...]}",
  "name": "example-inline-policy"
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| document | string | The JSON policy document. |
| name | string | Name of the inline policy. |

---

## Create user inline policy

Create an inline policy for a user.

### Request

```http
POST /1.3/object-storage-2/{service_uuid}/users/{username}/inline-policies
```

```json
{
  "name": "example-inline-policy",
  "document": "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Effect\":\"Allow\",\"Action\":[\"s3:*\"],\"Resource\":[\"*\"]}]}"
}
```

### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| name | String | Yes | Name of the inline policy. |
| document | String (JSON policy document) | Yes | The policy document. |

### Response

**HTTP 201 Created**

Returns the created inline policy object.

---

## Delete user inline policy

Delete an inline policy from a user.

### Request

```http
DELETE /1.3/object-storage-2/{service_uuid}/users/{username}/inline-policies/{policy_name}
```

### Response

**HTTP 204 No Content**

---

## Create access key

Create a new access key for a user. The `secret_access_key` is only returned once and cannot be retrieved again.

### Request

```http
POST /1.3/object-storage-2/{service_uuid}/users/{username}/access-keys
```

### Response

**HTTP 201 Created**

```json
{
  "access_key_id": "AKIA0000000000000001",
  "created_at": "2023-01-01T00:00:00Z",
  "last_used_at": null,
  "secret_access_key": "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
  "status": "Active"
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| access_key_id | string | The access key ID. |
| created_at | string | ISO 8601 timestamp of when the key was created. |
| last_used_at | string or null | ISO 8601 timestamp of when the key was last used. |
| secret_access_key | string | The secret access key. **Only returned once at creation time.** |
| status | string | Status of the key (`Active` or `Inactive`). |

---

## List access keys

List all access keys for a user. The `secret_access_key` is not included in list responses.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/users/{username}/access-keys
```

### Response

**HTTP 200 OK**

```json
[
  {
    "access_key_id": "AKIA0000000000000001",
    "created_at": "2023-01-01T00:00:00Z",
    "last_used_at": "2023-06-15T12:00:00Z",
    "status": "Active"
  }
]
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| access_key_id | string | The access key ID. |
| created_at | string | ISO 8601 timestamp of when the key was created. |
| last_used_at | string or null | ISO 8601 timestamp of when the key was last used. |
| status | string | Status of the key (`Active` or `Inactive`). |

---

## Get access key details

Get details of a specific access key for a user.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/users/{username}/access-keys/{access-key-id}
```

### Response

**HTTP 200 OK**

```json
{
  "access_key_id": "AKIA0000000000000001",
  "created_at": "2023-01-01T00:00:00Z",
  "last_used_at": "2023-06-15T12:00:00Z",
  "status": "Active"
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| access_key_id | string | The access key ID. |
| created_at | string | ISO 8601 timestamp of when the key was created. |
| last_used_at | string or null | ISO 8601 timestamp of when the key was last used. |
| status | string | Status of the key (`Active` or `Inactive`). |

---

## Modify access key

Modify the status of an access key.

### Request

```http
PATCH /1.3/object-storage-2/{service_uuid}/users/{username}/access-keys/{access-key-id}
```

```json
{
  "status": "Inactive"
}
```

### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| status | `Active` or `Inactive` | Yes | The new status of the access key. |

### Response

**HTTP 200 OK**

Returns the updated access key object.

---

## Delete access key

Delete an access key for a user.

### Request

```http
DELETE /1.3/object-storage-2/{service_uuid}/users/{username}/access-keys/{access-key-id}
```

### Response

**HTTP 204 No Content**

---

## List attached user policies

List all managed IAM policies attached to a user.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/users/{username}/policies
```

### Response

**HTTP 200 OK**

```json
[
  {
    "arn": "urn:ecs:iam::example-service:policy/ECSS3FullAccess",
    "name": "ECSS3FullAccess"
  }
]
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| arn | string | ARN of the policy. |
| name | string | Name of the policy. |

---

## Attach user policy

Attach a managed IAM policy to a user.

### Request

```http
POST /1.3/object-storage-2/{service_uuid}/users/{username}/policies
```

```json
{
  "name": "ECSS3FullAccess"
}
```

### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| name | String | Yes | Name of the policy to attach. |

### Response

**HTTP 200 OK**

---

## Detach user policy

Detach a managed IAM policy from a user.

### Request

```http
DELETE /1.3/object-storage-2/{service_uuid}/users/{username}/policies/{policy_name}
```

### Response

**HTTP 204 No Content**

---

## Create label

Add a label to a Managed Object Storage service.

### Request

```http
POST /1.3/object-storage-2/{service_uuid}/labels
```

```json
{
  "key": "foo",
  "value": "bar"
}
```

### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| key | 2–32 printable ASCII characters, must not start with `_` | Yes | Label key. |
| value | 0–255 characters | Yes | Label value. |

### Response

**HTTP 201 Created**

Returns the created label object.

---

## List labels

List all labels on a Managed Object Storage service.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/labels
```

### Response

**HTTP 200 OK**

```json
[
  {
    "key": "foo",
    "value": "bar"
  }
]
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| key | string | Label key. |
| value | string | Label value. |

---

## Get label details

Get a specific label on a Managed Object Storage service.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/labels/{key}
```

### Response

**HTTP 200 OK**

```json
{
  "key": "foo",
  "value": "bar"
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| key | string | Label key. |
| value | string | Label value. |

---

## Modify label

Modify an existing label on a Managed Object Storage service.

### Request

```http
PATCH /1.3/object-storage-2/{service_uuid}/labels/{key}
```

```json
{
  "value": "updated-value"
}
```

### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| value | 0–255 characters | No | Updated label value. |

### Response

**HTTP 200 OK**

Returns the updated label object.

---

## Delete label

Delete a label from a Managed Object Storage service.

### Request

```http
DELETE /1.3/object-storage-2/{service_uuid}/labels/{key}
```

### Response

**HTTP 204 No Content**

---

## Attach custom domain

Attach a custom domain to a Managed Object Storage service.

### Request

```http
POST /1.3/object-storage-2/{service_uuid}/custom-domains
```

```json
{
  "domain_name": "objects.example.com",
  "mode": "api",
  "type": "public"
}
```

### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| domain_name | Domain name with 2–5 parts | Yes | The custom domain name. |
| mode | `api` or `static-website` (default `api`) | No | Mode of the custom domain. |
| type | `public` | Yes | Type of the custom domain. |

### Response

**HTTP 201 Created**

Returns the created custom domain object.

---

## Get custom domain details

Get details of a specific custom domain.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/custom-domains/{domain_name}
```

### Response

**HTTP 200 OK**

```json
{
  "domain_name": "objects.example.com",
  "mode": "api",
  "type": "public"
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| domain_name | string | The custom domain name. |
| mode | string | Mode of the custom domain (`api` or `static-website`). |
| type | string | Type of the custom domain. |

---

## List custom domains

List all custom domains attached to a Managed Object Storage service.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/custom-domains
```

### Response

**HTTP 200 OK**

```json
[
  {
    "domain_name": "objects.example.com",
    "mode": "api",
    "type": "public"
  }
]
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| domain_name | string | The custom domain name. |
| mode | string | Mode of the custom domain (`api` or `static-website`). |
| type | string | Type of the custom domain. |

---

## Delete custom domain

Remove a custom domain from a Managed Object Storage service.

### Request

```http
DELETE /1.3/object-storage-2/{service_uuid}/custom-domains/{domain_name}
```

### Response

**HTTP 204 No Content**

---

## Modify custom domain

Modify an existing custom domain.

### Request

```http
PATCH /1.3/object-storage-2/{service_uuid}/custom-domains/{domain_name}
```

```json
{
  "domain_name": "objects.example.com",
  "type": "public"
}
```

### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| domain_name | Domain name with 2–5 parts | No | Updated custom domain name. |
| type | `public` | No | Type of the custom domain. |

### Response

**HTTP 200 OK**

Returns the updated custom domain object.

---

## List static website configs

List all static website configurations for a Managed Object Storage service.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/static-websites
```

### Response

**HTTP 200 OK**

```json
[
  {
    "domain_name": "static.example.com",
    "bucket_name": "my-website",
    "bucket_prefix": "public/",
    "index_document": "index.html",
    "spa_mode": false,
    "enabled": true,
    "error_pages": [
      {
        "status_code": 404,
        "error_document": "errors/404.html"
      }
    ]
  }
]
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| domain_name | string | Custom domain for the static website. |
| bucket_name | string | Name of the bucket serving the website. |
| bucket_prefix | string | Prefix path within the bucket. |
| index_document | string | Default index document. |
| spa_mode | boolean | Whether single-page application mode is enabled. |
| enabled | boolean | Whether the static website is enabled. |
| error_pages | array | Custom error page configurations. |
| error_pages[].status_code | integer | HTTP status code for the error page. |
| error_pages[].error_document | string | Path to the error document in the bucket. |

---

## Create static website config

Create a static website configuration for a Managed Object Storage service.

### Request

```http
POST /1.3/object-storage-2/{service_uuid}/static-websites
```

```json
{
  "domain_name": "static.example.com",
  "bucket_name": "my-website",
  "bucket_prefix": "public/",
  "index_document": "index.html",
  "spa_mode": false,
  "enabled": true,
  "error_pages": [
    {
      "status_code": 404,
      "error_document": "errors/404.html"
    }
  ]
}
```

### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| domain_name | String | No | Custom domain for the static website. |
| bucket_name | String | Yes | Name of the bucket serving the website. |
| bucket_prefix | String | No | Prefix path within the bucket. |
| index_document | String (default `index.html`) | No | Default index document. |
| spa_mode | Boolean | No | Whether to enable single-page application mode. |
| enabled | Boolean | No | Whether the static website is enabled. |
| error_pages | Array | No | Custom error page configurations. |

### Response

**HTTP 201 Created**

Returns the created static website configuration object.

---

## Get static website config

Get details of a specific static website configuration.

### Request

```http
GET /1.3/object-storage-2/{service_uuid}/static-websites/{domain_name}
```

### Response

**HTTP 200 OK**

```json
{
  "domain_name": "static.example.com",
  "bucket_name": "my-website",
  "bucket_prefix": "public/",
  "index_document": "index.html",
  "spa_mode": false,
  "enabled": true,
  "error_pages": [
    {
      "status_code": 404,
      "error_document": "errors/404.html"
    }
  ]
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| domain_name | string | Custom domain for the static website. |
| bucket_name | string | Name of the bucket serving the website. |
| bucket_prefix | string | Prefix path within the bucket. |
| index_document | string | Default index document. |
| spa_mode | boolean | Whether single-page application mode is enabled. |
| enabled | boolean | Whether the static website is enabled. |
| error_pages | array | Custom error page configurations. |

---

## Modify static website config

Modify an existing static website configuration.

### Request

```http
PATCH /1.3/object-storage-2/{service_uuid}/static-websites/{domain_name}
```

```json
{
  "enabled": false
}
```

### Request attributes

All attributes are optional.

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| bucket_name | String | No | Name of the bucket serving the website. |
| bucket_prefix | String | No | Prefix path within the bucket. |
| index_document | String | No | Default index document. |
| spa_mode | Boolean | No | Whether to enable single-page application mode. |
| enabled | Boolean | No | Whether the static website is enabled. |
| error_pages | Array | No | Custom error page configurations. |

### Response

**HTTP 200 OK**

Returns the updated static website configuration object.

---

## Delete static website config

Delete a static website configuration.

### Request

```http
DELETE /1.3/object-storage-2/{service_uuid}/static-websites/{domain_name}
```

### Response

**HTTP 204 No Content**

---

## List jobs

List all data migration jobs.

### Request

```http
GET /1.3/object-storage-2/jobs
```

### Response

**HTTP 200 OK**

```json
[
  {
    "input": {
      "source": {
        "access_key_id": "AKIA...",
        "endpoint_url": "https://s3.example.com",
        "bucket": "source-bucket"
      },
      "target": {
        "access_key_id": "AKIA...",
        "endpoint_url": "https://s3.target.com",
        "bucket": "target-bucket"
      },
      "type": "copy"
    },
    "operational_state": "running",
    "output": {},
    "uuid": "0a1b2c3d-4e5f-6a7b-8c9d-0e1f2a3b4c5d",
    "created_at": "2023-01-01T00:00:00Z",
    "updated_at": "2023-01-01T01:00:00Z"
  }
]
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| input | object | Job input configuration. |
| input.source | object | Source configuration. |
| input.target | object | Target configuration. |
| input.type | string | Type of the job (e.g., `copy`). |
| operational_state | string | Current operational state of the job. |
| output | object | Job output data. |
| uuid | string | UUID of the job. |
| created_at | string | ISO 8601 timestamp of when the job was created. |
| updated_at | string | ISO 8601 timestamp of the last update. |

---

## Get job details

Get details of a specific data migration job.

### Request

```http
GET /1.3/object-storage-2/jobs/{job-uuid}
```

### Response

**HTTP 200 OK**

```json
{
  "input": {
    "source": {
      "access_key_id": "AKIA...",
      "endpoint_url": "https://s3.example.com",
      "bucket": "source-bucket"
    },
    "target": {
      "access_key_id": "AKIA...",
      "endpoint_url": "https://s3.target.com",
      "bucket": "target-bucket"
    },
    "type": "copy"
  },
  "operational_state": "running",
  "output": {},
  "uuid": "0a1b2c3d-4e5f-6a7b-8c9d-0e1f2a3b4c5d",
  "created_at": "2023-01-01T00:00:00Z",
  "updated_at": "2023-01-01T01:00:00Z"
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| input | object | Job input configuration. |
| input.source | object | Source configuration. |
| input.target | object | Target configuration. |
| input.type | string | Type of the job. |
| operational_state | string | Current operational state of the job. |
| output | object | Job output data. |
| uuid | string | UUID of the job. |
| created_at | string | ISO 8601 timestamp of when the job was created. |
| updated_at | string | ISO 8601 timestamp of the last update. |

---

## Create job

Create a new data migration job to copy objects between storage endpoints.

### Request

```http
POST /1.3/object-storage-2/jobs
```

```json
{
  "type": "copy",
  "source": {
    "access_key_id": "AKIA0000000000000001",
    "secret_access_key": "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
    "endpoint_url": "https://s3.example.com",
    "bucket": "source-bucket"
  },
  "target": {
    "access_key_id": "AKIA0000000000000002",
    "secret_access_key": "anotherSecretAccessKeyExample1234567890ABC",
    "endpoint_url": "https://s3.target.com",
    "bucket": "target-bucket"
  }
}
```

### Request attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| type | `copy` | Yes | Type of the job. |
| source | Object | Yes | Source storage configuration. |
| source.access_key_id | String | Yes | Access key ID for the source. |
| source.secret_access_key | String | Yes | Secret access key for the source. |
| source.endpoint_url | String | Yes | Endpoint URL of the source storage. |
| source.bucket | String | No | Source bucket name. |
| target | Object | Yes | Target storage configuration. |
| target.access_key_id | String | Yes | Access key ID for the target. |
| target.secret_access_key | String | Yes | Secret access key for the target. |
| target.endpoint_url | String | Yes | Endpoint URL of the target storage. |
| target.bucket | String | No | Target bucket name. |

### Response

**HTTP 201 Created**

Returns the created job object.

---

## Cancel job

Cancel a running data migration job.

### Request

```http
POST /1.3/object-storage-2/jobs/{job-uuid}/cancel
```

### Response

**HTTP 200 OK**

Returns the updated job object with the new operational state.

---

## Service configured status

The configured status controls whether the service is running or stopped.

| Status | Description |
|--------|-------------|
| `started` | The service is configured to run. |
| `stopped` | The service is configured to be stopped. |

---

## Service operational state

The operational state represents the current lifecycle phase of the service.

| State | Description |
|-------|-------------|
| `pending` | Service creation has been requested. |
| `setup-service` | Setting up the service. |
| `setup-network` | Configuring service networks. |
| `setup-tls` | Configuring TLS certificates. |
| `setup-public-endpoint` | Setting up the public endpoint. |
| `setup-private-endpoint` | Setting up private endpoints. |
| `setup-dns` | Configuring DNS records. |
| `setup-iam` | Setting up IAM. |
| `cleanup-deleted-buckets` | Cleaning up deleted buckets. |
| `setup-checkup` | Performing setup verification. |
| `running` | The service is fully operational. |
| `stopped` | The service is stopped. |
| `delete-private-endpoint` | Removing private endpoints. |
| `delete-public-endpoint` | Removing the public endpoint. |
| `delete-buckets` | Deleting buckets. |
| `delete-iam` | Removing IAM configuration. |
| `delete-namespace` | Deleting the namespace. |
| `delete-dns` | Removing DNS records. |
| `delete-tls` | Removing TLS certificates. |
| `delete-network` | Removing network configuration. |
| `delete-service` | Completing service deletion. |

---

## Job operational state

The operational state of a data migration job.

| State | Description |
|-------|-------------|
| `pending` | Job has been created but not yet started. |
| `configuring` | Job is being configured. |
| `running` | Job is currently executing. |
| `completed` | Job has finished successfully. |
| `failed` | Job has failed. |
| `canceled` | Job has been canceled. |
