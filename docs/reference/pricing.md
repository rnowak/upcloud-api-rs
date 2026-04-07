# Pricing

> Source: [UpCloud API 1.3 — Pricing](https://developers.upcloud.com/1.3/4-pricing)

UpCloud resources are billed by the hour according to the price list.

## Licensing fees

In addition to resource prices, some resources have licensing fees.
Licenses apply to commercial operating system software. The licensing fee is
indicated by the `license` attribute of the storage resource. The total
licensing fee of a single server is indicated by the server's `license`
attribute, then multiplied by the number of used CPU cores.

## List prices

Returns a list of resource prices in the currency used by the account that
sent the request. Use
[get account details](https://developers.upcloud.com/1.3/3-accounts/#get-account-details)
to check the account currency.

### Request

`GET /1.3/price`

### Response

```json
{
  "prices": {
    "zone": [
      {
        "name": "fi-hel1",
        "firewall": {
          "amount": 1,
          "price": 0.5
        },
        "io_request_backup": {
          "amount": 1000000,
          "price": 10
        },
        "io_request_hdd": {
          "amount": 1000000,
          "price": 0
        },
        "io_request_maxiops": {
          "amount": 1000000,
          "price": 0
        },
        "ipv4_address": {
          "amount": 1,
          "price": 0.3
        },
        "ipv6_address": {
          "amount": 1,
          "price": 0
        },
        "public_ipv4_bandwidth_in": {
          "amount": 1,
          "price": 0
        },
        "public_ipv4_bandwidth_out": {
          "amount": 1,
          "price": 5
        },
        "public_ipv6_bandwidth_in": {
          "amount": 1,
          "price": 0
        },
        "public_ipv6_bandwidth_out": {
          "amount": 1,
          "price": 5
        },
        "server_core": {
          "amount": 1,
          "price": 1.3
        },
        "server_memory": {
          "amount": 256,
          "price": 0.45
        },
        "storage_backup": {
          "amount": 1,
          "price": 0.007
        },
        "storage_hdd": {
          "amount": 1,
          "price": 0.013
        },
        "storage_maxiops": {
          "amount": 1,
          "price": 0.028
        },
        "server_plan_1xCPU-2GB": {
          "amount": 1,
          "price": 2.2321
        },
        "server_plan_2xCPU-4GB": {
          "amount": 1,
          "price": 4.4642
        }
      }
    ]
  }
}
```

### Response attributes

#### Top-level

| Attribute | Type | Description |
|-----------|------|-------------|
| `prices` | object | Wrapper object containing the price list. |

#### `prices`

| Attribute | Type | Description |
|-----------|------|-------------|
| `zone` | array | List of per-zone pricing objects. |

#### Zone object

| Attribute | Type | Description |
|-----------|------|-------------|
| `name` | string | The zone identifier (e.g. `fi-hel1`). |
| `firewall` | price item | Hourly price per firewall. |
| `io_request_backup` | price item | Price per 1 000 000 I/O requests on backup storage. |
| `io_request_hdd` | price item | Price per 1 000 000 I/O requests on HDD storage. |
| `io_request_maxiops` | price item | Price per 1 000 000 I/O requests on MaxIOPS storage. |
| `ipv4_address` | price item | Hourly price per IPv4 address. |
| `ipv6_address` | price item | Hourly price per IPv6 address. |
| `public_ipv4_bandwidth_in` | price item | Price per GB of inbound public IPv4 transfer. |
| `public_ipv4_bandwidth_out` | price item | Price per GB of outbound public IPv4 transfer. |
| `public_ipv6_bandwidth_in` | price item | Price per GB of inbound public IPv6 transfer. |
| `public_ipv6_bandwidth_out` | price item | Price per GB of outbound public IPv6 transfer. |
| `server_core` | price item | Hourly price per server CPU core. |
| `server_memory` | price item | Hourly price per 256 MB of server memory. |
| `storage_backup` | price item | Hourly price per GB of backup storage. |
| `storage_hdd` | price item | Hourly price per GB of HDD storage. |
| `storage_maxiops` | price item | Hourly price per GB of MaxIOPS storage. |
| `server_plan_*` | price item | Hourly price for a specific server plan (dynamic key, e.g. `server_plan_1xCPU-2GB`). |

#### Price item

| Attribute | Type | Description |
|-----------|------|-------------|
| `amount` | number | The number of units the price covers. |
| `price` | number | The hourly price for the given amount of units, in the account's currency. |
