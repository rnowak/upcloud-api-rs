# IP Addresses

> **Base path:** `/1.3/ip_address`
>
> Source: [UpCloud API 1.3 — IP Addresses](https://developers.upcloud.com/1.3/10-ip-addresses)

---

## Overview

The UpCloud network has public and private IP addresses.

- **Public** IP addresses connect a server to the Internet.
- **Private (Utility)** network addresses are available for all servers within a
  single account and can communicate globally as long as utility network
  interfaces are configured (added by default).

A server may have **0–5** public IPv4 and IPv6 addresses and **exactly one**
utility-network IPv4 address. Every server must have at least one IP address.
IP addresses are assigned automatically via DHCP.

IP addresses cannot be reserved for later use, except for **floating IPs** that
can be reserved and switched from server to server within a specific zone.

A typical server has three network interfaces:

| Interface | Network        |
|-----------|----------------|
| 1st       | Public         |
| 2nd       | Utility        |
| 3rd       | IPv6 (if any)  |

### Floating IPs

Floating IPs are IPv4 addresses that can be attached to any IPv4 interface in
their zone. They can be instantly switched between servers for maintenance or
failover. See [UpCloud Floating IP tutorial](https://upcloud.com/community/tutorials/floating-ip-addresses/).

### Release Policy

The release policy controls what happens to an IP address when its dependent
resource (e.g. a server or service) is deleted.

| Value     | Normal (non-floating) address                         | Floating address                          |
|-----------|--------------------------------------------------------|-------------------------------------------|
| `release` | Address is deleted.                                    | Address is deleted.                        |
| `keep`    | Converted to a detached floating IP.                   | Remains as a detached floating IP.         |

Floating IPs default to `keep`. Release policy is only supported for **public
IPv4** addresses.

### Service Delegation

Floating IPv4 addresses can be delegated to a service (e.g. Load Balancer)
instead of a server. Delegated addresses are tracked via the `is_delegated`
attribute.

While delegated:
- The IP address **cannot** be deleted.
- Its MAC address **cannot** be modified.
- Only the release policy and PTR record may be updated.

Restrictions are lifted once the IP is removed from the service (with
`release_policy: "keep"`).

---

## Response Attributes

| Attribute        | Type    | Description |
|------------------|---------|-------------|
| `access`         | string  | `utility`, `public`, or `private`. |
| `address`        | string  | The IP address. |
| `family`         | string  | `IPv4` or `IPv6`. |
| `floating`       | string  | `yes` if the address is a floating IP, `no` otherwise. |
| `is_delegated`   | string  | `yes` if the address is delegated to a service. |
| `mac`            | string  | MAC address of the network interface the IP is attached to. |
| `part_of_plan`   | string  | `yes` if the address is included in a server plan. |
| `ptr_record`     | string  | Reverse DNS PTR record for the address. |
| `release_policy` | string  | `release` or `keep`. |
| `server`         | string  | UUID of the server the address is attached to. |
| `zone`           | string  | Availability zone identifier (e.g. `fi-hel2`). |

---

## List IP Addresses

Returns all IP addresses assigned to servers on the current account.

### Request

```
GET /1.3/ip_address HTTP/1.1
```

### Response `200 OK`

```json
{
  "ip_addresses": {
    "ip_address": [
      {
        "access": "utility",
        "address": "10.0.0.0",
        "family": "IPv4",
        "ptr_record": "",
        "server": "0053cd80-5945-4105-9081-11192806a8f7",
        "mac": "mm:mm:mm:mm:mm:m1",
        "floating": "no",
        "release_policy": "release",
        "zone": "fi-hel2"
      },
      {
        "access": "utility",
        "address": "10.0.0.1",
        "family": "IPv4",
        "ptr_record": "",
        "server": "006b6701-55d2-4374-ac40-56cc1501037f",
        "mac": "mm:mm:mm:mm:mm:m2",
        "floating": "no",
        "release_policy": "keep",
        "zone": "de-fra1"
      },
      {
        "access": "public",
        "address": "x.x.x.x",
        "family": "IPv4",
        "part_of_plan": "yes",
        "ptr_record": "x-x-x-x.zone.upcloud.host",
        "server": "0053cd80-5945-4105-9081-11192806a8f7",
        "mac": "mm:mm:mm:mm:mm:m1",
        "floating": "yes",
        "release_policy": "release",
        "zone": "de-fra1"
      },
      {
        "access": "public",
        "address": "xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx",
        "family": "IPv6",
        "ptr_record": "xxxx-xxxx-xxxx-xxxx.v6.zone.upcloud.host",
        "server": "006b6701-55d2-4374-ac40-56cc1501037f",
        "mac": "mm:mm:mm:mm:mm:m3",
        "floating": "no",
        "release_policy": "release",
        "zone": "fi-hel1"
      },
      {
        "access": "public",
        "address": "y.y.y.y",
        "family": "IPv4",
        "ptr_record": "y.y.y.y.zone.host.upcloud.com",
        "floating": "yes",
        "release_policy": "keep",
        "zone": "nl-ams1"
      },
      {
        "access": "public",
        "address": "z.z.z.z",
        "family": "IPv4",
        "ptr_record": "z.z.z.z.zone.host.upcloud.com",
        "floating": "yes",
        "release_policy": "keep",
        "is_delegated": "yes",
        "zone": "nl-ams1"
      }
    ]
  }
}
```

---

## Get IP Address Details

Returns detailed information about a specific IP address.

### Request

```
GET /1.3/ip_address/{address} HTTP/1.1
```

`{address}` — the IP address, e.g. `127.0.0.1`.

### Response `200 OK`

```json
{
  "ip_address": {
    "access": "public",
    "address": "0.0.0.0",
    "family": "IPv4",
    "part_of_plan": "yes",
    "ptr_record": "test.example.com",
    "server": "009d64ef-31d1-4684-a26b-c86c955cbf46",
    "mac": "mm:mm:mm:mm:mm:m5",
    "floating": "no",
    "release_policy": "keep",
    "is_delegated": "yes",
    "zone": "sg-sin1"
  }
}
```

---

## Assign IP Address

Assigns a new IP address to a server. Maximum of five public IP addresses per
server. If a MAC address is specified, the server must be in `stopped` state.

Floating IPs are attached via [Modify IP Address](#modify-ip-address).

### Request

```
POST /1.3/ip_address HTTP/1.1
```

#### Standard assignment

```json
{
  "ip_address": {
    "family": "IPv4",
    "server": "009d64ef-31d1-4684-a26b-c86c955cbf46"
  }
}
```

### Request Attributes

| Attribute        | Accepted values      | Default  | Required | Description |
|------------------|----------------------|----------|----------|-------------|
| `access`         | `utility` / `public` | `public` | no       | Utility or public network. |
| `family`         | `IPv4` / `IPv6`      | `IPv4`   | no       | Address family. |
| `address`        | IPv4 address         |          | no       | Assign a specific address. Requires special privileges; IPv4 only. |
| `server`         | Server UUID          |          | yes      | UUID of the server to assign to. |

### Creating Floating IPs

Two options:

1. **Attached** — specify `mac` of the server's public interface.
2. **Detached** — specify `zone` and `floating: "yes"` without a `mac`.

#### Floating IP Attributes

| Attribute        | Accepted values      | Default  | Required | Description |
|------------------|----------------------|----------|----------|-------------|
| `access`         | `utility` / `public` | `public` | no       | Utility or public network. |
| `family`         | `IPv4` / `IPv6`      | `IPv4`   | no       | Address family. |
| `floating`       | `yes` / `no`         | `no`     | no       | Whether the address is a floating IP. |
| `address`        | IPv4 address         |          | no       | Assign a specific address. Requires special privileges; IPv4 only. |
| `mac`            | MAC address          |          | varies   | MAC address of server interface. Required for non-floating addresses. |
| `zone`           | Zone identifier      |          | varies   | Required when creating a detached floating IP. |
| `release_policy` | `release` / `keep`   | `keep`   | no       | Behavior when dependent resource is deleted. |

#### Attached floating IP example

```json
{
  "ip_address": {
    "family": "IPv4",
    "mac": "mm:mm:mm:mm:mm:m1",
    "release_policy": "release",
    "floating": "yes"
  }
}
```

#### Response `201 Created`

```json
{
  "ip_address": {
    "access": "public",
    "address": "0.0.0.0",
    "family": "IPv4",
    "ptr_record": "xxxx-xxxx-xxxx-xxxx.v6.zone.upcloud.host",
    "server": "009d64ef-31d1-4684-a26b-c86c955cbf46",
    "mac": "mm:mm:mm:mm:mm:m1",
    "floating": "yes",
    "release_policy": "release",
    "zone": "fi-hel2"
  }
}
```

#### Detached floating IP example

```json
{
  "ip_address": {
    "family": "IPv4",
    "zone": "fi-hel2",
    "floating": "yes"
  }
}
```

#### Response `201 Created`

```json
{
  "ip_address": {
    "access": "public",
    "address": "xx.xx.xx.xx",
    "family": "IPv4",
    "floating": "yes",
    "ptr_record": "xx-xx-xx-xx.de-fra1.upcloud.host",
    "release_policy": "keep",
    "zone": "de-fra1"
  }
}
```

### Error Responses

| HTTP Status             | Error Code                        | Description |
|-------------------------|-----------------------------------|-------------|
| 400 Bad Request         | `SERVER_INVALID`                  | Invalid server attribute value. |
| 402 Payment Required    | `INSUFFICIENT_CREDITS`            | Not enough credits. |
| 403 Forbidden           | `SERVER_FORBIDDEN`                | Server owned by another account. |
| 403 Forbidden           | `ADDRESS_SELECTION_FORBIDDEN`     | Address selection not available for this account. |
| 404 Not Found           | `SERVER_NOT_FOUND`                | Server does not exist. |
| 409 Conflict            | `IP_ADDRESS_LIMIT_REACHED`        | Maximum IP addresses already attached to server. |
| 409 Conflict            | `IP_ADDRESS_RESOURCES_UNAVAILABLE`| Not enough IP addresses available in the zone. |
| 409 Conflict            | `SERVER_STATE_ILLEGAL`            | Server in an incompatible state. |
| 409 Conflict            | `RELEASE_POLICY_NOT_AVAILABLE`    | Release policy only available for public IPv4. |

---

## Modify IP Address

Modifies the reverse DNS PTR record and/or floating IP attachment. Attaching a
floating IP to an interface detaches it from its previous interface. Pass `null`
or `""` as `mac` to detach without re-attaching. PTR records can only be set on
public IP addresses.

### Request

```
PATCH /1.3/ip_address/{address} HTTP/1.1
```

`{address}` — the IP address, e.g. `127.0.0.1`.

```json
{
  "ip_address": {
    "mac": "mm:mm:mm:mm:mm:m1",
    "ptr_record": "hostname.example.com",
    "release_policy": "release"
  }
}
```

### Request Attributes

| Attribute        | Accepted values    | Default | Required | Description |
|------------------|--------------------|---------|----------|-------------|
| `mac`            | MAC address        |         | no       | MAC address of server interface to attach floating IP to. |
| `ptr_record`     | FQDN               |         | no       | Reverse DNS PTR record (fully qualified domain name). |
| `release_policy` | `release` / `keep` |         | no       | Behavior when dependent resource is deleted. |

### Response `202 Accepted`

```json
{
  "ip_address": {
    "access": "public",
    "address": "0.0.0.0",
    "family": "IPv4",
    "ptr_record": "hostname.example.com",
    "server": "009d64ef-31d1-4684-a26b-c86c95c5bf46",
    "mac": "mm:mm:mm:mm:mm:m1",
    "floating": "yes",
    "zone": "sg-sin1"
  }
}
```

### Error Responses

| HTTP Status      | Error Code                    | Description |
|------------------|-------------------------------|-------------|
| 400 Bad Request  | `IP_ADDRESS_INVALID`          | Invalid IP address attribute. |
| 400 Bad Request  | `PTR_RECORD_INVALID`          | Invalid PTR record value. |
| 400 Bad Request  | `MAC_INVALID`                 | Invalid MAC address value. |
| 403 Forbidden    | `IP_ADDRESS_FORBIDDEN`        | IP address belongs to another user. |
| 404 Not Found    | `IP_ADDRESS_NOT_FOUND`        | IP address does not exist. |
| 409 Conflict     | `PTR_RECORD_NOT_SUPPORTED`    | PTR record set on a utility address. |
| 409 Conflict     | `FLOATING_IP_NOT_AVAILABLE`   | Floating IP operation not available for this address. |
| 409 Conflict     | `IP_ADDRESS_DELEGATED`        | Address is delegated; MAC changes and deletion are blocked. |
| 409 Conflict     | `RELEASE_POLICY_NOT_AVAILABLE`| Release policy only available for public IPv4. |

---

## Release IP Address

Removes an IP address from a server.

### Request

```
DELETE /1.3/ip_address/{address} HTTP/1.1
```

`{address}` — the IP address, e.g. `127.0.0.1`.

### Response `204 No Content`

*(empty body)*

### Error Responses

| HTTP Status      | Error Code                       | Description |
|------------------|----------------------------------|-------------|
| 400 Bad Request  | `IP_ADDRESS_INVALID`             | Invalid IP address attribute. |
| 403 Forbidden    | `IP_ADDRESS_FORBIDDEN`           | IP address assigned to another account. |
| 404 Not Found    | `IP_ADDRESS_NOT_FOUND`           | IP address does not exist. |
| 409 Conflict     | `SERVER_STATE_ILLEGAL`           | Server in an incompatible state. |
| 409 Conflict     | `CANNOT_DELETE_PRIVATE_ADDRESS`  | Utility network address cannot be deleted. |
| 409 Conflict     | `IP_ADDRESS_DELEGATED`           | Cannot delete while delegated to a service. |
