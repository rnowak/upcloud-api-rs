# Simple Plans

> Source: https://developers.upcloud.com/1.3/7-plans

Simple plans are preconfigured server configurations with fixed price. Each plan includes a certain number of CPU cores, memory amount, storage and transfer quota in addition to one public IPv4 address (utility IPv4 addresses and public IPv6 addresses are currently free of charge, so they are not included in plan).

In addition to plans available in a zone, there is always possibility for free scaling of CPU, memory and disk resources. In such cases the plan used by a server will be reported as `"custom"` by the API. It is possible to change an existing `"custom"` server to a fixed plan provided that the CPU cores and memory amount match the selected plan.

GPU plans also have additional `gpu_amount` and `gpu_model` attributes, which indicate the number of GPUs and their model included in the plan.

If a server's storage or network resources do not fit a plan's specification, they are billed on an hourly basis. Same applies to excess network traffic. Those storage device(s) and IP addresses included in the fixed plan price have a `part_of_plan` attribute with value `"yes"` in the server details response.

## List available plans

Returns a list of available plans.

In addition to listed plans, a server can be freely configured with a `"custom"` plan which allows freely scalable CPU cores, memory amount and storage resources.

All plans are available in all zones but pricing may differ. See [List prices](https://developers.upcloud.com/1.3/4-pricing/#list-prices) for how to obtain the current price list.

> **Note:** Custom plans are available for new customers through contact with sales.

### Request

`GET /1.3/plan`

### Response

```json
{
  "plans" : {
    "plan" : [
      {
        "core_number" : 1,
        "memory_amount" : 2048,
        "name" : "1xCPU-2GB",
        "public_traffic_out" : 2048,
        "storage_size" : 50,
        "storage_tier" : "maxiops"
      },
      {
        "core_number" : 2,
        "memory_amount" : 4096,
        "name" : "2xCPU-4GB",
        "public_traffic_out" : 4096,
        "storage_size" : 80,
        "storage_tier" : "maxiops"
      },
      {
        "core_number": 32,
        "memory_amount": 393216,
        "name": "GPU-32xCPU-384GB-2xL40S",
        "public_traffic_out": 44,
        "gpu_amount": 2,
        "storage_size": 0,
        "storage_tier": null,
        "gpu_model": "NVIDIA L40S"
      }
    ]
  }
}
```

### Response attributes

| Attribute | Type | Description |
|-----------|------|-------------|
| `name` | String | Display name of the plan (e.g. `1xCPU-2GB`). Used when creating or modifying a server. |
| `core_number` | Number | Number of CPU cores included in the plan. |
| `memory_amount` | Number | Amount of RAM in MiB included in the plan. |
| `storage_size` | Number | Amount of storage in GiB included in the plan. `0` when the plan does not include storage (e.g. GPU plans). |
| `storage_tier` | String or `null` | Storage tier used by the plan (e.g. `"maxiops"`). `null` when the plan does not include storage. |
| `public_traffic_out` | Number | Amount of outbound transfer included in the plan, in GiB. Excess traffic is billed hourly. |
| `gpu_amount` | Number (optional) | Number of GPUs included in the plan. Only present on GPU plans. |
| `gpu_model` | String (optional) | GPU model identifier (e.g. `"NVIDIA L40S"`). Only present on GPU plans. |
