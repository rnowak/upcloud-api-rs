use super::*;
// Source: https://developers.upcloud.com/1.3/36-managed-kubernetes/#list-plans
const K8S_PLAN_LIST_JSON: &str = r#"
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
"#;

#[test]
fn deserialize_k8s_plan_list() {
    let plans: Vec<Plan> = serde_json::from_str(K8S_PLAN_LIST_JSON).unwrap();
    assert_eq!(plans.len(), 2);
    assert_eq!(plans[0].name, "dev-md");
    assert_eq!(plans[0].max_nodes, Some(30));
    assert_eq!(plans[0].deprecated, Some(false));
    assert!(plans[0].description.is_none());
    assert!(plans[0].server_plan.is_none());
    assert_eq!(plans[1].name, "prod-md");
    assert_eq!(plans[1].max_nodes, Some(120));
    assert_eq!(plans[1].deprecated, Some(false));
    assert!(plans[1].description.is_none());
    assert!(plans[1].server_plan.is_none());
}
// Source: https://developers.upcloud.com/1.3/36-managed-kubernetes/#list-clusters
const K8S_CLUSTER_LIST_JSON: &str = r#"
    [
        {
            "control_plane_ip_filter": [],
            "labels": [
                {
                    "key": "env",
                    "value": "staging"
                },
                {
                    "key": "app",
                    "value": "cats-dating-app"
                }
            ],
            "name": "example-cluster-1",
            "network": "03a85f64-5717-4562-b3fc-2c963f66afa6",
            "network_cidr": "172.16.0.0/24",
            "node_groups": [
                {
                    "count": 3,
                    "kubelet_args": [],
                    "labels": [
                        {
                            "key": "team",
                            "value": "development"
                        }
                    ],
                    "name": "example-node-group-1",
                    "plan": "4xCPU-8GB",
                    "ssh_keys": [
                        "ssh-rsa AAAAB3NzaC1yc2EAA"
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
            "control_plane_ip_filter": [
                "0.0.0.0/0"
            ],
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
"#;

#[test]
fn deserialize_k8s_cluster_list() {
    let clusters: Vec<Cluster> = serde_json::from_str(K8S_CLUSTER_LIST_JSON).unwrap();
    assert_eq!(clusters.len(), 2);

    let c1 = &clusters[0];
    assert_eq!(c1.name, "example-cluster-1");
    assert_eq!(c1.state, ClusterState::Running);
    assert_eq!(c1.zone, "fi-hel2");
    assert_eq!(c1.plan.as_deref(), Some("dev-md"));
    assert_eq!(c1.private_node_groups, Some(false));
    assert_eq!(c1.control_plane_ip_filter.as_ref().unwrap().len(), 0);
    assert_eq!(
        c1.network.as_deref(),
        Some("03a85f64-5717-4562-b3fc-2c963f66afa6")
    );
    assert_eq!(c1.network_cidr.as_deref(), Some("172.16.0.0/24"));
    assert_eq!(c1.uuid, "3fa85f64-5717-4562-b3fc-2c963f66afa6");
    assert!(c1.version.is_none());
    assert!(c1.created_at.is_none());

    let labels = c1.labels.as_ref().unwrap();
    assert_eq!(labels.len(), 2);
    assert_eq!(labels[0].key, "env");
    assert_eq!(labels[0].value, "staging");
    assert_eq!(labels[1].key, "app");
    assert_eq!(labels[1].value, "cats-dating-app");

    let groups = c1.node_groups.as_ref().unwrap();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].name, "example-node-group-1");
    assert_eq!(groups[0].count, 3);
    assert_eq!(groups[0].state, Some(NodeGroupState::Running));
    assert_eq!(groups[0].anti_affinity, Some(false));
    assert_eq!(groups[0].plan, "4xCPU-8GB");
    assert!(groups[0].kubelet_args.as_ref().unwrap().is_empty());
    let g0_labels = groups[0].labels.as_ref().unwrap();
    assert_eq!(g0_labels.len(), 1);
    assert_eq!(g0_labels[0].key, "team");
    assert_eq!(g0_labels[0].value, "development");
    let g0_ssh = groups[0].ssh_keys.as_ref().unwrap();
    assert_eq!(g0_ssh.len(), 1);
    assert_eq!(g0_ssh[0], "ssh-rsa AAAAB3NzaC1yc2EAA");
    assert_eq!(
        groups[0].storage.as_deref(),
        Some("01000000-0000-4000-8000-000160010100")
    );
    assert!(groups[0].taints.as_ref().unwrap().is_empty());
    assert_eq!(groups[0].utility_network_access, Some(true));
    assert!(groups[0].anti_affinity_status.is_none());
    assert!(groups[0].nodes.is_none());

    let c2 = &clusters[1];
    assert_eq!(c2.name, "example-cluster-2");
    assert_eq!(c2.state, ClusterState::Pending);
    assert_eq!(c2.zone, "de-fra1");
    assert_eq!(c2.plan.as_deref(), Some("prod-md"));
    assert_eq!(c2.private_node_groups, Some(true));
    assert_eq!(c2.uuid, "3fa85f64-5717-4562-b3fc-2c963f66afa6");
    assert_eq!(
        c2.network.as_deref(),
        Some("03a85f64-5717-4562-b3fc-2c963f66afa6")
    );
    assert_eq!(c2.network_cidr.as_deref(), Some("172.17.0.0/24"));
    let c2_filters = c2.control_plane_ip_filter.as_ref().unwrap();
    assert_eq!(c2_filters.len(), 1);
    assert_eq!(c2_filters[0], "0.0.0.0/0");
    assert!(c2.node_groups.as_ref().unwrap().is_empty());
    assert!(c2.labels.is_none());
    assert!(c2.version.is_none());
    assert!(c2.created_at.is_none());
}
// Source: https://developers.upcloud.com/1.3/36-managed-kubernetes/#get-cluster-details
const K8S_CLUSTER_DETAIL_JSON: &str = r#"
    {
        "control_plane_ip_filter": [
            "0.0.0.0/0"
        ],
        "labels": [
            {
                "key": "env",
                "value": "staging"
            },
            {
                "key": "app",
                "value": "cats-dating-app"
            }
        ],
        "name": "example-cluster-1",
        "network": "03a85f64-5717-4562-b3fc-2c963f66afa6",
        "network_cidr": "172.16.0.0/24",
        "node_groups": [
            {
                "count": 3,
                "kubelet_args": [],
                "labels": [
                    {
                        "key": "team",
                        "value": "development"
                    }
                ],
                "name": "example-node-group-1",
                "plan": "4xCPU-8GB",
                "ssh_keys": [
                    "ssh-rsa AAAAB3NzaC1yc2EAA"
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
"#;

#[test]
fn deserialize_k8s_cluster_detail() {
    let cluster: Cluster = serde_json::from_str(K8S_CLUSTER_DETAIL_JSON).unwrap();
    assert_eq!(cluster.uuid, "3fa85f64-5717-4562-b3fc-2c963f66afa6");
    assert_eq!(cluster.name, "example-cluster-1");
    assert_eq!(cluster.state, ClusterState::Running);
    assert_eq!(cluster.zone, "fi-hel2");
    assert_eq!(cluster.plan.as_deref(), Some("dev-md"));
    assert_eq!(cluster.private_node_groups, Some(false));
    assert_eq!(
        cluster.network.as_deref(),
        Some("03a85f64-5717-4562-b3fc-2c963f66afa6")
    );
    assert_eq!(cluster.network_cidr.as_deref(), Some("172.16.0.0/24"));
    assert_eq!(
        cluster.control_plane_ip_filter.as_ref().unwrap(),
        &["0.0.0.0/0"]
    );
    assert!(cluster.version.is_none());
    assert!(cluster.created_at.is_none());

    let labels = cluster.labels.as_ref().unwrap();
    assert_eq!(labels.len(), 2);
    assert_eq!(labels[0].key, "env");
    assert_eq!(labels[0].value, "staging");
    assert_eq!(labels[1].key, "app");
    assert_eq!(labels[1].value, "cats-dating-app");

    let groups = cluster.node_groups.as_ref().unwrap();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].name, "example-node-group-1");
    assert_eq!(groups[0].plan, "4xCPU-8GB");
    assert_eq!(groups[0].count, 3);
    assert_eq!(groups[0].state, Some(NodeGroupState::Running));
    assert_eq!(groups[0].anti_affinity, Some(false));
    assert!(groups[0].kubelet_args.as_ref().unwrap().is_empty());
    let g0_labels = groups[0].labels.as_ref().unwrap();
    assert_eq!(g0_labels.len(), 1);
    assert_eq!(g0_labels[0].key, "team");
    assert_eq!(g0_labels[0].value, "development");
    let g0_ssh = groups[0].ssh_keys.as_ref().unwrap();
    assert_eq!(g0_ssh.len(), 1);
    assert_eq!(g0_ssh[0], "ssh-rsa AAAAB3NzaC1yc2EAA");
    assert_eq!(
        groups[0].storage.as_deref(),
        Some("01000000-0000-4000-8000-000160010100")
    );
    assert!(groups[0].taints.as_ref().unwrap().is_empty());
    assert_eq!(groups[0].utility_network_access, Some(true));
    assert!(groups[0].anti_affinity_status.is_none());
    assert!(groups[0].nodes.is_none());
}
// Source: https://developers.upcloud.com/1.3/36-managed-kubernetes/#list-node-groups
const NODE_GROUP_LIST_JSON: &str = r#"
    [
        {
            "anti_affinity": false,
            "count": 3,
            "kubelet_args": [],
            "labels": [
                {
                    "key": "team",
                    "value": "development"
                }
            ],
            "name": "example-node-group-1",
            "plan": "4xCPU-8GB",
            "ssh_keys": [
                "ssh-rsa AAAAB3NzaC1yc2EAA"
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
                {
                    "key": "log-flush-frequency",
                    "value": "5s"
                }
            ],
            "labels": [
                {
                    "key": "team",
                    "value": "qa"
                }
            ],
            "name": "example-node-group-2",
            "plan": "4xCPU-8GB",
            "ssh_keys": [
                "ssh-rsa AAAAB3NzaC1yc2EAA"
            ],
            "state": "running",
            "storage": "01000000-0000-4000-8000-000160010100",
            "taints": [
                {
                    "effect": "NoSchedule",
                    "key": "team",
                    "value": "qa"
                }
            ],
            "utility_network_access": false
        }
    ]
"#;

#[test]
fn deserialize_node_group_list() {
    let groups: Vec<NodeGroup> = serde_json::from_str(NODE_GROUP_LIST_JSON).unwrap();
    assert_eq!(groups.len(), 2);

    assert_eq!(groups[0].name, "example-node-group-1");
    assert_eq!(groups[0].plan, "4xCPU-8GB");
    assert_eq!(groups[0].count, 3);
    assert_eq!(groups[0].state, Some(NodeGroupState::Running));
    assert_eq!(groups[0].utility_network_access, Some(true));
    assert!(groups[0].taints.as_ref().unwrap().is_empty());
    assert_eq!(groups[0].anti_affinity, Some(false));
    assert!(groups[0].kubelet_args.as_ref().unwrap().is_empty());
    let g0_labels = groups[0].labels.as_ref().unwrap();
    assert_eq!(g0_labels.len(), 1);
    assert_eq!(g0_labels[0].key, "team");
    assert_eq!(g0_labels[0].value, "development");
    let g0_ssh = groups[0].ssh_keys.as_ref().unwrap();
    assert_eq!(g0_ssh.len(), 1);
    assert_eq!(g0_ssh[0], "ssh-rsa AAAAB3NzaC1yc2EAA");
    assert_eq!(
        groups[0].storage.as_deref(),
        Some("01000000-0000-4000-8000-000160010100")
    );
    assert!(groups[0].anti_affinity_status.is_none());
    assert!(groups[0].nodes.is_none());

    assert_eq!(groups[1].name, "example-node-group-2");
    assert_eq!(groups[1].count, 9);
    assert_eq!(groups[1].anti_affinity, Some(true));
    assert_eq!(groups[1].utility_network_access, Some(false));
    assert_eq!(groups[1].plan, "4xCPU-8GB");
    assert_eq!(groups[1].state, Some(NodeGroupState::Running));
    assert_eq!(
        groups[1].storage.as_deref(),
        Some("01000000-0000-4000-8000-000160010100")
    );
    let g1_labels = groups[1].labels.as_ref().unwrap();
    assert_eq!(g1_labels.len(), 1);
    assert_eq!(g1_labels[0].key, "team");
    assert_eq!(g1_labels[0].value, "qa");
    let g1_ssh = groups[1].ssh_keys.as_ref().unwrap();
    assert_eq!(g1_ssh.len(), 1);
    assert_eq!(g1_ssh[0], "ssh-rsa AAAAB3NzaC1yc2EAA");
    assert!(groups[1].anti_affinity_status.is_none());
    assert!(groups[1].nodes.is_none());

    let args = groups[1].kubelet_args.as_ref().unwrap();
    assert_eq!(args.len(), 1);
    assert_eq!(args[0].key, "log-flush-frequency");
    assert_eq!(args[0].value, "5s");

    let taints = groups[1].taints.as_ref().unwrap();
    assert_eq!(taints.len(), 1);
    assert_eq!(taints[0].effect, "NoSchedule");
    assert_eq!(taints[0].key, "team");
    assert_eq!(taints[0].value, "qa");
}

// ─── New test JSON payloads (official docs, page 20) ──────────────────────

// Source: https://developers.upcloud.com/1.3/36-managed-kubernetes/#create-cluster
const CREATE_CLUSTER_REQUEST_JSON: &str = r#"
    {
        "control_plane_ip_filter": [],
        "name": "example-cluster-1",
        "network": "03a85f64-5717-4562-b3fc-2c963f66afa6",
        "network_cidr": "172.16.0.0/24",
        "node_groups": [
            {
                "count": 3,
                "labels": [
                    {
                        "key": "team",
                        "value": "development"
                    }
                ],
                "name": "example-node-group-1",
                "plan": "4xCPU-8GB",
                "ssh_keys": [
                    "ssh-rsa AAAAB3NzaC1yc2EAA"
                ]
            }
        ],
        "plan": "dev-md",
        "private_node_groups": false,
        "zone": "fi-hel2",
        "labels": [
            {
                "key": "env",
                "value": "staging"
            },
            {
                "key": "app",
                "value": "cats-dating-app"
            }
        ]
    }
"#;

// ─── New test functions ───────────────────────────────────────────────────

#[test]
fn deserialize_create_cluster_request() {
    let req: CreateClusterRequest = serde_json::from_str(CREATE_CLUSTER_REQUEST_JSON).unwrap();
    assert_eq!(req.name, "example-cluster-1");
    assert_eq!(req.zone, "fi-hel2");
    assert_eq!(req.network, "03a85f64-5717-4562-b3fc-2c963f66afa6");
    assert_eq!(req.network_cidr, "172.16.0.0/24");
    assert_eq!(req.node_groups.len(), 1);
    assert_eq!(req.plan.as_deref(), Some("dev-md"));
    assert_eq!(req.private_node_groups, Some(false));
    assert!(req.version.is_none());
    assert!(req.control_plane_ip_filter.as_ref().unwrap().is_empty());
    let labels = req.labels.as_ref().unwrap();
    assert_eq!(labels.len(), 2);
    assert_eq!(labels[0].key, "env");
    assert_eq!(labels[0].value, "staging");
    assert_eq!(labels[1].key, "app");
    assert_eq!(labels[1].value, "cats-dating-app");

    let ng = &req.node_groups[0];
    assert_eq!(ng.name, "example-node-group-1");
    assert_eq!(ng.plan, "4xCPU-8GB");
    assert_eq!(ng.count, 3);
    let ng_labels = ng.labels.as_ref().unwrap();
    assert_eq!(ng_labels.len(), 1);
    assert_eq!(ng_labels[0].key, "team");
    assert_eq!(ng_labels[0].value, "development");
    let ng_ssh = ng.ssh_keys.as_ref().unwrap();
    assert_eq!(ng_ssh.len(), 1);
    assert_eq!(ng_ssh[0], "ssh-rsa AAAAB3NzaC1yc2EAA");
    assert!(ng.anti_affinity.is_none());
    assert!(ng.taints.is_none());
    assert!(ng.kubelet_args.is_none());
    assert!(ng.utility_network_access.is_none());
    assert!(ng.storage.is_none());
}
// Source: https://developers.upcloud.com/1.3/36-managed-kubernetes/#create-cluster
const CREATE_CLUSTER_RESPONSE_JSON: &str = r#"
    {
        "control_plane_ip_filter": [],
        "labels": [
            {
                "key": "env",
                "value": "staging"
            },
            {
                "key": "app",
                "value": "cats-dating-app"
            }
        ],
        "name": "example-cluster-1",
        "network": "03a85f64-5717-4562-b3fc-2c963f66afa6",
        "network_cidr": "172.16.0.0/24",
        "node_groups": [
            {
                "count": 3,
                "kubelet_args": [],
                "labels": [
                    {
                        "key": "team",
                        "value": "development"
                    }
                ],
                "name": "example-node-group-1",
                "plan": "4xCPU-8GB",
                "ssh_keys": [
                    "ssh-rsa AAAAB3NzaC1yc2EAA"
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
"#;

#[test]
fn deserialize_create_cluster_response() {
    let cluster: Cluster = serde_json::from_str(CREATE_CLUSTER_RESPONSE_JSON).unwrap();
    assert_eq!(cluster.state, ClusterState::Pending);
    assert_eq!(cluster.name, "example-cluster-1");
    assert_eq!(cluster.uuid, "3fa85f64-5717-4562-b3fc-2c963f66afa6");
    assert_eq!(cluster.zone, "fi-hel2");
    assert_eq!(cluster.plan.as_deref(), Some("dev-md"));
    assert_eq!(cluster.private_node_groups, Some(false));
    assert_eq!(
        cluster.network.as_deref(),
        Some("03a85f64-5717-4562-b3fc-2c963f66afa6")
    );
    assert_eq!(cluster.network_cidr.as_deref(), Some("172.16.0.0/24"));
    assert!(cluster.control_plane_ip_filter.as_ref().unwrap().is_empty());
    assert!(cluster.version.is_none());
    assert!(cluster.created_at.is_none());

    let labels = cluster.labels.as_ref().unwrap();
    assert_eq!(labels.len(), 2);
    assert_eq!(labels[0].key, "env");
    assert_eq!(labels[0].value, "staging");
    assert_eq!(labels[1].key, "app");
    assert_eq!(labels[1].value, "cats-dating-app");

    let groups = cluster.node_groups.as_ref().unwrap();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].state, Some(NodeGroupState::Pending));
    assert_eq!(groups[0].name, "example-node-group-1");
    assert_eq!(groups[0].plan, "4xCPU-8GB");
    assert_eq!(groups[0].count, 3);
    assert_eq!(groups[0].anti_affinity, Some(false));
    assert_eq!(groups[0].utility_network_access, Some(true));
    assert_eq!(
        groups[0].storage.as_deref(),
        Some("01000000-0000-4000-8000-000160010100")
    );
    assert!(groups[0].kubelet_args.as_ref().unwrap().is_empty());
    assert!(groups[0].taints.as_ref().unwrap().is_empty());
    let g0_labels = groups[0].labels.as_ref().unwrap();
    assert_eq!(g0_labels.len(), 1);
    assert_eq!(g0_labels[0].key, "team");
    assert_eq!(g0_labels[0].value, "development");
    let g0_ssh = groups[0].ssh_keys.as_ref().unwrap();
    assert_eq!(g0_ssh.len(), 1);
    assert_eq!(g0_ssh[0], "ssh-rsa AAAAB3NzaC1yc2EAA");
    assert!(groups[0].anti_affinity_status.is_none());
    assert!(groups[0].nodes.is_none());
}
// Source: https://developers.upcloud.com/1.3/36-managed-kubernetes/#modify-cluster
const MODIFY_CLUSTER_REQUEST_JSON: &str = r#"
    {
        "control_plane_ip_filter": [
            "0.0.0.0/0"
        ],
        "labels": [
            {
                "key": "env",
                "value": "production"
            },
            {
                "key": "app",
                "value": "dogs-dating-app"
            }
        ]
    }
"#;

#[test]
fn deserialize_modify_cluster_request() {
    let val: serde_json::Value = serde_json::from_str(MODIFY_CLUSTER_REQUEST_JSON).unwrap();
    assert!(val.get("control_plane_ip_filter").is_some());
    assert!(val.get("labels").is_some());
    let filters = val["control_plane_ip_filter"].as_array().unwrap();
    assert_eq!(filters.len(), 1);
    assert_eq!(filters[0].as_str().unwrap(), "0.0.0.0/0");
    let labels = val["labels"].as_array().unwrap();
    assert_eq!(labels.len(), 2);
    assert_eq!(labels[0]["key"].as_str().unwrap(), "env");
    assert_eq!(labels[0]["value"].as_str().unwrap(), "production");
    assert_eq!(labels[1]["key"].as_str().unwrap(), "app");
    assert_eq!(labels[1]["value"].as_str().unwrap(), "dogs-dating-app");
}
// Source: https://developers.upcloud.com/1.3/36-managed-kubernetes/#modify-cluster
const MODIFY_CLUSTER_RESPONSE_JSON: &str = r#"
    {
        "control_plane_ip_filter": [
            "0.0.0.0/0"
        ],
        "labels": [
            {
                "key": "env",
                "value": "production"
            },
            {
                "key": "app",
                "value": "dogs-dating-app"
            }
        ],
        "name": "example-cluster-1",
        "network": "03a85f64-5717-4562-b3fc-2c963f66afa6",
        "network_cidr": "172.16.0.0/24",
        "node_groups": [
            {
                "count": 3,
                "kubelet_args": [],
                "labels": [
                    {
                        "key": "team",
                        "value": "development"
                    }
                ],
                "name": "example-node-group-1",
                "plan": "4xCPU-8GB",
                "ssh_keys": [
                    "ssh-rsa AAAAB3NzaC1yc2EAA"
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
"#;

#[test]
fn deserialize_modify_cluster_response() {
    let cluster: Cluster = serde_json::from_str(MODIFY_CLUSTER_RESPONSE_JSON).unwrap();
    assert_eq!(cluster.state, ClusterState::Running);
    assert_eq!(cluster.name, "example-cluster-1");
    assert_eq!(cluster.uuid, "3fa85f64-5717-4562-b3fc-2c963f66afa6");
    assert_eq!(cluster.zone, "fi-hel2");
    assert_eq!(cluster.plan.as_deref(), Some("dev-md"));
    assert_eq!(cluster.private_node_groups, Some(false));
    assert_eq!(
        cluster.network.as_deref(),
        Some("03a85f64-5717-4562-b3fc-2c963f66afa6")
    );
    assert_eq!(cluster.network_cidr.as_deref(), Some("172.16.0.0/24"));
    assert_eq!(
        cluster.control_plane_ip_filter.as_ref().unwrap(),
        &["0.0.0.0/0"]
    );
    assert!(cluster.version.is_none());
    assert!(cluster.created_at.is_none());

    let labels = cluster.labels.as_ref().unwrap();
    assert_eq!(labels.len(), 2);
    assert_eq!(labels[0].key, "env");
    assert_eq!(labels[0].value, "production");
    assert_eq!(labels[1].key, "app");
    assert_eq!(labels[1].value, "dogs-dating-app");

    let groups = cluster.node_groups.as_ref().unwrap();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].name, "example-node-group-1");
    assert_eq!(groups[0].plan, "4xCPU-8GB");
    assert_eq!(groups[0].count, 3);
    assert_eq!(groups[0].state, Some(NodeGroupState::Running));
    assert_eq!(groups[0].anti_affinity, Some(false));
    assert_eq!(groups[0].utility_network_access, Some(true));
    assert_eq!(
        groups[0].storage.as_deref(),
        Some("01000000-0000-4000-8000-000160010100")
    );
    assert!(groups[0].kubelet_args.as_ref().unwrap().is_empty());
    assert!(groups[0].taints.as_ref().unwrap().is_empty());
    let g0_labels = groups[0].labels.as_ref().unwrap();
    assert_eq!(g0_labels.len(), 1);
    assert_eq!(g0_labels[0].key, "team");
    assert_eq!(g0_labels[0].value, "development");
    let g0_ssh = groups[0].ssh_keys.as_ref().unwrap();
    assert_eq!(g0_ssh.len(), 1);
    assert_eq!(g0_ssh[0], "ssh-rsa AAAAB3NzaC1yc2EAA");
    assert!(groups[0].anti_affinity_status.is_none());
    assert!(groups[0].nodes.is_none());
}
// Source: https://developers.upcloud.com/1.3/36-managed-kubernetes/
const AVAILABLE_UPGRADES_JSON: &str = r#"
    {
        "versions": [
            "1.31"
        ]
    }
"#;

#[test]
fn deserialize_available_upgrades() {
    let resp: ClusterAvailableUpgrades = serde_json::from_str(AVAILABLE_UPGRADES_JSON).unwrap();
    assert_eq!(resp.versions.len(), 1);
    assert_eq!(resp.versions[0], "1.31");
}
// Source: https://developers.upcloud.com/1.3/36-managed-kubernetes/
const UPGRADE_CLUSTER_REQUEST_JSON: &str = r#"
    {
        "version": "1.31",
        "strategy": {
            "type": "manual"
        }
    }
"#;

#[test]
fn deserialize_upgrade_cluster_request() {
    let req: UpgradeClusterRequest = serde_json::from_str(UPGRADE_CLUSTER_REQUEST_JSON).unwrap();
    assert_eq!(req.version, "1.31");
    let strategy = req.strategy.as_ref().unwrap();
    assert_eq!(strategy.strategy_type, "manual");
}

// Source: https://developers.upcloud.com/1.3/36-managed-kubernetes/
const UPGRADE_CLUSTER_RESPONSE_JSON: &str = r#"
    {
        "version": "1.31",
        "strategy": {
            "type": "manual"
        }
    }
"#;

// ─── Upgrade cluster response (official docs, page 20) ───────────────────

#[test]
fn deserialize_upgrade_cluster_response() {
    let resp: UpgradeClusterRequest = serde_json::from_str(UPGRADE_CLUSTER_RESPONSE_JSON).unwrap();
    assert_eq!(resp.version, "1.31");
    let strategy = resp.strategy.as_ref().unwrap();
    assert_eq!(strategy.strategy_type, "manual");
}
// Source: https://developers.upcloud.com/1.3/36-managed-kubernetes/#get-kubeconfig
const KUBECONFIG_RESPONSE_JSON: &str = r#"
    {
        "kubeconfig": "apiVersion: v1\nclusters:\n  - cluster:\n      certificate-authority-data: BASE64\n      server: https://server:6443\n    name: example-cluster-1\ncontexts:\n  - context:\n      cluster: example-cluster-1\n      user: example-cluster-1-admin\n    name: example-cluster-1-admin@example-cluster-1\ncurrent-context: example-cluster-1-admin@example-cluster-1\nkind: Config\npreferences: {}\nusers:\n  - name: example-cluster-1-admin\n    user:\n      client-certificate-data: BASE64\n      client-key-data: BASE64\n"
    }
"#;

#[test]
fn deserialize_kubeconfig_response() {
    let resp: Kubeconfig = serde_json::from_str(KUBECONFIG_RESPONSE_JSON).unwrap();
    assert!(resp.kubeconfig.starts_with("apiVersion"));
    assert!(resp.kubeconfig.contains("example-cluster-1"));
    assert!(
        resp.kubeconfig
            .contains("certificate-authority-data: BASE64")
    );
    assert!(resp.kubeconfig.contains("client-certificate-data: BASE64"));
    assert!(resp.kubeconfig.contains("client-key-data: BASE64"));
}
// Source: https://developers.upcloud.com/1.3/36-managed-kubernetes/#create-node-group
const CREATE_NODE_GROUP_REQUEST_JSON: &str = r#"
    {
        "anti_affinity": true,
        "count": 9,
        "kubelet_args": [
            {
                "key": "log-flush-frequency",
                "value": "5s"
            }
        ],
        "labels": [
            {
                "key": "team",
                "value": "qa"
            }
        ],
        "name": "example-node-group-2",
        "plan": "4xCPU-8GB",
        "ssh_keys": [
            "ssh-rsa AAAAB3NzaC1yc2EAA"
        ],
        "storage": "01000000-0000-4000-8000-000160010100",
        "taints": [
            {
                "effect": "NoSchedule",
                "key": "team",
                "value": "qa"
            }
        ],
        "utility_network_access": true
    }
"#;

#[test]
fn deserialize_create_kubernetes_node_group_request() {
    let req: CreateNodeGroupRequest = serde_json::from_str(CREATE_NODE_GROUP_REQUEST_JSON).unwrap();
    assert_eq!(req.name, "example-node-group-2");
    assert_eq!(req.plan, "4xCPU-8GB");
    assert_eq!(req.count, 9);
    assert_eq!(req.anti_affinity, Some(true));
    assert_eq!(req.utility_network_access, Some(true));
    assert_eq!(
        req.storage.as_deref(),
        Some("01000000-0000-4000-8000-000160010100")
    );
    let labels = req.labels.as_ref().unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0].key, "team");
    assert_eq!(labels[0].value, "qa");
    let ssh_keys = req.ssh_keys.as_ref().unwrap();
    assert_eq!(ssh_keys.len(), 1);
    assert_eq!(ssh_keys[0], "ssh-rsa AAAAB3NzaC1yc2EAA");
    let args = req.kubelet_args.as_ref().unwrap();
    assert_eq!(args.len(), 1);
    assert_eq!(args[0].key, "log-flush-frequency");
    assert_eq!(args[0].value, "5s");
    let taints = req.taints.as_ref().unwrap();
    assert_eq!(taints.len(), 1);
    assert_eq!(taints[0].effect, "NoSchedule");
    assert_eq!(taints[0].key, "team");
    assert_eq!(taints[0].value, "qa");
}
// Source: https://developers.upcloud.com/1.3/36-managed-kubernetes/#create-node-group
const CREATE_NODE_GROUP_RESPONSE_JSON: &str = r#"
    {
        "anti_affinity": true,
        "count": 9,
        "kubelet_args": [
            {
                "key": "log-flush-frequency",
                "value": "5s"
            }
        ],
        "labels": [
            {
                "key": "team",
                "value": "qa"
            }
        ],
        "name": "example-node-group-2",
        "plan": "4xCPU-8GB",
        "ssh_keys": [
            "ssh-rsa AAAAB3NzaC1yc2EAA"
        ],
        "state": "pending",
        "storage": "01000000-0000-4000-8000-000160010100",
        "taints": [
            {
                "effect": "NoSchedule",
                "key": "team",
                "value": "qa"
            }
        ],
        "utility_network_access": true
    }
"#;

#[test]
fn deserialize_create_kubernetes_node_group_response() {
    let group: NodeGroup = serde_json::from_str(CREATE_NODE_GROUP_RESPONSE_JSON).unwrap();
    assert_eq!(group.state, Some(NodeGroupState::Pending));
    assert_eq!(group.name, "example-node-group-2");
    assert_eq!(group.count, 9);
    assert_eq!(group.plan, "4xCPU-8GB");
    assert_eq!(group.anti_affinity, Some(true));
    assert_eq!(group.utility_network_access, Some(true));
    assert_eq!(
        group.storage.as_deref(),
        Some("01000000-0000-4000-8000-000160010100")
    );
    let labels = group.labels.as_ref().unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0].key, "team");
    assert_eq!(labels[0].value, "qa");
    let ssh_keys = group.ssh_keys.as_ref().unwrap();
    assert_eq!(ssh_keys.len(), 1);
    assert_eq!(ssh_keys[0], "ssh-rsa AAAAB3NzaC1yc2EAA");
    let args = group.kubelet_args.as_ref().unwrap();
    assert_eq!(args.len(), 1);
    assert_eq!(args[0].key, "log-flush-frequency");
    assert_eq!(args[0].value, "5s");
    let taints = group.taints.as_ref().unwrap();
    assert_eq!(taints.len(), 1);
    assert_eq!(taints[0].effect, "NoSchedule");
    assert_eq!(taints[0].key, "team");
    assert_eq!(taints[0].value, "qa");
    assert!(group.anti_affinity_status.is_none());
    assert!(group.nodes.is_none());
}
// Source: https://developers.upcloud.com/1.3/36-managed-kubernetes/#get-node-group-details
const NODE_GROUP_DETAIL_JSON: &str = r#"
    {
        "anti_affinity_status": true,
        "count": 2,
        "kubelet_args": [
            {
                "key": "log-flush-frequency",
                "value": "5s"
            }
        ],
        "labels": [
            {
                "key": "team",
                "value": "qa"
            }
        ],
        "name": "example-node-group-2",
        "plan": "4xCPU-8GB",
        "ssh_keys": [
            "ssh-rsa AAAAB3NzaC1yc2EAA"
        ],
        "state": "running",
        "storage": "01000000-0000-4000-8000-000160010100",
        "taints": [
            {
                "effect": "NoSchedule",
                "key": "team",
                "value": "qa"
            }
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
"#;

#[test]
fn deserialize_node_group_detail() {
    let group: NodeGroup = serde_json::from_str(NODE_GROUP_DETAIL_JSON).unwrap();
    assert_eq!(group.anti_affinity_status, Some(true));
    assert_eq!(group.state, Some(NodeGroupState::Running));
    assert_eq!(group.count, 2);
    assert_eq!(group.name, "example-node-group-2");
    assert_eq!(group.plan, "4xCPU-8GB");
    assert_eq!(group.utility_network_access, Some(true));
    assert_eq!(
        group.storage.as_deref(),
        Some("01000000-0000-4000-8000-000160010100")
    );
    assert!(group.anti_affinity.is_none());
    let labels = group.labels.as_ref().unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0].key, "team");
    assert_eq!(labels[0].value, "qa");
    let ssh_keys = group.ssh_keys.as_ref().unwrap();
    assert_eq!(ssh_keys.len(), 1);
    assert_eq!(ssh_keys[0], "ssh-rsa AAAAB3NzaC1yc2EAA");
    let args = group.kubelet_args.as_ref().unwrap();
    assert_eq!(args.len(), 1);
    assert_eq!(args[0].key, "log-flush-frequency");
    assert_eq!(args[0].value, "5s");
    let taints = group.taints.as_ref().unwrap();
    assert_eq!(taints.len(), 1);
    assert_eq!(taints[0].effect, "NoSchedule");
    assert_eq!(taints[0].key, "team");
    assert_eq!(taints[0].value, "qa");
    let nodes = group.nodes.as_ref().unwrap();
    assert_eq!(nodes.len(), 2);
    assert_eq!(nodes[0].uuid, "564c8cd9-bc16-4f89-a328-daf3f138cc44");
    assert_eq!(nodes[0].name, "example-fzwwt-8smpz");
    assert_eq!(nodes[0].state, NodeState::Running);
    assert_eq!(nodes[0].kubelet_version.as_deref(), Some("1.30"));
    assert_eq!(nodes[1].uuid, "2e8039c6-95d4-41a9-92ca-46808f560abb");
    assert_eq!(nodes[1].name, "example-fzwwt-zmjfs");
    assert_eq!(nodes[1].state, NodeState::Running);
    assert_eq!(nodes[1].kubelet_version.as_deref(), Some("1.30"));
}
// Source: https://developers.upcloud.com/1.3/36-managed-kubernetes/#modify-node-group
const MODIFY_NODE_GROUP_REQUEST_JSON: &str = r#"
    {
        "count": 15
    }
"#;

#[test]
fn deserialize_modify_kubernetes_node_group_request() {
    let req: ModifyNodeGroupRequest = serde_json::from_str(MODIFY_NODE_GROUP_REQUEST_JSON).unwrap();
    assert_eq!(req.count, Some(15));
    assert!(req.labels.is_none());
}
// Source: https://developers.upcloud.com/1.3/36-managed-kubernetes/#modify-node-group
const MODIFY_NODE_GROUP_RESPONSE_JSON: &str = r#"
    {
        "anti_affinity": true,
        "count": 15,
        "kubelet_args": [
            {
                "key": "log-flush-frequency",
                "value": "5s"
            }
        ],
        "labels": [
            {
                "key": "team",
                "value": "qa"
            }
        ],
        "name": "example-node-group-2",
        "plan": "4xCPU-8GB",
        "ssh_keys": [
            "ssh-rsa AAAAB3NzaC1yc2EAA"
        ],
        "state": "running",
        "storage": "01000000-0000-4000-8000-000160010100",
        "taints": [
            {
                "effect": "NoSchedule",
                "key": "team",
                "value": "qa"
            }
        ],
        "utility_network_access": true
    }
"#;

#[test]
fn deserialize_modify_kubernetes_node_group_response() {
    let group: NodeGroup = serde_json::from_str(MODIFY_NODE_GROUP_RESPONSE_JSON).unwrap();
    assert_eq!(group.count, 15);
    assert_eq!(group.name, "example-node-group-2");
    assert_eq!(group.state, Some(NodeGroupState::Running));
    assert_eq!(group.plan, "4xCPU-8GB");
    assert_eq!(group.anti_affinity, Some(true));
    assert_eq!(group.utility_network_access, Some(true));
    assert_eq!(
        group.storage.as_deref(),
        Some("01000000-0000-4000-8000-000160010100")
    );
    let labels = group.labels.as_ref().unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0].key, "team");
    assert_eq!(labels[0].value, "qa");
    let ssh_keys = group.ssh_keys.as_ref().unwrap();
    assert_eq!(ssh_keys.len(), 1);
    assert_eq!(ssh_keys[0], "ssh-rsa AAAAB3NzaC1yc2EAA");
    let args = group.kubelet_args.as_ref().unwrap();
    assert_eq!(args.len(), 1);
    assert_eq!(args[0].key, "log-flush-frequency");
    assert_eq!(args[0].value, "5s");
    let taints = group.taints.as_ref().unwrap();
    assert_eq!(taints.len(), 1);
    assert_eq!(taints[0].effect, "NoSchedule");
    assert_eq!(taints[0].key, "team");
    assert_eq!(taints[0].value, "qa");
    assert!(group.anti_affinity_status.is_none());
    assert!(group.nodes.is_none());
}

// ─── New parity tests ─────────────────────────────────────────────────────

#[test]
fn serialize_modify_cluster_request() {
    let req = ModifyClusterRequest {
        ip_filter: None,
        labels: Some(vec![crate::types::common::Label {
            key: "env".into(),
            value: "production".into(),
        }]),
        control_plane_ip_filter: Some(vec!["0.0.0.0/0".into()]),
        plan: None,
    };
    let json = serde_json::to_value(&req).unwrap();
    assert!(json.get("ip_filter").is_none());
    assert!(json.get("plan").is_none());
    assert_eq!(json["control_plane_ip_filter"].as_array().unwrap().len(), 1);
    assert_eq!(json["labels"].as_array().unwrap().len(), 1);
    assert_eq!(json["labels"][0]["key"].as_str().unwrap(), "env");
    assert_eq!(json["labels"][0]["value"].as_str().unwrap(), "production");
}

#[test]
fn deserialize_modify_cluster_request_typed() {
    let req: ModifyClusterRequest = serde_json::from_str(MODIFY_CLUSTER_REQUEST_JSON).unwrap();
    assert_eq!(
        req.control_plane_ip_filter.as_ref().unwrap(),
        &["0.0.0.0/0"]
    );
    let labels = req.labels.as_ref().unwrap();
    assert_eq!(labels.len(), 2);
    assert_eq!(labels[0].key, "env");
    assert_eq!(labels[0].value, "production");
    assert!(req.ip_filter.is_none());
    assert!(req.plan.is_none());
}
// Source: https://developers.upcloud.com/1.3/36-managed-kubernetes/#list-versions
const KUBERNETES_VERSIONS_JSON: &str = r#"
    [
        {
            "id": "1.30",
            "version": "1.30"
        },
        {
            "id": "1.31",
            "version": "1.31"
        }
    ]
"#;

#[test]
fn deserialize_kubernetes_versions() {
    let versions: Vec<KubernetesVersion> = serde_json::from_str(KUBERNETES_VERSIONS_JSON).unwrap();
    assert_eq!(versions.len(), 2);
    assert_eq!(versions[0].id, "1.30");
    assert_eq!(versions[0].version, "1.30");
    assert_eq!(versions[1].id, "1.31");
    assert_eq!(versions[1].version, "1.31");
}

// Source: https://developers.upcloud.com/1.3/36-managed-kubernetes/#get-node-group-details
const NODE_GROUP_WITH_CUSTOM_PLAN_JSON: &str = r#"
    {
        "count": 3,
        "kubelet_args": [],
        "labels": [
            {
                "key": "team",
                "value": "development"
            }
        ],
        "name": "custom-plan-group",
        "plan": "custom",
        "custom_plan": {
            "cores": 4,
            "memory": 8192,
            "storage_size": 50,
            "storage_tier": "maxiops"
        },
        "ssh_keys": [
            "ssh-rsa AAAAB3NzaC1yc2EAA"
        ],
        "state": "running",
        "storage": "01000000-0000-4000-8000-000160010100",
        "taints": [],
        "anti_affinity": false,
        "utility_network_access": true,
        "storage_encryption": "data-at-rest"
    }
"#;

// ─── Node group with custom_plan (docs section "Custom plan") ─────────────

#[test]
fn deserialize_node_group_with_custom_plan() {
    let group: NodeGroup = serde_json::from_str(NODE_GROUP_WITH_CUSTOM_PLAN_JSON).unwrap();
    assert_eq!(group.name, "custom-plan-group");
    assert_eq!(group.plan, "custom");
    assert_eq!(group.count, 3);
    assert_eq!(group.state, Some(NodeGroupState::Running));
    assert_eq!(group.anti_affinity, Some(false));
    assert_eq!(group.utility_network_access, Some(true));
    assert_eq!(group.storage_encryption.as_deref(), Some("data-at-rest"));

    let cp = group.custom_plan.as_ref().unwrap();
    assert_eq!(cp.cores, 4);
    assert_eq!(cp.memory, 8192);
    assert_eq!(cp.storage_size, 50);
    assert_eq!(cp.storage_tier, "maxiops");

    assert!(group.cloud_native_plan.is_none());
    assert!(group.gpu_plan.is_none());
}

// Source: https://developers.upcloud.com/1.3/36-managed-kubernetes/#get-node-group-details
const NODE_GROUP_WITH_CLOUD_NATIVE_PLAN_JSON: &str = r#"
    {
        "count": 2,
        "kubelet_args": [],
        "labels": [],
        "name": "cloud-native-group",
        "plan": "CLOUDNATIVE-48xCPU-384GB",
        "cloud_native_plan": {
            "storage_size": 25,
            "storage_tier": "maxiops"
        },
        "ssh_keys": [],
        "state": "running",
        "storage": "01000000-0000-4000-8000-000160010100",
        "taints": [],
        "anti_affinity": false,
        "utility_network_access": true
    }
"#;

// ─── Node group with cloud_native_plan (docs section "Cloud native plan") ────

#[test]
fn deserialize_node_group_with_cloud_native_plan() {
    let group: NodeGroup = serde_json::from_str(NODE_GROUP_WITH_CLOUD_NATIVE_PLAN_JSON).unwrap();
    assert_eq!(group.name, "cloud-native-group");
    assert_eq!(group.plan, "CLOUDNATIVE-48xCPU-384GB");
    assert_eq!(group.count, 2);
    assert_eq!(group.state, Some(NodeGroupState::Running));

    let cnp = group.cloud_native_plan.as_ref().unwrap();
    assert_eq!(cnp.storage_size, 25);
    assert_eq!(cnp.storage_tier, "maxiops");

    assert!(group.custom_plan.is_none());
    assert!(group.gpu_plan.is_none());
}

// Source: https://developers.upcloud.com/1.3/36-managed-kubernetes/#get-node-group-details
const NODE_GROUP_WITH_GPU_PLAN_JSON: &str = r#"
    {
        "count": 1,
        "kubelet_args": [],
        "labels": [],
        "name": "gpu-group",
        "plan": "GPU-12xCPU-128GB-1xL40s",
        "gpu_plan": {
            "storage_size": 100,
            "storage_tier": "maxiops"
        },
        "ssh_keys": [],
        "state": "running",
        "storage": "01000000-0000-4000-8000-000160010100",
        "taints": [
            {
                "effect": "NoSchedule",
                "key": "nvidia.com/gpu",
                "value": "present"
            }
        ],
        "anti_affinity": false,
        "utility_network_access": true
    }
"#;

// ─── Node group with gpu_plan (docs section "GPU plan") ──────────────────────

#[test]
fn deserialize_node_group_with_gpu_plan() {
    let group: NodeGroup = serde_json::from_str(NODE_GROUP_WITH_GPU_PLAN_JSON).unwrap();
    assert_eq!(group.name, "gpu-group");
    assert_eq!(group.plan, "GPU-12xCPU-128GB-1xL40s");
    assert_eq!(group.count, 1);
    assert_eq!(group.state, Some(NodeGroupState::Running));

    let gp = group.gpu_plan.as_ref().unwrap();
    assert_eq!(gp.storage_size, 100);
    assert_eq!(gp.storage_tier, "maxiops");

    let taints = group.taints.as_ref().unwrap();
    assert_eq!(taints.len(), 1);
    assert_eq!(taints[0].effect, "NoSchedule");
    assert_eq!(taints[0].key, "nvidia.com/gpu");
    assert_eq!(taints[0].value, "present");

    assert!(group.custom_plan.is_none());
    assert!(group.cloud_native_plan.is_none());
}

// Source: https://developers.upcloud.com/1.3/36-managed-kubernetes/#get-cluster-details
const K8S_CLUSTER_WITH_ENCRYPTION_JSON: &str = r#"
    {
        "control_plane_ip_filter": [
            "0.0.0.0/0"
        ],
        "labels": [
            {
                "key": "env",
                "value": "production"
            }
        ],
        "name": "encrypted-cluster",
        "network": "03a85f64-5717-4562-b3fc-2c963f66afa6",
        "network_cidr": "172.16.0.0/24",
        "node_groups": [
            {
                "count": 3,
                "kubelet_args": [],
                "labels": [
                    {
                        "key": "team",
                        "value": "platform"
                    }
                ],
                "name": "encrypted-node-group",
                "plan": "4xCPU-8GB",
                "ssh_keys": [
                    "ssh-rsa AAAAB3NzaC1yc2EAA"
                ],
                "state": "running",
                "storage": "01000000-0000-4000-8000-000160010100",
                "taints": [],
                "anti_affinity": false,
                "utility_network_access": true,
                "storage_encryption": "data-at-rest"
            }
        ],
        "state": "running",
        "plan": "prod-md",
        "private_node_groups": false,
        "storage_encryption": "data-at-rest",
        "uuid": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
        "zone": "fi-hel2"
    }
"#;

// ─── Full cluster response with storage_encryption (docs, page 20) ───────────

#[test]
fn deserialize_cluster_with_storage_encryption() {
    let cluster: Cluster = serde_json::from_str(K8S_CLUSTER_WITH_ENCRYPTION_JSON).unwrap();
    assert_eq!(cluster.name, "encrypted-cluster");
    assert_eq!(cluster.uuid, "3fa85f64-5717-4562-b3fc-2c963f66afa6");
    assert_eq!(cluster.state, ClusterState::Running);
    assert_eq!(cluster.zone, "fi-hel2");
    assert_eq!(cluster.plan.as_deref(), Some("prod-md"));
    assert_eq!(cluster.private_node_groups, Some(false));
    assert_eq!(cluster.storage_encryption.as_deref(), Some("data-at-rest"));
    assert_eq!(
        cluster.control_plane_ip_filter.as_ref().unwrap(),
        &["0.0.0.0/0"]
    );

    let labels = cluster.labels.as_ref().unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0].key, "env");
    assert_eq!(labels[0].value, "production");

    let groups = cluster.node_groups.as_ref().unwrap();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].name, "encrypted-node-group");
    assert_eq!(groups[0].plan, "4xCPU-8GB");
    assert_eq!(groups[0].count, 3);
    assert_eq!(groups[0].state, Some(NodeGroupState::Running));
    assert_eq!(
        groups[0].storage_encryption.as_deref(),
        Some("data-at-rest")
    );
}
