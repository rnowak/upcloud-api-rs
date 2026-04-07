# Hosts

> Source: https://developers.upcloud.com/1.3/14-hosts

A host identifies the host server that virtual machines are run on.

Only hosts on private cloud to which the calling account has access to are available through this API.

## List available hosts

Returns a list of available hosts, along with basic statistics of them when available. The optional query string parameter `zone` can be set to limit the list to hosts on the specified private cloud.

CPU idle values in the host list statistics are given as an average percentage over the past 15 minutes, and free memory is the current value in gigabytes.

### Request

`GET /1.3/host`

### Response

```json
{
  "hosts": {
    "host": [
      {
        "id": 7653311107,
        "description": "My Host #1",
        "zone": "private-zone-id",
        "windows_enabled": "no",
        "stats": {
          "stat": [
            {
              "name": "cpu_idle",
              "timestamp": "2019-08-09T12:46:57Z",
              "value": 95.2
            },
            {
              "name": "memory_free",
              "timestamp": "2019-08-09T12:46:57Z",
              "value": 102
            }
          ]
        }
      },
      {
        "id": 8055964291,
        "description": "My Host #2",
        "zone": "private-zone-id",
        "windows_enabled": "no",
        "stats": {
          "stat": [
            {
              "name": "cpu_idle",
              "timestamp": "2019-08-09T12:46:57Z",
              "value": 80.1
            },
            {
              "name": "memory_free",
              "timestamp": "2019-08-09T12:46:57Z",
              "value": 61
            }
          ]
        }
      }
    ]
  }
}
```

### Response attributes — Host

| Attribute | Type | Description |
|-----------|------|-------------|
| `id` | Integer | Unique host identifier. |
| `description` | String | Freeform description of the host. |
| `zone` | String | The private cloud zone the host belongs to. |
| `windows_enabled` | String (`yes` / `no`) | Whether the host supports running Windows servers. |
| `stats` | Object | Host performance statistics (see below). |

### Response attributes — Host stat

| Attribute | Type | Description |
|-----------|------|-------------|
| `name` | String | Name of the statistic. Known values: `cpu_idle` (average CPU idle percentage over 15 min), `memory_free` (free memory in GiB). |
| `timestamp` | String (ISO 8601) | Timestamp of the measurement. |
| `value` | Number | Measured value. |

### Error responses

| HTTP status | Error code | Description |
|-------------|------------|-------------|
| 403 | `ZONE_HOST_FORBIDDEN` | Private cloud access denied. |

## Get host details

Returns detailed information about a specific host.

CPU idle values in the host detail statistics are given as percentage on the specified timestamp, and free memory is the value in gigabytes on the specified timestamp.

### Request

`GET /1.3/host/{host_id}`

### Response

```json
{
  "host": {
    "id": 7653311107,
    "description": "My Host #1",
    "zone": "private-zone-id",
    "windows_enabled": "no",
    "stats": {
      "stat": [
        {
          "name": "cpu_idle",
          "timestamp": "2019-08-09T12:46:57Z",
          "value": 95.2
        },
        {
          "name": "memory_free",
          "timestamp": "2019-08-09T12:46:57Z",
          "value": 102
        }
      ]
    }
  }
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|------------|-------------|
| 403 | `HOST_FORBIDDEN` | Access denied to specified host information. |
| 404 | `HOST_NOT_FOUND` | The specified host does not exist. |

## Modify host details

Modifies the description of a specific host.

### Request

`PATCH /1.3/host/{host_id}`

```json
{
  "host": {
    "description": "My New Host"
  }
}
```

### Request attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `description` | String (optional) | New freeform description for the host. |

### Response

```json
{
  "host": {
    "id": 7653311107,
    "description": "My New Host",
    "zone": "private-zone-id",
    "windows_enabled": "no",
    "stats": {
      "stat": [
        {
          "name": "cpu_idle",
          "timestamp": "2019-08-09T13:37:12Z",
          "value": 93.3
        },
        {
          "name": "memory_free",
          "timestamp": "2019-08-09T13:37:12Z",
          "value": 104
        }
      ]
    }
  }
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|------------|-------------|
| 403 | `HOST_FORBIDDEN` | Access denied to modify information of specified host. |
| 404 | `HOST_NOT_FOUND` | The specified host does not exist. |
