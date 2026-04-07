# Networks

> Source: https://developers.upcloud.com/1.3/13-networks

The network API manages SDN networks, server network interfaces, routers, and network peerings.

Networks define secure connections between attached cloud servers. A cloud server may belong to any number of virtual networks simultaneously. Server interfaces are identified and ordered by their index.

## Network types

| Type | Description |
|------|-------------|
| **Public** | Provides internet connectivity. Automatically added as public IP addresses are attached. |
| **Utility** | Simple shared private network connecting all servers under the same account. One connection per server. IP addressing is automatic and cannot be changed. |
| **Private** | SDN private networks with flexible, customisable configuration. Recommended for production workloads. Configured per zone. |

## Available IP subnets in Private virtual LANs

Recommended private ranges: `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`.

Allowed prefix lengths: minimum 8 bits, maximum 29 bits.

Excluded ranges:

- `100.64.0.0/10` (CGNAT)
- `127.0.0.0/8` (localhost)
- `224.0.0.0/4` (IP multicast)
- `169.254.0.0/16` (link local)

---

## List all networks

Returns a list of all networks. Supports label filtering via URL parameters (e.g. `?label=env%3Dprod`).

### Request

```
GET /1.3/network/ HTTP/1.1
```

### Response `200 OK`

```json
{
  "networks": {
    "network": [
      {
        "ip_networks": {
          "ip_network": [
            {
              "address": "80.69.172.0/22",
              "dhcp": "yes",
              "dhcp_default_route": "yes",
              "dhcp_dns": [
                "94.237.127.9",
                "94.237.40.9"
              ],
              "dhcp_routes_configuration": {
                "effective_routes_auto_population": {
                  "enabled": "yes",
                  "exclude_by_source": ["static-route"],
                  "filter_by_destination": ["10.0.50.0/24"]
                }
              },
              "dhcp_effective_routes": [
                {
                  "auto_populated": "yes",
                  "route": "10.0.50.0/24",
                  "nexthop": "80.69.172.1"
                }
              ],
              "family": "IPv4",
              "gateway": "80.69.172.1"
            }
          ]
        },
        "effective_routes": [
          {
            "source": "router-connected-network",
            "route": "10.0.50.0/24",
            "source_resource_id": "034c12bc-cf15-4b19-97b2-0ab4e51bb98d"
          }
        ],
        "labels": [],
        "name": "Public 80.69.172.0/22",
        "type": "public",
        "uuid": "03000000-0000-4000-8001-000000000000",
        "zone": "fi-hel1",
        "servers": {
          "server": [
            {"uuid": "007e3200-268f-4848-8b45-bd88c44555d2", "title": "Helsinki server #1"},
            {"uuid": "00c8f13a-945a-48b8-bf5c-db2d7a3a37fe", "title": "Helsinki server #2"}
          ]
        },
        "peerings": {
          "peering": [
            {"uuid": "0f5ab9c7-959a-4bbf-9994-8322517fc359", "name": "my-peering-1", "state": "active"},
            {"uuid": "0f292fdc-73bf-4bf2-9e53-e6364de57a98", "name": "my-peering-2", "state": "active"}
          ]
        }
      }
    ]
  }
}
```

---

## List networks within zone

Returns networks within a specific zone.

### Request

```
GET /1.3/network/?zone={zone} HTTP/1.1
```

### Response `200 OK`

```json
{
  "networks": {
    "network": [
      {
        "ip_networks": {
          "ip_network": [
            {
              "address": "80.69.172.0/22",
              "dhcp": "yes",
              "dhcp_default_route": "yes",
              "dhcp_dns": [
                "94.237.127.9",
                "94.237.40.9"
              ],
              "dhcp_routes_configuration": {
                "effective_routes_auto_population": {
                  "enabled": "yes",
                  "exclude_by_source": ["router-connected-network"],
                  "filter_by_route_type": ["user"]
                }
              },
              "dhcp_effective_routes": [
                {
                  "auto_populated": "yes",
                  "route": "192.168.0.1/24",
                  "nexthop": "80.69.172.1"
                }
              ],
              "family": "IPv4",
              "gateway": "80.69.172.1"
            }
          ]
        },
        "effective_routes": [
          {
            "source": "static-route",
            "route": "192.168.0.1/24",
            "nexthop": "80.69.172.1",
            "type": "user"
          }
        ],
        "labels": [],
        "name": "Public 80.69.172.0/22",
        "type": "public",
        "uuid": "03000000-0000-4000-8001-000000000000",
        "zone": "fi-hel1"
      }
    ]
  }
}
```

---

## Create SDN private network

Creates a new SDN private network that cloud servers from the same zone can be attached to.

### Request

```
POST /1.3/network/ HTTP/1.1
```

```json
{
  "network": {
    "name": "Test private net",
    "zone": "uk-lon1",
    "router": "04c0df35-2658-4b0c-8ac7-962090f4e92a",
    "ip_networks": {
      "ip_network": [
        {
          "address": "172.16.0.0/22",
          "dhcp": "yes",
          "dhcp_default_route": "no",
          "dhcp_dns": [
            "172.16.0.10",
            "172.16.1.10"
          ],
          "dhcp_routes_configuration": {
            "effective_routes_auto_population": {
              "enabled": "yes",
              "exclude_by_source": ["static-route"]
            }
          },
          "dhcp_routes": [
            "192.168.0.0/24-nexthop=10.0.1.100",
            "192.168.100.100/32"
          ],
          "family": "IPv4",
          "gateway": "172.16.0.1"
        }
      ]
    },
    "labels": [
      {
        "key": "test",
        "value": ""
      }
    ]
  }
}
```

### Response `201 OK`

```json
{
  "network": {
    "name": "Test private net",
    "type": "private",
    "uuid": "034c12bc-cf15-4b19-97b2-0ab4e51bb98d",
    "zone": "uk-lon1",
    "router": "04c0df35-2658-4b0c-8ac7-962090f4e92a",
    "ip_networks": {
      "ip_network": [
        {
          "address": "172.16.0.0/22",
          "dhcp": "yes",
          "dhcp_default_route": "no",
          "dhcp_dns": [
            "172.16.0.10",
            "172.16.1.10"
          ],
          "dhcp_routes": [
            "192.168.0.0/24-nexthop=10.0.1.100",
            "192.168.100.100/32"
          ],
          "dhcp_routes_configuration": {
            "effective_routes_auto_population": {
              "enabled": "yes",
              "exclude_by_source": ["static-route"]
            }
          },
          "dhcp_effective_routes": [
            {
              "auto_populated": "no",
              "route": "192.168.0.0/24",
              "nexthop": "10.0.1.100"
            },
            {
              "auto_populated": "no",
              "route": "192.168.100.100/32",
              "nexthop": "172.16.0.1"
            },
            {
              "auto_populated": "yes",
              "route": "10.0.0.0/24",
              "nexthop": "172.16.0.1"
            }
          ],
          "family": "IPv4",
          "gateway": "172.16.0.1"
        }
      ]
    },
    "effective_routes": [
      {
        "source": "router-connected-network",
        "route": "10.0.0.0/24",
        "source_resource_id": "034c12bc-cf15-4b19-97b2-0ab4e51bb98d"
      }
    ],
    "labels": [
      {
        "key": "test",
        "value": ""
      }
    ]
  }
}
```

### Network attributes

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| `name` | A valid name | — | yes | Names the network. |
| `zone` | A valid zone name | — | yes | The zone in which the network is configured. |
| `router` | Existing router UUID, empty string, or `null` | — | no | Attach this network to an existing router. |
| `ip_networks` | Array of IP networks | — | yes | Array of IP networks under `ip_network`. |
| `labels` | List of label objects | — | no | Labels with `key` and `value` fields. |

### IP network attributes (`ip_networks.ip_network[]`)

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| `address` | Address block with netmask | — | yes | Address space for the network (CIDR). |
| `dhcp` | `yes` / `no` | — | yes | Toggles DHCP service. |
| `dhcp_default_route` | `yes` / `no` | `yes` (public) / `no` (other) | no | Whether the gateway is given as default route by DHCP. |
| `dhcp_dns` | IP addresses separated by comma | — | no | DNS server addresses given by DHCP. |
| `dhcp_routes` | Array of IPv4/IPv6 prefix strings | — | no | Additional DHCP classless static routes. Supports `-nexthop=` override. |
| `dhcp_routes_configuration` | Configuration block (see below) | — | no | DHCP effective routes auto-population configuration. |
| `dhcp_bootfile_url` | TFTP URL | — | no | PXE boot file URL (e.g. `tftp://172.16.0.253/pxelinux.0`). |
| `family` | `IPv4` | — | yes | IP address family. Currently only IPv4. |
| `gateway` | IPv4 address | first address of network | no | Gateway address given by DHCP. |

### DHCP routes configuration (`effective_routes_auto_population`)

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| `enabled` | `yes` / `no` | `no` | no | Enable DHCP effective route auto-population. Peering routes are added regardless. |
| `exclude_by_source` | Array of `router-connected-networks` and/or `static-route` | — | no | Route sources to exclude from auto-population. |
| `filter_by_destination` | Array of IPv4/IPv6 prefixes | — | no | Destination route filters for auto-population. |
| `filter_by_route_type` | Array of `user` and/or `service` | — | no | Static route type filters for auto-population. |

### Label attributes

| Attribute | Accepted values | Required | Description |
|-----------|----------------|----------|-------------|
| `key` | 2–32 printable ASCII chars (0x20–0x7E), must not start with `_` | yes | Label key. |
| `value` | 0–255 characters | yes | Label value. |

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 Bad Request | `NETWORK_NAME_INVALID` | Invalid network name. |
| 400 Bad Request | `NETWORK_NAME_MISSING` | Required `name` attribute missing. |
| 400 Bad Request | `NETWORK_TYPE_INVALID` | Invalid network type. |
| 400 Bad Request | `NETWORK_TYPE_MISSING` | Required `type` attribute missing. |
| 400 Bad Request | `DHCP_ROUTES_INVALID` | Invalid `dhcp_routes` value. |
| 400 Bad Request | `ZONE_INVALID` | Invalid zone. |
| 400 Bad Request | `ZONE_MISSING` | Required `zone` attribute missing. |

---

## Get network details

Retrieves the details of a specific network.

### Request

```
GET /1.3/network/{uuid} HTTP/1.1
```

### Response `200 OK`

```json
{
  "network": {
    "name": "Test private net",
    "type": "private",
    "uuid": "034c12bc-cf15-4b19-97b2-0ab4e51bb98d",
    "zone": "uk-lon1",
    "ip_networks": {
      "ip_network": [
        {
          "address": "172.16.0.0/22",
          "dhcp": "yes",
          "dhcp_default_route": "no",
          "dhcp_dns": [
            "172.16.0.10",
            "172.16.1.10"
          ],
          "dhcp_routes_configuration": {
            "effective_routes_auto_population": {
              "enabled": "yes"
            }
          },
          "dhcp_effective_routes": [
            {
              "auto_populated": "no",
              "route": "192.168.0.0/24",
              "nexthop": "192.168.0.100"
            },
            {
              "auto_populated": "yes",
              "route": "10.0.0.0/24",
              "nexthop": "172.16.0.1"
            },
            {
              "auto_populated": "yes",
              "route": "172.16.10.0/24",
              "nexthop": "172.16.0.1"
            }
          ],
          "family": "IPv4",
          "gateway": "172.16.0.1"
        }
      ]
    },
    "effective_routes": [
      {
        "source": "static-route",
        "route": "10.0.0.0/24",
        "nexthop": "192.168.0.100",
        "type": "user"
      },
      {
        "source": "router-connected-network",
        "route": "10.0.0.0/24",
        "source_resource_id": "034c12bc-cf15-4b19-97b2-0ab4e51bb98d"
      },
      {
        "source": "network-peering",
        "route": "172.16.10.0/24",
        "source_resource_id": "03126dc1-a69f-4bc2-8b24-e31c22d64712"
      }
    ],
    "labels": [],
    "servers": {
      "server": [
        {"uuid": "009d64ef-31d1-4684-a26b-c86c955cbf46", "title": "London server #1"},
        {"uuid": "0035079f-9d66-42d5-aa74-12090e7b4ed1", "title": "London server #2"}
      ]
    },
    "peerings": {
      "peering": [
        {"uuid": "0f5ab9c7-959a-4bbf-9994-8322517fc359", "name": "my-peering-1", "state": "active"},
        {"uuid": "0f292fdc-73bf-4bf2-9e53-e6364de57a98", "name": "my-peering-2", "state": "active"}
      ]
    }
  }
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 403 Forbidden | `NETWORK_FORBIDDEN` | No permission to access the network. |
| 404 Not Found | `NETWORK_NOT_FOUND` | The network does not exist. |

---

## Modify network details

Modifies the details of a specific SDN private network. Utility and public networks cannot be modified.

### Request

```
PUT /1.3/network/{uuid} HTTP/1.1
```

```json
{
  "network": {
    "name": "My private network",
    "ip_networks": {
      "ip_network": [
        {
          "address": "172.16.0.0/22",
          "dhcp": "yes",
          "dhcp_default_route": "no",
          "dhcp_dns": [
            "172.16.0.10",
            "172.16.1.10"
          ],
          "dhcp_routes": [
            "192.168.0.0/24",
            "192.168.100.100/32"
          ],
          "dhcp_routes_configuration": {
            "effective_routes_auto_population": {
              "enabled": "no"
            }
          },
          "family": "IPv4",
          "gateway": "172.16.0.1"
        }
      ]
    },
    "labels": [
      {
        "key": "purpose",
        "value": "databases"
      }
    ]
  }
}
```

#### Attaching to a router

```json
{
  "network": {
    "router": "04c0df35-2658-4b0c-8ac7-962090f4e92a"
  }
}
```

### Modify attributes

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| `name` | A valid name | — | no | Names the private network. |
| `router` | Existing router UUID, empty string, or `null` | — | no | Change or clear router attachment. |
| `dhcp` | `yes` / `no` | `yes` | — | Toggles DHCP service. |
| `family` | `IPv4` | — | yes | IP address family. |
| `dhcp_default_route` | `yes` / `no` | `yes`/`no` | no | Default route via DHCP. |
| `dhcp_dns` | IP addresses | — | no | DNS servers via DHCP. |
| `dhcp_routes` | Array of IPv4/IPv6 prefix strings | — | no | Additional DHCP static routes. |
| `dhcp_routes_configuration` | Configuration block | — | no | Effective routes auto-population config. |
| `dhcp_bootfile_url` | TFTP URL | — | no | PXE boot file URL. |
| `gateway` | IPv4 address | first address | no | Gateway address. |
| `labels` | List of label objects | — | no | Labels with `key` and `value`. |

### Response `200 OK`

```json
{
  "network": {
    "name": "My private network",
    "type": "private",
    "uuid": "034c12bc-cf15-4b19-97b2-0ab4e51bb98d",
    "zone": "uk-lon1",
    "ip_networks": {
      "ip_network": [
        {
          "address": "172.16.0.0/22",
          "dhcp": "yes",
          "dhcp_default_route": "no",
          "dhcp_dns": [
            "172.16.0.10",
            "172.16.1.10"
          ],
          "dhcp_routes": [
            "192.168.0.0/24",
            "192.168.100.100/32"
          ],
          "dhcp_routes_configuration": {
            "effective_routes_auto_population": {
              "enabled": "no"
            }
          },
          "dhcp_effective_routes": [],
          "family": "IPv4",
          "gateway": "172.16.0.1"
        }
      ]
    },
    "effective_routes": [
      {
        "source": "network-peering",
        "route": "172.16.10.0/24",
        "source_resource_id": "03126dc1-a69f-4bc2-8b24-e31c22d64712"
      }
    ],
    "labels": [
      {
        "key": "purpose",
        "value": "databases"
      }
    ]
  }
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 Bad Request | `NETWORK_NAME_INVALID` | Invalid network name. |
| 400 Bad Request | `NETWORK_TYPE_INVALID` | Invalid network type. |
| 400 Bad Request | `ZONE_INVALID` | Invalid zone. |
| 400 Bad Request | `UNKNOWN_ATTRIBUTE` | Unknown attribute in request. |
| 400 Bad Request | `DUPLICATE_LABEL_KEYS` | Label keys must be unique. |
| 400 Bad Request | `INVALID_LABEL_KEY` | Invalid label key. |
| 400 Bad Request | `INVALID_LABEL_VALUE` | Invalid label value. |
| 400 Bad Request | `DHCP_ROUTES_INVALID` | Invalid `dhcp_routes` value. |
| 403 Forbidden | `NETWORK_FORBIDDEN` | No permission to modify the network. |

---

## Delete network

Deletes an SDN private network. All attached servers must be detached first.

### Request

```
DELETE /1.3/network/{uuid} HTTP/1.1
```

### Response `204 No Content`

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 403 Forbidden | `NETWORK_FORBIDDEN` | No permission to delete the network. |
| 404 Not Found | `NETWORK_NOT_FOUND` | The network does not exist. |
| 409 Conflict | `NETWORK_NOT_EMPTY` | All servers must be detached first. |

---

## List server networks

Lists all networks a specific cloud server is connected to.

### Request

```
GET /1.3/server/{uuid}/networking HTTP/1.1
```

### Response `200 OK`

```json
{
  "networking": {
    "interfaces": {
      "interface": [
        {
          "index": 2,
          "ip_addresses": {
            "ip_address": [
              {
                "address": "94.237.0.207",
                "dhcp_provided": "yes",
                "family": "IPv4"
              }
            ]
          },
          "mac": "de:ff:ff:ff:66:89",
          "network": "037fcf2a-6745-45dd-867e-f9479ea8c044",
          "source_ip_filtering": "yes",
          "type": "public",
          "bootable": "no"
        },
        {
          "index": 3,
          "ip_addresses": {
            "ip_address": [
              {
                "address": "10.199.3.15",
                "dhcp_provided": "yes",
                "family": "IPv4"
              }
            ]
          },
          "mac": "de:ff:ff:ff:ed:85",
          "network": "03c93fd8-cc60-4849-91b8-6e404b228e2a",
          "source_ip_filtering": "yes",
          "type": "utility",
          "bootable": "no"
        },
        {
          "index": 4,
          "ip_addresses": {
            "ip_address": [
              {
                "address": "10.0.0.20",
                "dhcp_provided": "yes",
                "family": "IPv4"
              }
            ]
          },
          "mac": "de:ff:ff:ff:cc:20",
          "network": "0374ce47-4303-4490-987d-32dc96cfd79b",
          "source_ip_filtering": "yes",
          "type": "private",
          "bootable": "no"
        }
      ]
    }
  }
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 404 Not Found | `SERVER_NOT_FOUND` | The server does not exist. |

---

## Create network interface

Creates a new network interface on a cloud server and attaches the specified network. Server must be `stopped`. Combined limit of interfaces and storage devices is 24.

### Request

```
POST /1.3/server/{uuid}/networking/interface HTTP/1.1
```

```json
{
  "interface": {
    "type": "private",
    "network": "0374ce47-4303-4490-987d-32dc96cfd79b",
    "ip_addresses": {
      "ip_address": [
        {
          "family": "IPv4",
          "address": "10.0.0.20",
          "dhcp_provided": "yes"
        },
        {
          "family": "IPv4",
          "address": "10.0.0.30"
        }
      ]
    },
    "source_ip_filtering": "yes"
  }
}
```

### Interface attributes

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| `network` | Valid network UUID | varies | Req. for `private` | Network UUID to join. |
| `type` | `public` / `utility` / `private` | — | yes | Interface type. |
| `index` | Positive integer | next available | no | Interface index. |
| `ip_addresses` | IP address array | — | yes | Array of IP addresses under `ip_address`. |
| `source_ip_filtering` | `yes` / `no` | `yes` | no | Source IP filtering. Can only be disabled for private interfaces. |
| `bootable` | `yes` / `no` | `no` | no | Whether to try booting through this interface. |

### IP address attributes (`ip_addresses.ip_address[]`)

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| `family` | `IPv4` / `IPv6` | `IPv4` | yes | Address family. IPv6 only for `public` interfaces. |
| `address` | IPv4/IPv6 address | auto | no | Specific IP within the network's address space. |
| `dhcp_provided` | `yes` / `no` | — | no | Marks the address offered by DHCP. Defaults to `yes` for the first IP. |

### Response `200 OK`

```json
{
  "interface": {
    "index": 4,
    "ip_addresses": {
      "ip_address": [
        {
          "address": "10.0.0.20",
          "dhcp_provided": "yes",
          "family": "IPv4",
          "floating": "no"
        },
        {
          "address": "10.0.0.30",
          "dhcp_provided": "no",
          "family": "IPv4",
          "floating": "no"
        }
      ]
    },
    "mac": "de:ff:ff:ff:86:cf",
    "network": "0374ce47-4303-4490-987d-32dc96cfd79b",
    "source_ip_filtering": "yes",
    "type": "private",
    "bootable": "no"
  }
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 404 Not Found | `SERVER_NOT_FOUND` | The server does not exist. |
| 404 Not Found | `NETWORK_NOT_FOUND` | The network does not exist. |
| 409 Conflict | `INTERFACE_EXISTS` | Interface at the specified index already exists. |
| 409 Conflict | `SERVER_STATE_ILLEGAL` | Server must be stopped. |

---

## Modify network interface

Modifies the network interface at the selected index. Server must be `stopped`.

### Request

```
PUT /1.3/server/{uuid}/networking/interface/{index} HTTP/1.1
```

```json
{
  "interface": {
    "ip_addresses": {
      "ip_address": [
        {
          "family": "IPv4",
          "address": "10.0.0.10"
        }
      ]
    },
    "source_ip_filtering": "no",
    "bootable": "yes"
  }
}
```

### Modify interface attributes

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| `index` (path) | Positive integer | — | yes | Index of the interface to modify. |
| `index` (body) | Positive integer | path index | no | New index for reordering. |
| `ip_addresses` | IP address array | — | no | Updated IP addresses. |
| `source_ip_filtering` | `yes` / `no` | — | no | Source IP filtering toggle. |
| `bootable` | `yes` / `no` | — | no | Boot interface toggle. |

### Modify IP address attributes

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| `family` | `IPv4` | — | yes | Currently only IPv4 for private networks. |
| `address` | IPv4 address | auto | no | Specific IP within the network. |

### Response `200 OK`

```json
{
  "interface": {
    "index": 4,
    "ip_addresses": {
      "ip_address": [
        {
          "address": "10.0.0.10",
          "dhcp_provided": "yes",
          "family": "IPv4",
          "floating": "no"
        }
      ]
    },
    "mac": "de:ff:ff:ff:86:cf",
    "network": "0374ce47-4303-4490-987d-32dc96cfd79b",
    "source_ip_filtering": "yes",
    "type": "private",
    "bootable": "no"
  }
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 404 Not Found | `SERVER_NOT_FOUND` | The server does not exist. |
| 404 Not Found | `INTERFACE_NOT_FOUND` | The interface does not exist. |
| 409 Conflict | `INTERFACE_EXISTS` | Interface at the specified body index already exists. |
| 409 Conflict | `SERVER_STATE_ILLEGAL` | Server must be stopped. |

---

## Delete network interface

Detaches a network by deleting the interface at the selected index. Server must be `stopped`.

### Request

```
DELETE /1.3/server/{uuid}/networking/interface/{index} HTTP/1.1
```

### Response `204 No Content`

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 404 Not Found | `SERVER_NOT_FOUND` | The server does not exist. |
| 404 Not Found | `INTERFACE_NOT_FOUND` | The interface does not exist. |
| 409 Conflict | `SERVER_STATE_ILLEGAL` | Server must be stopped. |

---

## Assign IP address to network interface

Assigns a new IP address to a `private` network interface. Maximum of five IP addresses per private interface.

### Request

```
POST /1.3/server/{uuid}/networking/interface/{index}/ip-address?force=no HTTP/1.1
```

```json
{
  "ip_address": {
    "family": "IPv4",
    "address": "10.0.0.30"
  }
}
```

### Attributes

| Attribute | Accepted values | Required | Description |
|-----------|----------------|----------|-------------|
| `family` | `IPv4` / `IPv6` | yes | Address family. |
| `address` | IPv4/IPv6 address | no | Specific IP. Auto-assigned if omitted. |
| `force` (query) | `yes` / `no` | no | Force-move an already-assigned IP from another interface. |

### Response `200 OK`

```json
{
  "ip_address": {
    "address": "10.0.0.30",
    "dhcp_provided": "no",
    "family": "IPv4"
  }
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 404 Not Found | `SERVER_NOT_FOUND` | The server does not exist. |
| 404 Not Found | `INTERFACE_NOT_FOUND` | The interface does not exist. |
| 409 Conflict | `INTERFACE_FORBIDDEN` | Operation forbidden for interface type. |
| 409 Conflict | `ADDRESS_ATTACHED` | IP already attached to the target interface. |
| 409 Conflict | `DUAL_STACK_INTERFACE` | Interface has IPs from a different family. |
| 409 Conflict | `IP_ADDRESS_RESOURCES_UNAVAILABLE` | No IPs available in the network. |
| 409 Conflict | `ADDRESS_INVALID` | IP not valid for the network. |
| 409 Conflict | `IP_ADDRESS_LIMIT_REACHED` | Max IPs per interface reached. |

---

## Delete IP address from network interface

Deletes an IP address from a network interface.

### Request

```
DELETE /1.3/server/{uuid}/networking/interface/{index}/ip-address/{address} HTTP/1.1
```

### Response `204 No Content`

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 404 Not Found | `SERVER_NOT_FOUND` | The server does not exist. |
| 404 Not Found | `INTERFACE_NOT_FOUND` | The interface does not exist. |
| 409 Conflict | `INTERFACE_FORBIDDEN` | Operation forbidden for interface type. |
| 409 Conflict | `IP_ADDRESS_LIMIT_REACHED` | Allowed limit reached. |

---

## List routers

Returns all routers associated with the current account. Supports label filtering.

### Request

```
GET /1.3/router HTTP/1.1
```

### Response `200 OK`

```json
{
  "routers": {
    "router": [
      {
        "attached_networks": {
          "network": []
        },
        "labels": [],
        "name": "Example router",
        "static_routes": [],
        "type": "normal",
        "uuid": "04c0df35-2658-4b0c-8ac7-962090f4e92a"
      }
    ]
  }
}
```

---

## Get router details

Returns detailed information about a specific router.

### Request

```
GET /1.3/router/{uuid} HTTP/1.1
```

### Response `200 OK`

```json
{
  "router": {
    "attached_networks": {
      "network": []
    },
    "labels": [],
    "name": "Example router",
    "static_routes": [],
    "type": "normal",
    "uuid": "04c0df35-2658-4b0c-8ac7-962090f4e92a"
  }
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 404 Not Found | `ROUTER_NOT_FOUND` | The router does not exist. |

---

## Create router

Creates a new router for connecting multiple private networks.

### Request

```
POST /1.3/router HTTP/1.1
```

```json
{
  "router": {
    "name": "Example router",
    "static_routes": [
      {
        "route": "0.0.0.0/0",
        "nexthop": "10.0.0.100",
        "name": "static_route_0"
      }
    ]
  }
}
```

### Router attributes

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| `name` | Non-empty string | — | yes | Name of the router. |
| `static_routes` | 0–50 `static_route` blocks | — | no | Static routes for this router. |
| `labels` | List of label objects | — | no | Labels with `key` and `value`. |

### Static route attributes (`static_routes[]`)

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| `name` | 1–64 chars, `^[a-zA-Z0-9_-]+$` | `static-route-X` | no | Name of the route. |
| `route` | Valid IPv4 or IPv6 prefix | — | yes | Destination prefix. |
| `nexthop` | Valid IPv4 or IPv6 address | — | yes | Next hop address. Must be a reachable server in an attached private network. |
| `type` | `user` or `service` | — | yes | Route type. |

### Response `201 Created`

```json
{
  "router": {
    "attached_networks": {
      "network": []
    },
    "labels": [],
    "name": "Example router",
    "static_routes": [
      {
        "name": "static_route_0",
        "route": "0.0.0.0/0",
        "nexthop": "10.0.0.100",
        "type": "user"
      }
    ],
    "type": "normal",
    "uuid": "04c0df35-2658-4b0c-8ac7-962090f4e92a"
  }
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 Bad Request | `ROUTER_NAME_MISSING` | Required `name` missing. |
| 400 Bad Request | `DUPLICATE_ROUTE` | Duplicate static route. |
| 400 Bad Request | `NEXTHOP_INVALID` | Invalid next hop value. |
| 400 Bad Request | `STATIC_ROUTE_TARGET_INVALID` | Invalid route target. |
| 400 Bad Request | `INVALID_ROUTE` | Invalid route format. |
| 400 Bad Request | `INVALID_ROUTE_FAMILY` | Invalid IP family for next hop. |
| 400 Bad Request | `DUPLICATE_LABEL_KEYS` | Label keys must be unique. |
| 400 Bad Request | `INVALID_LABEL_KEY` | Invalid label key. |
| 400 Bad Request | `INVALID_LABEL_VALUE` | Invalid label value. |
| 409 Conflict | `STATIC_ROUTE_LIMIT_REACHED` | Static route limit exceeded. |

---

## Modify router

Modifies an existing router. Modifications to `service`-type static routes are ignored.

### Request

```
PATCH /1.3/router/{uuid} HTTP/1.1
```

```json
{
  "router": {
    "name": "Example router",
    "labels": [
      {
        "key": "service",
        "value": "postgres cluster connectivity"
      }
    ],
    "static_routes": []
  }
}
```

### Modify router attributes

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| `name` | Non-empty string | — | yes | Name of the router. |
| `static_routes` | 0–50 `static_route` blocks | — | no | Static routes. Service-type changes ignored. |
| `labels` | List of label objects | — | no | Labels with `key` and `value`. |

### Response `200 OK`

```json
{
  "router": {
    "attached_networks": {
      "network": []
    },
    "labels": [
      {
        "key": "service",
        "value": "postgres cluster connectivity"
      }
    ],
    "name": "Example router",
    "static_routes": [],
    "type": "normal",
    "uuid": "04c0df35-2658-4b0c-8ac7-962090f4e92a"
  }
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 404 Not Found | `ROUTER_NOT_FOUND` | The router does not exist. |
| 400 Bad Request | `DUPLICATE_ROUTE` | Duplicate static route. |
| 400 Bad Request | `NEXTHOP_INVALID` | Invalid next hop value. |
| 400 Bad Request | `STATIC_ROUTE_TARGET_INVALID` | Invalid route target. |
| 400 Bad Request | `INVALID_ROUTE` | Invalid route format. |
| 400 Bad Request | `INVALID_ROUTE_FAMILY` | Invalid IP family for next hop. |
| 400 Bad Request | `DUPLICATE_LABEL_KEYS` | Label keys must be unique. |
| 400 Bad Request | `INVALID_LABEL_KEY` | Invalid label key. |
| 400 Bad Request | `INVALID_LABEL_VALUE` | Invalid label value. |
| 409 Conflict | `STATIC_ROUTE_LIMIT_REACHED` | Static route limit exceeded. |

---

## Delete router

Deletes an existing router. Router must have no attached networks.

### Request

```
DELETE /1.3/router/{uuid} HTTP/1.1
```

### Response `204 No Content`

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 404 Not Found | `ROUTER_NOT_FOUND` | The router does not exist. |
| 400 Conflict | `ROUTER_ATTACHED` | Router has attached networks. |
| 409 Conflict | `ROUTER_CHANGE_CONFLICT` | Router change conflict. Try again later. |

---

## List network peerings

Returns all peerings within the current account. Supports label filtering.

### Request

```
GET /1.3/network-peering HTTP/1.1
```

### Response `200 OK`

```json
{
  "network_peerings": {
    "network_peering": [
      {
        "configured_status": "active",
        "labels": [
          {
            "key": "env",
            "value": "prod"
          }
        ],
        "name": "Peering A->B",
        "network": {
          "ip_networks": {
            "ip_network": [
              {"address": "192.168.0.0/24", "family": "IPv4"},
              {"address": "fc02:c4f3::/64", "family": "IPv6"}
            ]
          },
          "uuid": "03126dc1-a69f-4bc2-8b24-e31c22d64712"
        },
        "peer_network": {
          "ip_networks": {
            "ip_network": [
              {"address": "192.168.99.0/24", "family": "IPv4"},
              {"address": "fc02:c4f3:99::/64", "family": "IPv6"}
            ]
          },
          "uuid": "03585987-bf7d-4544-8e9b-5a1b4d74a333"
        },
        "state": "active",
        "uuid": "0f7984bc-5d72-4aaf-b587-90e6a8f32efc"
      }
    ]
  }
}
```

---

## Get network peering details

Returns the state of an existing network peering. The `peer_network` field only contains network details if the peering has been defined by both accounts.

### Peering states

| State | Description |
|-------|-------------|
| `pending-peer` | Waiting for the peer account to create a corresponding peering. |
| `active` | Peering is active and routing traffic. |
| `provisioning` | Peering is being applied (transitionary). |
| `conflict-subnet` | Peering network subnets overlap. |
| `missing-local-router` | Local router has been deleted. |
| `missing-peer-router` | Peer network router is missing or deleted. |
| `deleted-peer-network` | Peer has deleted their network. |
| `disabled` | Peering disabled by user. |
| `peer-disabled` | Peer has disabled their side. |
| `error` | Unexpected error state. |

### Request

```
GET /1.3/network-peering/{uuid} HTTP/1.1
```

### Response `200 OK`

```json
{
  "network_peering": {
    "configured_status": "active",
    "labels": [
      {
        "key": "env",
        "value": "prod"
      }
    ],
    "name": "Peering A->B",
    "network": {
      "ip_networks": {
        "ip_network": [
          {"address": "192.168.0.0/24", "family": "IPv4"},
          {"address": "fc02:c4f3::/64", "family": "IPv6"}
        ]
      },
      "uuid": "03126dc1-a69f-4bc2-8b24-e31c22d64712"
    },
    "peer_network": {
      "ip_networks": {
        "ip_network": [
          {"address": "192.168.99.0/24", "family": "IPv4"},
          {"address": "fc02:c4f3:99::/64", "family": "IPv6"}
        ]
      },
      "uuid": "03585987-bf7d-4544-8e9b-5a1b4d74a333"
    },
    "state": "active",
    "uuid": "0f7984bc-5d72-4aaf-b587-90e6a8f32efc"
  }
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 404 Not Found | `PEERING_NOT_FOUND` | The peering was not found. |

---

## Create network peering

Creates a peering between two private networks (can be cross-account). Must be established both ways to become active. Both networks must have a router attached.

> **Note**: Only create peerings between trusted accounts. There are no traffic limits. Server firewall has no effect for private networks.

### Request

```
POST /1.3/network-peering HTTP/1.1
```

```json
{
  "network_peering": {
    "configured_status": "active",
    "name": "Peering A->B",
    "network": {
      "uuid": "03126dc1-a69f-4bc2-8b24-e31c22d64712"
    },
    "peer_network": {
      "uuid": "03585987-bf7d-4544-8e9b-5a1b4d74a333"
    },
    "labels": [
      {"key": "env", "value": "staging"},
      {"key": "foo", "value": "bar"}
    ]
  }
}
```

### Peering attributes

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| `configured_status` | `active` or `disabled` | `active` | no | Administrative up/down status. |
| `name` | String, 1–255 chars | — | yes | Descriptive name. |
| `network.uuid` | Valid network UUID | — | yes | Local network. |
| `peer_network.uuid` | Valid network UUID | — | yes | Peer network. |
| `labels` | List of label objects | — | no | Labels with `key` and `value`. |

### Response `201 Created`

```json
{
  "network_peering": {
    "configured_status": "active",
    "labels": [
      {"key": "env", "value": "prod"}
    ],
    "name": "Peering A->B",
    "network": {
      "ip_networks": {
        "ip_network": [
          {"address": "192.168.0.0/24", "family": "IPv4"},
          {"address": "fc02:c4f3::/64", "family": "IPv6"}
        ]
      },
      "uuid": "03126dc1-a69f-4bc2-8b24-e31c22d64712"
    },
    "peer_network": {
      "uuid": "03585987-bf7d-4544-8e9b-5a1b4d74a333"
    },
    "state": "pending-peer",
    "uuid": "0f7984bc-5d72-4aaf-b587-90e6a8f32efc"
  }
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 409 Conflict | `LOCAL_NETWORK_NO_ROUTER` | Local network has no router. |
| 404 Not Found | `NETWORK_NOT_FOUND` | Local network not found. |
| 404 Not Found | `PEER_NETWORK_NOT_FOUND` | Peer network not found. |
| 409 Conflict | `PEERING_CONFLICT` | Peering between these accounts already exists. |
| 409 Conflict | `LABEL_LIMIT_REACHED` | Maximum labels per resource reached. |

---

## Modify network peering

Modifies an existing network peering.

### Request

```
PATCH /1.3/network-peering/{uuid} HTTP/1.1
```

```json
{
  "network_peering": {
    "configured_status": "disabled",
    "name": "Peering A->B modified",
    "labels": [
      {"key": "env", "value": "staging"},
      {"key": "foo", "value": "bar"}
    ]
  }
}
```

### Modify peering attributes

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| `configured_status` | `active` or `disabled` | current value | no | Administrative up/down status. |
| `name` | String, 1–255 chars | current value | no | Descriptive name. |
| `labels` | List of label objects | — | no | Labels with `key` and `value`. |

### Response `200 OK`

```json
{
  "network_peering": {
    "configured_status": "disabled",
    "labels": [
      {"key": "env", "value": "prod"}
    ],
    "name": "Peering A->B modified",
    "network": {
      "ip_networks": {
        "ip_network": [
          {"address": "192.168.0.0/24", "family": "IPv4"},
          {"address": "fc02:c4f3::/64", "family": "IPv6"}
        ]
      },
      "uuid": "03126dc1-a69f-4bc2-8b24-e31c22d64712"
    },
    "peer_network": {
      "ip_networks": {
        "ip_network": [
          {"address": "192.168.99.0/24", "family": "IPv4"},
          {"address": "fc02:c4f3:99::/64", "family": "IPv6"}
        ]
      },
      "uuid": "03585987-bf7d-4544-8e9b-5a1b4d74a333"
    },
    "state": "provisioning",
    "uuid": "0f7984bc-5d72-4aaf-b587-90e6a8f32efc"
  }
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 404 Not Found | `PEERING_NOT_FOUND` | The peering was not found. |
| 409 Conflict | `LABEL_LIMIT_REACHED` | Maximum labels per resource reached. |

---

## Delete network peering

Deletes a network peering. The peering must be in `disabled` state first.

### Request

```
DELETE /1.3/network-peering/{uuid} HTTP/1.1
```

### Response `204 No Content`

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 404 Not Found | `PEERING_NOT_FOUND` | The peering was not found. |
| 409 Conflict | `PEERING_NOT_DISABLED` | Peering is not in disabled state. |
