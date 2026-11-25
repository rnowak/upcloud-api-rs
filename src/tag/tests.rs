use super::*;
// Source: https://developers.upcloud.com/1.3/12-tags/#list-existing-tags
const LIST_TAGS_JSON: &str = r#"
    {
        "tags": {
            "tag": [
                {
                    "description": "Development servers",
                    "name": "DEV",
                    "servers": {
                        "server": [
                            "0077fa3d-32db-4b09-9f5f-30d9e9afb565"
                        ]
                    }
                },
                {
                    "description": "My own servers",
                    "name": "private",
                    "servers": {
                        "server": []
                    }
                },
                {
                    "description": "Production servers",
                    "name": "PROD",
                    "servers": {
                        "server": []
                    }
                }
            ]
        }
    }
"#;

#[test]
fn deserialize_tag_list() {
    let resp: TagListResponse = serde_json::from_str(LIST_TAGS_JSON).unwrap();
    let tags = resp.tags.tag;
    assert_eq!(tags.len(), 3);

    assert_eq!(tags[0].name, "DEV");
    assert_eq!(tags[0].description.as_deref(), Some("Development servers"));
    assert_eq!(tags[0].servers.server.len(), 1);
    assert_eq!(
        tags[0].servers.server[0],
        "0077fa3d-32db-4b09-9f5f-30d9e9afb565"
    );

    assert_eq!(tags[1].name, "private");
    assert_eq!(tags[1].description.as_deref(), Some("My own servers"));
    assert!(tags[1].servers.server.is_empty());

    assert_eq!(tags[2].name, "PROD");
    assert_eq!(tags[2].description.as_deref(), Some("Production servers"));
    assert!(tags[2].servers.server.is_empty());
}

// Source: https://developers.upcloud.com/1.3/12-tags/#create-a-new-tag
const CREATE_TAG_RESPONSE_JSON: &str = r#"
    {
        "tag": {
            "name": "DEV",
            "description": "Development servers",
            "servers": {
                "server": [
                    "0077fa3d-32db-4b09-9f5f-30d9e9afb565"
                ]
            }
        }
    }
"#;

#[test]
fn deserialize_tag_detail() {
    let resp: TagResponse = serde_json::from_str(CREATE_TAG_RESPONSE_JSON).unwrap();
    let tag = resp.tag;
    assert_eq!(tag.name, "DEV");
    assert_eq!(tag.servers.server.len(), 1);
    assert_eq!(
        tag.servers.server[0],
        "0077fa3d-32db-4b09-9f5f-30d9e9afb565"
    );
    assert_eq!(tag.description.as_deref(), Some("Development servers"));
}

// Source: https://developers.upcloud.com/1.3/12-tags/#create-a-new-tag
const CREATE_TAG_REQUEST_JSON: &str = r#"
    {
        "tag": {
            "name": "DEV",
            "description": "Development servers",
            "servers": {
                "server": [
                    "0077fa3d-32db-4b09-9f5f-30d9e9afb565"
                ]
            }
        }
    }
"#;

// ─── Create tag request ─────────────────────────────────────────────────

#[test]
fn deserialize_create_tag_request() {
    let resp: TagResponse = serde_json::from_str(CREATE_TAG_REQUEST_JSON).unwrap();
    let tag = resp.tag;
    assert_eq!(tag.name, "DEV");
    assert_eq!(tag.description.as_deref(), Some("Development servers"));
    assert_eq!(tag.servers.server.len(), 1);
    assert_eq!(
        tag.servers.server[0],
        "0077fa3d-32db-4b09-9f5f-30d9e9afb565"
    );
}

// Source: https://developers.upcloud.com/1.3/12-tags/#create-a-new-tag
const CREATE_TAG_RESP_JSON: &str = r#"
    {
        "tag": {
            "name": "DEV",
            "description": "Development servers",
            "servers": {
                "server": [
                    "0077fa3d-32db-4b09-9f5f-30d9e9afb565"
                ]
            }
        }
    }
"#;

// ─── Create tag response ────────────────────────────────────────────────

#[test]
fn deserialize_create_tag_response() {
    let resp: TagResponse = serde_json::from_str(CREATE_TAG_RESP_JSON).unwrap();
    assert_eq!(resp.tag.name, "DEV");
    assert_eq!(resp.tag.servers.server.len(), 1);
    assert_eq!(
        resp.tag.servers.server[0],
        "0077fa3d-32db-4b09-9f5f-30d9e9afb565"
    );
    assert_eq!(resp.tag.description.as_deref(), Some("Development servers"));
}

// Source: https://developers.upcloud.com/1.3/12-tags/#modify-existing-tag
const MODIFY_TAG_REQUEST_JSON: &str = r#"
    {
        "tag": {
            "name": "PROD",
            "description": "Production servers",
            "servers": {
                "server": [
                    "0077fa3d-32db-4b09-9f5f-30d9e9afb565"
                ]
            }
        }
    }
"#;

// ─── Modify tag request ─────────────────────────────────────────────────

#[test]
fn deserialize_modify_tag_request() {
    let resp: TagResponse = serde_json::from_str(MODIFY_TAG_REQUEST_JSON).unwrap();
    assert_eq!(resp.tag.name, "PROD");
    assert_eq!(resp.tag.description.as_deref(), Some("Production servers"));
    assert_eq!(resp.tag.servers.server.len(), 1);
    assert_eq!(
        resp.tag.servers.server[0],
        "0077fa3d-32db-4b09-9f5f-30d9e9afb565"
    );
}

// Source: https://developers.upcloud.com/1.3/12-tags/#modify-existing-tag
const MODIFY_TAG_RESP_JSON: &str = r#"
    {
        "tag": {
            "name": "PROD",
            "description": "Production servers",
            "servers": {
                "server": [
                    "0077fa3d-32db-4b09-9f5f-30d9e9afb565"
                ]
            }
        }
    }
"#;

// ─── Modify tag response ────────────────────────────────────────────────

#[test]
fn deserialize_modify_tag_response() {
    let resp: TagResponse = serde_json::from_str(MODIFY_TAG_RESP_JSON).unwrap();
    assert_eq!(resp.tag.name, "PROD");
    assert_eq!(resp.tag.servers.server.len(), 1);
    assert_eq!(resp.tag.description.as_deref(), Some("Production servers"));
    assert_eq!(
        resp.tag.servers.server[0],
        "0077fa3d-32db-4b09-9f5f-30d9e9afb565"
    );
}

// ─── Serialize create tag request ────────────────────────────────────────

#[test]
fn serialize_create_tag_request_with_servers() {
    let req = CreateTagRequest {
        name: "DEV".to_string(),
        description: Some("Development servers".to_string()),
        servers: Some(TagServers {
            server: vec!["0077fa3d-32db-4b09-9f5f-30d9e9afb565".to_string()],
        }),
    };
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["name"], "DEV");
    assert_eq!(json["description"], "Development servers");
    assert_eq!(
        json["servers"]["server"][0],
        "0077fa3d-32db-4b09-9f5f-30d9e9afb565"
    );
}

#[test]
fn serialize_create_tag_request_without_servers() {
    let req = CreateTagRequest {
        name: "DEV".to_string(),
        description: None,
        servers: None,
    };
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["name"], "DEV");
    assert!(json.get("servers").is_none());
    assert!(json.get("description").is_none());
}

// ─── Serialize modify tag request ───────────────────────────────────────

#[test]
fn serialize_modify_tag_request_with_servers() {
    let req = ModifyTagRequest {
        name: Some("PROD".to_string()),
        description: Some("Production servers".to_string()),
        servers: Some(TagServers {
            server: vec!["0077fa3d-32db-4b09-9f5f-30d9e9afb565".to_string()],
        }),
    };
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["name"], "PROD");
    assert_eq!(json["description"], "Production servers");
    assert_eq!(
        json["servers"]["server"][0],
        "0077fa3d-32db-4b09-9f5f-30d9e9afb565"
    );
}

#[test]
fn serialize_modify_tag_request_without_servers() {
    let req = ModifyTagRequest {
        name: Some("PROD".to_string()),
        description: None,
        servers: None,
    };
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["name"], "PROD");
    assert!(json.get("servers").is_none());
    assert!(json.get("description").is_none());
}

// Source: https://developers.upcloud.com/1.3/12-tags/#assign-tag-to-a-server
const ASSIGN_TAG_SERVER_JSON: &str = r#"
    {
        "server": {
            "boot_order": "disk",
            "core_number": "2",
            "firewall": "on",
            "hostname": "debian.example.com",
            "ip_addresses": {
                "ip_address": [
                    {
                        "access": "utility",
                        "address": "10.0.0.0",
                        "family": "IPv4"
                    },
                    {
                        "access": "public",
                        "address": "x.x.x.x",
                        "family": "IPv4"
                    },
                    {
                        "access": "public",
                        "address": "xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx",
                        "family": "IPv6"
                    }
                ]
            },
            "license": 0,
            "memory_amount": "4096",
            "nic_model": "virtio",
            "plan": "2xCPU-4GB",
            "plan_ipv4_bytes": "345346754423",
            "plan_ipv6_bytes": "676300234",
            "state": "started",
            "storage_devices": {
                "storage_device": [
                    {
                        "address": "virtio:0",
                        "part_of_plan": "yes",
                        "storage": "0169b4f8-051c-4a86-9484-f5b798249949",
                        "storage_size": 50,
                        "storage_title": "Debian from a template",
                        "type": "disk"
                    }
                ]
            },
            "tags": {
                "tag": [
                    "DEV",
                    "private",
                    "RHEL"
                ]
            },
            "timezone": "UTC",
            "title": "My Debian server",
            "uuid": "00c78863-db86-44ea-af70-d6edc4d162bf",
            "video_model": "cirrus",
            "vnc": "off",
            "vnc_password": "aabbccdd",
            "zone": "fi-hel1"
        }
    }
"#;

use crate::server::ServerDetails;

// ─── Assign tag to server response ──────────────────────────────────────

#[test]
fn deserialize_assign_tag_server_response() {
    #[derive(Debug, Deserialize)]
    struct Wrapper {
        server: ServerDetails,
    }
    let resp: Wrapper =
        serde_json::from_str(ASSIGN_TAG_SERVER_JSON).expect("deserialize assign tag response");
    let s = resp.server;
    assert_eq!(s.hostname, "debian.example.com");
    assert_eq!(s.uuid, "00c78863-db86-44ea-af70-d6edc4d162bf");
    assert_eq!(s.state, crate::server::State::Started);
    assert_eq!(s.plan, "2xCPU-4GB");

    let tags = &s.tags.as_ref().unwrap().tag;
    assert_eq!(tags.len(), 3);
    assert_eq!(tags[0], "DEV");
    assert_eq!(tags[1], "private");
    assert_eq!(tags[2], "RHEL");

    let ips = &s.ip_addresses.as_ref().unwrap().ip_address;
    assert_eq!(ips.len(), 3);
    assert_eq!(ips[0].access, crate::server::NetworkType::Utility);
    assert_eq!(ips[0].address, "10.0.0.0");
    assert_eq!(ips[0].family, crate::server::Family::IPv4);
    assert_eq!(ips[1].access, crate::server::NetworkType::Public);
    assert_eq!(ips[1].address, "x.x.x.x");
    assert_eq!(ips[1].family, crate::server::Family::IPv4);
    assert_eq!(ips[2].access, crate::server::NetworkType::Public);
    assert_eq!(ips[2].family, crate::server::Family::IPv6);

    assert_eq!(s.boot_order, Some(crate::server::BootOrder::Disk));
    assert_eq!(s.core_number, 2);
    assert_eq!(s.firewall, Some(true));
    assert_eq!(s.license, Some(0.0));
    assert_eq!(s.memory_amount, 4096);
    assert_eq!(s.nic_model, Some(crate::server::NicModel::Virtio));
    assert_eq!(s.plan_ipv4_bytes.as_deref(), Some("345346754423"));
    assert_eq!(s.plan_ipv6_bytes.as_deref(), Some("676300234"));
    assert_eq!(s.timezone.as_deref(), Some("UTC"));
    assert_eq!(s.title, "My Debian server");
    assert_eq!(s.video_model, Some(crate::server::VideoModel::Cirrus));
    assert_eq!(s.zone, "fi-hel1");

    let storages = &s.storage_devices.as_ref().unwrap().storage_device;
    assert_eq!(storages.len(), 1);
    assert_eq!(storages[0].storage_size, Some(50));
    assert_eq!(storages[0].address, "virtio:0");
    assert_eq!(storages[0].part_of_plan, Some(true));
    assert_eq!(
        storages[0].storage.as_deref(),
        Some("0169b4f8-051c-4a86-9484-f5b798249949")
    );
    assert_eq!(
        storages[0].storage_title.as_deref(),
        Some("Debian from a template")
    );
    assert_eq!(
        storages[0].device_type,
        Some(crate::server::StorageDeviceType::Disk)
    );
}

// Source: https://developers.upcloud.com/1.3/12-tags/#remove-tag-from-server
const REMOVE_TAG_SERVER_JSON: &str = r#"
    {
        "server": {
            "boot_order": "disk",
            "core_number": "2",
            "firewall": "on",
            "hostname": "debian.example.com",
            "ip_addresses": {
                "ip_address": [
                    {
                        "access": "utility",
                        "address": "10.0.0.0",
                        "family": "IPv4"
                    },
                    {
                        "access": "public",
                        "address": "x.x.x.x",
                        "family": "IPv4"
                    },
                    {
                        "access": "public",
                        "address": "xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx",
                        "family": "IPv6"
                    }
                ]
            },
            "license": 0,
            "memory_amount": "4096",
            "nic_model": "virtio",
            "plan": "2xCPU-4GB",
            "plan_ipv4_bytes": "345346754423",
            "plan_ipv6_bytes": "676300234",
            "state": "started",
            "storage_devices": {
                "storage_device": [
                    {
                        "address": "virtio:0",
                        "part_of_plan": "yes",
                        "storage": "0169b4f8-051c-4a86-9484-f5b798249949",
                        "storage_size": 50,
                        "storage_title": "Debian from a template",
                        "type": "disk"
                    }
                ]
            },
            "tags": {
                "tag": [
                    "DEV",
                    "RHEL"
                ]
            },
            "timezone": "UTC",
            "title": "My Debian server",
            "uuid": "00c78863-db86-44ea-af70-d6edc4d162bf",
            "video_model": "cirrus",
            "vnc": "off",
            "vnc_password": "aabbccdd",
            "zone": "fi-hel1"
        }
    }
"#;

// ─── Remove tag from server response ────────────────────────────────────

#[test]
fn deserialize_remove_tag_server_response() {
    #[derive(Debug, Deserialize)]
    struct Wrapper {
        server: ServerDetails,
    }
    let resp: Wrapper =
        serde_json::from_str(REMOVE_TAG_SERVER_JSON).expect("deserialize remove tag response");
    let s = resp.server;
    assert_eq!(s.uuid, "00c78863-db86-44ea-af70-d6edc4d162bf");

    let tags = &s.tags.as_ref().unwrap().tag;
    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0], "DEV");
    assert_eq!(tags[1], "RHEL");

    assert_eq!(s.boot_order, Some(crate::server::BootOrder::Disk));
    assert_eq!(s.core_number, 2);
    assert_eq!(s.firewall, Some(true));
    assert_eq!(s.hostname, "debian.example.com");
    assert_eq!(s.state, crate::server::State::Started);
    assert_eq!(s.plan, "2xCPU-4GB");
    assert_eq!(s.license, Some(0.0));
    assert_eq!(s.memory_amount, 4096);
    assert_eq!(s.nic_model, Some(crate::server::NicModel::Virtio));
    assert_eq!(s.plan_ipv4_bytes.as_deref(), Some("345346754423"));
    assert_eq!(s.plan_ipv6_bytes.as_deref(), Some("676300234"));
    assert_eq!(s.timezone.as_deref(), Some("UTC"));
    assert_eq!(s.title, "My Debian server");
    assert_eq!(s.video_model, Some(crate::server::VideoModel::Cirrus));
    assert_eq!(s.zone, "fi-hel1");

    let ips = &s.ip_addresses.as_ref().unwrap().ip_address;
    assert_eq!(ips.len(), 3);

    let storages = &s.storage_devices.as_ref().unwrap().storage_device;
    assert_eq!(storages.len(), 1);
    assert_eq!(storages[0].storage_size, Some(50));
}
