# Accounts

> Source: https://developers.upcloud.com/1.3/3-accounts

An API user account and password is required to access the UpCloud API. The API user is associated with an UpCloud account. A user account may have multiple API users to be used in different client software. In order to use resources from the cloud, the account must have enough credits.

Main account can create subaccounts that have access only to a limited set of resources (servers and storages). A subaccount can have API access, GUI access or both. API access can be further controlled with IP access lists. A subaccount can only query and modify its own details.

Credits are used to pay for cloud resources such as servers, storages, network traffic and IP addresses. Credits are automatically deducted for used resources on an hourly basis. Should the user run out of credits, active resources will be disabled and reactivated after the credit balance returns to positive. Credits can be purchased from the UpCloud website.

---

## Get account information

Returns information on the user's account and resource limits.

### Request

`GET /1.3/account`

### Response

```json
{
  "account": {
    "credits": 9972.2324,
    "username": "username",
    "resource_limits": {
      "cores": 200,
      "detached_floating_ips": 10,
      "load_balancers": 50,
      "managed_databases": 100,
      "managed_kubernetes": 100,
      "managed_object_storages": 100,
      "memory": 1048576,
      "network_gateways": 100,
      "network_peerings": 100,
      "networks": 100,
      "ntp_excess_gib": 20000,
      "public_ipv4": 100,
      "public_ipv6": 100,
      "routers": 100,
      "storage_hdd": 10240,
      "storage_maxiops": 10240,
      "storage_standard": 10240,
      "storage_total": 30720
    }
  }
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| credits | float | Account credit balance |
| username | string | Account username |
| resource_limits | object | Resource limit quotas (see below) |

### Resource limit attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| cores | integer | Maximum number of CPU cores |
| detached_floating_ips | integer | Maximum number of detached floating IP addresses |
| load_balancers | integer | Maximum number of Load Balancer services |
| managed_databases | integer | Maximum number of managed databases |
| managed_kubernetes | integer | Maximum number of managed Kubernetes clusters |
| managed_object_storages | integer | Maximum number of managed object storages |
| memory | integer | Maximum amount of memory in MiB |
| network_gateways | integer | Maximum number of network gateways |
| network_peerings | integer | Maximum number of network peerings |
| networks | integer | Maximum number of networks |
| ntp_excess_gib | integer | Network Transfer Pool excess transfer limit after which bandwidth caps are applied |
| public_ipv4 | integer | Maximum number of IPv4 addresses |
| public_ipv6 | integer | Maximum number of IPv6 addresses |
| routers | integer | Maximum number of routers |
| storage_hdd | integer | Maximum amount of Archive tier storage space in MiB |
| storage_maxiops | integer | Maximum amount of MaxIOPS storage space in MiB |
| storage_standard | integer | Maximum amount of Standard tier storage space in MiB |
| storage_total | integer | Maximum total storage space in MiB |

---

## Get account list

Returns a list of all accounts. Account listing is only available to the main account.

It is possible to filter shown accounts with label URL parameters, e.g. `?label=organisation` or `?label=org%3Dit`. URL parameter can be given multiple times to add more filters (e.g. `?label=org%3Dit&label=developers`), where only accounts that match all labels are returned. Label keys are matched case insensitively.

### Request

`GET /1.3/account/list`

### Query parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| label | string | Filter accounts by label. Can be specified multiple times; only accounts matching all labels are returned. |

### Response

```json
{
  "accounts": {
    "account": [
      {
        "labels": [],
        "roles": {
          "role": ["technical"]
        },
        "type": "mymain",
        "username": "test"
      },
      {
        "labels": [],
        "roles": {
          "role": ["technical"]
        },
        "type": "sub",
        "username": "my_sub_account"
      },
      {
        "labels": [
          {
            "key": "to_be_removed",
            "value": "after 2022-31-12"
          }
        ],
        "roles": {
          "role": []
        },
        "type": "sub",
        "username": "my_temp_account"
      },
      {
        "labels": [],
        "roles": {
          "role": ["billing"]
        },
        "type": "sub",
        "username": "my_billing_account"
      }
    ]
  }
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| labels | array | Labels assigned to the account |
| roles | object | Wrapper containing a `role` array of role name strings |
| type | string | Account type (`mymain` or `sub`) |
| username | string | Account username |

---

## Get account details

Returns detailed information about a specific account.

### Request

`GET /1.3/account/details/{username}`

Where `username` is the account on which we want the details of.

### Response

```json
{
  "account": {
    "main_account": "mymain",
    "type": "sub",
    "username": "my_sub_account",
    "first_name": "first",
    "last_name": "last",
    "company": "UpCloud Ltd",
    "address": "my address",
    "postal_code": "00130",
    "city": "Helsinki",
    "state": "",
    "country": "FIN",
    "currency": "EUR",
    "language": "fi",
    "phone": "+358.31245434",
    "email": "test@example.com",
    "vat_number": "FI24315605",
    "timezone": "UTC",
    "campaigns": {
      "campaign": []
    },
    "roles": {
      "role": []
    },
    "allow_api": "yes",
    "allow_gui": "no",
    "enable_3rd_party_services": "yes",
    "network_access": {
      "network": []
    },
    "server_access": {
      "server": [
        {
          "storage": "no",
          "uuid": "*"
        }
      ]
    },
    "storage_access": {
      "storage": [
        "*"
      ]
    },
    "tag_access": {
      "tag": []
    },
    "ip_filters": {
      "ip_filter": []
    },
    "labels": []
  }
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| main_account | string | Username of the parent main account |
| type | string | Account type (`mymain` or `sub`) |
| username | string | Account username |
| first_name | string | First name |
| last_name | string | Last name |
| company | string | Company name |
| address | string | Street address |
| postal_code | string | Postal/zip code |
| city | string | City |
| state | string | U.S. state |
| country | string | ISO 3166-1 three character country code |
| currency | string | Currency (EUR/GBP/USD/SGD) |
| language | string | Language (fi/en) |
| phone | string | Phone number in international format |
| email | string | E-mail address |
| vat_number | string | Valid VAT number |
| timezone | string | Time zone in Continent/Location format |
| campaigns | object | Account campaigns |
| roles | object | Wrapper containing a `role` array of role name strings |
| allow_api | string | Whether API access via HTTP Basic is enabled (`yes`/`no`) |
| allow_gui | string | Whether control panel access is enabled (`yes`/`no`) |
| enable_3rd_party_services | string | Whether 3rd party services are allowed (`yes`/`no`) |
| network_access | object | Network UUIDs the account can manage (`*` means all) |
| server_access | object | Server UUIDs the account can manage with storage access flag |
| storage_access | object | Storage UUIDs the account can manage (`*` means all) |
| tag_access | object | Tag-based server/storage access permissions |
| ip_filters | object | IP address restrictions on API access |
| labels | array | Labels describing the account |

---

## Modify account details

Modifies the details of an existing account.

### Request

`PUT /1.3/account/details/{username}`

or

`PUT /1.3/account/sub/{username}`

Where `username` is the account which we want to modify.

```json
{
  "account": {
    "first_name": "first",
    "last_name": "last",
    "company": "my company name",
    "address": "my address",
    "postal_code": "00130",
    "city": "Helsinki",
    "state": "",
    "country": "FIN",
    "currency": "EUR",
    "language": "en",
    "phone": "+358.31245434",
    "email": "test@example.com",
    "vat_number": "my vat number",
    "timezone": "Europe/Helsinki",
    "roles": {
      "role": [
        "billing",
        "aux_billing",
        "technical"
      ]
    },
    "allow_api": "yes",
    "allow_gui": "no",
    "enable_3rd_party_services": "yes",
    "network_access": {
      "network": []
    },
    "server_access": {
      "server": [
        {
          "uuid": "*",
          "storage": "no"
        }
      ]
    },
    "storage_access": {
      "storage": [
        "*"
      ]
    },
    "tag_access": {
      "tag": [
        {
          "name": "mytag",
          "storage": "yes"
        },
        {
          "name": "mytag2",
          "storage": "no"
        }
      ]
    },
    "ip_filters": {
      "ip_filter": []
    },
    "labels": [
      {
        "key": "role",
        "value": "staging"
      }
    ]
  }
}
```

### Request attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| first_name | string | First name (1–50 chars) |
| last_name | string | Last name (1–50 chars) |
| company | string | Company name (1–100 chars) |
| address | string | Street address (1–2 lines, each 1–100 chars) |
| postal_code | string | Postal/zip code (1–100 chars) |
| city | string | City (1–100 chars) |
| state | string | U.S. state |
| country | string | ISO 3166-1 three character country code |
| currency | string | Currency (EUR/GBP/USD/SGD) |
| language | string | Language (fi/en) |
| phone | string | Phone number in international format (`+countrycode.nationalpart`) |
| email | string | E-mail address |
| vat_number | string | Valid VAT number |
| timezone | string | Time zone in Continent/Location format |
| roles | object | Roles: `billing`, `aux_billing`, or `technical` |
| allow_api | string | Whether API access via HTTP Basic is enabled (`yes`/`no`) |
| allow_gui | string | Whether control panel access is enabled (`yes`/`no`) |
| enable_3rd_party_services | string | Whether 3rd party services are allowed (`yes`/`no`) |
| network_access | object | Network UUIDs the account can manage (`*` means all) |
| server_access | object | Server UUIDs with storage access flag |
| storage_access | object | Storage UUIDs the account can manage (`*` means all) |
| tag_access | object | Tag-based server/storage access permissions |
| ip_filters | object | IP address restrictions on API access (CIDR, ranges, or single IPs) |
| labels | array | Labels describing the account |

### Response

`HTTP/1.1 204 OK`

---

## Add subaccount

Creates a new subaccount under the main account.

### Request

`POST /1.3/account/sub`

```json
{
  "sub_account": {
    "first_name": "first",
    "last_name": "last",
    "company": "my company name",
    "address": "my address",
    "postal_code": "00130",
    "city": "Helsinki",
    "state": "",
    "country": "FIN",
    "phone": "+358.31245434",
    "email": "test@example.com",
    "vat_number": "my vat number",
    "timezone": "Europe/Helsinki",
    "username": "myusername",
    "password": "mysecr3tPassword",
    "currency": "EUR",
    "language": "en",
    "roles": {
      "role": ["technical", "billing", "aux_billing"]
    },
    "allow_gui": "no",
    "allow_api": "yes",
    "network_access": {
      "network": []
    },
    "server_access": {
      "server": []
    },
    "storage_access": {
      "storage": []
    },
    "tag_access": {
      "tag": []
    },
    "ip_filters": {
      "ip_filter": []
    },
    "labels": [
      {
        "key": "department",
        "value": "it"
      }
    ]
  }
}
```

### Request attributes

| Attribute | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| first_name | string, 1–50 chars | | yes (billing) | First name |
| last_name | string, 1–50 chars | | yes (billing) | Last name |
| company | string, 1–100 chars | | | Company name |
| address | string, 1–2 lines, each 1–100 chars | | yes (billing) | Street address |
| postal_code | string, 1–100 chars | | yes (billing) | Postal/zip code |
| city | string, 1–100 chars | | yes (billing) | City |
| state | string | | yes (billing, USA) | U.S. state |
| country | ISO 3166-1 alpha-3 code | | yes (billing) | ISO 3166-1 three character country code |
| currency | EUR/GBP/USD/SGD | | yes | Currency |
| language | fi/en | | yes | Language |
| phone | +countrycode.nationalpart | | yes | Phone number in international format |
| email | valid e-mail address | | yes | E-mail address |
| vat_number | valid VAT number | | no | Valid VAT number |
| timezone | valid time zone | | yes | Time zone in Continent/Location format |
| username | 4–64 characters | | yes | Username |
| password | string | | no | Password |
| roles | list of role names | | no | Roles: `billing`, `aux_billing`, or `technical` |
| labels | list of label objects | | no | Labels with `key` and `value` |
| allow_api | yes/no | yes | | Whether API access via HTTP Basic is enabled. Has no effect on API tokens. |
| allow_gui | yes/no | yes | | Whether control panel access is enabled |
| enable_3rd_party_services | yes/no | yes | no | Whether 3rd party services are allowed |
| network | list of network UUIDs or `*` | `*` | no | Network UUIDs the account can manage |
| server | list of server objects | | yes | Server UUIDs with `uuid` and `storage` (yes/no) attributes |
| storage | list of storage UUIDs or `*` | | yes | Storage UUIDs the account can manage |
| tag | list of tag objects | | yes | Tag-based access with `name` and `storage` (yes/no) attributes |
| ip_filter | list of IP ranges | | no | IP address restrictions (CIDR, ranges, or single IPs) |

### Response

`HTTP/1.1 204 OK`

---

## Delete subaccount

Deletes a subaccount.

### Request

`DELETE /1.3/account/sub/{username}`

Where `username` is the subaccount to delete.

### Response

`HTTP/1.1 204 No Content`

---

## Get monthly billing summary with resource details

Returns billing summary with resource details. API user account must have permission to read billing details.

### Request

`GET /1.3/account/billing/summary/{yearmonth}`

Where `yearmonth` specifies the year and month in the form `YYYY-MM`, e.g. `2024-09`.

### Query parameters

| Parameter | Accepted values | Description |
|-----------|----------------|-------------|
| resource_id | Resource UUID or IP address | If specified, only the details of the given resource are returned. |
| username | Valid username | Retrieves the billing summary of the given account instead of the calling account. Requires Partner API access. |

### Response

```json
{
  "currency": "EUR",
  "managed_databases": {
    "managed_database": {
      "resources": [
        {
          "amount": 15.10223,
          "details": [
            {
              "amount": 15.10223,
              "hours": 420,
              "plan": "1x1xCPU-2GB-25GB",
              "zone": "fi-hel2"
            }
          ],
          "hours": 420,
          "resource_id": "09001a4d-b525-4ab8-835d-000000000000"
        }
      ],
      "total_amount": 15.10223
    },
    "total_amount": 15.10223
  },
  "managed_object_storages": {
    "managed_object_storage": {
      "resources": [
        {
          "amount": 5.03384,
          "details": [
            {
              "amount": 5.03384,
              "billable_size_gib": 500,
              "hours": 420,
              "zone": "fi-hel2"
            }
          ],
          "hours": 420,
          "resource_id": "127ee42a-8304-477c-84e1-000000000000"
        }
      ],
      "total_amount": 5.03384
    },
    "total_amount": 5.03384
  },
  "servers": {
    "server": {
      "resources": [
        {
          "amount": 16.18091,
          "details": [
            {
              "amount": 16.18091,
              "hours": 420,
              "labels": [
                {
                  "key": "test",
                  "value": ""
                }
              ],
              "plan": "2xCPU-4GB",
              "zone": "fi-hel2"
            }
          ],
          "hours": 420,
          "resource_id": "001c2caa-00dc-4eff-9321-000000000000"
        },
        {
          "amount": 16.10386,
          "details": [
            {
              "amount": 16.10386,
              "hours": 418,
              "labels": [
                {
                  "key": "test2",
                  "value": ""
                }
              ],
              "plan": "2xCPU-4GB",
              "zone": "fi-hel2"
            },
            {
              "amount": 0.9288,
              "hours": 12,
              "labels": [
                {
                  "key": "test2",
                  "value": ""
                }
              ],
              "plan": "4xCPU-8GB",
              "zone": "fi-hel2"
            }
          ],
          "hours": 430,
          "resource_id": "0010cf04-3608-4512-b4de-000000000000"
        }
      ],
      "total_amount": 33.21357
    },
    "total_amount": 33.21357
  },
  "total_amount": 53.34964
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| currency | string | Currency code. All amounts are in this currency. |
| amount | float | Charge for a particular resource and per unique metadata. |
| hours | integer | Amount of hours accounted for a particular resource and per unique metadata. |
| details | array | Set of unique metadata objects for a given resource. |
| total_amount | float | Cumulative charge from the beginning of the month. |

### Detail attributes

The `details` array contains metadata objects relevant for billing:

| Attribute | Type | Description |
|-----------|------|-------------|
| plan | string | Resource plan. Included when billing depends on plan type. |
| zone | string | Zone the resource belongs to. New entry emitted on relocation. |
| size | integer | Resource size in GiB. Included when billing depends on size. |
| cores | integer | CPU cores (Flexible Plan servers). |
| memory | integer | Memory in MiB (Flexible Plan servers). |
| firewall | float | Firewall price included in server charge, if applicable. |
| licenses | float | License price per storage included in server charge, if applicable. |
| simple_backup | float | Simple backup price included in server charge, if applicable. |
| billable_size_gib | integer | Billable size in GiB (when indirectly dependent on actual size). |
| labels | array | Resource labels at the time the charge was generated. |

---

## Get monthly billing summary (deprecated)

> **Deprecated.** Use [Get monthly billing summary with resource details](#get-monthly-billing-summary-with-resource-details) instead.

Returns billing summary. API user account must have permission to read billing details.

### Request

`GET /1.3/account/billing_summary/{yearmonth}`

Where `yearmonth` specifies the year and month in the form `YYYY-MM`, e.g. `2019-12`.

### Response

```json
{
  "billing": {
    "currency": "EUR",
    "networks": {
      "bandwidth": {
        "total_amount": 52.136
      },
      "total_amount": 52.136
    },
    "managed_databases": {
      "managed_database": {
        "total_amount": 80.13127
      },
      "total_amount": 80.13127
    },
    "managed_object_storages": {
      "managed_object_storage": {
        "total_amount": 2.2253
      },
      "total_amount": 2.2253
    },
    "servers": {
      "server": {
        "total_amount": 28.05132
      },
      "total_amount": 28.05132
    },
    "storages": {
      "storage": {
        "total_amount": 3.6224
      },
      "total_amount": 3.6224
    },
    "total_amount": 166.16629
  }
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| currency | string | Currency code |
| networks | object | All network resources combined |
| servers | object | All server resources combined |
| storages | object | All storage resources combined |
| managed_databases | object | All managed database resources combined |
| managed_object_storages | object | All managed object storage resources combined |
| total_amount | float | Cumulative amount from the beginning of the month |

---

## Get detailed monthly billing summary (deprecated)

> **Deprecated.** Use [Get monthly billing summary with resource details](#get-monthly-billing-summary-with-resource-details) instead.

Returns detailed billing summary. API user account must have permission to read billing details.

### Request

`GET /1.3/account/billing_summary/{yearmonth}/detailed`

Where `yearmonth` specifies the year and month in the form `YYYY-MM`, e.g. `2019-12`.

### Response

```json
{
  "billing": {
    "currency": "EUR",
    "networks": {
      "bandwidth": {
        "resources": [
          {
            "amount": 52.136,
            "bandwidth_out": 953344,
            "bandwidth_type": "public_ipv4",
            "hours": 0,
            "resource_id": "00ce9363-9065-4c2a-9d66-3b0c762f556b"
          }
        ],
        "total_amount": 52.136
      },
      "total_amount": 52.136
    },
    "managed_databases": {
      "managed_database": {
        "resources": [
          {
            "amount": 15.83323,
            "hours": 114,
            "plan": "2x2xCPU-4GB-50GB",
            "resource_id": "09de34f1-c87e-48f5-9c99-2e1152b953e1"
          },
          {
            "amount": 33.29854,
            "hours": 137,
            "plan": "3x2xCPU-4GB-100GB",
            "resource_id": "09f858ad-aa13-4ab1-98d4-2366b32c0027"
          },
          {
            "amount": 30.9995,
            "hours": 744,
            "plan": "1x1xCPU-2GB-25GB",
            "resource_id": "095d8803-d6e7-4b03-add1-3938a4a32345"
          }
        ],
        "total_amount": 80.13127
      },
      "total_amount": 80.13127
    },
    "managed_object_storages": {
      "managed_object_storage": {
        "resources": [
          {
            "amount": 2.2253,
            "hours": 2,
            "resource_id": "1292388b-0b4a-46e7-ac58-d31c9effed64",
            "size": 500
          }
        ],
        "total_amount": 2.2253
      },
      "total_amount": 2.2253
    },
    "servers": {
      "server": {
        "resources": [
          {
            "amount": 7.66668,
            "firewall": 2.8,
            "hours": 553,
            "plan": "1xCPU-1GB",
            "resource_id": "008c1044-208e-4344-b03b-fc16e9e27a09"
          },
          {
            "amount": 6.16032,
            "hours": 552,
            "plan": "1xCPU-1GB",
            "resource_id": "001b9e08-362f-4004-8318-43d0e61358f6"
          },
          {
            "amount": 14.22432,
            "hours": 552,
            "plan": "1xCPU-1GB",
            "resource_id": "00ce9363-9065-4c2a-9d66-3b0c762f556b"
          }
        ],
        "total_amount": 28.05132
      },
      "total_amount": 28.05132
    },
    "storages": {
      "storage": {
        "resources": [
          {
            "amount": 3.4224,
            "hours": 552,
            "resource_id": "01af6d71-43d4-433c-8342-0c9bc4068dda",
            "size": 20
          }
        ],
        "template": [
          {
            "amount": 0.1000,
            "resource_id": "01bfe607-dbac-44a3-9143-313d9c285e59",
            "size": 10
          }
        ],
        "backup": [
          {
            "amount": 0.1000,
            "resource_id": "01bfe607-76ac-42a3-914a-237dcc285ab2",
            "size": 10
          }
        ],
        "total_amount": 3.6224
      },
      "total_amount": 3.6224
    },
    "total_amount": 166.16629
  }
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| currency | string | Currency code |
| amount | float | Cumulative resource cost from the beginning of the month |
| resource_id | string | Server UUID, storage UUID, or IP address |
| plan | string | Simple plan's server plan type |
| cores | integer | Flexible plan's server cores |
| memory | integer | Flexible plan's server memory in MB |
| size | integer | Storage size in GB |
| bandwidth_out | integer | Outgoing traffic in bytes |
| total_amount | float | Cumulative amount from the beginning of the month |

---

## Get monthly resource billing summary

Returns resource billing summary. API user account must have permission to read billing details.

### Request

`GET /1.3/account/resource_billing_summary/{resource_id}/{yearmonth}`

### Query parameters

| Parameter | Description | Example |
|-----------|-------------|---------|
| resource_id | Server UUID, storage UUID, or IP address | `001a6e07-232e-5928-8318-4ad0e4135853` |
| yearmonth | Year and month in `YYYY-MM` format | `2019-12` |

### Response

```json
{
  "billing": {
    "daily_sums": {
      "2019-12-02": 3.33684,
      "2019-12-03": 52.77344,
      "2019-12-04": 1.07424,
      "2019-12-05": 1.07424,
      "2019-12-06": 1.07424,
      "2019-12-07": 1.07424,
      "2019-12-08": 1.07424,
      "2019-12-09": 1.07424,
      "2019-12-12": 3.22272,
      "2019-12-13": 0.58188
    },
    "details": {
      "hours": 792,
      "plan": "1xCPU-1GB",
      "resource_id": "00ce9363-9065-4c2a-9d66-3b0c762f556b",
      "title": "My Glorious Server",
      "type": "server",
      "zone": "fi-hel1"
    },
    "total_amount": 66.36032
  }
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| type | string | Type of resource |
| currency | string | Currency code |
| zone | string | Zone the resource belongs to |
| hours | integer | Hours billed from the beginning of the month |
| daily_sums | object | Daily resource cost (date → amount) |
| total_amount | float | Cumulative amount from the beginning of the month |

---

## Get network transfer statistics

Returns network transfer statistics. If output type is JSON, maximum window size is 31 days. If output type is CSV, there's no limit to window size. Output type can be changed by setting the `Accept` header as `application/json` or `text/csv`.

### Request

`GET /1.3/account/network_usage/?from={from}&to={to}&accumulate={accumulate}`

### Query parameters

| Parameter | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| from | RFC 3339 timestamp | | yes | First start time timestamp (UTC) |
| to | RFC 3339 timestamp | | yes | Last start time timestamp (UTC) |
| accumulate | hour / day | day | no | Data accumulation period |

### Response

```json
{
  "stats": {
    "stat": [
      {
        "accumulated_quota_bytes": 6352371508013,
        "projected_monthly_quota_bytes": 6362243967561,
        "quota_increase_bytes": 9872459548,
        "sent_bytes": 1254,
        "start_time": "2020-09-30T22:00:00Z",
        "total_sent_bytes": 225149331390
      },
      {
        "accumulated_quota_bytes": 6362243967561,
        "projected_monthly_quota_bytes": 6362243967561,
        "quota_increase_bytes": 9872459548,
        "sent_bytes": 1254,
        "start_time": "2020-09-30T23:00:00Z",
        "total_sent_bytes": 225149332644
      },
      {
        "accumulated_quota_bytes": 9872459548,
        "projected_monthly_quota_bytes": 7345109903712,
        "quota_increase_bytes": 9872459548,
        "sent_bytes": 906,
        "start_time": "2020-10-01T00:00:00Z",
        "total_sent_bytes": 906
      }
    ]
  }
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| accumulated_quota_bytes | integer | Accumulated quota in bytes |
| projected_monthly_quota_bytes | integer | Projected end-of-month quota estimate |
| quota_increase_bytes | integer | Hourly/daily quota incrementation |
| sent_bytes | integer | Bytes sent in the period |
| start_time | string | Start time of the measurement (UTC) |
| total_sent_bytes | integer | Bytes sent so far in the month |

### Error responses

| HTTP status | Error code | Description |
|-------------|------------|-------------|
| 400 Bad Request | INVALID_DATE | TO date is older than FROM date |
| 400 Bad Request | INVALID_TIME_WINDOW_SIZE | Days between FROM and TO can't be over 31 days (JSON output) |

---

## Get resource transfer statistics

Returns resource transfer statistics. If output type is JSON, maximum window size is 31 days. If output type is CSV, there's no limit to window size. Output type can be changed by setting the `Accept` header as `application/json` or `text/csv`.

### Request

`GET /1.3/account/resource_network_usage/?from={from}&to={to}&accumulate={accumulate}&service={service}&zone={zone}&resource_id={resource_id}`

### Query parameters

| Parameter | Accepted values | Default | Required | Description |
|-----------|----------------|---------|----------|-------------|
| from | RFC 3339 timestamp | | yes | Data start timestamp (UTC) |
| to | RFC 3339 timestamp | | yes | Data end timestamp (UTC) |
| zone | Zone name | All zones | no | Filter by zone(s). Can specify multiple. |
| accumulate | hour / day | day | no | Data accumulation period |
| resource_id | Valid UUID | | no | Limit results to a specific resource |
| service | server_public / object_storage_public | All services | no | Limit results to specific service types |

### Response

```json
{
  "stats": {
    "stat": [
      {
        "resource_id": "00777436-a6f8-43af-9daf-89e8e6833a7a",
        "sent_bytes": 10094,
        "service": "server_public",
        "start_time": "2020-09-07T00:00:00Z",
        "zone": "de-fra1"
      },
      {
        "resource_id": "00777436-a6f8-43af-9daf-89e8e6833a7a",
        "sent_bytes": 35174050,
        "service": "server_public",
        "start_time": "2020-09-08T00:00:00Z",
        "zone": "de-fra1"
      },
      {
        "resource_id": "00777436-a6f8-43af-9daf-89e8e6833a7a",
        "sent_bytes": 32790622,
        "service": "server_public",
        "start_time": "2020-09-09T00:00:00Z",
        "zone": "de-fra1"
      }
    ]
  }
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| resource_id | string | Resource UUID |
| sent_bytes | integer | Total bytes sent |
| service | string | Service type |
| start_time | string | Start time of the measurement (UTC) |
| zone | string | Zone name |

### Error responses

| HTTP status | Error code | Description |
|-------------|------------|-------------|
| 400 Bad Request | INVALID_DATE | TO date is older than FROM date |
| 400 Bad Request | INVALID_TIME_WINDOW_SIZE | Days between FROM and TO can't be over 31 days (JSON output) |

---

## Get current network usage

Returns the latest network transfer pool usage report.

### Request

`GET /1.3/account/current_network_usage`

### Response

```json
{
  "stats": {
    "current_network_usage": {
      "accumulated_quota_bytes": 3546328504778,
      "hourly_quota_increase_bytes": 10577635230,
      "projected_monthly_quota_bytes": 7777382596778,
      "total_sent_bytes": 13682943146,
      "updated": "2020-10-15T07:00:00Z"
    }
  }
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| accumulated_quota_bytes | integer | Accumulated quota in bytes |
| hourly_quota_increase_bytes | integer | Hourly quota incrementation |
| projected_monthly_quota_bytes | integer | Projected end-of-month quota estimate |
| total_sent_bytes | integer | Bytes sent so far in the current month |
| updated | string | Last update time of usage data (UTC) |
