# Managed Kubernetes

> Source: https://developers.upcloud.com/1.3/20-managed-kubernetes

Kubernetes is a container orchestration system for automating, managing and scaling software deployment. Managed Kubernetes allows you to easily create Kubernetes clusters without having to take care about low level details.

---

## List plans

Returns a list of available Kubernetes cluster plans.

### Request

```
GET /1.3/kubernetes/plans HTTP/1.1
```

### Response `200 OK`

```json
[
  {
    "name": "dev-md",
    "max_nodes": 30,
    "deprecated": false
  },
  {
    "name": "prod-md",
    "max_nodes": 120,
    "deprecated": false
  }
]
```

### Plan attributes

| Attribute    | Type    | Description                              |
|--------------|---------|------------------------------------------|
| `name`       | string  | Plan name identifier.                    |
| `max_nodes`  | integer | Maximum number of nodes allowed.         |
| `deprecated` | boolean | Whether this plan is deprecated.         |

---

## Create cluster

Creates a new Kubernetes cluster.

### Request

```
POST /1.3/kubernetes HTTP/1.1
```

```json
{
  "control_plane_ip_filter": [],
  "name": "example-cluster-1",
  "network": "03a85f64-5717-4562-b3fc-2c963f66afa6",
  "network_cidr": "172.16.0.0/24",
  "node_groups": [
    {
      "count": 3,
      "labels": [
        { "key": "team", "value": "development" }
      ],
      "name": "example-node-group-1",
      "plan": "4xCPU-8GB",
      "ssh_keys": [
        "ssh-rsa AAAAB3NzaC1yc2EAA[...]ptshi44x user@example.com",
        "ssh-dss AAAAB3NzaC1kc3MAA[...]VHRzAA== user@example.com"
      ]
    }
  ],
  "plan": "dev-md",
  "private_node_groups": false,
  "zone": "fi-hel2",
  "labels": [
    { "key": "env", "value": "staging" },
    { "key": "app", "value": "cats-dating-app" }
  ]
}
```

### Attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `control_plane_ip_filter` | Array of 0+ IP addresses/CIDRs | no | IP addresses or CIDR ranges allowed to access the cluster control plane. Defaults to `null` (no filtering). Use `["0.0.0.0/0"]` to allow all, `[]` to deny all. Does not restrict access to node groups or exposed services. |
| `name` | 3-55 lowercase letters, numbers & `-`. Cannot start/end with `-` | yes | Cluster name, must be unique within the account. |
| `network` | Valid network UUID | yes | Network UUID where node groups will be provisioned. Must reside in the cluster zone. |
| `network_cidr` | IP range in CIDR format | yes | IP range of the given network. |
| `zone` | Valid zone identifier, e.g. `fi-hel2` | yes | Zone in which the cluster will be hosted. |
| `node_groups` | Array of 0+ node group objects | no | Initial node groups. See [Node groups](#create-node-group). |
| `plan` | Plan name string | no | Control plane plan. Defaults to `dev-md`. See [List plans](#list-plans). |
| `private_node_groups` | boolean | no | Enable private node groups. Defaults to `false`. Requires a network routed through NAT gateway. |
| `labels` | Array of [labels](#label) | no | Labels on the cluster API resource. Not propagated to resources inside the cluster. |
| `storage_encryption` | e.g. `data-at-rest` | no | Default storage encryption strategy for all nodes in the cluster. |

### Response `201 Created`

```json
{
  "control_plane_ip_filter": [],
  "labels": [
    { "key": "env", "value": "staging" },
    { "key": "app", "value": "cats-dating-app" }
  ],
  "name": "example-cluster-1",
  "network": "03a85f64-5717-4562-b3fc-2c963f66afa6",
  "network_cidr": "172.16.0.0/24",
  "node_groups": [
    {
      "count": 3,
      "kubelet_args": [],
      "labels": [
        { "key": "team", "value": "development" }
      ],
      "name": "example-node-group-1",
      "plan": "4xCPU-8GB",
      "ssh_keys": [
        "ssh-rsa AAAAB3NzaC1yc2EAA[...]ptshi44x user@example.com",
        "ssh-dss AAAAB3NzaC1kc3MAA[...]VHRzAA== user@example.com"
      ],
      "state": "pending",
      "storage": "01000000-0000-4000-8000-000160010100",
      "taints": [],
      "anti_affinity": false,
      "utility_network_access": true
    }
  ],
  "state": "pending",
  "plan": "dev-md",
  "private_node_groups": false,
  "uuid": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
  "zone": "fi-hel2"
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 402 Payment required | `INSUFFICIENT_CREDITS` | Not enough credits. |
| 400 Bad request | `INVALID_REQUEST` | Validation error. |
| 422 Unprocessable entity | `INVALID_REQUEST` | Validation error. |

---

## List clusters

Returns a list of Kubernetes clusters.

### Request

```
GET /1.3/kubernetes HTTP/1.1
```

### Response `200 OK`

```json
[
  {
    "control_plane_ip_filter": [],
    "labels": [
      { "key": "env", "value": "staging" },
      { "key": "app", "value": "cats-dating-app" }
    ],
    "name": "example-cluster-1",
    "network": "03a85f64-5717-4562-b3fc-2c963f66afa6",
    "network_cidr": "172.16.0.0/24",
    "node_groups": [
      {
        "count": 3,
        "kubelet_args": [],
        "labels": [
          { "key": "team", "value": "development" }
        ],
        "name": "example-node-group-1",
        "plan": "4xCPU-8GB",
        "ssh_keys": [
          "ssh-rsa AAAAB3NzaC1yc2EAA[...]ptshi44x user@example.com",
          "ssh-dss AAAAB3NzaC1kc3MAA[...]VHRzAA== user@example.com"
        ],
        "state": "running",
        "storage": "01000000-0000-4000-8000-000160010100",
        "taints": [],
        "anti_affinity": false,
        "utility_network_access": true
      }
    ],
    "state": "running",
    "plan": "dev-md",
    "private_node_groups": false,
    "uuid": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "zone": "fi-hel2"
  },
  {
    "control_plane_ip_filter": ["0.0.0.0/0"],
    "name": "example-cluster-2",
    "network": "03a85f64-5717-4562-b3fc-2c963f66afa6",
    "network_cidr": "172.17.0.0/24",
    "node_groups": [],
    "state": "pending",
    "plan": "prod-md",
    "private_node_groups": true,
    "uuid": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "zone": "de-fra1"
  }
]
```

---

## Get cluster details

Returns Kubernetes cluster details by given `{uuid}`.

### Request

```
GET /1.3/kubernetes/{uuid} HTTP/1.1
```

### Response `200 OK`

```json
{
  "control_plane_ip_filter": ["0.0.0.0/0"],
  "labels": [
    { "key": "env", "value": "staging" },
    { "key": "app", "value": "cats-dating-app" }
  ],
  "name": "example-cluster-1",
  "network": "03a85f64-5717-4562-b3fc-2c963f66afa6",
  "network_cidr": "172.16.0.0/24",
  "node_groups": [
    {
      "count": 3,
      "kubelet_args": [],
      "labels": [
        { "key": "team", "value": "development" }
      ],
      "name": "example-node-group-1",
      "plan": "4xCPU-8GB",
      "ssh_keys": [
        "ssh-rsa AAAAB3NzaC1yc2EAA[...]ptshi44x user@example.com",
        "ssh-dss AAAAB3NzaC1kc3MAA[...]VHRzAA== user@example.com"
      ],
      "state": "running",
      "storage": "01000000-0000-4000-8000-000160010100",
      "taints": [],
      "anti_affinity": false,
      "utility_network_access": true
    }
  ],
  "state": "running",
  "plan": "dev-md",
  "private_node_groups": false,
  "uuid": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
  "zone": "fi-hel2"
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 404 Not Found | `RESOURCE_NOT_FOUND` | Cluster not found. |

---

## Modify cluster

Modifies an existing Kubernetes cluster by given `{uuid}`.

### Request

```
PATCH /1.3/kubernetes/{uuid} HTTP/1.1
```

```json
{
  "control_plane_ip_filter": ["0.0.0.0/0"],
  "labels": [
    { "key": "env", "value": "production" },
    { "key": "app", "value": "dogs-dating-app" }
  ]
}
```

### Attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `control_plane_ip_filter` | Array of 0+ IP addresses/CIDRs. `null` not accepted in PATCH. | yes | IP addresses or CIDR ranges allowed to access the control plane. Use `["0.0.0.0/0"]` to allow all, `[]` to deny all. |
| `labels` | Array of [labels](#label) | no | Overwrites existing labels. Send `null` or empty array to delete all. |

### Response `200 OK`

```json
{
  "control_plane_ip_filter": ["0.0.0.0/0"],
  "labels": [
    { "key": "env", "value": "production" },
    { "key": "app", "value": "dogs-dating-app" }
  ],
  "name": "example-cluster-1",
  "network": "03a85f64-5717-4562-b3fc-2c963f66afa6",
  "network_cidr": "172.16.0.0/24",
  "node_groups": [
    {
      "count": 3,
      "kubelet_args": [],
      "labels": [
        { "key": "team", "value": "development" }
      ],
      "name": "example-node-group-1",
      "plan": "4xCPU-8GB",
      "ssh_keys": [
        "ssh-rsa AAAAB3NzaC1yc2EAA[...]ptshi44x user@example.com",
        "ssh-dss AAAAB3NzaC1kc3MAA[...]VHRzAA== user@example.com"
      ],
      "state": "running",
      "storage": "01000000-0000-4000-8000-000160010100",
      "taints": [],
      "anti_affinity": false,
      "utility_network_access": true
    }
  ],
  "state": "running",
  "plan": "dev-md",
  "private_node_groups": false,
  "uuid": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
  "zone": "fi-hel2"
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 Bad Request | `INVALID_REQUEST` | Validation error. |
| 404 Not Found | `RESOURCE_NOT_FOUND` | Cluster or node group not found. |
| 422 Unprocessable entity | `INVALID_REQUEST` | Validation error. |

---

## Get available upgrades

Returns a list of available versions the cluster can be upgraded to.

### Request

```
GET /1.3/kubernetes/{uuid}/available-upgrades HTTP/1.1
```

### Response `200 OK`

```json
{
  "versions": ["1.31"]
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 404 Not Found | `RESOURCE_NOT_FOUND` | Cluster not found. |

---

## Upgrade cluster

Upgrades an existing Kubernetes cluster to a specific version.

### Request

```
POST /1.3/kubernetes/{uuid}/upgrade HTTP/1.1
```

```json
{
  "version": "1.31",
  "strategy": {
    "type": "manual"
  }
}
```

### Attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `version` | Version string | yes | Target cluster version. |
| `strategy` | [Strategy](#upgrade-strategy) object | no | Node group upgrade strategy. Defaults to `manual`. |

### Response `202 Accepted`

```json
{
  "version": "1.31",
  "strategy": {
    "type": "manual"
  }
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 Bad Request | `INVALID_REQUEST` | Validation error. |
| 404 Not Found | `RESOURCE_NOT_FOUND` | Cluster or node group not found. |
| 422 Unprocessable entity | `INVALID_REQUEST` | Validation error. |

---

## Get kubeconfig

Returns the kubeconfig YAML for a cluster by given `{uuid}`.

### Request

```
GET /1.3/kubernetes/{uuid}/kubeconfig HTTP/1.1
```

### Response `200 OK`

```json
{
  "kubeconfig": "apiVersion: v1\nclusters:\n  - cluster:\n      certificate-authority-data: BASE64\n      server: https://server:6443\n    name: example-cluster-1\ncontexts:\n  - context:\n      cluster: example-cluster-1\n      user: example-cluster-1-admin\n    name: example-cluster-1-admin@example-cluster-1\ncurrent-context: example-cluster-1-admin@example-cluster-1\nkind: Config\npreferences: {}\nusers:\n  - name: example-cluster-1-admin\n    user:\n      client-certificate-data: BASE64\n      client-key-data: BASE64\n"
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 404 Not Found | `RESOURCE_NOT_FOUND` | Cluster not found. |

---

## Delete cluster

Deletes an existing Kubernetes cluster by given `{uuid}`.

### Request

```
DELETE /1.3/kubernetes/{uuid} HTTP/1.1
```

### Response `202 Accepted`

*(empty body)*

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 404 Not Found | `RESOURCE_NOT_FOUND` | Cluster not found. |

---

## Create node group

Creates a new node group in an existing Kubernetes cluster identified by `{uuid}`.

### Request

```
POST /1.3/kubernetes/{uuid}/node-groups HTTP/1.1
```

```json
{
  "anti_affinity": true,
  "count": 9,
  "kubelet_args": [
    { "key": "log-flush-frequency", "value": "5s" }
  ],
  "labels": [
    { "key": "team", "value": "qa" }
  ],
  "name": "example-node-group-2",
  "plan": "4xCPU-8GB",
  "ssh_keys": [
    "ssh-rsa AAAAB3NzaC1yc2EAA[...]ptshi44x user@example.com",
    "ssh-dss AAAAB3NzaC1kc3MAA[...]VHRzAA== user@example.com"
  ],
  "storage": "01000000-0000-4000-8000-000160010100",
  "taints": [
    { "effect": "NoSchedule", "key": "team", "value": "qa" }
  ],
  "utility_network_access": true
}
```

### Attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `count` | >= 0 | yes | Number of nodes. |
| `name` | 1-63 lowercase letters, numbers & `-`. Cannot start/end with `-` | yes | Node group name, must be unique within the cluster. |
| `plan` | Valid plan identifier | yes | Server plan used for each node. |
| `anti_affinity` | boolean | no | Nodes avoid underlying hosts that already have nodes from the same group. |
| `kubelet_args` | Array of 0+ kubelet argument objects | no | Extra kubelet arguments. Keys without `--` prefix. See [Kubelet argument](#kubelet-argument). |
| `labels` | Array of 0+ label objects | no | See [Label](#label). |
| `ssh_keys` | Array of 0-32 strings | no | Public SSH keys for remote node access. |
| `storage` | Valid storage UUID | no | Storage template for node provisioning. |
| `taints` | Array of 0+ taint objects | no | See [Taint](#taint). |
| `utility_network_access` | boolean | no | Create utility network interfaces for each node. Defaults to `true`. |
| `custom_plan` | [Custom plan](#custom-plan) object | no | Required when `plan` is `"custom"`. |
| `cloud_native_plan` | [Cloud native plan](#cloud-native-plan) object | no | Required when a cloud native plan is used (e.g. `"CLOUDNATIVE-48xCPU-384GB"`). |
| `gpu_plan` | [GPU plan](#gpu-plan) object | no | Required when a GPU plan is used (e.g. `"GPU-12xCPU-128GB-1xL40s"`). |
| `storage_encryption` | e.g. `data-at-rest` | no | Storage encryption strategy. Use `none` to override cluster-level encryption. |

### Response `201 Created`

```json
{
  "anti_affinity": true,
  "count": 9,
  "kubelet_args": [
    { "key": "log-flush-frequency", "value": "5s" }
  ],
  "labels": [
    { "key": "team", "value": "qa" }
  ],
  "name": "example-node-group-2",
  "plan": "4xCPU-8GB",
  "ssh_keys": [
    "ssh-rsa AAAAB3NzaC1yc2EAA[...]ptshi44x user@example.com",
    "ssh-dss AAAAB3NzaC1kc3MAA[...]VHRzAA== user@example.com"
  ],
  "state": "pending",
  "storage": "01000000-0000-4000-8000-000160010100",
  "taints": [
    { "effect": "NoSchedule", "key": "team", "value": "qa" }
  ],
  "utility_network_access": true
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 402 Payment required | `INSUFFICIENT_CREDITS` | Not enough credits. |
| 400 Bad request | `INVALID_REQUEST` | Validation error. |
| 404 Not Found | `RESOURCE_NOT_FOUND` | Cluster not found. |
| 422 Unprocessable entity | `INVALID_REQUEST` | Validation error. |

---

## List node groups

Returns node groups of an existing Kubernetes cluster identified by `{uuid}`.

A node group is a uniform set of worker nodes attached to a cluster.

### Request

```
GET /1.3/kubernetes/{uuid}/node-groups HTTP/1.1
```

### Response `200 OK`

```json
[
  {
    "anti_affinity": false,
    "count": 3,
    "kubelet_args": [],
    "labels": [
      { "key": "team", "value": "development" }
    ],
    "name": "example-node-group-1",
    "plan": "4xCPU-8GB",
    "ssh_keys": [
      "ssh-rsa AAAAB3NzaC1yc2EAA[...]ptshi44x user@example.com",
      "ssh-dss AAAAB3NzaC1kc3MAA[...]VHRzAA== user@example.com"
    ],
    "state": "running",
    "storage": "01000000-0000-4000-8000-000160010100",
    "taints": [],
    "utility_network_access": true
  },
  {
    "anti_affinity": true,
    "count": 9,
    "kubelet_args": [
      { "key": "log-flush-frequency", "value": "5s" }
    ],
    "labels": [
      { "key": "team", "value": "qa" }
    ],
    "name": "example-node-group-2",
    "plan": "4xCPU-8GB",
    "ssh_keys": [
      "ssh-rsa AAAAB3NzaC1yc2EAA[...]ptshi44x user@example.com",
      "ssh-dss AAAAB3NzaC1kc3MAA[...]VHRzAA== user@example.com"
    ],
    "state": "running",
    "storage": "01000000-0000-4000-8000-000160010100",
    "taints": [
      { "effect": "NoSchedule", "key": "team", "value": "qa" }
    ],
    "utility_network_access": false
  }
]
```

---

## Get node group details

Returns node group details by given `{node_group_name}`. Cluster is identified by `{uuid}`.

### Request

```
GET /1.3/kubernetes/{uuid}/node-groups/{node_group_name} HTTP/1.1
```

### Response `200 OK`

```json
{
  "anti_affinity_status": true,
  "count": 2,
  "kubelet_args": [
    { "key": "log-flush-frequency", "value": "5s" }
  ],
  "labels": [
    { "key": "team", "value": "qa" }
  ],
  "name": "example-node-group-2",
  "plan": "4xCPU-8GB",
  "ssh_keys": [
    "ssh-rsa AAAAB3NzaC1yc2EAA[...]ptshi44x user@example.com",
    "ssh-dss AAAAB3NzaC1kc3MAA[...]VHRzAA== user@example.com"
  ],
  "state": "running",
  "storage": "01000000-0000-4000-8000-000160010100",
  "taints": [
    { "effect": "NoSchedule", "key": "team", "value": "qa" }
  ],
  "utility_network_access": true,
  "nodes": [
    {
      "uuid": "564c8cd9-bc16-4f89-a328-daf3f138cc44",
      "name": "example-fzwwt-8smpz",
      "state": "running",
      "kubelet_version": "1.30"
    },
    {
      "uuid": "2e8039c6-95d4-41a9-92ca-46808f560abb",
      "name": "example-fzwwt-zmjfs",
      "state": "running",
      "kubelet_version": "1.30"
    }
  ]
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 404 Not Found | `RESOURCE_NOT_FOUND` | Cluster or node group not found. |

---

## Modify node group

Modifies an existing node group by given cluster `{uuid}` and `{node_group_name}`.

### Request

```
PATCH /1.3/kubernetes/{uuid}/node-groups/{node_group_name} HTTP/1.1
```

```json
{
  "count": 15
}
```

### Attributes

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `count` | >= 0 | yes | Number of nodes. |

### Response `200 OK`

```json
{
  "anti_affinity": true,
  "count": 15,
  "kubelet_args": [
    { "key": "log-flush-frequency", "value": "5s" }
  ],
  "labels": [
    { "key": "team", "value": "qa" }
  ],
  "name": "example-node-group-2",
  "plan": "4xCPU-8GB",
  "ssh_keys": [
    "ssh-rsa AAAAB3NzaC1yc2EAA[...]ptshi44x user@example.com",
    "ssh-dss AAAAB3NzaC1kc3MAA[...]VHRzAA== user@example.com"
  ],
  "state": "running",
  "storage": "01000000-0000-4000-8000-000160010100",
  "taints": [
    { "effect": "NoSchedule", "key": "team", "value": "qa" }
  ],
  "utility_network_access": true
}
```

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 400 Bad Request | `INVALID_REQUEST` | Validation error. |
| 402 Payment required | `INSUFFICIENT_CREDITS` | Not enough credits. |
| 404 Not Found | `RESOURCE_NOT_FOUND` | Cluster or node group not found. |
| 422 Unprocessable entity | `INVALID_REQUEST` | Validation error. |

---

## Delete node group

Deletes an existing node group by given cluster `{uuid}` and `{node_group_name}`.

### Request

```
DELETE /1.3/kubernetes/{uuid}/node-groups/{node_group_name} HTTP/1.1
```

### Response `202 Accepted`

*(empty body)*

### Error responses

| HTTP status | Error code | Description |
|-------------|-----------|-------------|
| 404 Not Found | `RESOURCE_NOT_FOUND` | Cluster or node group not found. |

---

## Shared types

### Kubelet argument

A key-value pair representing a kubelet command-line argument.

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `key` | string | yes | Kubelet argument key, without `--` prefix. |
| `value` | string | yes | Argument value. |

### Label

A key-value pair for classifying the resource.

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `key` | 2-32 printable ASCII chars (0x20-0x7E), must not start with `_` | yes | Label key. |
| `value` | 0-63 letters, numbers, `-` & `_`. Cannot start/end with `-` or `_` | yes | Label value. |

### Taint

Allows a node to repel a set of pods. See [Taints and Tolerations](https://kubernetes.io/docs/concepts/scheduling-eviction/taint-and-toleration/).

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `effect` | `NoExecute` / `NoSchedule` / `PreferNoSchedule` | yes | Taint effect. |
| `key` | 0-255 characters | yes | Taint key. |
| `value` | 0-255 characters | yes | Taint value. |

### State

Indicates the current operational state of a cluster or node group. Managed by the system.

| State | Description |
|-------|-------------|
| `pending` | Newly created resource or reconfiguration started. |
| `running` | Resource is up and running. |
| `terminating` | Termination is in progress. |
| `failed` | Internal system failure. |
| `unknown` | Resource state is unknown. |

### Node

Status information about a single worker node.

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `uuid` | string | yes | Server UUID of the node. |
| `name` | string | yes | Name of the node. |
| `state` | string | yes | Node state. |
| `kubelet_version` | string | no | Kubelet version running on the node (experimental, default storage templates only). |

### Upgrade strategy

Defines how node groups are upgraded to match the cluster version.

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `type` | `manual` / `rolling-update` | yes | Upgrade strategy type. |

#### Strategy types

| Type | Description |
|------|-------------|
| `manual` | Upgrade nodes manually by re-creating node groups. |
| `rolling-update` | Nodes are automatically upgraded one at a time per node group. |

### Custom plan

Specify exact resource requirements for nodes in a node group.

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `cores` | positive integer | yes | Number of CPU cores. |
| `memory` | positive integer | yes | Amount of memory in megabytes. |
| `storage_size` | positive integer | yes | Storage device size in gigabytes. |
| `storage_tier` | `maxiops` / `standard` / `archive` | no | Storage tier. |

### Cloud native plan

Storage configuration for nodes using cloud native plans (e.g. `CLOUDNATIVE-48xCPU-384GB`).

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `storage_size` | positive integer | yes | Storage device size in gigabytes. |
| `storage_tier` | `maxiops` / `standard` / `archive` | no | Storage tier. |

### GPU plan

Storage configuration for nodes using GPU plans (e.g. `GPU-12xCPU-128GB-1xL40s`).

| Attribute | Accepted value | Required | Description |
|-----------|---------------|----------|-------------|
| `storage_size` | positive integer | yes | Storage device size in gigabytes. |
| `storage_tier` | `maxiops` / `standard` / `archive` | no | Storage tier. |
