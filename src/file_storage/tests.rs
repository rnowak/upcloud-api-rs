use super::*;

// Source: https://developers.upcloud.com/1.3/25-file-storage/#list-services
const FS_LIST_JSON: &str = r#"
    [
        {
            "configured_status": "started",
            "created_at": "2025-08-01T10:40:04.140473Z",
            "encrypted": false,
            "labels": [
                {
                    "key": "environment",
                    "value": "production"
                }
            ],
            "name": "test2",
            "networks": [
                {
                    "family": "IPv4",
                    "ip_address": "10.10.10.50",
                    "name": "net",
                    "uuid": "0391f203-6eef-4a69-bfcc-fffffbdfdc32"
                }
            ],
            "operational_state": "running",
            "shares": [
                {
                    "acl": [
                        {
                            "name": "read",
                            "permission": "ro",
                            "target": "*"
                        }
                    ],
                    "name": "data",
                    "path": "/data"
                },
                {
                    "acl": [
                        {
                            "name": "write",
                            "permission": "rw",
                            "target": "10.10.10.0/24"
                        }
                    ],
                    "name": "proj",
                    "path": "/project/data"
                }
            ],
            "size_gib": 250,
            "updated_at": "2025-08-03T19:23:52.350216Z",
            "uuid": "17871d62-6964-4336-b68d-4a85a8ee10fe",
            "zone": "fi-hel2"
        }
    ]
"#;

// Source: https://developers.upcloud.com/1.3/25-file-storage/#get-service-details
const FS_DETAIL_JSON: &str = r#"
    {
        "configured_status": "started",
        "created_at": "2025-08-01T10:40:04.140473Z",
        "encrypted": false,
        "labels": [
            {
                "key": "environment",
                "value": "production"
            }
        ],
        "name": "test2",
        "networks": [
            {
                "family": "IPv4",
                "ip_address": "10.10.10.50",
                "name": "net",
                "uuid": "0391f203-6eef-4a69-bfcc-fffffbdfdc32"
            }
        ],
        "operational_state": "running",
        "shares": [
            {
                "acl": [
                    {
                        "name": "read",
                        "permission": "ro",
                        "target": "*"
                    }
                ],
                "name": "data",
                "path": "/data"
            }
        ],
        "size_gib": 250,
        "updated_at": "2025-08-03T19:23:52.350216Z",
        "uuid": "17871d62-6964-4336-b68d-4a85a8ee10fe",
        "zone": "fi-hel2"
    }
"#;

// Source: https://developers.upcloud.com/1.3/25-file-storage/#list-networks
const FS_NETWORK_LIST_JSON: &str = r#"
    [
        {
            "family": "IPv4",
            "ip_address": "192.168.147.250",
            "name": "test-name",
            "uuid": "03720d07-80a9-41ac-90b0-68a932109339"
        }
    ]
"#;

// Source: https://developers.upcloud.com/1.3/25-file-storage/#list-shares
const FS_SHARE_LIST_JSON: &str = r#"
    [
        {
            "acl": [
                {
                    "name": "write-all",
                    "permission": "rw",
                    "target": "*"
                }
            ],
            "name": "share1",
            "path": "/share-1"
        },
        {
            "acl": [
                {
                    "name": "read-only-all",
                    "permission": "ro",
                    "target": "*"
                }
            ],
            "name": "share2",
            "path": "/share-1/share-2"
        }
    ]
"#;

#[test]
fn deserialize_file_storage_list() {
    let storages: Vec<Service> = serde_json::from_str(FS_LIST_JSON).unwrap();
    assert_eq!(storages.len(), 1);
    let fs = &storages[0];
    assert_eq!(fs.uuid, "17871d62-6964-4336-b68d-4a85a8ee10fe");
    assert_eq!(fs.name, "test2");
    assert_eq!(fs.zone, "fi-hel2");
    assert_eq!(fs.operational_state.as_deref(), Some("running"));
    assert_eq!(fs.size_gib, Some(250));
    assert_eq!(fs.encrypted, Some(false));
    assert_eq!(fs.configured_status, Some(ConfiguredStatus::Started));

    assert!(fs.plan.is_none());
    assert_eq!(
        fs.created_at.as_deref(),
        Some("2025-08-01T10:40:04.140473Z")
    );
    assert_eq!(
        fs.updated_at.as_deref(),
        Some("2025-08-03T19:23:52.350216Z")
    );
    assert!(fs.state_messages.is_empty());

    // Networks
    assert_eq!(fs.networks.len(), 1);
    assert_eq!(fs.networks[0].family, "IPv4");
    assert_eq!(fs.networks[0].name, "net");
    assert_eq!(fs.networks[0].uuid, "0391f203-6eef-4a69-bfcc-fffffbdfdc32");
    assert_eq!(fs.networks[0].ip_address.as_deref(), Some("10.10.10.50"));
    assert!(fs.networks[0].network_type.is_none());
    assert!(fs.networks[0].dns_name.is_none());

    // Shares
    assert_eq!(fs.shares.len(), 2);
    assert_eq!(fs.shares[0].name, "data");
    assert_eq!(fs.shares[0].path, "/data");
    assert_eq!(fs.shares[0].acl.len(), 1);
    assert_eq!(fs.shares[0].acl[0].name, "read");
    assert_eq!(fs.shares[0].acl[0].target, "*");
    assert_eq!(fs.shares[0].acl[0].permission, ShareACLPermission::Ro);
    assert!(fs.shares[0].deleting.is_none());
    assert_eq!(fs.shares[1].name, "proj");
    assert_eq!(fs.shares[1].path, "/project/data");
    assert_eq!(fs.shares[1].acl.len(), 1);
    assert_eq!(fs.shares[1].acl[0].name, "write");
    assert_eq!(fs.shares[1].acl[0].target, "10.10.10.0/24");
    assert_eq!(fs.shares[1].acl[0].permission, ShareACLPermission::Rw);
    assert!(fs.shares[1].deleting.is_none());

    // Labels
    let labels = fs.labels.as_ref().unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0].key, "environment");
    assert_eq!(labels[0].value, "production");
}

#[test]
fn deserialize_file_storage_detail() {
    let fs: Service = serde_json::from_str(FS_DETAIL_JSON).unwrap();
    assert_eq!(fs.uuid, "17871d62-6964-4336-b68d-4a85a8ee10fe");
    assert_eq!(fs.name, "test2");
    assert_eq!(fs.zone, "fi-hel2");
    assert!(fs.plan.is_none());
    assert_eq!(fs.configured_status, Some(ConfiguredStatus::Started));
    assert_eq!(fs.operational_state.as_deref(), Some("running"));
    assert_eq!(fs.size_gib, Some(250));
    assert_eq!(fs.encrypted, Some(false));
    assert_eq!(
        fs.created_at.as_deref(),
        Some("2025-08-01T10:40:04.140473Z")
    );
    assert_eq!(
        fs.updated_at.as_deref(),
        Some("2025-08-03T19:23:52.350216Z")
    );
    assert!(fs.state_messages.is_empty());

    // Networks
    assert_eq!(fs.networks.len(), 1);
    assert_eq!(fs.networks[0].family, "IPv4");
    assert_eq!(fs.networks[0].name, "net");
    assert_eq!(fs.networks[0].uuid, "0391f203-6eef-4a69-bfcc-fffffbdfdc32");
    assert_eq!(fs.networks[0].ip_address.as_deref(), Some("10.10.10.50"));
    assert!(fs.networks[0].network_type.is_none());
    assert!(fs.networks[0].dns_name.is_none());

    // Shares
    assert_eq!(fs.shares.len(), 1);
    assert_eq!(fs.shares[0].name, "data");
    assert_eq!(fs.shares[0].path, "/data");
    assert_eq!(fs.shares[0].acl.len(), 1);
    assert_eq!(fs.shares[0].acl[0].name, "read");
    assert_eq!(fs.shares[0].acl[0].target, "*");
    assert_eq!(fs.shares[0].acl[0].permission, ShareACLPermission::Ro);
    assert!(fs.shares[0].deleting.is_none());

    // Labels
    let labels = fs.labels.as_ref().unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0].key, "environment");
    assert_eq!(labels[0].value, "production");
}

#[test]
fn deserialize_file_storage_network_list() {
    let nets: Vec<Network> = serde_json::from_str(FS_NETWORK_LIST_JSON).unwrap();
    assert_eq!(nets.len(), 1);
    assert_eq!(nets[0].family, "IPv4");
    assert_eq!(nets[0].name, "test-name");
    assert_eq!(nets[0].uuid, "03720d07-80a9-41ac-90b0-68a932109339");
    assert_eq!(nets[0].ip_address.as_deref(), Some("192.168.147.250"));
    assert!(nets[0].network_type.is_none());
    assert!(nets[0].dns_name.is_none());
}

#[test]
fn deserialize_file_storage_share_list() {
    let shares: Vec<Share> = serde_json::from_str(FS_SHARE_LIST_JSON).unwrap();
    assert_eq!(shares.len(), 2);
    assert_eq!(shares[0].name, "share1");
    assert_eq!(shares[0].path, "/share-1");
    assert_eq!(shares[0].acl.len(), 1);
    assert_eq!(shares[0].acl[0].name, "write-all");
    assert_eq!(shares[0].acl[0].target, "*");
    assert_eq!(shares[0].acl[0].permission, ShareACLPermission::Rw);
    assert!(shares[0].deleting.is_none());

    assert_eq!(shares[1].name, "share2");
    assert_eq!(shares[1].path, "/share-1/share-2");
    assert_eq!(shares[1].acl.len(), 1);
    assert_eq!(shares[1].acl[0].name, "read-only-all");
    assert_eq!(shares[1].acl[0].target, "*");
    assert_eq!(shares[1].acl[0].permission, ShareACLPermission::Ro);
    assert!(shares[1].deleting.is_none());
}

// ─── JSON constants from official docs (page 25) ─────────────────────────

// Source: https://developers.upcloud.com/1.3/25-file-storage/#create-service
const CREATE_FS_REQUEST_JSON: &str = r#"
    {
        "name": "test-name",
        "zone": "fi-hel2",
        "configured_status": "started",
        "size_gib": 250,
        "encrypted": false,
        "networks": [
            {
                "uuid": "03720d07-80a9-41ac-90b0-68a932109339",
                "name": "net",
                "family": "IPv4",
                "ip_address": "192.168.147.120"
            }
        ],
        "shares": [
            {
                "name": "data",
                "path": "/data",
                "acl": [
                    {
                        "name": "read",
                        "target": "*",
                        "permission": "ro"
                    }
                ]
            },
            {
                "name": "proj",
                "path": "/project/data",
                "acl": [
                    {
                        "name": "write",
                        "target": "192.168.147.0/24",
                        "permission": "rw"
                    }
                ]
            }
        ],
        "labels": [
            {
                "key": "environment",
                "value": "production"
            }
        ]
    }
"#;

// Source: https://developers.upcloud.com/1.3/25-file-storage/#modify-service
const REPLACE_FS_REQUEST_JSON: &str = r#"
    {
        "name": "test-name",
        "configured_status": "started",
        "size_gib": 250,
        "networks": [
            {
                "uuid": "03720d07-80a9-41ac-90b0-68a932109339",
                "name": "net",
                "family": "IPv4",
                "ip_address": "192.168.147.120"
            }
        ],
        "labels": [
            {
                "key": "environment",
                "value": "production"
            }
        ]
    }
"#;

// Source: https://developers.upcloud.com/1.3/25-file-storage/#modify-service
const MODIFY_FS_REQUEST_JSON: &str = r#"
    {
        "name": "test-name",
        "configured_status": "started",
        "size_gib": 250,
        "networks": [
            {
                "uuid": "03720d07-80a9-41ac-90b0-68a932109339",
                "name": "net",
                "family": "IPv4",
                "ip_address": "192.168.147.120"
            }
        ]
    }
"#;

// Source: https://developers.upcloud.com/1.3/25-file-storage/#create-network
const NETWORK_INPUT_JSON: &str = r#"
    {
        "uuid": "03720d07-80a9-41ac-90b0-68a932109339",
        "name": "test-name",
        "family": "IPv4",
        "ip_address": "10.20.30.40"
    }
"#;

// Source: https://developers.upcloud.com/1.3/25-file-storage/#list-networks
const NETWORK_DETAIL_SINGLE_JSON: &str = r#"
    {
        "family": "IPv4",
        "ip_address": "192.168.147.250",
        "name": "test-name",
        "uuid": "03720d07-80a9-41ac-90b0-68a932109339"
    }
"#;

// Source: https://developers.upcloud.com/1.3/25-file-storage/#list-shares
const SHARE_ACL_NESTED_JSON: &str = r#"
    [
        {
            "acl": [
                {
                    "name": "write-all",
                    "permission": "rw",
                    "target": "*"
                }
            ],
            "name": "share1",
            "path": "/share-1"
        },
        {
            "acl": [
                {
                    "name": "read-only-all",
                    "permission": "ro",
                    "target": "*"
                }
            ],
            "name": "share2",
            "path": "/share-1/share-2"
        }
    ]
"#;

// Source: https://developers.upcloud.com/1.3/25-file-storage/#list-shares
const SHARE_ACL_HOST_JSON: &str = r#"
    [
        {
            "acl": [
                {
                    "name": "acl1",
                    "permission": "rw",
                    "target": "192.168.147.2"
                }
            ],
            "name": "share1",
            "path": "/share-1"
        },
        {
            "acl": [
                {
                    "name": "acl1",
                    "permission": "ro",
                    "target": "*"
                }
            ],
            "name": "share2",
            "path": "/share-1/share-2"
        }
    ]
"#;

// Source: https://developers.upcloud.com/1.3/25-file-storage/#list-shares
const SHARE_ACL_EMPTY_JSON: &str = r#"
    [
        {
            "acl": [],
            "name": "no-access",
            "path": "/share-1"
        },
        {
            "acl": [
                {
                    "name": "acl1",
                    "permission": "rw",
                    "target": "*"
                }
            ],
            "name": "write",
            "path": "/share-2"
        }
    ]
"#;

// Source: https://developers.upcloud.com/1.3/25-file-storage/#create-share
const CREATE_SHARE_REQUEST_JSON: &str = r#"
    {
        "name": "data",
        "path": "/data",
        "acl": [
            {
                "name": "read",
                "target": "192.168.147.0/24",
                "permission": "ro"
            }
        ]
    }
"#;

// Source: https://developers.upcloud.com/1.3/25-file-storage/#list-shares
const SHARE_LIST_DELETING_JSON: &str = r#"
    [
        {
            "name": "proj",
            "path": "/project/data",
            "acl": [
                {
                    "name": "write",
                    "target": "*",
                    "permission": "rw"
                }
            ]
        },
        {
            "name": "logs",
            "path": "/project/logs",
            "deleting": true,
            "acl": [
                {
                    "name": "read",
                    "target": "192.168.147.0/24",
                    "permission": "ro"
                }
            ]
        }
    ]
"#;

// Source: https://developers.upcloud.com/1.3/25-file-storage/#get-share-details
const SHARE_DETAIL_JSON: &str = r#"
    {
        "acl": [
            {
                "name": "write",
                "permission": "rw",
                "target": "*"
            }
        ],
        "name": "proj",
        "path": "/project/data"
    }
"#;

// Source: https://developers.upcloud.com/1.3/25-file-storage/#modify-share
const MODIFY_SHARE_REQUEST_JSON: &str = r#"
    {
        "name": "test-name",
        "acl": [
            {
                "name": "read-only",
                "target": "192.168.25.0/24",
                "permission": "ro"
            }
        ]
    }
"#;

// Source: https://developers.upcloud.com/1.3/25-file-storage/
const ACL_LIST_JSON: &str = r#"
    [
        {
            "name": "read-only",
            "target": "192.168.147.0/24",
            "permission": "rw"
        },
        {
            "name": "guest",
            "target": "192.168.150.0/24",
            "permission": "ro"
        }
    ]
"#;

// Source: https://developers.upcloud.com/1.3/25-file-storage/
const ACL_DETAIL_JSON: &str = r#"
    {
        "name": "read-only",
        "permission": "rw",
        "target": "*"
    }
"#;

// Source: https://developers.upcloud.com/1.3/25-file-storage/
const CREATE_ACL_JSON: &str = r#"
    {
        "name": "read-only",
        "target": "192.168.25.0/24",
        "permission": "ro"
    }
"#;

// Source: https://developers.upcloud.com/1.3/25-file-storage/
const LABEL_LIST_JSON: &str = r#"
    [
        {
            "key": "env",
            "value": "staging"
        },
        {
            "key": "foo",
            "value": "bar"
        }
    ]
"#;

// ─── Test: Create FS service request (#1) ────────────────────────────────

// Source: https://developers.upcloud.com/1.3/25-file-storage/
const LABEL_DETAIL_JSON: &str = r#"
    {
        "key": "env",
        "value": "staging"
    }
"#;

#[test]
fn deserialize_create_file_storage_request() {
    let req: CreateRequest = serde_json::from_str(CREATE_FS_REQUEST_JSON).unwrap();
    assert_eq!(req.name, "test-name");
    assert_eq!(req.zone, "fi-hel2");
    assert_eq!(req.configured_status.as_deref(), Some("started"));
    assert_eq!(req.size_gib, Some(250));
    assert_eq!(req.encrypted, Some(false));
    assert_eq!(req.networks.len(), 1);
    assert_eq!(req.networks[0].uuid, "03720d07-80a9-41ac-90b0-68a932109339");
    assert_eq!(req.networks[0].name, "net");
    assert_eq!(req.networks[0].family, "IPv4");
    assert_eq!(
        req.networks[0].ip_address.as_deref(),
        Some("192.168.147.120")
    );
    assert!(req.networks[0].network_type.is_none());
    assert_eq!(req.shares.len(), 2);
    assert_eq!(req.shares[0].name, "data");
    assert_eq!(req.shares[0].path, "/data");
    assert_eq!(req.shares[0].acl.len(), 1);
    assert_eq!(req.shares[0].acl[0].name, "read");
    assert_eq!(req.shares[0].acl[0].target, "*");
    assert_eq!(req.shares[0].acl[0].permission, ShareACLPermission::Ro);
    assert_eq!(req.shares[1].name, "proj");
    assert_eq!(req.shares[1].path, "/project/data");
    assert_eq!(req.shares[1].acl.len(), 1);
    assert_eq!(req.shares[1].acl[0].name, "write");
    assert_eq!(req.shares[1].acl[0].target, "192.168.147.0/24");
    assert_eq!(req.shares[1].acl[0].permission, ShareACLPermission::Rw);
    let labels = req.labels.as_ref().unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0].key, "environment");
    assert_eq!(labels[0].value, "production");
}

// ─── Test: Replace FS service request (#2) ──────────────────────────────

#[test]
fn deserialize_replace_file_storage_request() {
    let val: serde_json::Value = serde_json::from_str(REPLACE_FS_REQUEST_JSON).unwrap();
    assert_eq!(val["name"], "test-name");
    assert_eq!(val["configured_status"], "started");
    assert_eq!(val["size_gib"], 250);
    assert!(val["networks"].is_array());
    assert_eq!(val["networks"].as_array().unwrap().len(), 1);
    assert_eq!(
        val["networks"][0]["uuid"],
        "03720d07-80a9-41ac-90b0-68a932109339"
    );
    assert_eq!(val["networks"][0]["name"], "net");
    assert_eq!(val["networks"][0]["family"], "IPv4");
    assert_eq!(val["networks"][0]["ip_address"], "192.168.147.120");
    assert!(val["labels"].is_array());
    assert_eq!(val["labels"].as_array().unwrap().len(), 1);
    assert_eq!(val["labels"][0]["key"], "environment");
    assert_eq!(val["labels"][0]["value"], "production");
}

// ─── Test: Modify FS service request (#3) ───────────────────────────────

#[test]
fn deserialize_modify_file_storage_request() {
    let req: ModifyRequest = serde_json::from_str(MODIFY_FS_REQUEST_JSON).unwrap();
    assert_eq!(req.name.as_deref(), Some("test-name"));
    assert_eq!(req.configured_status.as_deref(), Some("started"));
    assert_eq!(req.size_gib, Some(250));
    assert!(req.labels.is_none());
}

// ─── Test: Create / modify network request (#4, #6) ─────────────────────

#[test]
fn deserialize_network_input() {
    let net: NetworkInput = serde_json::from_str(NETWORK_INPUT_JSON).unwrap();
    assert_eq!(net.uuid, "03720d07-80a9-41ac-90b0-68a932109339");
    assert_eq!(net.name, "test-name");
    assert_eq!(net.family, "IPv4");
    assert_eq!(net.ip_address.as_deref(), Some("10.20.30.40"));
    assert!(net.network_type.is_none());
}

// ─── Test: Network detail single (#5) ────────────────────────────────────

#[test]
fn deserialize_network_detail_single() {
    let net: Network = serde_json::from_str(NETWORK_DETAIL_SINGLE_JSON).unwrap();
    assert_eq!(net.family, "IPv4");
    assert_eq!(net.ip_address.as_deref(), Some("192.168.147.250"));
    assert_eq!(net.name, "test-name");
    assert_eq!(net.uuid, "03720d07-80a9-41ac-90b0-68a932109339");
    assert!(net.network_type.is_none());
    assert!(net.dns_name.is_none());
}

// ─── Test: Share ACL nested (#7) ─────────────────────────────────────────

#[test]
fn deserialize_share_acl_nested() {
    let shares: Vec<Share> = serde_json::from_str(SHARE_ACL_NESTED_JSON).unwrap();
    assert_eq!(shares.len(), 2);
    assert_eq!(shares[0].name, "share1");
    assert_eq!(shares[0].path, "/share-1");
    assert_eq!(shares[0].acl.len(), 1);
    assert_eq!(shares[0].acl[0].name, "write-all");
    assert_eq!(shares[0].acl[0].target, "*");
    assert_eq!(shares[0].acl[0].permission, ShareACLPermission::Rw);
    assert!(shares[0].deleting.is_none());
    assert_eq!(shares[1].name, "share2");
    assert_eq!(shares[1].path, "/share-1/share-2");
    assert_eq!(shares[1].acl.len(), 1);
    assert_eq!(shares[1].acl[0].name, "read-only-all");
    assert_eq!(shares[1].acl[0].target, "*");
    assert_eq!(shares[1].acl[0].permission, ShareACLPermission::Ro);
    assert!(shares[1].deleting.is_none());
}

// ─── Test: Share ACL host-specific (#8) ──────────────────────────────────

#[test]
fn deserialize_share_acl_host_specific() {
    let shares: Vec<Share> = serde_json::from_str(SHARE_ACL_HOST_JSON).unwrap();
    assert_eq!(shares.len(), 2);
    assert_eq!(shares[0].name, "share1");
    assert_eq!(shares[0].path, "/share-1");
    assert_eq!(shares[0].acl.len(), 1);
    assert_eq!(shares[0].acl[0].name, "acl1");
    assert_eq!(shares[0].acl[0].target, "192.168.147.2");
    assert_eq!(shares[0].acl[0].permission, ShareACLPermission::Rw);
    assert!(shares[0].deleting.is_none());
    assert_eq!(shares[1].name, "share2");
    assert_eq!(shares[1].path, "/share-1/share-2");
    assert_eq!(shares[1].acl.len(), 1);
    assert_eq!(shares[1].acl[0].name, "acl1");
    assert_eq!(shares[1].acl[0].target, "*");
    assert_eq!(shares[1].acl[0].permission, ShareACLPermission::Ro);
    assert!(shares[1].deleting.is_none());
}

// ─── Test: Share ACL empty + non-empty (#9) ─────────────────────────────

#[test]
fn deserialize_share_acl_empty() {
    let shares: Vec<Share> = serde_json::from_str(SHARE_ACL_EMPTY_JSON).unwrap();
    assert_eq!(shares.len(), 2);
    assert_eq!(shares[0].name, "no-access");
    assert_eq!(shares[0].path, "/share-1");
    assert!(shares[0].acl.is_empty());
    assert!(shares[0].deleting.is_none());
    assert_eq!(shares[1].name, "write");
    assert_eq!(shares[1].path, "/share-2");
    assert_eq!(shares[1].acl.len(), 1);
    assert_eq!(shares[1].acl[0].name, "acl1");
    assert_eq!(shares[1].acl[0].target, "*");
    assert_eq!(shares[1].acl[0].permission, ShareACLPermission::Rw);
    assert!(shares[1].deleting.is_none());
}

// ─── Test: Create share request (#10) ────────────────────────────────────

#[test]
fn deserialize_create_share_request() {
    let req: CreateShareRequest = serde_json::from_str(CREATE_SHARE_REQUEST_JSON).unwrap();
    assert_eq!(req.name, "data");
    assert_eq!(req.path, "/data");
    assert_eq!(req.acl.len(), 1);
    assert_eq!(req.acl[0].name, "read");
    assert_eq!(req.acl[0].target, "192.168.147.0/24");
    assert_eq!(req.acl[0].permission, ShareACLPermission::Ro);
}

// ─── Test: Share list with deleting flag (#11) ──────────────────────────

#[test]
fn deserialize_share_list_with_deleting() {
    let shares: Vec<Share> = serde_json::from_str(SHARE_LIST_DELETING_JSON).unwrap();
    assert_eq!(shares.len(), 2);
    assert_eq!(shares[0].name, "proj");
    assert_eq!(shares[0].path, "/project/data");
    assert_eq!(shares[0].acl.len(), 1);
    assert_eq!(shares[0].acl[0].name, "write");
    assert_eq!(shares[0].acl[0].target, "*");
    assert_eq!(shares[0].acl[0].permission, ShareACLPermission::Rw);
    assert!(shares[0].deleting.is_none());
    assert_eq!(shares[1].name, "logs");
    assert_eq!(shares[1].path, "/project/logs");
    assert_eq!(shares[1].deleting, Some(true));
    assert_eq!(shares[1].acl.len(), 1);
    assert_eq!(shares[1].acl[0].name, "read");
    assert_eq!(shares[1].acl[0].target, "192.168.147.0/24");
    assert_eq!(shares[1].acl[0].permission, ShareACLPermission::Ro);
}

// ─── Test: Share detail (#12) ────────────────────────────────────────────

#[test]
fn deserialize_share_detail() {
    let share: Share = serde_json::from_str(SHARE_DETAIL_JSON).unwrap();
    assert_eq!(share.name, "proj");
    assert_eq!(share.path, "/project/data");
    assert_eq!(share.acl.len(), 1);
    assert_eq!(share.acl[0].name, "write");
    assert_eq!(share.acl[0].target, "*");
    assert_eq!(share.acl[0].permission, ShareACLPermission::Rw);
    assert!(share.deleting.is_none());
}

// ─── Test: Modify share request (#13) ────────────────────────────────────

#[test]
fn deserialize_modify_share_request() {
    let req: ModifyShareRequest = serde_json::from_str(MODIFY_SHARE_REQUEST_JSON).unwrap();
    assert_eq!(req.name.as_deref(), Some("test-name"));
    let acl = req.acl.as_ref().unwrap();
    assert_eq!(acl.len(), 1);
    assert_eq!(acl[0].name, "read-only");
    assert_eq!(acl[0].target, "192.168.25.0/24");
    assert_eq!(acl[0].permission, ShareACLPermission::Ro);
}

// ─── Test: ACL list (#14) ────────────────────────────────────────────────

#[test]
fn deserialize_acl_list() {
    let acls: Vec<Acl> = serde_json::from_str(ACL_LIST_JSON).unwrap();
    assert_eq!(acls.len(), 2);
    assert_eq!(acls[0].name, "read-only");
    assert_eq!(acls[0].target, "192.168.147.0/24");
    assert_eq!(acls[0].permission, ShareACLPermission::Rw);
    assert_eq!(acls[1].name, "guest");
    assert_eq!(acls[1].target, "192.168.150.0/24");
    assert_eq!(acls[1].permission, ShareACLPermission::Ro);
}

// ─── Test: ACL detail (#15) ──────────────────────────────────────────────

#[test]
fn deserialize_acl_detail() {
    let acl: Acl = serde_json::from_str(ACL_DETAIL_JSON).unwrap();
    assert_eq!(acl.name, "read-only");
    assert_eq!(acl.target, "*");
    assert_eq!(acl.permission, ShareACLPermission::Rw);
}

// ─── Test: Create/modify ACL (#16) ───────────────────────────────────────

#[test]
fn deserialize_create_acl() {
    let acl: CreateShareAcl = serde_json::from_str(CREATE_ACL_JSON).unwrap();
    assert_eq!(acl.name, "read-only");
    assert_eq!(acl.target, "192.168.25.0/24");
    assert_eq!(acl.permission, ShareACLPermission::Ro);
}

// ─── Test: Label list (#17) ──────────────────────────────────────────────

#[test]
fn deserialize_label_list() {
    let labels: Vec<crate::types::common::Label> = serde_json::from_str(LABEL_LIST_JSON).unwrap();
    assert_eq!(labels.len(), 2);
    assert_eq!(labels[0].key, "env");
    assert_eq!(labels[0].value, "staging");
    assert_eq!(labels[1].key, "foo");
    assert_eq!(labels[1].value, "bar");
}

// ─── Test: Label detail (#18) ────────────────────────────────────────────

#[test]
fn deserialize_label_detail() {
    let label: crate::types::common::Label = serde_json::from_str(LABEL_DETAIL_JSON).unwrap();
    assert_eq!(label.key, "env");
    assert_eq!(label.value, "staging");
}

// ─── Test: Current state response (#19) ──────────────────────────────────

#[test]
fn deserialize_current_state_response() {
    let fs: Service = serde_json::from_str(FS_DETAIL_JSON).unwrap();
    assert_eq!(fs.uuid, "17871d62-6964-4336-b68d-4a85a8ee10fe");
    assert_eq!(fs.name, "test2");
    assert_eq!(fs.zone, "fi-hel2");
    assert!(fs.plan.is_none());
    assert_eq!(fs.configured_status, Some(ConfiguredStatus::Started));
    assert_eq!(fs.operational_state.as_deref(), Some("running"));
    assert_eq!(fs.size_gib, Some(250));
    assert_eq!(fs.encrypted, Some(false));
    assert_eq!(
        fs.created_at.as_deref(),
        Some("2025-08-01T10:40:04.140473Z")
    );
    assert_eq!(
        fs.updated_at.as_deref(),
        Some("2025-08-03T19:23:52.350216Z")
    );
    assert!(fs.state_messages.is_empty());
    assert_eq!(fs.networks.len(), 1);
    assert_eq!(fs.shares.len(), 1);
    let labels = fs.labels.as_ref().unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0].key, "environment");
    assert_eq!(labels[0].value, "production");
}
