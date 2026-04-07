# Audit Log API Reference

The UpCloud Audit Logs provide a security-relevant, chronological record of events for an account
and all its resources. Data is retained for up to **180 days** per account.

Included event categories:

- Account-related events
- Successful Hub login events
- Server events
- Storage events
- Managed Database events
- Managed Load Balancer events
- Managed Object Storage events
- UpCloud Kubernetes Service events

---

## List audit logs

Returns a paginated list of audit log entries matching the given query parameters.

The response includes an `Upcloud-Total-Count` header indicating the total number of matching
audit logs (excluding `limit` and `offset`). Use `limit` and `offset` for pagination.

### Request

```
GET /1.3/audit-logs HTTP/1.1
```

### Query parameters

| Attribute | Accepted values | Default | Required | Description |
|---|---|---|---|---|
| `limit` | `0`–`200` | `50` | no | Number of entries to receive at most. |
| `offset` | `>= 0` | `0` | no | Offset for retrieved results. |
| `actions` | `create / read / update / delete / access-update / attach / detach / login / logout` | — | no | List of actions to filter by. |
| `origins` | `api / gui / upcloud_internal` | — | no | List of origins to filter by. |
| `resource_types` | `account / auth / init-script / managed-database / managed-loadbalancer / managed-loadbalancer-certificate-bundle / managed-object-storage / server / ssh-key / storage / uks` | — | no | List of resource types to filter by. |
| `created_after` | ISO 8601 timestamp, e.g. `2024-01-01T23:45:56Z` | 180 days ago | no | List audit logs created at or after the given timestamp. |
| `created_before` | ISO 8601 timestamp, e.g. `2024-04-05T15:04:05Z` | Current time | no | List audit logs created at or before the given timestamp. |
| `q` | Valid UUID, username, or IP address | — | no | Filter audit logs by keyword such as account username, resource ID, or IP address. |

> **Note:** The `resource_types`, `actions`, and `origins` values can be discovered via the
> `/filter-options` endpoint below.

### Response attributes

| Attribute | Description |
|---|---|
| `audit_logs` | The list of audit log objects. |

#### Audit log object attributes

| Attribute | Value | Description |
|---|---|---|
| `time` | string (ISO 8601) | Time at which the audit log was recorded. |
| `auth_type` | `basic_auth / gui / daemon / null` | Authentication method used for the event. `null` for internal events. |
| `upcloud_correlation_id` | string | Unique ID given to the event. May appear in multiple related audit log entries. |
| `resource_type` | string (see `/filter-options`) | Type of the affected resource, e.g. `"account"`, `"server"`. |
| `action` | string / null (see `/filter-options`) | Action performed on the resource. |
| `origin` | `api / gui / upcloud_internal` | Interface where the event was triggered. |
| `account_username` | string / null | Username of the account or subaccount performing the event. |
| `ip_address` | string / null | Source IP address of the event, if determinable. |
| `resource_id` | string / null | UUID of the resource the audit log relates to. |

### Example response

```
HTTP/1.1 200 OK
```

```json
{
  "audit_logs": [
    {
      "time": "2024-04-02T11:52:25.719619Z",
      "auth_type": "gui",
      "upcloud_correlation_id": "01HTFAEGT48JP7M5YFHC7W21MG",
      "resource_type": "auth",
      "action": "login",
      "origin": "gui",
      "account_username": "mrmagoo",
      "ip_address": "1.2.3.4",
      "resource_id": null
    },
    {
      "time": "2024-03-13T17:50:26.647075Z",
      "auth_type": "daemon",
      "upcloud_correlation_id": "",
      "resource_type": "storage",
      "action": "delete",
      "origin": "upcloud_internal",
      "account_username": "mrmagoo",
      "ip_address": "",
      "resource_id": "00000000-0000-0000-0000-000000000000"
    }
  ]
}
```

---

## List filter options

Returns the available field values that can be used to filter audit logs.

### Request

```
GET /1.3/audit-logs/filter-options HTTP/1.1
```

### Example response

```
HTTP/1.1 200 OK
```

```json
{
  "resource_types": [
    "account",
    "auth",
    "init-script",
    "managed-database",
    "managed-loadbalancer",
    "managed-loadbalancer-certificate-bundle",
    "managed-object-storage",
    "server",
    "ssh-key",
    "storage",
    "uks"
  ],
  "actions": [
    "create",
    "read",
    "update",
    "delete",
    "access-update",
    "attach",
    "detach",
    "login",
    "logout"
  ],
  "origins": ["api", "gui", "upcloud_internal"]
}
```

---

## Export audit logs

Exports audit logs as a CSV or JSON file. The export covers up to 180 days. The response is
streamed synchronously to the client.

### Request

```
GET /1.3/audit-logs/export HTTP/1.1
```

### Query parameters

| Attribute | Accepted values | Default | Required | Description |
|---|---|---|---|---|
| `format` | `csv` / `json` | `csv` | no | Export format. |

### Example response

```
HTTP/1.1 200 OK
```

The response body contains the exported data in the requested format.

### Example commands

**curl (macOS / Linux):**

```bash
curl -u <username>:<password> \
  "https://api.upcloud.com/1.3/audit-logs/export?format=csv" \
  > audit-logs-export.csv
```

**wget:**

```bash
wget --auth-no-challenge \
  --user=<username> --password=<password> \
  "https://api.upcloud.com/1.3/audit-logs/export?format=csv" \
  -O audit-logs-export.csv
```

**PowerShell (v6.0.0+):**

```powershell
Invoke-WebRequest `
  -Uri "https://api.upcloud.com/1.3/audit-logs/export?format=csv" `
  -OutFile audit-logs-export.csv `
  -Authentication Basic `
  -Credential (Get-Credential)
```
