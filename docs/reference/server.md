# Servers

> Source: https://developers.upcloud.com/1.3/8-servers

---

## List server configurations

```
GET /1.3/server_size
```

Returns a list of available server CPU/memory configurations.

### Response

```json
{
  "server_sizes": {
    "server_size": [
      {
        "core_number": "1",
        "memory_amount": "1024"
      },
      {
        "core_number": "1",
        "memory_amount": "2048"
      },
      {
        "core_number": "20",
        "memory_amount": "131072"
      }
    ]
  }
}
```

| Attribute | Description |
|-----------|-------------|
| core_number | Number of CPU cores. |
| memory_amount | Amount of memory in MB. |

---

## List servers

```
GET /1.3/server
```

Returns a list of all servers associated with the current account.

### Query parameters — Filtering

| Parameter | Description |
|-----------|-------------|
| host | Matches by Private Cloud host ID. Can be given multiple times. |
| label | Matches by label key (case-insensitive). Multiple = AND. |
| search | Matches pattern by name, title, uuid, zone, IP, hostname, tag, or template name. |
| state | Matches by server state (started/stopped/maintenance/error). |
| tag | Matches by tag. Comma-separated = OR, colon-separated = AND. |
| uuid | Matches by UUID. Can be given multiple times. |

### Query parameters — Ordering

| Parameter | Values |
|-----------|--------|
| order_by | title / zone / size / cores / memory / popularity |
| sort_by | asc / desc |

Pagination is supported via `limit` and `offset` query parameters.

### Response

```json
{
  "servers": {
    "server": [
      {
        "core_number": "1",
        "hostname": "fi.example.com",
        "labels": {
          "label": [
            {
              "key": "env",
              "value": "prod"
            }
          ]
        },
        "license": 0,
        "memory_amount": "2048",
        "plan": "1xCPU-2GB",
        "plan_ivp4_bytes": "34253332",
        "plan_ipv6_bytes": "0",
        "state": "started",
        "tags": {
          "tag": [
            "PROD",
            "CentOS"
          ]
        },
        "title": "Helsinki server",
        "uuid": "00798b85-efdc-41ca-8021-f6ef457b8531",
        "zone": "fi-hel1"
      }
    ]
  }
}
```

| Attribute | Description |
|-----------|-------------|
| core_number | Number of CPU cores. |
| hostname | Server hostname. |
| labels | Key-value classification labels. |
| license | License amount (0 for self-managed). |
| memory_amount | Memory in MB. |
| plan | Pricing plan name. |
| plan_ivp4_bytes | IPv4 transfer usage in bytes. |
| plan_ipv6_bytes | IPv6 transfer usage in bytes. |
| state | Server state. |
| tags | Tags assigned to the server. |
| title | Human-readable title. |
| uuid | Unique server identifier. |
| zone | Data center zone. |

---

## Get server details

```
GET /1.3/server/{uuid}
```

Returns detailed information about a specific server.

### Response

```json
{
  "server": {
    "boot_order": "disk",
    "core_number": "12",
    "firewall": "on",
    "host": 7653311107,
    "hostname": "server1.example.com",
    "labels": {
      "label": [
        {
          "key": "env",
          "value": "dev"
        }
      ]
    },
    "ip_addresses": {
      "ip_address": [
        {
          "access": "utility",
          "address": "10.0.0.00",
          "family": "IPv4"
        },
        {
          "access": "public",
          "address": "0.0.0.0",
          "family": "IPv4"
        },
        {
          "access": "public",
          "address": "xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx",
          "family": "IPv6"
        }
      ]
    },
    "license": 0,
    "memory_amount": "130048",
    "networking": {
      "interfaces": {
        "interface": [
          {
            "index": 1,
            "ip_addresses": {
              "ip_address": [
                {
                  "address": "94.237.0.207",
                  "dhcp_provided": "yes",
                  "family": "IPv4",
                  "release_policy": "release",
                  "floating": "no"
                }
              ]
            },
            "mac": "de:ff:ff:ff:66:89",
            "network": "037fcf2a-6745-45dd-867e-f9479ea8c044",
            "type": "public",
            "bootable": "no"
          }
        ]
      }
    },
    "nic_model": "virtio",
    "plan": "GPU-12xCPU-128GB-2xL40S",
    "plan_ipv4_bytes": "3565675343",
    "plan_ipv6_bytes": "4534432",
    "simple_backup": "0100,dailies",
    "state": "started",
    "server_group": "0b3d85b4-b3be-46ca-a1e2-3b5d40d60fb1",
    "storage_devices": {
      "storage_device": [
        {
          "address": "virtio:0",
          "part_of_plan": "yes",
          "labels": [
            {
              "key": "foo",
              "value": "bar"
            }
          ],
          "storage": "012580a1-32a1-466e-a323-689ca16f2d43",
          "storage_size": 20,
          "storage_encrypted": "yes",
          "storage_tier": "maxiops",
          "storage_title": "Storage for server1.example.com",
          "type": "disk",
          "boot_disk": "0"
        }
      ]
    },
    "devices": {
      "device": [
        {
          "model": "NVIDIA L40S",
          "type": "gpu",
          "serial": "1790925007018",
          "metadata": {
            "serial": "1723925007011",
            "cores": 18176,
            "memory": 48
          }
        }
      ]
    },
    "tags": {
      "tag": [
        "DEV",
        "Ubuntu"
      ]
    },
    "timezone": "UTC",
    "title": "server1.example.com",
    "uuid": "0077fa3d-32db-4b09-9f5f-30d9e9afb565",
    "video_model": "cirrus",
    "remote_access_enabled": "yes",
    "remote_access_type": "vnc",
    "remote_access_host": "fi-hel1.vnc.upcloud.com",
    "remote_access_password": "aabbccdd",
    "remote_access_port": "3000",
    "zone": "fi-hel1"
  }
}
```

| Attribute | Description |
|-----------|-------------|
| boot_order | Boot device order. |
| core_number | Number of CPU cores. |
| firewall | Firewall state (`on`/`off`). |
| host | Private Cloud host ID. |
| hostname | Server fully qualified domain name. |
| labels | Key-value classification labels. |
| ip_addresses | List of assigned IP addresses. |
| license | License amount. |
| memory_amount | Memory in MB. |
| networking | Network interfaces with detailed IP configuration. |
| nic_model | Network interface card model. |
| plan | Pricing plan name. |
| plan_ipv4_bytes | IPv4 transfer usage in bytes. |
| plan_ipv6_bytes | IPv6 transfer usage in bytes. |
| simple_backup | Simple backup schedule. |
| state | Server state. |
| server_group | Server group UUID. |
| storage_devices | Attached storage devices with labels, encryption, and tier info. |
| devices | Attached devices such as GPUs with metadata. |
| tags | Tags assigned to the server. |
| timezone | Server timezone. |
| title | Human-readable title. |
| uuid | Unique server identifier. |
| video_model | Video interface model. |
| remote_access_enabled | Whether remote access is enabled. |
| remote_access_type | Remote access protocol type. |
| remote_access_host | Remote access hostname. |
| remote_access_password | Remote access password. |
| remote_access_port | Remote access port number. |
| zone | Data center zone. |

---

## Get server GPUs

```
GET /1.3/server/{uuid}/gpus
```

Returns a list of GPUs attached to the specified server.

### Response

```json
{
  "gpus": [
    {
      "cores": 18176,
      "firmware": "95.02.66.00.15",
      "memory_gib": 48,
      "model": "NVIDIA L40S",
      "part": "900-2G133-0180-030",
      "serial": "1723925007011"
    }
  ]
}
```

| Attribute | Description |
|-----------|-------------|
| cores | Number of GPU cores. |
| firmware | GPU firmware version. |
| memory_gib | GPU memory in GiB. |
| model | GPU model name. |
| part | GPU part number. |
| serial | GPU serial number. |

---

## Create server

```
POST /1.3/server
```

Creates a new server. A server can be created from a template (clone), by cloning an existing storage, or from scratch (create).

### Request (from template)

```json
{
  "server": {
    "zone": "fi-hel1",
    "title": "My Debian server",
    "hostname": "debian.example.com",
    "labels": {
      "label": [
        {
          "key": "test",
          "value": ""
        }
      ]
    },
    "plan": "2xCPU-4GB",
    "storage_devices": {
      "storage_device": [
        {
          "action": "clone",
          "labels": [
            {
              "key": "foo",
              "value": "bar"
            }
          ],
          "storage": "01000000-0000-4000-8000-000020030100",
          "encrypted": "yes",
          "title": "Debian from a template",
          "size": 50,
          "tier": "maxiops"
        }
      ]
    },
    "networking": {
      "interfaces": {
        "interface": [
          {
            "ip_addresses": {
              "ip_address": [
                {
                  "family": "IPv4"
                }
              ]
            },
            "type": "public"
          },
          {
            "ip_addresses": {
              "ip_address": [
                {
                  "family": "IPv4"
                }
              ]
            },
            "type": "utility"
          },
          {
            "ip_addresses": {
              "ip_address": [
                {
                  "family": "IPv6"
                }
              ]
            },
            "type": "public"
          },
          {
            "type": "private",
            "network": "03b5b0a0-ad4c-4817-9632-dafdb3ace5d9",
            "ip_addresses": {
              "ip_address": [
                {
                  "family": "IPv4",
                  "address": "10.0.0.50",
                  "dhcp_provided": "yes"
                }
              ]
            }
          }
        ]
      }
    },
    "login_user": {
      "username": "upclouduser",
      "ssh_keys": {
        "ssh_key": [
          "ssh-rsa AAAAB3NzaC1yc2EAA[...]ptshi44x user@example.com"
        ]
      }
    }
  }
}
```

### Server attributes

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| avoid_host | Valid host id | | No | Avoid specific host for HA. |
| host | Valid host id | | No | Start on specific host (private cloud only). |
| boot_order | disk / cdrom / network or combo | disk | No | Boot device order. |
| core_number | Valid combo with memory_amount | smallest | No | CPU cores. |
| firewall | on / off | on | No | Firewall state. |
| hostname | Valid hostname | | Yes | FQDN, max 128 chars. |
| interfaces | 1–10 interface objects | | Yes | Network interfaces. |
| labels | 0–255 label blocks | | No | Key-value classification labels. |
| login_user | login_user block | | No | Login account and SSH keys. |
| memory_amount | Valid combo with core_number | smallest | No | RAM in MB. |
| metadata | yes / no | no | No | Metadata service. |
| nic_model | e1000 / virtio / rtl8139 | virtio | No | NIC model. |
| password_delivery | none / email / sms | email | No | Password delivery method. |
| plan | Plan name or "custom" | custom | No | Pricing plan. |
| storage_devices | 1–16 storage_device blocks | | Yes | Attached storages. |
| timezone | Valid timezone | UTC | No | Timezone identifier. |
| title | 0–255 chars | | Yes | Short description. |
| user_data | URL or shell script | | No | Cloud-init user data. |
| video_model | vga / cirrus | vga | No | Video model. |
| remote_access_type | vnc | vnc | No | Remote access type. |
| remote_access_enabled | yes / no | no | No | Enable remote access. |
| remote_access_password | 8 chars a-zA-Z0-9 | random | No | Remote access password. |
| simple_backup | HHMM,schedule / no | account default | No | Simple backup schedule. |
| zone | Valid zone ID | | Yes | Data center zone. |
| server_group | Server group UUID | | No | Server group to join. |

### Storage device attributes

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| action | create / clone / attach | | Yes | Method to create or attach storage. |
| address | ide / scsi / virtio addresses | Next virtio | No | Device address. |
| size | 10–1024 | original (clone) | Yes (create) | Size in GB. |
| storage | Valid UUID | | Yes (clone/attach) | Storage UUID. |
| tier | Valid tier | plan default or maxiops | No | Storage tier. |
| title | 0–255 chars | | No | Storage title. |
| type | disk / cdrom | disk | No | Device type. |
| boot_disk | 0 / 1 | 0 | No | Mark as boot disk. |
| labels | label list | | No | Storage labels. |
| encrypted | yes / no | no | No | Encrypt at rest. |

### Interface attributes

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| network | Network UUID | varies | Required for private | Network ID. |
| type | public / utility / private | | Yes | Network type. |
| index | Positive integer | array position | No | Interface index. |
| ip_addresses | IP address array | | Yes | IP addresses. |
| source_ip_filtering | yes / no | yes | No | Source IP filtering. |
| bootable | yes / no | no | No | Boot via interface. |

### IP address attributes

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| family | IPv4 / IPv6 | IPv4 | Yes | Address family. |
| address | IP address | | No | Specific IP. |
| dhcp_provided | yes / no | | No | DHCP flag. |

### Login user attributes

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| create_password | yes / no | yes | No | Generate password. |
| username | Valid username | root | No | Login username. |
| ssh_keys | 1–32 ssh_key attrs | | No | SSH public keys. |

### Label attributes

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| key | 2–32 printable ASCII, no `_` prefix | | Yes | Label key. |
| value | 0–255 chars | | Yes | Label value. |

---

## Rebuild server

```
POST /1.3/server/{uuid}/rebuild
```

Rebuilds a server from a new template. The existing boot disk is replaced with a clone of the specified source storage.

### Request

```json
{
  "server_rebuild": {
    "storage_title": "new encrypted storage",
    "clone_source": "01000000-0000-4000-8000-000020050100",
    "detach_disk": "0169b4f8-051c-4a86-9484-f5b798249949",
    "delete_detached_disk": "yes",
    "password_delivery": "email",
    "encrypted": "yes",
    "login_user": {
      "create_password": 0,
      "username": "foobar",
      "ssh_keys": {
        "ssh_key": [
          "ssh-ed25519 ..."
        ]
      }
    }
  }
}
```

### Attributes

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| storage_title | 0–255 chars | | No | New storage title. |
| clone_source | Storage UUID | | Yes | Template to clone from. |
| detach_disk | Storage UUID | | No | Disk to detach. |
| delete_detached_disk | yes / no | no | No | Delete detached disk. |
| password_delivery | none / email / sms | email | No | Password delivery. |
| encrypted | yes / no | no | No | Encrypt new storage. |
| login_user | login_user block | | No | Login config. |

---

## Start server

```
POST /1.3/server/{uuid}/start
```

Starts a stopped server.

### Request

```json
{
  "server": {
    "host": 8055964291
  }
}
```

### Attributes

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| avoid_host | Valid host id | | No | Avoid specific host. |
| host | Valid host id | | No | Start on specific host. |
| start_type | async / sync | sync | No | Start type. |

---

## Stop server

```
POST /1.3/server/{uuid}/stop
```

Stops a started server.

### Request

```json
{
  "stop_server": {
    "stop_type": "soft",
    "timeout": "60"
  }
}
```

### Attributes

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| stop_type | soft / hard | soft | No | Stop type. |
| timeout | 1–600 | | No | Timeout in seconds. |

---

## Restart server

```
POST /1.3/server/{uuid}/restart
```

Restarts a started server.

### Request

```json
{
  "restart_server": {
    "stop_type": "soft",
    "timeout": "60",
    "timeout_action": "destroy"
  }
}
```

### Attributes

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| stop_type | soft / hard | soft | No | Restart type. |
| timeout | 1–600 | | Yes (soft) | Timeout seconds. |
| timeout_action | destroy / ignore | ignore | No | Action on timeout. |
| host | Valid host id | | No | Host for restart. |

---

## Modify server

```
PUT /1.3/server/{uuid}
```

Modifies the configuration of an existing server. Certain changes (CPU, memory) require the server to be stopped first.

### Request

```json
{
  "server": {
    "core_number": "8",
    "memory_amount": "16384",
    "plan": "custom"
  }
}
```

### Attributes

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| boot_order | disk / cdrom / network combo | disk | No | Boot order. |
| core_number | Valid combo with memory_amount | | No | CPU cores. |
| firewall | on / off | on | No | Firewall state. |
| hostname | Valid hostname | | No | Hostname. |
| labels | 0–255 label blocks | | No | Labels. |
| memory_amount | Valid combo with core_number | | No | RAM in MB. |
| metadata | yes / no | no | No | Metadata service. |
| nic_model | e1000 / virtio / rtl8139 | virtio | No | NIC model. |
| plan | Plan or "custom" | custom | No | Pricing plan. |
| simple_backup | HHMM,schedule / no | | No | Backup schedule. |
| title | 0–255 chars | | No | Title. |
| timezone | Valid timezone | | No | Timezone. |
| video_model | vga / cirrus | vga | No | Video model. |
| remote_access_type | vnc | vnc | No | Remote access type. |
| remote_access_enabled | yes / no | no | No | Enable remote access. |
| remote_access_password | 8 chars | | No | Remote access password. |

---

## Cancel server operation

```
POST /1.3/server/{uuid}/cancel
```

Cancels a running server operation (e.g. a pending clone). The response has no body.

### Response

`204 No Content`

---

## Delete server

```
DELETE /1.3/server/{uuid}?storages={storages}&backups={backups}
```

Deletes a server. Optionally deletes attached storages and their backups.

### Query parameters

| Parameter | Description | Accepted values | Default |
|-----------|-------------|-----------------|---------|
| storages | Delete attached storages. | 0, 1, true, false | 0 |
| backups | Handle storage backups. | keep, keep_latest, delete | keep |

### Response

`204 No Content`

---

## Server Groups — List all

```
GET /1.3/server-group
```

Returns a list of all server groups.

### Response

```json
{
  "server_groups": {
    "server_group": [
      {
        "anti_affinity": "yes",
        "anti_affinity_status": [
          {
            "uuid": "...",
            "status": "met"
          }
        ],
        "labels": {
          "label": [
            {
              "key": "foo",
              "value": "bar"
            }
          ]
        },
        "servers": {
          "server": [
            "uuid1",
            "uuid2"
          ]
        },
        "title": "nameserver group",
        "uuid": "0b3d85b4-b3be-46ca-a1e2-3b5d30d60ab0"
      }
    ]
  }
}
```

| Attribute | Description |
|-----------|-------------|
| anti_affinity | Anti-affinity policy (strict/yes/no). |
| anti_affinity_status | Per-server anti-affinity status (met/unmet). |
| labels | Key-value classification labels. |
| servers | List of server UUIDs in the group. |
| title | Group title. |
| uuid | Unique group identifier. |

---

## Server Groups — Get details

```
GET /1.3/server-group/{uuid}
```

Returns detailed information about a specific server group.

### Response

```json
{
  "server_group": {
    "anti_affinity": "yes",
    "anti_affinity_status": [
      {
        "uuid": "...",
        "status": "met"
      }
    ],
    "labels": {
      "label": [
        {
          "key": "foo",
          "value": "bar"
        }
      ]
    },
    "servers": {
      "server": [
        "uuid1",
        "uuid2"
      ]
    },
    "title": "nameserver group",
    "uuid": "0b3d85b4-b3be-46ca-a1e2-3b5d30d60ab0"
  }
}
```

---

## Server Groups — List servers in group

```
GET /1.3/server-group/{uuid}/servers
```

Returns the list of server UUIDs belonging to a specific server group.

### Response

```json
{
  "servers": {
    "server": [
      "uuid1",
      "uuid2"
    ]
  }
}
```

---

## Server Groups — Create

```
POST /1.3/server-group
```

Creates a new server group.

### Request

```json
{
  "server_group": {
    "title": "My brand new server group",
    "servers": {
      "server": [
        "uuid1",
        "uuid2",
        "uuid3"
      ]
    },
    "labels": {
      "label": [
        {
          "key": "foo",
          "value": "bar"
        }
      ]
    },
    "anti_affinity": "yes"
  }
}
```

### Attributes

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| title | 0–255 chars | | No | Group title. |
| server | Server UUID | | No | Server UUID. |
| anti_affinity | strict / yes / no | no | No | Anti-affinity policy. |
| (label) key | 2–32 printable ASCII | | No | Label key. |
| (label) value | 0–255 chars | | No | Label value. |

---

## Server Groups — Modify

```
PATCH /1.3/server-group/{uuid}
```

Modifies an existing server group.

### Request

```json
{
  "server_group": {
    "title": "New fancy title for the group",
    "servers": {
      "server": [
        "uuid1",
        "uuid2"
      ]
    },
    "anti_affinity": "no"
  }
}
```

### Attributes

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| title | 0–255 chars | | No | Group title. |
| server | Server UUID | | No | Server UUID. |
| anti_affinity | strict / yes / no | no | No | Anti-affinity policy. |
| (label) key | 2–32 printable ASCII | | No | Label key. |
| (label) value | 0–255 chars | | No | Label value. |

---

## Server Groups — Delete

```
DELETE /1.3/server-group/{uuid}
```

Deletes a server group. The servers in the group are not deleted.

### Response

`204 No Content`

---

## Server Groups — Add server

```
POST /1.3/server-group/{uuid}/servers
```

Adds a server to an existing server group.

### Request

```json
{
  "server": {
    "uuid": "00235adf-add4-66f1-ef34-e331516bb6a1"
  }
}
```

### Response

`204 No Content`

---

## Server Groups — Remove server

```
DELETE /1.3/server-group/{uuid}/servers/{server_uuid}
```

Removes a server from a server group.

### Response

`204 No Content`

---

## Relocate server

```
POST /1.3/server/{uuid}/relocate
```

Relocates a server to a different zone. The server must be stopped before relocating.

### Request

```json
{
  "zone": "fi-priv-example"
}
```

### Attributes

| Attribute | Accepted values | Required | Description |
|-----------|----------------|----------|-------------|
| zone | Valid zone ID | Yes | Target zone. |
