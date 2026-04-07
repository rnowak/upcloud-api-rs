# Tags

> **Note:** Consider using labels instead of tags. Tags are an access control
> feature and only available for a limited set of resources. Use labels to
> describe and filter your resources.

Users can define a set of free-form tags to group and identify a subset of their
servers. Tags can be for example *webserver*, *database*, *PROD*, *QA*, *DEV*,
etc.

A server can have several tags. The API can be used to return only servers
matching specified tags. All operations changing a server's state or attributes
must still be done individually for each server UUID — tags can only be used to
return a list of servers.

Creating and modifying tags is allowed **only by the main account**.
Subaccounts have access only to listing tags and tagged servers they are granted
access to.

## Tag attributes

| Attribute   | Accepted values                                      | Default value | Required | Description                                               |
|-------------|------------------------------------------------------|---------------|----------|-----------------------------------------------------------|
| name        | 1–32 characters `A-Z`, `a-z`, `0-9`, `_`            |               | Yes      | The tag name                                              |
| description | A string of max 255 characters                       |               | No       | Description of the tag                                    |
| servers     | A `server` element with a list of server UUIDs       |               | No       | List of valid UUIDs of existing servers to be tagged      |

---

## List tags

Returns all existing tags with their properties and tagged servers.

### Request

```
GET /1.3/tag HTTP/1.1
```

### Normal response

```
HTTP/1.1 200 OK
```

```json
{
  "tags": {
    "tag": [
      {
        "description": "Development servers",
        "name": "DEV",
        "servers": {
          "server": [
            "0077fa3d-32db-4b09-9f5f-30d9e9afb565"
          ]
        }
      },
      {
        "description": "My own servers",
        "name": "private",
        "servers": {
          "server": []
        }
      },
      {
        "description": "Production servers",
        "name": "PROD",
        "servers": {
          "server": []
        }
      }
    ]
  }
}
```

---

## Create a new tag

Creates a new tag. Existing servers can be tagged in the same request.

### Request

```
POST /1.3/tag HTTP/1.1
```

```json
{
  "tag": {
    "name": "DEV",
    "description": "Development servers",
    "servers": {
      "server": [
        "0077fa3d-32db-4b09-9f5f-30d9e9afb565"
      ]
    }
  }
}
```

### Attributes

| Attribute   | Accepted values                                      | Default value | Required | Description                                               |
|-------------|------------------------------------------------------|---------------|----------|-----------------------------------------------------------|
| name        | 1–32 characters `A-Z`, `a-z`, `0-9`, `_`            |               | Yes      | The new tag                                               |
| description | A string of max 255 characters                       |               | No       | Description of the tag                                    |
| servers     | A `server` element with a list of server UUIDs       |               | No       | List of valid UUIDs of existing servers to be tagged      |

### Normal response

```
HTTP/1.1 200 OK
```

```json
{
  "tag": {
    "name": "DEV",
    "description": "Development servers",
    "servers": {
      "server": [
        "0077fa3d-32db-4b09-9f5f-30d9e9afb565"
      ]
    }
  }
}
```

### Error responses

| HTTP status | Error code     | Description                                  |
|-------------|----------------|----------------------------------------------|
| 400         | TAG_INVALID    | The tag name is invalid                      |
| 403         | TAG_FORBIDDEN  | Username does not have access to this tag    |
| 409         | TAG_EXISTS     | Tag already exists                           |

---

## Modify existing tag

Changes attributes of an existing tag. Attributes are the same as with new tag
creation.

### Request

```
PUT /1.3/tag/{tagname} HTTP/1.1
```

```json
{
  "tag": {
    "name": "PROD",
    "description": "Production servers",
    "servers": {
      "server": [
        "0077fa3d-32db-4b09-9f5f-30d9e9afb565"
      ]
    }
  }
}
```

### Normal response

```
HTTP/1.1 200 OK
```

```json
{
  "tag": {
    "name": "PROD",
    "description": "Production servers",
    "servers": {
      "server": [
        "0077fa3d-32db-4b09-9f5f-30d9e9afb565"
      ]
    }
  }
}
```

### Error responses

| HTTP status | Error code     | Description                                  |
|-------------|----------------|----------------------------------------------|
| 400         | TAG_INVALID    | The tag name is invalid                      |
| 403         | TAG_FORBIDDEN  | Username does not have access to this tag    |
| 404         | TAG_NOT_FOUND  | Tag does not exist                           |
| 409         | TAG_EXISTS     | Tag already exists                           |

---

## Delete tag

Deleting an existing tag untags all servers from the specified tag and deletes
the tag definition.

### Request

```
DELETE /1.3/tag/{tagname} HTTP/1.1
```

### Normal response

```
HTTP/1.1 204 No Content
```

### Error responses

| HTTP status | Error code     | Description                                  |
|-------------|----------------|----------------------------------------------|
| 400         | TAG_INVALID    | The tag name is invalid                      |
| 403         | TAG_FORBIDDEN  | Username does not have access to this tag    |
| 404         | TAG_NOT_FOUND  | Tag does not exist                           |

---

## Assign tag to a server

Servers can be tagged with one or more tags. The tags used must already exist.

### Request

```
POST /1.3/server/{uuid}/tag/{tags} HTTP/1.1
```

The `tags` parameter lists the tags to add to the server as a comma-separated
list, e.g. `dev,test`.

### Normal response

```
HTTP/1.1 200 OK
```

```json
{
  "server": {
    "boot_order": "disk",
    "core_number": "2",
    "firewall": "on",
    "hostname": "debian.example.com",
    "ip_addresses": {
      "ip_address": [
        {
          "access": "utility",
          "address": "10.0.0.0",
          "family": "IPv4"
        },
        {
          "access": "public",
          "address": "x.x.x.x",
          "family": "IPv4"
        },
        {
          "access": "public",
          "address": "xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx",
          "family": "IPv6"
        }
      ]
    },
    "license": 0,
    "memory_amount": "4096",
    "nic_model": "virtio",
    "plan": "2xCPU-4GB",
    "plan_ipv4_bytes": "345346754423",
    "plan_ipv6_bytes": "676300234",
    "state": "started",
    "storage_devices": {
      "storage_device": [
        {
          "address": "virtio:0",
          "part_of_plan": "yes",
          "storage": "0169b4f8-051c-4a86-9484-f5b798249949",
          "storage_size": 50,
          "storage_title": "Debian from a template",
          "type": "disk"
        }
      ]
    },
    "tags": {
      "tag": [
        "DEV",
        "private",
        "RHEL"
      ]
    },
    "timezone": "UTC",
    "title": "My Debian server",
    "uuid": "00c78863-db86-44ea-af70-d6edc4d162bf",
    "video_model": "cirrus",
    "vnc": "off",
    "vnc_password": "aabbccdd",
    "zone": "fi-hel1"
  }
}
```

### Error responses

| HTTP status | Error code     | Description                                  |
|-------------|----------------|----------------------------------------------|
| 400         | TAG_INVALID    | The tag name is invalid                      |
| 403         | TAG_FORBIDDEN  | Username does not have access to this tag    |
| 404         | TAG_NOT_FOUND  | Tag does not exist                           |

---

## Remove tag from server

Untags tags from a given server. The tag(s) must exist.

### Request

```
POST /1.3/server/{uuid}/untag/{tags} HTTP/1.1
```

The `tags` parameter lists the tags to remove from the server as a
comma-separated list, e.g. `dev,test`.

### Normal response

```
HTTP/1.1 200 OK
```

```json
{
  "server": {
    "boot_order": "disk",
    "core_number": "2",
    "firewall": "on",
    "hostname": "debian.example.com",
    "ip_addresses": {
      "ip_address": [
        {
          "access": "utility",
          "address": "10.0.0.0",
          "family": "IPv4"
        },
        {
          "access": "public",
          "address": "x.x.x.x",
          "family": "IPv4"
        },
        {
          "access": "public",
          "address": "xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx",
          "family": "IPv6"
        }
      ]
    },
    "license": 0,
    "memory_amount": "4096",
    "nic_model": "virtio",
    "plan": "2xCPU-4GB",
    "plan_ipv4_bytes": "345346754423",
    "plan_ipv6_bytes": "676300234",
    "state": "started",
    "storage_devices": {
      "storage_device": [
        {
          "address": "virtio:0",
          "part_of_plan": "yes",
          "storage": "0169b4f8-051c-4a86-9484-f5b798249949",
          "storage_size": 50,
          "storage_title": "Debian from a template",
          "type": "disk"
        }
      ]
    },
    "tags": {
      "tag": [
        "DEV",
        "RHEL"
      ]
    },
    "timezone": "UTC",
    "title": "My Debian server",
    "uuid": "00c78863-db86-44ea-af70-d6edc4d162bf",
    "video_model": "cirrus",
    "vnc": "off",
    "vnc_password": "aabbccdd",
    "zone": "fi-hel1"
  }
}
```

### Error responses

| HTTP status | Error code     | Description                                  |
|-------------|----------------|----------------------------------------------|
| 400         | TAG_INVALID    | The tag name is invalid                      |
| 403         | TAG_FORBIDDEN  | Username does not have access to this tag    |
| 404         | TAG_NOT_FOUND  | Tag does not exist                           |
