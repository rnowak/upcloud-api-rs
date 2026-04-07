# Storages

> Source: https://developers.upcloud.com/1.3/9-storages

A storage device is a block device similar to a physical hard disk. It can be used to install an
operating system or hold some other filesystem for a server.

Storages can be freely attached to any server within the same zone and under the same account.
A maximum of 16 storage devices can be attached to a server at the same time; however, the combined
limit for network interfaces and storage devices is 24.

## Storage types

There are four different storage types: `disk`, `cdrom`, `template` and `backup`.

| Type | Description |
|------|-------------|
| **Normal** (`disk`) | User-writable storage for OS and application data. |
| **CD-ROM** (`cdrom`) | Read-only media, typically for installations or crash recovery. |
| **Template** (`template`) | Special storage used to create servers with a preconfigured OS. |
| **Backup** (`backup`) | Point-in-time backup of a normal storage. |

## Storage tiers

| Storage tier | API name | Description |
|-------------|----------|-------------|
| MaxIOPS | `maxiops` | High performance storage |
| Standard | `standard` | General purpose storage |
| Archive | `hdd`\* | High capacity storage |

\* For backwards compatibility, Archive tier is called `hdd` on the API.

## Storage access types

| Access | Description |
|--------|-------------|
| `public` | Visible to all users. Includes CD-ROM images and templates. |
| `private` | Visible only to the specific user account and subaccounts. |

## Storage states

| State | Description |
|-------|-------------|
| `online` | The storage resource is ready for use. Can be attached or detached. |
| `maintenance` | Maintenance work is currently performed on the storage. |
| `cloning` | The storage is currently the clone source for another storage. |
| `backuping` | The storage is currently being backed up. |
| `syncing` | The storage is currently synchronizing its data. |
| `error` | The storage has encountered an error and is currently inaccessible. |

---

## List storages

Returns a list of all accessible storages. The list can be narrowed down by access type, storage
type, or favorite status. Storages can also be filtered by label URL parameters.

### Requests

```
GET /1.3/storage
GET /1.3/storage/public
GET /1.3/storage/private
GET /1.3/storage/normal
GET /1.3/storage/backup
GET /1.3/storage/cdrom
GET /1.3/storage/template
GET /1.3/storage/favorite
```

Label filtering: `?label=env` or `?label=env%3Dprod`. Multiple `?label=` parameters are
AND-combined. Label keys are matched case-insensitively.

### Response `200 OK`

```json
{
  "storages": {
    "storage": [
      {
        "access": "private",
        "encrypted": "no",
        "labels": [],
        "license": 0,
        "size": 10,
        "state": "online",
        "tier": "hdd",
        "title": "Operating system disk",
        "type": "normal",
        "uuid": "01eff7ad-168e-413e-83b0-054f6a28fa23",
        "zone": "uk-lon1"
      },
      {
        "access": "private",
        "encrypted": "no",
        "labels": [],
        "created": "2019-09-17T14:34:43Z",
        "license": 0,
        "origin": "01eff7ad-168e-413e-83b0-054f6a28fa23",
        "size": 10,
        "state": "online",
        "title": "On demand backup",
        "type": "backup",
        "uuid": "01287ad1-496c-4b5f-bb67-0fc2e3494740",
        "zone": "uk-lon1"
      },
      {
        "access": "private",
        "encrypted": "no",
        "labels": [],
        "license": 0,
        "part_of_plan": "yes",
        "size": 50,
        "state": "online",
        "tier": "maxiops",
        "title": "Databases",
        "type": "normal",
        "uuid": "01f3286c-a5ea-4670-8121-d0b9767d625b",
        "zone": "fi-hel1"
      },
      {
        "access": "public",
        "encrypted": "no",
        "labels": [],
        "license": 0,
        "size": 5,
        "state": "online",
        "template_type": "cloud-init",
        "title": "Ubuntu Server 22.04 LTS (Jammy Jellyfish)",
        "type": "template",
        "uuid": "01000000-0000-4000-8000-000030220200"
      }
    ]
  }
}
```

### Storage list attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `access` | string | `public` or `private`. |
| `encrypted` | yes/no | Whether the storage is encrypted at rest. |
| `labels` | array | Labels assigned to the storage. |
| `license` | number | Per-hour license cost. `0` for no license. |
| `size` | number | Storage size in gigabytes. |
| `state` | string | Current storage state. |
| `tier` | string | Storage tier (`hdd`, `standard`, `maxiops`). |
| `title` | string | Short description. |
| `type` | string | Storage type (`normal`, `backup`, `cdrom`, `template`). |
| `uuid` | string | Unique identifier. |
| `zone` | string | Data center zone. |
| `origin` | string | UUID of the origin storage (backups/clones). |
| `part_of_plan` | yes/no | Whether the storage is included in a server plan. |
| `created` | string | Creation timestamp (ISO 8601). Present on backups. |
| `template_type` | string | Template type (e.g. `cloud-init`). Present on templates. |

---

## Get storage details

Returns detailed information about a specific storage resource.

### Request

```
GET /1.3/storage/{uuid}
```

### Response `200 OK`

```json
{
  "storage": {
    "access": "private",
    "encrypted": "yes",
    "backup_rule": "",
    "backups": {
      "backup": []
    },
    "labels": [
      {
        "key": "env",
        "value": "prod"
      }
    ],
    "license": 0,
    "servers": {
      "server": [
        "00798b85-efdc-41ca-8021-f6ef457b8531"
      ]
    },
    "size": 10,
    "state": "online",
    "tier": "maxiops",
    "title": "Operating system disk",
    "type": "normal",
    "uuid": "01d4fcd4-e446-433b-8a9c-551a1284952e",
    "zone": "fi-hel1"
  }
}
```

### Additional detail attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `backup_rule` | object or `""` | Automatic backup schedule. Empty string if not set. |
| `backups` | object | Contains `backup` array of backup UUIDs. |
| `servers` | object | Contains `server` array of server UUIDs this storage is attached to. |

---

## Create storage

Creates a new storage resource in a specific zone.

### Request

```
POST /1.3/storage
```

```json
{
  "storage": {
    "encrypted": "yes",
    "size": "10",
    "tier": "maxiops",
    "title": "My data collection",
    "zone": "fi-hel1",
    "labels": [
      {
        "key": "env",
        "value": "prod"
      }
    ],
    "backup_rule": {
      "interval": "daily",
      "time": "0430",
      "retention": "365"
    }
  }
}
```

### Attributes

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| `size` | 1–4096 | | yes | Size in gigabytes. |
| `tier` | `hdd` / `standard` / `maxiops` | `hdd` | no | Storage tier. |
| `title` | 1–255 characters | | yes | Short description. |
| `zone` | Valid zone identifier | | yes | Data center zone (e.g. `fi-hel1`). |
| `backup_rule` | A `backup_rule` block | | no | Automatic backup schedule. |
| `labels` | Array of label objects | | no | Labels with `key` and `value`. |
| `encrypted` | `yes` / `no` | `no` | no | Encrypt storage at rest. |

### `backup_rule` attributes

| Attribute | Accepted values | Required | Description |
|-----------|----------------|----------|-------------|
| `interval` | `daily` / `mon` / `tue` / `wed` / `thu` / `fri` / `sat` / `sun` | yes | Weekday for backup. `daily` = every day. |
| `time` | `0000`–`2359` | yes | Time of day (UTC) in HHMM format. |
| `retention` | 1–1095 | yes | Days before auto-deletion (max 3 years). |

If any one of `interval`, `time`, or `retention` is specified, all must be specified.

### `labels` attributes

| Attribute | Accepted values | Required | Description |
|-----------|----------------|----------|-------------|
| `key` | 2–32 printable ASCII chars (0x20–0x7E), must not start with `_` | yes | Label key. |
| `value` | 0–255 characters | yes | Label value. |

### Response `201 Created`

```json
{
  "storage": {
    "access": "private",
    "encrypted": "no",
    "backup_rule": {
      "interval": "daily",
      "hour": "0430",
      "retention": "365"
    },
    "backups": {
      "backup": []
    },
    "labels": [
      {
        "key": "env",
        "value": "prod"
      }
    ],
    "license": 0,
    "servers": {
      "server": []
    },
    "size": 10,
    "state": "online",
    "tier": "maxiops",
    "title": "test",
    "type": "normal",
    "uuid": "01d4fcd4-e446-433b-8a9c-551a1284952e",
    "zone": "fi-hel1"
  }
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 | `BACKUP_RULE_INVALID` | Invalid backup rule. |
| 400 | `INTERVAL_INVALID` | Invalid interval value. |
| 400 | `INTERVAL_MISSING` | Missing required interval. |
| 400 | `HOUR_INVALID` | Invalid hour value. |
| 400 | `HOUR_MISSING` | Missing required hour. |
| 400 | `RETENTION_INVALID` | Invalid retention value. |
| 400 | `RETENTION_MISSING` | Missing required retention. |
| 400 | `SIZE_INVALID` | Invalid size value. |
| 400 | `SIZE_MISSING` | Missing required size. |
| 400 | `TIER_INVALID` | Invalid tier value. |
| 400 | `TITLE_INVALID` | Invalid title value. |
| 400 | `TITLE_MISSING` | Missing required title. |
| 400 | `ZONE_INVALID` | Invalid zone value. |
| 400 | `ZONE_MISSING` | Missing required zone. |
| 402 | `INSUFFICIENT_CREDITS` | Not enough credits. |
| 403 | `*_LIMIT_REACHED` | Various account quota limits. |
| 404 | `ZONE_NOT_FOUND` | Zone does not exist. |
| 409 | `LABEL_LIMIT_REACHED` | Max labels per resource reached. |
| 409 | `STORAGE_RESOURCES_UNAVAILABLE` | Insufficient storage resources in zone/tier. |

---

## Modify storage

Modifies an existing storage resource (rename, resize, update backup rule or labels).

New size must be **greater** than current size. Storage state must be `online`. For resize, the
storage must be detached or the server must be `stopped`. Partition table and filesystem are
**not** resized automatically.

### Request

```
PUT /1.3/storage/{uuid}
```

```json
{
  "storage": {
    "size": "20",
    "title": "A larger storage"
  }
}
```

### Attributes

| Attribute | Accepted values | Required | Description |
|-----------|----------------|----------|-------------|
| `size` | 1–4096 | no | New size in gigabytes (can only increase). |
| `title` | 1–255 characters | no | Short description. |
| `backup_rule` | A `backup_rule` block | no | Automatic backup schedule. |
| `labels` | Array of label objects | no | Storage labels. |

### Response `202 Accepted`

```json
{
  "storage": {
    "access": "private",
    "encrypted": "no",
    "backup_rule": "",
    "backups": {
      "backup": []
    },
    "license": 0,
    "servers": {
      "server": []
    },
    "size": 20,
    "state": "online",
    "tier": "maxiops",
    "title": "A larger storage",
    "type": "normal",
    "uuid": "011d671f-e803-484d-920a-c25b4bb05c01",
    "zone": "uk-lon1"
  }
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 | `BACKUP_RULE_INVALID` | Invalid backup rule. |
| 400 | `SIZE_INVALID` | Invalid size value. |
| 400 | `STORAGE_INVALID` | Invalid storage value. |
| 400 | `TITLE_INVALID` | Invalid title value. |
| 403 | `STORAGE_FORBIDDEN` | Storage owned by another account. |
| 403 | `*_LIMIT_REACHED` | Various account quota limits. |
| 404 | `STORAGE_NOT_FOUND` | Storage does not exist. |
| 409 | `LABEL_LIMIT_REACHED` | Max labels per resource reached. |
| 409 | `STORAGE_ATTACHED` | Storage attached to a server; detach or stop server first for resize. |
| 409 | `STORAGE_RESOURCES_UNAVAILABLE` | Insufficient resources for resize. |
| 409 | `STORAGE_STATE_ILLEGAL` | Storage in unusable state. |
| 409 | `STORAGE_TYPE_ILLEGAL` | Only `normal` type can be resized. |
| 409 | `BACKUP_RULE_CONFLICT` | Conflicts with server's simple backup rule. |

---

## Resize storage partition & filesystem

Resizes the last partition and the ext3/ext4/XFS/NTFS filesystem if the partition does not
extend to the end of the storage. A backup is taken before the resize attempt.

### Request

```
POST /1.3/storage/{uuid}/resize
```

### Response `200 OK`

```json
{
  "resize_backup": {
    "access": "private",
    "created": "2021-12-03T06:25:15Z",
    "license": 0,
    "origin": "017ca4cc-def2-458d-a797-7782959b30a7",
    "servers": {
      "server": []
    },
    "size": 30,
    "state": "online",
    "title": "Resize Backup",
    "type": "backup",
    "uuid": "01beec3a-14ac-4f71-9c63-3338341121c3",
    "zone": "uk-lon1"
  }
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 | `STORAGE_INVALID` | Invalid storage value. |
| 402 | `INSUFFICIENT_CREDITS` | Not enough credits for backup. |
| 403 | `STORAGE_FORBIDDEN` | Storage owned by another account. |
| 404 | `STORAGE_NOT_FOUND` | Storage does not exist. |
| 409 | `RESIZE_FAILED` | Resize did not succeed. |
| 409 | `STORAGE_RESOURCES_UNAVAILABLE` | Not enough resources for backup. |
| 409 | `SERVER_STATE_ILLEGAL` | Attached server not in `stopped` state. |
| 409 | `STORAGE_STATE_ILLEGAL` | Storage not in `online` state. |
| 409 | `STORAGE_TYPE_ILLEGAL` | Only `normal` type can be resized. |

---

## Clone storage

Creates an exact copy of an existing storage resource. Can clone across zones.

This operation is asynchronous. The cloned storage state is `maintenance` until complete.
Cloning can be cancelled with [Cancel storage operation](#cancel-storage-operation).

### Request

```
POST /1.3/storage/{uuid}/clone
```

```json
{
  "storage": {
    "zone": "fi-hel1",
    "tier": "maxiops",
    "title": "Clone of operating system disk",
    "encrypted": "yes"
  }
}
```

### Attributes

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| `zone` | Valid zone identifier | | yes | Target zone. |
| `tier` | `hdd` / `standard` / `maxiops` | `hdd` | no | Storage tier. |
| `title` | 1–255 characters | | yes | Short description. |
| `encrypted` | `yes` / `no` | `no` | no | Encrypt at rest. |

### Response `201 Created`

```json
{
  "storage": {
    "access": "private",
    "encrypted": "yes",
    "license": 0,
    "servers": {
      "server": []
    },
    "size": "10",
    "state": "maintenance",
    "tier": "maxiops",
    "title": "Clone of operating system disk",
    "type": "normal",
    "uuid": "01f3286c-a5ea-4670-8121-d0b9767d625b",
    "zone": "fi-hel1"
  }
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 | `STORAGE_INVALID` | Invalid storage UUID. |
| 400 | `TIER_INVALID` | Invalid tier value. |
| 400 | `TITLE_INVALID` / `TITLE_MISSING` | Invalid or missing title. |
| 400 | `ZONE_INVALID` / `ZONE_MISSING` | Invalid or missing zone. |
| 402 | `INSUFFICIENT_CREDITS` | Not enough credits. |
| 403 | `STORAGE_FORBIDDEN` | Storage owned by another account. |
| 403 | `*_LIMIT_REACHED` | Various account quota limits. |
| 404 | `STORAGE_NOT_FOUND` / `ZONE_NOT_FOUND` | Resource not found. |
| 409 | `STORAGE_RESOURCES_UNAVAILABLE` | Insufficient resources in target zone. |
| 409 | `STORAGE_STATE_ILLEGAL` | Storage in unusable state. |
| 409 | `STORAGE_TYPE_ILLEGAL` | Illegal storage type for cloning. |
| 409 | `LABEL_LIMIT_REACHED` | Max labels per resource reached. |

---

## Cancel storage operation

Cancels a running cloning operation and deletes the incomplete copy. The source storage returns
to `online` state after a few seconds.

### Request

```
POST /1.3/storage/{uuid}/cancel
```

### Response `204 No Content`

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 | `STORAGE_INVALID` | Invalid storage UUID. |
| 403 | `STORAGE_FORBIDDEN` | Storage owned by another account. |
| 409 | `STORAGE_STATE_ILLEGAL` | Storage in unusable state. |
| 409 | `UNABLE_TO_CANCEL` | Cancellation was not successful. |

---

## Templatize storage

Creates an exact copy of an existing storage as a template for creating new servers.
Templates can be created from any MaxIOPS, Standard, or Archive storage. Deployments from
custom templates are restricted to the same storage tier.

This operation is asynchronous. The template state is `maintenance` until complete.

### Request

```
POST /1.3/storage/{uuid}/templatize
```

```json
{
  "storage": {
    "title": "My server template"
  }
}
```

### Attributes

| Attribute | Accepted values | Required | Description |
|-----------|----------------|----------|-------------|
| `title` | 1–255 characters | yes | Short description. |

### Response `201 Created`

```json
{
  "storage": {
    "access": "private",
    "license": 0,
    "servers": {
      "server": []
    },
    "size": 10,
    "state": "maintenance",
    "tier": "maxiops",
    "title": "My server template",
    "type": "template",
    "uuid": "012cfe08-ed18-4dce-93c0-c7e39e6276c8",
    "zone": "fi-hel1"
  }
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 | `STORAGE_INVALID` | Invalid storage UUID. |
| 400 | `TITLE_INVALID` / `TITLE_MISSING` | Invalid or missing title. |
| 402 | `INSUFFICIENT_CREDITS` | Not enough credits. |
| 403 | `STORAGE_FORBIDDEN` | Storage owned by another account. |
| 404 | `STORAGE_NOT_FOUND` | Storage does not exist. |
| 409 | `STORAGE_RESOURCES_UNAVAILABLE` | Insufficient resources. |
| 409 | `STORAGE_STATE_ILLEGAL` | Storage in unusable state or attached to running server. |
| 409 | `STORAGE_TIER_ILLEGAL` | Only `maxiops` storages can be templatized. |

---

## Create backup

Creates a point-in-time backup of a storage resource. For automatic backups, use `backup_rule`
in [Create storage](#create-storage) or [Modify storage](#modify-storage).

This operation is asynchronous. Storage state changes to `backuping` and returns to `online`
when complete.

### Request

```
POST /1.3/storage/{uuid}/backup
```

```json
{
  "storage": {
    "title": "Manually created backup"
  }
}
```

### Attributes

| Attribute | Accepted values | Required | Description |
|-----------|----------------|----------|-------------|
| `title` | 1–255 characters | yes | Short description. |

### Response `201 Created`

```json
{
  "storage": {
    "access": "private",
    "created": "2015-03-27T10:02:05Z",
    "license": 0,
    "origin": "01eff7ad-168e-413e-83b0-054f6a28fa23",
    "progress": "0",
    "servers": {
      "server": []
    },
    "size": "10",
    "state": "maintenance",
    "title": "Manually created backup",
    "type": "normal",
    "uuid": "01f3286c-a5ea-4670-8121-d0b9767d625b",
    "zone": "fi-hel1"
  }
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 | `STORAGE_INVALID` | Invalid storage UUID. |
| 400 | `TITLE_INVALID` / `TITLE_MISSING` | Invalid or missing title. |
| 402 | `INSUFFICIENT_CREDITS` | Not enough credits. |
| 403 | `STORAGE_FORBIDDEN` | Storage owned by another account. |
| 404 | `STORAGE_NOT_FOUND` | Storage does not exist. |
| 409 | `STORAGE_RESOURCES_UNAVAILABLE` | Insufficient resources. |
| 409 | `STORAGE_STATE_ILLEGAL` | Storage in unusable state. |
| 409 | `STORAGE_TYPE_ILLEGAL` | Illegal storage type for backup. |

---

## Restore backup

Restores the origin storage with data from the specified backup storage. If the origin
storage is attached to a server, the server must first be `stopped`.

### Request

```
POST /1.3/storage/{uuid}/restore
```

### Response `204 No Content`

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 | `STORAGE_INVALID` | Invalid storage UUID. |
| 403 | `STORAGE_FORBIDDEN` | Storage owned by another account. |
| 404 | `STORAGE_NOT_FOUND` | Storage does not exist. |
| 409 | `SERVER_STATE_ILLEGAL` | Origin attached to a started server; must stop first. |
| 409 | `STORAGE_RESOURCES_UNAVAILABLE` | Insufficient resources. |
| 409 | `STORAGE_STATE_ILLEGAL` | Storage in unusable state. |
| 409 | `STORAGE_TYPE_ILLEGAL` | Illegal storage type. |

---

## Attach storage

Attaches a storage as a device to a server.

- **Normal storages (disk)**: IDE devices require server `stopped`. SCSI/virtio can be
  attached while server is `started`.
- **CD-ROM devices**: Only one CD-ROM per server. Server must be `stopped` to attach.

### Request (via Server API)

```
POST /1.3/server/{server_uuid}/storage/attach
```

```json
{
  "storage_device": {
    "type": "disk",
    "address": "scsi",
    "storage": "00798b85-efdc-41ca-8021-f6ef457b8531",
    "boot_disk": "0"
  }
}
```

### Request (via Storage API)

```
POST /1.3/storage/{storage_uuid}/attach
```

```json
{
  "storage_device": {
    "type": "disk",
    "address": "scsi",
    "server": "009d64ef-31d1-4684-a26b-c86c955cbf46",
    "boot_disk": "0"
  }
}
```

### Attributes (Server API)

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| `type` | `disk` / `cdrom` | `disk` | no | Type of attached storage. |
| `address` | `ide[:[01]:[01]]` / `scsi[:0:[0-15]]` / `virtio[:[0-15]]` | Next available virtio | no | Device address. Bus name only auto-selects. |
| `storage` | Valid storage UUID | | yes (if disk) | UUID of storage to attach. |
| `boot_disk` | `0` / `1` | `0` | no | Use as boot disk. |

### Attributes (Storage API)

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| `type` | `disk` / `cdrom` | `disk` | no | Type of attached storage. |
| `address` | `ide[:[01]:[01]]` / `scsi[:0:[0-15]]` / `virtio[:[0-15]]` | Next available | no | Device address. |
| `server` | Valid server UUID | | yes | UUID of server to attach to. |
| `boot_disk` | `0` / `1` | `0` | no | Use as boot disk. |

### Response `200 OK` (Server API)

Returns the full server object including `storage_devices`.

### Response `200 OK` (Storage API)

Returns the full storage details object including `servers`.

```json
{
  "storage": {
    "access": "private",
    "encrypted": "yes",
    "backup_rule": "",
    "backups": {
      "backup": []
    },
    "license": 0,
    "servers": {
      "server": [
        "009d64ef-31d1-4684-a26b-c86c955cbf46"
      ]
    },
    "size": 10,
    "state": "online",
    "tier": "maxiops",
    "title": "Debian from a template",
    "type": "normal",
    "uuid": "00798b85-efdc-41ca-8021-f6ef457b8531",
    "zone": "fi-hel1"
  }
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 | `ADDRESS_INVALID` | Invalid address value. |
| 400 | `SERVER_INVALID` | Invalid server UUID. |
| 400 | `STORAGE_INVALID` / `STORAGE_MISSING` | Invalid or missing storage UUID. |
| 400 | `TYPE_INVALID` | Invalid type value. |
| 403 | `SERVER_FORBIDDEN` / `STORAGE_FORBIDDEN` | Resource owned by another account. |
| 404 | `SERVER_NOT_FOUND` / `STORAGE_NOT_FOUND` | Resource not found. |
| 409 | `CDROM_DEVICE_IN_USE` | CD-ROM device already attached. |
| 409 | `CDROM_HOTPLUG_UNSUPPORTED` | Cannot hotplug CD-ROM. |
| 409 | `DEVICE_ADDRESS_IN_USE` | Address already occupied. |
| 409 | `IDE_HOTPLUG_UNSUPPORTED` | Cannot hotplug IDE. |
| 409 | `PUBLIC_STORAGE_ATTACH` | Cannot attach public storage. |
| 409 | `SERVER_STATE_ILLEGAL` | Server in unusable state. |
| 409 | `STORAGE_ATTACHED` | Already attached to specified server. |
| 409 | `STORAGE_ATTACHED_AS_CDROM` / `STORAGE_ATTACHED_AS_DISK` | Already attached as different type. |
| 409 | `STORAGE_DEVICE_LIMIT_REACHED` | Max 16 devices per server. |
| 409 | `STORAGE_STATE_ILLEGAL` / `STORAGE_TYPE_ILLEGAL` | Illegal state or type. |
| 409 | `ZONE_MISMATCH` | Storage and server in different zones. |
| 409 | `TOO_MANY_BOOT_DISKS` | Only one boot disk allowed. |
| 409 | `BACKUP_RULE_CONFLICT` | Conflicts with server's simple backup rule. |

---

## Detach storage

Detaches a storage resource from a server. IDE devices cannot be detached while server is `started`.

### Request (via Server API)

```
POST /1.3/server/{server_uuid}/storage/detach
```

```json
{
  "storage_device": {
    "address": "scsi:0:0"
  }
}
```

### Request (via Storage API)

```
POST /1.3/storage/{storage_uuid}/detach
```

### Attributes

| Attribute | Accepted values | Required | Description |
|-----------|----------------|----------|-------------|
| `address` | `ide:[01]:[01]` / `scsi:0:[0-15]` / `virtio:[0-15]` | yes (if no `storage`) | Device address to detach. |
| `storage` | UUID | yes (if no `address`) | Storage UUID to detach. |

### Response `200 OK` (Server API)

Returns the full server object.

### Response `200 OK` (Storage API)

Returns the full storage details object with empty `servers`.

```json
{
  "storage": {
    "access": "private",
    "encrypted": "no",
    "backup_rule": "",
    "backups": {
      "backup": []
    },
    "license": 0,
    "servers": {
      "server": []
    },
    "size": 10,
    "state": "online",
    "tier": "maxiops",
    "title": "Debian from a template",
    "type": "normal",
    "uuid": "00798b85-efdc-41ca-8021-f6ef457b8531",
    "zone": "fi-hel1"
  }
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 | `ADDRESS_INVALID` / `ADDRESS_MISSING` | Invalid or missing address. |
| 400 | `SERVER_INVALID` | Invalid server UUID. |
| 400 | `ADDRESS_OR_UUID_REQUIRED` | Either address or UUID must be given. |
| 403 | `SERVER_FORBIDDEN` | Server owned by another account. |
| 404 | `SERVER_NOT_FOUND` | Server not found. |
| 409 | `CDROM_HOTPLUG_UNSUPPORTED` | Cannot hotplug CD-ROM. |
| 409 | `DEVICE_ADDRESS_NOT_IN_USE` | No device at specified address. |
| 409 | `STORAGE_STATE_ILLEGAL` / `SERVER_STATE_ILLEGAL` | Illegal state. |
| 409 | `IDE_HOTPLUG_UNSUPPORTED` | Cannot hotplug IDE. |
| 511 | `HOTPLUG_FAILED` | Hotplug operation failed. |

---

## Load CD-ROM

Loads a storage as a CD-ROM in the CD-ROM device of a server. Requires a CD-ROM device to be
attached. Any storage of type `normal` or `cdrom` can be loaded.

### Request

```
POST /1.3/server/{server_uuid}/cdrom/load
```

```json
{
  "storage_device": {
    "storage": "01000000-0000-4000-8000-000060010101"
  }
}
```

### Attributes

| Attribute | Accepted values | Required | Description |
|-----------|----------------|----------|-------------|
| `storage` | UUID of `normal` or `cdrom` storage | yes | Storage to load into CD-ROM. |

### Response `200 OK`

Returns the full server object with updated `storage_devices`.

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 | `STORAGE_INVALID` / `STORAGE_MISSING` | Invalid or missing storage. |
| 400 | `SERVER_INVALID` | Invalid server UUID. |
| 403 | `SERVER_FORBIDDEN` / `STORAGE_FORBIDDEN` | Resource owned by another account. |
| 404 | `SERVER_NOT_FOUND` / `STORAGE_NOT_FOUND` | Resource not found. |
| 409 | `NO_CDROM_DEVICE` | No CD-ROM device attached. |
| 409 | `SERVER_STATE_ILLEGAL` / `STORAGE_STATE_ILLEGAL` | Illegal state. |
| 409 | `STORAGE_TYPE_ILLEGAL` | Only `normal` and `cdrom` types allowed. |
| 409 | `STORAGE_ATTACHED` | Already loaded as CD-ROM on this server. |
| 409 | `STORAGE_ATTACHED_AS_DISK` | Already attached as disk elsewhere. |
| 409 | `ZONE_MISMATCH` | Storage and server in different zones. |

---

## Eject CD-ROM

Ejects the storage from the CD-ROM device of a server.

### Request

```
POST /1.3/server/{server_uuid}/cdrom/eject
```

### Response `200 OK`

Returns the full server object with CD-ROM device showing empty `storage`.

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 | `SERVER_INVALID` | Invalid server UUID. |
| 403 | `SERVER_FORBIDDEN` | Server owned by another account. |
| 404 | `SERVER_NOT_FOUND` | Server not found. |
| 409 | `NO_CDROM_DEVICE` | No CD-ROM device attached. |
| 409 | `SERVER_STATE_ILLEGAL` / `STORAGE_STATE_ILLEGAL` | Illegal state. |
| 511 | `CDROM_EJECT_FAILED` | Could not eject; may still be mounted. |

---

## Create import

Creates an import task to import data into an existing storage. If attached to a server, the
server must be `stopped`. Supported data type is raw (`.raw`, `.img`, `.iso`), optionally
compressed with GZip or XZ.

### Import modes

- **`http_import`**: Import from a publicly reachable HTTP/HTTPS URL. Supports embedded
  credentials: `https://user:pass@host/path`.
- **`direct_upload`**: Returns a `direct_upload_url` for pushing data via HTTP PUT.

### Request

```
POST /1.3/storage/{uuid}/import
```

```json
{
  "storage_import": {
    "source": "http_import",
    "source_location": "https://cdimage.debian.org/debian-cd/current/amd64/iso-cd/debian-10.4.0-amd64-netinst.iso"
  }
}
```

### Attributes

| Attribute | Accepted values | Required | Description |
|-----------|----------------|----------|-------------|
| `source` | `http_import` / `direct_upload` | yes | Import mode. |
| `source_location` | Valid URL | if `http_import` | URL to import from. |

### Response `201 Created` (HTTP Import)

```json
{
  "storage_import": {
    "client_content_length": 0,
    "client_content_type": "",
    "completed": "",
    "created": "2020-06-26T08:51:07Z",
    "error_code": "",
    "error_message": "",
    "md5sum": "",
    "read_bytes": 0,
    "sha256sum": "",
    "source": "http_import",
    "source_location": "https://cdimage.debian.org/debian-cd/current/amd64/iso-cd/debian-10.4.0-amd64-netinst.iso",
    "state": "prepared",
    "uuid": "07a6c9a3-300e-4d0e-b935-624f3dbdff3f",
    "written_bytes": 0
  }
}
```

### Response `201 Created` (Direct Upload)

```json
{
  "storage_import": {
    "client_content_length": 0,
    "client_content_type": "",
    "completed": "",
    "created": "2020-06-26T08:51:07Z",
    "direct_upload_url": "https://fi-hel1.img.upcloud.com/uploader/session/07a6c9a3-300e-4d0e-b935-624f3dbdff3f",
    "error_code": "",
    "error_message": "",
    "md5sum": "",
    "read_bytes": 0,
    "sha256sum": "",
    "source": "direct_upload",
    "state": "prepared",
    "uuid": "07a6c9a3-300e-4d0e-b935-624f3dbdff3f",
    "written_bytes": 0
  }
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 | `STORAGE_TYPE_ILLEGAL` | Only `normal` storages support import. |
| 409 | `STORAGE_STATE_ILLEGAL` | Storage in unusable state. |
| 409 | `SERVER_STATE_ILLEGAL` | Server must be stopped. |
| 503 | `IMPORT_UNAVAILABLE` | Import temporarily unavailable. |
| 503 | `STORAGE_INCONSISTENT` | Import temporarily unavailable. |

---

## Get import details

Returns detailed information of an ongoing or finished import task.

### Import states

| State | Description |
|-------|-------------|
| `prepared` | Task submitted; initial state. |
| `pending` | Waiting for data transfer to start. |
| `importing` | Data is being written. |
| `failed` | Import failed; check `error_code` and `error_message`. |
| `cancelling` | Cancellation requested by user. |
| `cancelled` | Import was cancelled. |
| `completed` | Import completed successfully. |

### Request

```
GET /1.3/storage/{uuid}/import
```

### Response `200 OK`

```json
{
  "storage_import": {
    "client_content_length": 0,
    "client_content_type": "",
    "completed": "",
    "created": "2020-06-26T08:51:07Z",
    "error_code": "",
    "error_message": "",
    "md5sum": "",
    "read_bytes": 0,
    "sha256sum": "",
    "source": "http_import",
    "source_location": "https://cdimage.debian.org/debian-cd/current/amd64/iso-cd/debian-10.4.0-amd64-netinst.iso",
    "state": "pending",
    "uuid": "07a6c9a3-300e-4d0e-b935-624f3dbdff3f",
    "written_bytes": 0
  }
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 404 | `STORAGE_IMPORT_NOT_FOUND` | No existing import task for this storage. |

---

## Cancel import

Cancels an ongoing import task. The task transitions to `cancelling`. Storage rolls back if
data was written.

### Request

```
POST /1.3/storage/{uuid}/import/cancel
```

### Response `202 Accepted`

```json
{
  "storage_import": {
    "client_content_length": 0,
    "client_content_type": "",
    "completed": "",
    "created": "2020-06-26T08:51:07Z",
    "error_code": "",
    "error_message": "",
    "md5sum": "",
    "read_bytes": 0,
    "sha256sum": "",
    "source": "http_import",
    "source_location": "https://cdimage.debian.org/debian-cd/current/amd64/iso-cd/debian-10.4.0-amd64-netinst.iso",
    "state": "cancelling",
    "uuid": "07a6c9a3-300e-4d0e-b935-624f3dbdff3f",
    "written_bytes": 0
  }
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 404 | `STORAGE_IMPORT_NOT_FOUND` | No existing import task. |
| 409 | `STORAGE_IMPORT_NOT_IN_PROGRESS` | Only `pending`, `prepared`, or `importing` can be cancelled. |

---

## Add storage to favorites

Adds a storage to the list of favorite storages. Succeeds even if already a favorite.

### Request

```
POST /1.3/storage/{uuid}/favorite
```

### Response `204 No Content`

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 | `STORAGE_INVALID` | Invalid storage UUID. |
| 403 | `STORAGE_FORBIDDEN` | Storage owned by another account. |
| 404 | `STORAGE_NOT_FOUND` | Storage does not exist. |

---

## Remove storage from favorites

Removes a storage from the list of favorite storages. Succeeds even if not a favorite.

### Request

```
DELETE /1.3/storage/{uuid}/favorite
```

### Response `204 No Content`

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 | `STORAGE_INVALID` | Invalid storage UUID. |
| 403 | `STORAGE_FORBIDDEN` | Storage owned by another account. |
| 404 | `STORAGE_NOT_FOUND` | Storage does not exist. |

---

## Delete storage

Deletes an existing storage resource. Storage must be `online` and not attached to any server.
Backups are not deleted by default.

### Request

```
DELETE /1.3/storage/{uuid}?backups={backups}
```

### Query parameters

| Parameter | Accepted values | Default | Description |
|-----------|----------------|---------|-------------|
| `backups` | `keep` / `keep_latest` / `delete` | `keep` | What to do with associated backups. |

### Response `204 No Content`

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 | `STORAGE_INVALID` | Invalid storage value. |
| 400 | `BACKUP_DELETION_POLICY_INVALID` | Invalid backups parameter. Allowed: `keep`, `keep_latest`, `delete`. |
| 403 | `STORAGE_FORBIDDEN` | Storage owned by another account. |
| 404 | `STORAGE_NOT_FOUND` | Storage does not exist. |
| 409 | `STORAGE_ATTACHED` | Storage currently attached to a server. |
| 409 | `STORAGE_STATE_ILLEGAL` | Storage in unusable state. |
