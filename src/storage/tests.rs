use super::*;
// Source: https://developers.upcloud.com/1.3/9-storages/#list-storages
const LIST_STORAGES_JSON: &str = r#"
    {
        "storages": {
            "storage": [
                {
                    "access": "private",
                    "encrypted": "no",
                    "labels": [],
                    "license": 0,
                    "size": 10,
                    "state": "online",
                    "tier": "hdd",
                    "title": "Operating system disk",
                    "type": "normal",
                    "uuid": "01eff7ad-168e-413e-83b0-054f6a28fa23",
                    "zone": "uk-lon1"
                },
                {
                    "access": "private",
                    "encrypted": "no",
                    "labels": [],
                    "created": "2019-09-17T14:34:43Z",
                    "license": 0,
                    "origin": "01eff7ad-168e-413e-83b0-054f6a28fa23",
                    "size": 10,
                    "state": "online",
                    "title": "On demand backup",
                    "type": "backup",
                    "uuid": "01287ad1-496c-4b5f-bb67-0fc2e3494740",
                    "zone": "uk-lon1"
                },
                {
                    "access": "private",
                    "encrypted": "no",
                    "labels": [],
                    "license": 0,
                    "part_of_plan": "yes",
                    "size": 50,
                    "state": "online",
                    "tier": "maxiops",
                    "title": "Databases",
                    "type": "normal",
                    "uuid": "01f3286c-a5ea-4670-8121-d0b9767d625b",
                    "zone": "fi-hel1"
                },
                {
                    "access": "public",
                    "encrypted": "no",
                    "labels": [],
                    "license": 0,
                    "size": 5,
                    "state": "online",
                    "template_type": "cloud-init",
                    "title": "Ubuntu Server 22.04 LTS (Jammy Jellyfish)",
                    "type": "template",
                    "uuid": "01000000-0000-4000-8000-000030220200"
                }
            ]
        }
    }
"#;

#[test]
fn deserialize_storage_list() {
    let resp: StorageListResponse = serde_json::from_str(LIST_STORAGES_JSON).unwrap();
    let storages = resp.storages.storage;
    assert_eq!(storages.len(), 4);

    // storages[0]
    assert_eq!(storages[0].uuid, "01eff7ad-168e-413e-83b0-054f6a28fa23");
    assert_eq!(storages[0].access, Access::Private);
    assert_eq!(storages[0].encrypted, Some(false));
    assert!(storages[0].labels.as_ref().unwrap().is_empty());
    assert_eq!(storages[0].license, Some(0.0));
    assert_eq!(storages[0].size, 10);
    assert_eq!(storages[0].state, State::Online);
    assert_eq!(storages[0].tier, Some(Tier::Hdd));
    assert_eq!(storages[0].title, "Operating system disk");
    assert_eq!(storages[0].storage_type, Kind::Normal);
    assert_eq!(storages[0].zone.as_deref(), Some("uk-lon1"));
    assert!(storages[0].origin.is_none());
    assert!(storages[0].part_of_plan.is_none());
    assert!(storages[0].created.is_none());
    assert!(storages[0].template_type.is_none());
    assert!(storages[0].progress.is_none());

    // storages[1] — Backup without tier
    assert_eq!(storages[1].uuid, "01287ad1-496c-4b5f-bb67-0fc2e3494740");
    assert_eq!(storages[1].access, Access::Private);
    assert_eq!(storages[1].encrypted, Some(false));
    assert!(storages[1].labels.as_ref().unwrap().is_empty());
    assert_eq!(storages[1].license, Some(0.0));
    assert_eq!(
        storages[1].origin.as_deref(),
        Some("01eff7ad-168e-413e-83b0-054f6a28fa23")
    );
    assert_eq!(storages[1].size, 10);
    assert_eq!(storages[1].state, State::Online);
    assert_eq!(storages[1].title, "On demand backup");
    assert_eq!(storages[1].storage_type, Kind::Backup);
    assert_eq!(storages[1].zone.as_deref(), Some("uk-lon1"));
    assert_eq!(storages[1].created.as_deref(), Some("2019-09-17T14:34:43Z"));
    assert!(storages[1].tier.is_none());
    assert!(storages[1].part_of_plan.is_none());
    assert!(storages[1].template_type.is_none());
    assert!(storages[1].progress.is_none());

    // storages[2] — Part of plan
    assert_eq!(storages[2].uuid, "01f3286c-a5ea-4670-8121-d0b9767d625b");
    assert_eq!(storages[2].access, Access::Private);
    assert_eq!(storages[2].encrypted, Some(false));
    assert!(storages[2].labels.as_ref().unwrap().is_empty());
    assert_eq!(storages[2].license, Some(0.0));
    assert_eq!(storages[2].part_of_plan, Some(true));
    assert_eq!(storages[2].size, 50);
    assert_eq!(storages[2].state, State::Online);
    assert_eq!(storages[2].tier, Some(Tier::Maxiops));
    assert_eq!(storages[2].title, "Databases");
    assert_eq!(storages[2].storage_type, Kind::Normal);
    assert_eq!(storages[2].zone.as_deref(), Some("fi-hel1"));
    assert!(storages[2].origin.is_none());
    assert!(storages[2].created.is_none());
    assert!(storages[2].template_type.is_none());
    assert!(storages[2].progress.is_none());

    // storages[3] — Template without tier or zone
    assert_eq!(storages[3].uuid, "01000000-0000-4000-8000-000030220200");
    assert_eq!(storages[3].access, Access::Public);
    assert_eq!(storages[3].encrypted, Some(false));
    assert!(storages[3].labels.as_ref().unwrap().is_empty());
    assert_eq!(storages[3].license, Some(0.0));
    assert_eq!(storages[3].size, 5);
    assert_eq!(storages[3].state, State::Online);
    assert_eq!(storages[3].template_type.as_deref(), Some("cloud-init"));
    assert_eq!(
        storages[3].title,
        "Ubuntu Server 22.04 LTS (Jammy Jellyfish)"
    );
    assert_eq!(storages[3].storage_type, Kind::Template);
    assert!(storages[3].tier.is_none());
    assert!(storages[3].zone.is_none());
    assert!(storages[3].origin.is_none());
    assert!(storages[3].part_of_plan.is_none());
    assert!(storages[3].created.is_none());
    assert!(storages[3].progress.is_none());
}
// Source: https://developers.upcloud.com/1.3/9-storages/#get-storage-details
const GET_STORAGE_JSON: &str = r#"
    {
        "storage": {
            "access": "private",
            "encrypted": "yes",
            "backup_rule": "",
            "backups": {
                "backup": []
            },
            "labels": [
                {
                    "key": "env",
                    "value": "prod"
                }
            ],
            "license": 0,
            "servers": {
                "server": [
                    "00798b85-efdc-41ca-8021-f6ef457b8531"
                ]
            },
            "size": 10,
            "state": "online",
            "tier": "maxiops",
            "title": "Operating system disk",
            "type": "normal",
            "uuid": "01d4fcd4-e446-433b-8a9c-551a1284952e",
            "zone": "fi-hel1"
        }
    }
"#;

#[test]
fn deserialize_storage_detail_empty_backup_rule() {
    let resp: StorageResponse = serde_json::from_str(GET_STORAGE_JSON).unwrap();
    let s = resp.storage;
    assert_eq!(s.uuid, "01d4fcd4-e446-433b-8a9c-551a1284952e");
    assert_eq!(s.access, Access::Private);
    assert_eq!(s.encrypted, Some(true));
    // backup_rule is "" in the JSON → deserialized as None
    assert!(s.backup_rule.is_none());
    assert!(s.backups.as_ref().unwrap().backup.is_empty());
    assert_eq!(s.labels.as_ref().unwrap().len(), 1);
    assert_eq!(s.labels.as_ref().unwrap()[0].key, "env");
    assert_eq!(s.labels.as_ref().unwrap()[0].value, "prod");
    assert_eq!(s.license, Some(0.0));
    assert_eq!(s.servers.as_ref().unwrap().server.len(), 1);
    assert_eq!(
        s.servers.as_ref().unwrap().server[0],
        "00798b85-efdc-41ca-8021-f6ef457b8531"
    );
    assert_eq!(s.size, 10);
    assert_eq!(s.state, State::Online);
    assert_eq!(s.tier, Some(Tier::Maxiops));
    assert_eq!(s.title, "Operating system disk");
    assert_eq!(s.storage_type, Kind::Normal);
    assert_eq!(s.zone.as_deref(), Some("fi-hel1"));
}
// Source: https://developers.upcloud.com/1.3/9-storages/#create-storage
const CREATE_STORAGE_RESPONSE_JSON: &str = r#"
    {
        "storage": {
            "access": "private",
            "encrypted": "no",
            "backup_rule": {
                "interval": "daily",
                "time": "0430",
                "retention": "365"
            },
            "backups": {
                "backup": []
            },
            "labels": [
                {
                    "key": "env",
                    "value": "prod"
                }
            ],
            "license": 0,
            "servers": {
                "server": []
            },
            "size": 10,
            "state": "online",
            "tier": "maxiops",
            "title": "test",
            "type": "normal",
            "uuid": "01d4fcd4-e446-433b-8a9c-551a1284952e",
            "zone": "fi-hel1"
        }
    }
"#;

#[test]
fn deserialize_storage_with_backup_rule() {
    let resp: StorageResponse = serde_json::from_str(CREATE_STORAGE_RESPONSE_JSON).unwrap();
    let s = resp.storage;
    assert_eq!(s.uuid, "01d4fcd4-e446-433b-8a9c-551a1284952e");
    assert_eq!(s.access, Access::Private);
    assert_eq!(s.encrypted, Some(false));
    assert_eq!(s.labels.as_ref().unwrap().len(), 1);
    assert_eq!(s.labels.as_ref().unwrap()[0].key, "env");
    assert_eq!(s.labels.as_ref().unwrap()[0].value, "prod");
    assert_eq!(s.license, Some(0.0));
    assert!(s.servers.as_ref().unwrap().server.is_empty());
    assert_eq!(s.size, 10);
    assert_eq!(s.state, State::Online);
    assert_eq!(s.tier, Some(Tier::Maxiops));
    assert_eq!(s.title, "test");
    assert_eq!(s.storage_type, Kind::Normal);
    assert_eq!(s.zone.as_deref(), Some("fi-hel1"));
    assert!(s.backups.as_ref().unwrap().backup.is_empty());
    let rule = s.backup_rule.unwrap();
    assert_eq!(rule.interval.as_deref(), Some("daily"));
    assert_eq!(rule.time.as_deref(), Some("0430"));
    assert_eq!(rule.retention.as_deref(), Some("365"));
}

// ── Test-only wrapper types ──

#[derive(Debug, Deserialize)]
struct ResizeBackupTestResponse {
    resize_backup: StorageDetails,
}

// ── Create storage (docs: POST /1.3/storage) ──
// Source: https://developers.upcloud.com/1.3/9-storages/#create-storage
const CREATE_STORAGE_REQUEST_JSON: &str = r#"
    {
        "storage": {
            "encrypted": "yes",
            "size": "10",
            "tier": "maxiops",
            "title": "My data collection",
            "zone": "fi-hel1",
            "labels": [
                {
                    "key": "env",
                    "value": "prod"
                }
            ],
            "backup_rule": {
                "interval": "daily",
                "time": "0430",
                "retention": "365"
            }
        }
    }
"#;

#[test]
fn deserialize_create_storage_request() {
    let v: serde_json::Value = serde_json::from_str(CREATE_STORAGE_REQUEST_JSON).unwrap();
    let s = &v["storage"];
    assert_eq!(s["encrypted"], "yes");
    assert_eq!(s["size"], "10");
    assert_eq!(s["tier"], "maxiops");
    assert_eq!(s["title"], "My data collection");
    assert_eq!(s["zone"], "fi-hel1");
    assert_eq!(s["backup_rule"]["interval"], "daily");
    assert_eq!(s["backup_rule"]["time"], "0430");
    assert_eq!(s["backup_rule"]["retention"], "365");
    assert_eq!(s["labels"][0]["key"], "env");
    assert_eq!(s["labels"][0]["value"], "prod");
}
// Source: https://developers.upcloud.com/1.3/9-storages/#create-storage
const CREATE_STORAGE_RESPONSE_DOCS_JSON: &str = r#"
    {
        "storage": {
            "access": "private",
            "encrypted": "no",
            "backup_rule": {
                "interval": "daily",
                "hour": "0430",
                "retention": "365"
            },
            "backups": {
                "backup": []
            },
            "labels": [
                {
                    "key": "env",
                    "value": "prod"
                }
            ],
            "license": 0,
            "servers": {
                "server": []
            },
            "size": 10,
            "state": "online",
            "tier": "maxiops",
            "title": "test",
            "type": "normal",
            "uuid": "01d4fcd4-e446-433b-8a9c-551a1284952e",
            "zone": "fi-hel1"
        }
    }
"#;

#[test]
fn deserialize_create_storage_response_docs() {
    // The docs response uses "hour" instead of "time" in backup_rule
    let resp: StorageResponse = serde_json::from_str(CREATE_STORAGE_RESPONSE_DOCS_JSON).unwrap();
    let s = resp.storage;
    assert_eq!(s.uuid, "01d4fcd4-e446-433b-8a9c-551a1284952e");
    assert_eq!(s.access, Access::Private);
    assert_eq!(s.encrypted, Some(false));
    assert_eq!(s.state, State::Online);
    assert_eq!(s.storage_type, Kind::Normal);
    assert_eq!(s.size, 10);
    assert_eq!(s.tier, Some(Tier::Maxiops));
    assert_eq!(s.title, "test");
    assert_eq!(s.zone.as_deref(), Some("fi-hel1"));
    assert_eq!(s.license, Some(0.0));
    assert!(s.servers.as_ref().unwrap().server.is_empty());
    assert!(s.backups.as_ref().unwrap().backup.is_empty());
    assert_eq!(s.labels.as_ref().unwrap().len(), 1);
    assert_eq!(s.labels.as_ref().unwrap()[0].key, "env");
    assert_eq!(s.labels.as_ref().unwrap()[0].value, "prod");
    let rule = s.backup_rule.unwrap();
    assert_eq!(rule.time.as_deref(), Some("0430")); // deserialized from "hour" via alias
    assert_eq!(rule.interval.as_deref(), Some("daily"));
    assert_eq!(rule.retention.as_deref(), Some("365"));
}

// ── Modify storage (docs: PUT /1.3/storage/{uuid}) ──
// Source: https://developers.upcloud.com/1.3/9-storages/#modify-storage
#[allow(dead_code)]
const MODIFY_STORAGE_REQUEST_JSON: &str = r#"
    {
        "storage": {
            "size": "20",
            "title": "A larger storage"
        }
    }
"#;

#[test]
fn deserialize_modify_storage_request() {
    #[derive(Debug, Deserialize)]
    struct Wrapper {
        storage: ModifyRequest,
    }
    let w: Wrapper = serde_json::from_str(MODIFY_STORAGE_REQUEST_JSON).unwrap();
    let req = w.storage;
    assert_eq!(req.size, Some(20));
    assert_eq!(req.title.as_deref(), Some("A larger storage"));
    assert!(req.backup_rule.is_none());
}
// Source: https://developers.upcloud.com/1.3/9-storages/#modify-storage
const MODIFY_STORAGE_RESPONSE_JSON: &str = r#"
    {
        "storage": {
            "access": "private",
            "encrypted": "no",
            "backup_rule": "",
            "backups": {
                "backup": []
            },
            "license": 0,
            "servers": {
                "server": []
            },
            "size": 20,
            "state": "online",
            "tier": "maxiops",
            "title": "A larger storage",
            "type": "normal",
            "uuid": "011d671f-e803-484d-920a-c25b4bb05c01",
            "zone": "uk-lon1"
        }
    }
"#;

#[test]
fn deserialize_modify_storage_response() {
    let resp: StorageResponse = serde_json::from_str(MODIFY_STORAGE_RESPONSE_JSON).unwrap();
    let s = resp.storage;
    assert_eq!(s.uuid, "011d671f-e803-484d-920a-c25b4bb05c01");
    assert_eq!(s.access, Access::Private);
    assert_eq!(s.encrypted, Some(false));
    assert_eq!(s.title, "A larger storage");
    assert_eq!(s.size, 20);
    assert_eq!(s.state, State::Online);
    assert_eq!(s.storage_type, Kind::Normal);
    assert_eq!(s.tier, Some(Tier::Maxiops));
    assert_eq!(s.zone.as_deref(), Some("uk-lon1"));
    assert_eq!(s.license, Some(0.0));
    assert!(s.servers.as_ref().unwrap().server.is_empty());
    assert!(s.backups.as_ref().unwrap().backup.is_empty());
    assert!(s.backup_rule.is_none());
    assert!(s.labels.is_none());
}

// ── Resize storage (docs: POST /1.3/storage/{uuid}/resize) ──
// Source: https://developers.upcloud.com/1.3/9-storages/#resize-storage-partition-filesystem
const RESIZE_STORAGE_RESPONSE_JSON: &str = r#"
    {
        "resize_backup": {
            "access": "private",
            "created": "2021-12-03T06:25:15Z",
            "license": 0,
            "origin": "017ca4cc-def2-458d-a797-7782959b30a7",
            "servers": {
                "server": []
            },
            "size": 30,
            "state": "online",
            "title": "Resize Backup",
            "type": "backup",
            "uuid": "01beec3a-14ac-4f71-9c63-3338341121c3",
            "zone": "uk-lon1"
        }
    }
"#;

#[test]
fn deserialize_resize_storage_response() {
    let resp: ResizeBackupTestResponse =
        serde_json::from_str(RESIZE_STORAGE_RESPONSE_JSON).unwrap();
    let s = resp.resize_backup;
    assert_eq!(s.uuid, "01beec3a-14ac-4f71-9c63-3338341121c3");
    assert_eq!(s.access, Access::Private);
    assert_eq!(s.storage_type, Kind::Backup);
    assert_eq!(s.state, State::Online);
    assert_eq!(s.title, "Resize Backup");
    assert_eq!(s.size, 30);
    assert_eq!(s.zone.as_deref(), Some("uk-lon1"));
    assert_eq!(s.license, Some(0.0));
    assert!(s.servers.as_ref().unwrap().server.is_empty());
    assert_eq!(
        s.origin.as_deref(),
        Some("017ca4cc-def2-458d-a797-7782959b30a7")
    );
    assert_eq!(s.created.as_deref(), Some("2021-12-03T06:25:15Z"));
    assert!(s.tier.is_none());
    assert!(s.encrypted.is_none());
    assert!(s.labels.is_none());
    assert!(s.backup_rule.is_none());
    assert!(s.backups.is_none());
    assert!(s.template_type.is_none());
    assert!(s.progress.is_none());
    assert!(s.part_of_plan.is_none());
}

// ── Attach storage via Server API (docs: POST /1.3/server/{uuid}/storage/attach) ──
// Source: https://developers.upcloud.com/1.3/9-storages/
const ATTACH_STORAGE_SERVER_API_REQUEST_JSON: &str = r#"
    {
        "storage_device": {
            "type": "disk",
            "address": "scsi",
            "storage": "00798b85-efdc-41ca-8021-f6ef457b8531",
            "boot_disk": "0"
        }
    }
"#;

#[test]
fn deserialize_attach_storage_server_api_request() {
    let v: serde_json::Value =
        serde_json::from_str(ATTACH_STORAGE_SERVER_API_REQUEST_JSON).unwrap();
    let sd = &v["storage_device"];
    assert_eq!(sd["type"], "disk");
    assert_eq!(sd["address"], "scsi");
    assert_eq!(sd["storage"], "00798b85-efdc-41ca-8021-f6ef457b8531");
    assert_eq!(sd["boot_disk"], "0");
}
// Source: https://developers.upcloud.com/1.3/9-storages/
const ATTACH_STORAGE_SERVER_API_RESPONSE_JSON: &str = r#"
    {
        "server": {
            "boot_order": "disk",
            "core_number": "4",
            "firewall": "on",
            "hostname": "debian.example.com",
            "host": 8055964291,
            "ip_addresses": {
                "ip_address": [
                    {
                        "access": "private",
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
            "memory_amount": "8192",
            "nic_model": "virtio",
            "plan": "custom",
            "plan_ipv4_bytes": "0",
            "plan_ipv6_bytes": "0",
            "state": "started",
            "storage_devices": {
                "storage_device": [
                    {
                        "address": "scsi:0:0",
                        "storage": "00798b85-efdc-41ca-8021-f6ef457b8531",
                        "storage_size": "10",
                        "storage_title": "Debian from a template",
                        "storage_encrypted": "yes",
                        "type": "disk",
                        "boot_disk": "0"
                    }
                ]
            },
            "tags": {
                "tag": []
            },
            "timezone": "UTC",
            "title": "My Debian server",
            "uuid": "00c78863-db86-44ea-af70-d6edc4d162bf",
            "video_model": "cirrus",
            "vnc_password": "aabbccdd",
            "vnc": "off",
            "zone": "fi-hel1"
        }
    }
"#;

#[test]
fn deserialize_attach_storage_server_api_response() {
    let v: serde_json::Value =
        serde_json::from_str(ATTACH_STORAGE_SERVER_API_RESPONSE_JSON).unwrap();
    let server = &v["server"];
    assert_eq!(server["uuid"], "00c78863-db86-44ea-af70-d6edc4d162bf");
    assert_eq!(server["state"], "started");
    assert_eq!(server["boot_order"], "disk");
    assert_eq!(server["core_number"], "4");
    assert_eq!(server["firewall"], "on");
    assert_eq!(server["hostname"], "debian.example.com");
    assert_eq!(server["host"], 8055964291_u64);
    assert_eq!(
        server["ip_addresses"]["ip_address"]
            .as_array()
            .unwrap()
            .len(),
        3
    );
    assert_eq!(
        server["ip_addresses"]["ip_address"][0]["address"],
        "10.0.0.0"
    );
    assert_eq!(server["license"], 0);
    assert_eq!(server["memory_amount"], "8192");
    assert_eq!(server["nic_model"], "virtio");
    assert_eq!(server["plan"], "custom");
    assert_eq!(server["plan_ipv4_bytes"], "0");
    assert_eq!(server["plan_ipv6_bytes"], "0");
    assert!(server["tags"]["tag"].as_array().unwrap().is_empty());
    assert_eq!(server["timezone"], "UTC");
    assert_eq!(server["title"], "My Debian server");
    assert_eq!(server["video_model"], "cirrus");
    assert_eq!(server["vnc_password"], "aabbccdd");
    assert_eq!(server["vnc"], "off");
    assert_eq!(server["zone"], "fi-hel1");
    let devices = server["storage_devices"]["storage_device"]
        .as_array()
        .unwrap();
    assert_eq!(devices.len(), 1);
    assert_eq!(
        devices[0]["storage"],
        "00798b85-efdc-41ca-8021-f6ef457b8531"
    );
    assert_eq!(devices[0]["storage_encrypted"], "yes");
    assert_eq!(devices[0]["type"], "disk");
    assert_eq!(devices[0]["address"], "scsi:0:0");
    assert_eq!(devices[0]["storage_size"], "10");
    assert_eq!(devices[0]["storage_title"], "Debian from a template");
    assert_eq!(devices[0]["boot_disk"], "0");
}

// ── Attach storage via Storage API (docs: POST /1.3/storage/{uuid}/attach) ──
// Source: https://developers.upcloud.com/1.3/9-storages/
const ATTACH_STORAGE_STORAGE_API_REQUEST_JSON: &str = r#"
    {
        "storage_device": {
            "type": "disk",
            "address": "scsi",
            "server": "009d64ef-31d1-4684-a26b-c86c955cbf46",
            "boot_disk": "0"
        }
    }
"#;

#[test]
fn deserialize_attach_storage_storage_api_request() {
    let v: serde_json::Value =
        serde_json::from_str(ATTACH_STORAGE_STORAGE_API_REQUEST_JSON).unwrap();
    let sd = &v["storage_device"];
    assert_eq!(sd["type"], "disk");
    assert_eq!(sd["server"], "009d64ef-31d1-4684-a26b-c86c955cbf46");
    assert_eq!(sd["address"], "scsi");
    assert_eq!(sd["boot_disk"], "0");
}
// Source: https://developers.upcloud.com/1.3/9-storages/
const ATTACH_STORAGE_STORAGE_API_RESPONSE_JSON: &str = r#"
    {
        "storage": {
            "access": "private",
            "encrypted": "yes",
            "backup_rule": "",
            "backups": {
                "backup": []
            },
            "license": 0,
            "servers": {
                "server": [
                    "009d64ef-31d1-4684-a26b-c86c955cbf46"
                ]
            },
            "size": 10,
            "state": "online",
            "tier": "maxiops",
            "title": "Debian from a template",
            "type": "normal",
            "uuid": "00798b85-efdc-41ca-8021-f6ef457b8531",
            "zone": "fi-hel1"
        }
    }
"#;

#[test]
fn deserialize_attach_storage_storage_api_response() {
    let resp: StorageResponse =
        serde_json::from_str(ATTACH_STORAGE_STORAGE_API_RESPONSE_JSON).unwrap();
    let s = resp.storage;
    assert_eq!(s.uuid, "00798b85-efdc-41ca-8021-f6ef457b8531");
    assert_eq!(s.access, Access::Private);
    assert_eq!(s.encrypted, Some(true));
    assert!(s.backup_rule.is_none());
    assert_eq!(s.license, Some(0.0));
    assert_eq!(s.size, 10);
    assert_eq!(s.state, State::Online);
    assert_eq!(s.tier, Some(Tier::Maxiops));
    assert_eq!(s.title, "Debian from a template");
    assert_eq!(s.storage_type, Kind::Normal);
    assert_eq!(s.zone.as_deref(), Some("fi-hel1"));
    assert_eq!(s.servers.as_ref().unwrap().server.len(), 1);
    assert_eq!(
        s.servers.as_ref().unwrap().server[0],
        "009d64ef-31d1-4684-a26b-c86c955cbf46"
    );
    assert!(s.backups.as_ref().unwrap().backup.is_empty());
}

// ── Detach storage (docs: POST /1.3/server/{uuid}/storage/detach) ──
// Source: https://developers.upcloud.com/1.3/9-storages/
const DETACH_STORAGE_REQUEST_JSON: &str = r#"
    {
        "storage_device": {
            "address": "scsi:0:0"
        }
    }
"#;

#[test]
fn deserialize_detach_storage_request() {
    let v: serde_json::Value = serde_json::from_str(DETACH_STORAGE_REQUEST_JSON).unwrap();
    assert_eq!(v["storage_device"]["address"], "scsi:0:0");
}
// Source: https://developers.upcloud.com/1.3/9-storages/
const DETACH_STORAGE_SERVER_API_RESPONSE_JSON: &str = r#"
    {
        "server": {
            "boot_order": "disk",
            "core_number": "4",
            "firewall": "on",
            "hostname": "debian.example.com",
            "host": 8055964291,
            "ip_addresses": {
                "ip_address": [
                    {
                        "access": "private",
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
            "memory_amount": "8192",
            "nic_model": "virtio",
            "plan": "custom",
            "plan_ipv4_bytes": "0",
            "plan_ipv6_bytes": "0",
            "state": "stopped",
            "storage_devices": {
                "storage_device": []
            },
            "tags": {
                "tag": []
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

#[test]
fn deserialize_detach_storage_server_api_response() {
    let v: serde_json::Value =
        serde_json::from_str(DETACH_STORAGE_SERVER_API_RESPONSE_JSON).unwrap();
    let server = &v["server"];
    assert_eq!(server["uuid"], "00c78863-db86-44ea-af70-d6edc4d162bf");
    assert_eq!(server["state"], "stopped");
    assert_eq!(server["boot_order"], "disk");
    assert_eq!(server["core_number"], "4");
    assert_eq!(server["firewall"], "on");
    assert_eq!(server["hostname"], "debian.example.com");
    assert_eq!(server["host"], 8055964291_u64);
    assert_eq!(
        server["ip_addresses"]["ip_address"]
            .as_array()
            .unwrap()
            .len(),
        3
    );
    assert_eq!(
        server["ip_addresses"]["ip_address"][0]["address"],
        "10.0.0.0"
    );
    assert_eq!(server["license"], 0);
    assert_eq!(server["memory_amount"], "8192");
    assert_eq!(server["nic_model"], "virtio");
    assert_eq!(server["plan"], "custom");
    assert_eq!(server["plan_ipv4_bytes"], "0");
    assert_eq!(server["plan_ipv6_bytes"], "0");
    assert!(server["tags"]["tag"].as_array().unwrap().is_empty());
    assert_eq!(server["timezone"], "UTC");
    assert_eq!(server["title"], "My Debian server");
    assert_eq!(server["video_model"], "cirrus");
    assert_eq!(server["vnc"], "off");
    assert_eq!(server["vnc_password"], "aabbccdd");
    assert_eq!(server["zone"], "fi-hel1");
    let devices = server["storage_devices"]["storage_device"]
        .as_array()
        .unwrap();
    assert!(devices.is_empty());
}
// Source: https://developers.upcloud.com/1.3/9-storages/
const DETACH_STORAGE_STORAGE_API_RESPONSE_JSON: &str = r#"
    {
        "storage": {
            "access": "private",
            "encrypted": "no",
            "backup_rule": "",
            "backups": {
                "backup": []
            },
            "license": 0,
            "servers": {
                "server": []
            },
            "size": 10,
            "state": "online",
            "tier": "maxiops",
            "title": "Debian from a template",
            "type": "normal",
            "uuid": "00798b85-efdc-41ca-8021-f6ef457b8531",
            "zone": "fi-hel1"
        }
    }
"#;

#[test]
fn deserialize_detach_storage_storage_api_response() {
    let resp: StorageResponse =
        serde_json::from_str(DETACH_STORAGE_STORAGE_API_RESPONSE_JSON).unwrap();
    let s = resp.storage;
    assert_eq!(s.uuid, "00798b85-efdc-41ca-8021-f6ef457b8531");
    assert_eq!(s.access, Access::Private);
    assert_eq!(s.encrypted, Some(false));
    assert!(s.backup_rule.is_none());
    assert_eq!(s.license, Some(0.0));
    assert_eq!(s.size, 10);
    assert_eq!(s.state, State::Online);
    assert_eq!(s.tier, Some(Tier::Maxiops));
    assert_eq!(s.title, "Debian from a template");
    assert_eq!(s.storage_type, Kind::Normal);
    assert_eq!(s.zone.as_deref(), Some("fi-hel1"));
    assert!(s.servers.as_ref().unwrap().server.is_empty());
    assert!(s.backups.as_ref().unwrap().backup.is_empty());
}

// ── Load CD-ROM (docs: POST /1.3/server/{uuid}/cdrom/load) ──
// Source: https://developers.upcloud.com/1.3/9-storages/
const LOAD_CDROM_REQUEST_JSON: &str = r#"
    {
        "storage_device": {
            "storage": "01000000-0000-4000-8000-000060010101"
        }
    }
"#;

#[test]
fn deserialize_load_cdrom_request() {
    let v: serde_json::Value = serde_json::from_str(LOAD_CDROM_REQUEST_JSON).unwrap();
    assert_eq!(
        v["storage_device"]["storage"],
        "01000000-0000-4000-8000-000060010101"
    );
}
// Source: https://developers.upcloud.com/1.3/9-storages/
const LOAD_CDROM_RESPONSE_JSON: &str = r#"
    {
        "server": {
            "boot_order": "cdrom,disk",
            "core_number": "0",
            "firewall": "on",
            "hostname": "test.example.com",
            "host": 8055964291,
            "ip_addresses": {
                "ip_address": [
                    {
                        "access": "private",
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
            "memory_amount": "512",
            "nic_model": "virtio",
            "state": "started",
            "storage_devices": {
                "storage_device": [
                    {
                        "type": "cdrom",
                        "address": "ide:0:1",
                        "storage": "01000000-0000-4000-8000-000060010101"
                    },
                    {
                        "type": "disk",
                        "address": "ide:0:0",
                        "storage": "01d4fcd4-e446-433b-8a9c-551a1284952e"
                    }
                ]
            },
            "tags": {
                "tag": []
            },
            "timezone": "UTC",
            "title": "Example server",
            "uuid": "00798b85-efdc-41ca-8021-f6ef457b8531",
            "video_model": "cirrus",
            "vnc_password": "aabbccdd",
            "vnc": "off",
            "zone": "fi-hel1"
        }
    }
"#;

#[test]
fn deserialize_load_cdrom_response() {
    let v: serde_json::Value = serde_json::from_str(LOAD_CDROM_RESPONSE_JSON).unwrap();
    let server = &v["server"];
    assert_eq!(server["boot_order"], "cdrom,disk");
    assert_eq!(server["core_number"], "0");
    assert_eq!(server["firewall"], "on");
    assert_eq!(server["hostname"], "test.example.com");
    assert_eq!(server["host"], 8055964291_u64);
    assert_eq!(
        server["ip_addresses"]["ip_address"]
            .as_array()
            .unwrap()
            .len(),
        3
    );
    assert_eq!(server["license"], 0);
    assert_eq!(server["memory_amount"], "512");
    assert_eq!(server["nic_model"], "virtio");
    assert_eq!(server["state"], "started");
    assert!(server["tags"]["tag"].as_array().unwrap().is_empty());
    assert_eq!(server["timezone"], "UTC");
    assert_eq!(server["title"], "Example server");
    assert_eq!(server["uuid"], "00798b85-efdc-41ca-8021-f6ef457b8531");
    assert_eq!(server["video_model"], "cirrus");
    assert_eq!(server["vnc_password"], "aabbccdd");
    assert_eq!(server["vnc"], "off");
    assert_eq!(server["zone"], "fi-hel1");
    let devices = server["storage_devices"]["storage_device"]
        .as_array()
        .unwrap();
    assert_eq!(devices.len(), 2);
    assert_eq!(devices[0]["type"], "cdrom");
    assert_eq!(
        devices[0]["storage"],
        "01000000-0000-4000-8000-000060010101"
    );
    assert_eq!(devices[1]["type"], "disk");
    assert_eq!(devices[1]["address"], "ide:0:0");
    assert_eq!(
        devices[1]["storage"],
        "01d4fcd4-e446-433b-8a9c-551a1284952e"
    );
}

// ── Eject CD-ROM (docs: POST /1.3/server/{uuid}/cdrom/eject) ──
// Source: https://developers.upcloud.com/1.3/9-storages/
const EJECT_CDROM_RESPONSE_JSON: &str = r#"
    {
        "server": {
            "boot_order": "cdrom,disk",
            "core_number": "1",
            "firewall": "on",
            "hostname": "test.example.com",
            "ip_addresses": {
                "ip_address": [
                    {
                        "access": "private",
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
            "memory_amount": "512",
            "nic_model": "virtio",
            "state": "started",
            "storage_devices": {
                "storage_device": [
                    {
                        "type": "cdrom",
                        "address": "ide:0:1",
                        "storage": ""
                    },
                    {
                        "type": "disk",
                        "address": "ide:0:0",
                        "storage": "01d4fcd4-e446-433b-8a9c-551a1284952e"
                    }
                ]
            },
            "tags": {
                "tag": []
            },
            "timezone": "UTC",
            "title": "Example server",
            "uuid": "00798b85-efdc-41ca-8021-f6ef457b8531",
            "video_model": "cirrus",
            "vnc_password": "aabbccdd",
            "vnc": "off",
            "zone": "fi-hel1"
        }
    }
"#;

#[test]
fn deserialize_eject_cdrom_response() {
    let v: serde_json::Value = serde_json::from_str(EJECT_CDROM_RESPONSE_JSON).unwrap();
    let server = &v["server"];
    let devices = server["storage_devices"]["storage_device"]
        .as_array()
        .unwrap();
    assert_eq!(devices.len(), 2);
    // CD-ROM storage is empty after eject
    assert_eq!(devices[0]["type"], "cdrom");
    assert_eq!(devices[0]["storage"], "");
    assert_eq!(server["boot_order"], "cdrom,disk");
    assert_eq!(server["core_number"], "1");
    assert_eq!(server["firewall"], "on");
    assert_eq!(server["hostname"], "test.example.com");
    assert_eq!(
        server["ip_addresses"]["ip_address"]
            .as_array()
            .unwrap()
            .len(),
        3
    );
    assert_eq!(server["license"], 0);
    assert_eq!(server["memory_amount"], "512");
    assert_eq!(server["nic_model"], "virtio");
    assert_eq!(server["state"], "started");
    assert!(server["tags"]["tag"].as_array().unwrap().is_empty());
    assert_eq!(server["timezone"], "UTC");
    assert_eq!(server["title"], "Example server");
    assert_eq!(server["uuid"], "00798b85-efdc-41ca-8021-f6ef457b8531");
    assert_eq!(server["video_model"], "cirrus");
    assert_eq!(server["vnc"], "off");
    assert_eq!(server["vnc_password"], "aabbccdd");
    assert_eq!(server["zone"], "fi-hel1");
    assert_eq!(devices[1]["type"], "disk");
    assert_eq!(devices[1]["address"], "ide:0:0");
    assert_eq!(
        devices[1]["storage"],
        "01d4fcd4-e446-433b-8a9c-551a1284952e"
    );
}

// ── Clone storage (docs: POST /1.3/storage/{uuid}/clone) ──
// Source: https://developers.upcloud.com/1.3/9-storages/#clone-storage
const CLONE_STORAGE_REQUEST_JSON: &str = r#"
    {
        "storage": {
            "zone": "fi-hel1",
            "tier": "maxiops",
            "title": "Clone of operating system disk",
            "encrypted": "yes"
        }
    }
"#;

#[test]
fn deserialize_clone_storage_request() {
    let v: serde_json::Value = serde_json::from_str(CLONE_STORAGE_REQUEST_JSON).unwrap();
    let s = &v["storage"];
    assert_eq!(s["zone"], "fi-hel1");
    assert_eq!(s["tier"], "maxiops");
    assert_eq!(s["title"], "Clone of operating system disk");
    assert_eq!(s["encrypted"], "yes");
}
// Source: https://developers.upcloud.com/1.3/9-storages/#clone-storage
const CLONE_STORAGE_RESPONSE_JSON: &str = r#"
    {
        "storage": {
            "access": "private",
            "encrypted": "yes",
            "license": 0,
            "servers": {
                "server": []
            },
            "size": "10",
            "state": "maintenance",
            "tier": "maxiops",
            "title": "Clone of operating system disk",
            "type": "normal",
            "uuid": "01f3286c-a5ea-4670-8121-d0b9767d625b",
            "zone": "fi-hel1"
        }
    }
"#;

#[test]
fn deserialize_clone_storage_response() {
    let resp: StorageResponse = serde_json::from_str(CLONE_STORAGE_RESPONSE_JSON).unwrap();
    let s = resp.storage;
    assert_eq!(s.uuid, "01f3286c-a5ea-4670-8121-d0b9767d625b");
    assert_eq!(s.access, Access::Private);
    assert_eq!(s.state, State::Maintenance);
    assert_eq!(s.encrypted, Some(true));
    assert_eq!(s.license, Some(0.0));
    assert!(s.servers.as_ref().unwrap().server.is_empty());
    assert_eq!(s.tier, Some(Tier::Maxiops));
    assert_eq!(s.title, "Clone of operating system disk");
    assert_eq!(s.storage_type, Kind::Normal);
    assert_eq!(s.zone.as_deref(), Some("fi-hel1"));
    // size is a string "10" in the docs; PickFirst handles both
    assert_eq!(s.size, 10);
}

// ── Templatize storage (docs: POST /1.3/storage/{uuid}/templatize) ──
// Source: https://developers.upcloud.com/1.3/9-storages/#list-storages
#[allow(dead_code)]
const TEMPLATIZE_STORAGE_REQUEST_JSON: &str = r#"
    {
        "storage": {
            "title": "My server template"
        }
    }
"#;

#[test]
fn deserialize_templatize_storage_request() {
    #[derive(Debug, Deserialize)]
    struct Wrapper {
        storage: TemplatizeRequest,
    }
    let w: Wrapper = serde_json::from_str(TEMPLATIZE_STORAGE_REQUEST_JSON).unwrap();
    let req = w.storage;
    assert_eq!(req.title, "My server template");
}
// Source: https://developers.upcloud.com/1.3/9-storages/#list-storages
const TEMPLATIZE_STORAGE_RESPONSE_JSON: &str = r#"
    {
        "storage": {
            "access": "private",
            "license": 0,
            "servers": {
                "server": []
            },
            "size": 10,
            "state": "maintenance",
            "tier": "maxiops",
            "title": "My server template",
            "type": "template",
            "uuid": "012cfe08-ed18-4dce-93c0-c7e39e6276c8",
            "zone": "fi-hel1"
        }
    }
"#;

#[test]
fn deserialize_templatize_storage_response() {
    let resp: StorageResponse = serde_json::from_str(TEMPLATIZE_STORAGE_RESPONSE_JSON).unwrap();
    let s = resp.storage;
    assert_eq!(s.uuid, "012cfe08-ed18-4dce-93c0-c7e39e6276c8");
    assert_eq!(s.access, Access::Private);
    assert_eq!(s.storage_type, Kind::Template);
    assert_eq!(s.state, State::Maintenance);
    assert_eq!(s.license, Some(0.0));
    assert!(s.servers.as_ref().unwrap().server.is_empty());
    assert_eq!(s.size, 10);
    assert_eq!(s.tier, Some(Tier::Maxiops));
    assert_eq!(s.title, "My server template");
    assert_eq!(s.zone.as_deref(), Some("fi-hel1"));
}

// ── Create backup (docs: POST /1.3/storage/{uuid}/backup) ──
// Source: https://developers.upcloud.com/1.3/9-storages/#create-backup
const CREATE_BACKUP_REQUEST_JSON: &str = r#"
    {
        "storage": {
            "title": "Manually created backup"
        }
    }
"#;

#[test]
fn deserialize_create_backup_request() {
    #[derive(Debug, Deserialize)]
    struct Wrapper {
        storage: CreateBackupRequest,
    }
    let w: Wrapper = serde_json::from_str(CREATE_BACKUP_REQUEST_JSON).unwrap();
    let req = w.storage;
    assert_eq!(req.title.as_deref(), Some("Manually created backup"));
}
// Source: https://developers.upcloud.com/1.3/9-storages/#create-backup
const CREATE_BACKUP_RESPONSE_JSON: &str = r#"
    {
        "storage": {
            "access": "private",
            "created": "2015-03-27T10:02:05Z",
            "license": 0,
            "origin": "01eff7ad-168e-413e-83b0-054f6a28fa23",
            "progress": "0",
            "servers": {
                "server": []
            },
            "size": "10",
            "state": "maintenance",
            "title": "Manually created backup",
            "type": "normal",
            "uuid": "01f3286c-a5ea-4670-8121-d0b9767d625b",
            "zone": "fi-hel1"
        }
    }
"#;

#[test]
fn deserialize_create_backup_response() {
    let resp: StorageResponse = serde_json::from_str(CREATE_BACKUP_RESPONSE_JSON).unwrap();
    let s = resp.storage;
    assert_eq!(s.uuid, "01f3286c-a5ea-4670-8121-d0b9767d625b");
    assert_eq!(s.access, Access::Private);
    assert_eq!(
        s.origin.as_deref(),
        Some("01eff7ad-168e-413e-83b0-054f6a28fa23")
    );
    assert_eq!(s.progress.as_deref(), Some("0"));
    assert_eq!(s.state, State::Maintenance);
    assert_eq!(s.size, 10);
    assert_eq!(s.created.as_deref(), Some("2015-03-27T10:02:05Z"));
    assert_eq!(s.license, Some(0.0));
    assert!(s.servers.as_ref().unwrap().server.is_empty());
    assert_eq!(s.title, "Manually created backup");
    assert_eq!(s.storage_type, Kind::Normal);
    assert_eq!(s.zone.as_deref(), Some("fi-hel1"));
}

// ── Restore backup (docs: POST /1.3/storage/{uuid}/restore → 204 No Content) ──

#[test]
fn restore_backup_has_no_response_body() {
    // Restore returns 204 No Content — no JSON to deserialize.
    // This test documents that the endpoint has no response body.
    let empty = "";
    assert!(serde_json::from_str::<serde_json::Value>(empty).is_err());
}

// ── Create import (docs: POST /1.3/storage/{uuid}/import) ──
// Source: https://developers.upcloud.com/1.3/9-storages/#create-import
const CREATE_IMPORT_REQUEST_JSON: &str = r#"
    {
        "storage_import": {
            "source": "http_import",
            "source_location": "https://cdimage.debian.org/debian-cd/current/amd64/iso-cd/debian-10.4.0-amd64-netinst.iso"
        }
    }
"#;

#[test]
fn deserialize_create_import_request() {
    let v: serde_json::Value = serde_json::from_str(CREATE_IMPORT_REQUEST_JSON).unwrap();
    let si = &v["storage_import"];
    assert_eq!(si["source"], "http_import");
    assert!(si["source_location"].as_str().unwrap().contains("debian"));
    assert_eq!(
        si["source_location"],
        "https://cdimage.debian.org/debian-cd/current/amd64/iso-cd/debian-10.4.0-amd64-netinst.iso"
    );
}
// Source: https://developers.upcloud.com/1.3/9-storages/#create-import
const CREATE_IMPORT_HTTP_RESPONSE_JSON: &str = r#"
    {
        "storage_import": {
            "client_content_length": 0,
            "client_content_type": "",
            "completed": "",
            "created": "2020-06-26T08:51:07Z",
            "error_code": "",
            "error_message": "",
            "md5sum": "",
            "read_bytes": 0,
            "sha256sum": "",
            "source": "http_import",
            "source_location": "https://cdimage.debian.org/debian-cd/current/amd64/iso-cd/debian-10.4.0-amd64-netinst.iso",
            "state": "prepared",
            "uuid": "07a6c9a3-300e-4d0e-b935-624f3dbdff3f",
            "written_bytes": 0
        }
    }
"#;

#[test]
fn deserialize_create_import_http_response() {
    let resp: StorageImportResponse =
        serde_json::from_str(CREATE_IMPORT_HTTP_RESPONSE_JSON).unwrap();
    let si = resp.storage_import;
    assert_eq!(si.source.as_deref(), Some("http_import"));
    assert_eq!(si.state, Some(StorageImportState::Prepared));
    assert_eq!(
        si.uuid.as_deref(),
        Some("07a6c9a3-300e-4d0e-b935-624f3dbdff3f")
    );
    assert_eq!(si.client_content_length, Some(0));
    assert_eq!(si.client_content_type.as_deref(), Some(""));
    assert_eq!(si.completed.as_deref(), Some(""));
    assert_eq!(si.created.as_deref(), Some("2020-06-26T08:51:07Z"));
    assert_eq!(si.error_code.as_deref(), Some(""));
    assert_eq!(si.error_message.as_deref(), Some(""));
    assert_eq!(si.md5sum.as_deref(), Some(""));
    assert_eq!(si.read_bytes, Some(0));
    assert_eq!(si.sha256sum.as_deref(), Some(""));
    assert_eq!(
        si.source_location.as_deref(),
        Some(
            "https://cdimage.debian.org/debian-cd/current/amd64/iso-cd/debian-10.4.0-amd64-netinst.iso"
        )
    );
    assert_eq!(si.written_bytes, Some(0));
    assert!(si.direct_upload_url.is_none());
}
// Source: https://developers.upcloud.com/1.3/9-storages/#create-import
const CREATE_IMPORT_DIRECT_RESPONSE_JSON: &str = r#"
    {
        "storage_import": {
            "client_content_length": 0,
            "client_content_type": "",
            "completed": "",
            "created": "2020-06-26T08:51:07Z",
            "direct_upload_url": "https://fi-hel1.img.upcloud.com/uploader/session/07a6c9a3-300e-4d0e-b935-624f3dbdff3f",
            "error_code": "",
            "error_message": "",
            "md5sum": "",
            "read_bytes": 0,
            "sha256sum": "",
            "source": "direct_upload",
            "state": "prepared",
            "uuid": "07a6c9a3-300e-4d0e-b935-624f3dbdff3f",
            "written_bytes": 0
        }
    }
"#;

#[test]
fn deserialize_create_import_direct_response() {
    let resp: StorageImportResponse =
        serde_json::from_str(CREATE_IMPORT_DIRECT_RESPONSE_JSON).unwrap();
    let si = resp.storage_import;
    assert_eq!(si.source.as_deref(), Some("direct_upload"));
    assert_eq!(
        si.direct_upload_url.as_deref(),
        Some(
            "https://fi-hel1.img.upcloud.com/uploader/session/07a6c9a3-300e-4d0e-b935-624f3dbdff3f"
        )
    );
    assert!(si.source_location.is_none());
    assert_eq!(si.client_content_length, Some(0));
    assert_eq!(si.client_content_type.as_deref(), Some(""));
    assert_eq!(si.completed.as_deref(), Some(""));
    assert_eq!(si.created.as_deref(), Some("2020-06-26T08:51:07Z"));
    assert_eq!(si.error_code.as_deref(), Some(""));
    assert_eq!(si.error_message.as_deref(), Some(""));
    assert_eq!(si.md5sum.as_deref(), Some(""));
    assert_eq!(si.read_bytes, Some(0));
    assert_eq!(si.sha256sum.as_deref(), Some(""));
    assert_eq!(si.state, Some(StorageImportState::Prepared));
    assert_eq!(
        si.uuid.as_deref(),
        Some("07a6c9a3-300e-4d0e-b935-624f3dbdff3f")
    );
    assert_eq!(si.written_bytes, Some(0));
}

// ── Get import details (docs: GET /1.3/storage/{uuid}/import) ──
// Source: https://developers.upcloud.com/1.3/9-storages/#get-import-details
const GET_IMPORT_DETAILS_RESPONSE_JSON: &str = r#"
    {
        "storage_import": {
            "client_content_length": 0,
            "client_content_type": "",
            "completed": "",
            "created": "2020-06-26T08:51:07Z",
            "error_code": "",
            "error_message": "",
            "md5sum": "",
            "read_bytes": 0,
            "sha256sum": "",
            "source": "http_import",
            "source_location": "https://cdimage.debian.org/debian-cd/current/amd64/iso-cd/debian-10.4.0-amd64-netinst.iso",
            "state": "pending",
            "uuid": "07a6c9a3-300e-4d0e-b935-624f3dbdff3f",
            "written_bytes": 0
        }
    }
"#;

#[test]
fn deserialize_get_import_details_response() {
    let resp: StorageImportResponse =
        serde_json::from_str(GET_IMPORT_DETAILS_RESPONSE_JSON).unwrap();
    let si = resp.storage_import;
    assert_eq!(si.state, Some(StorageImportState::Pending));
    assert_eq!(si.created.as_deref(), Some("2020-06-26T08:51:07Z"));
    assert_eq!(si.error_code.as_deref(), Some(""));
    assert_eq!(si.error_message.as_deref(), Some(""));
    assert_eq!(si.client_content_length, Some(0));
    assert_eq!(si.client_content_type.as_deref(), Some(""));
    assert_eq!(si.completed.as_deref(), Some(""));
    assert_eq!(si.md5sum.as_deref(), Some(""));
    assert_eq!(si.read_bytes, Some(0));
    assert_eq!(si.sha256sum.as_deref(), Some(""));
    assert_eq!(si.source.as_deref(), Some("http_import"));
    assert_eq!(
        si.source_location.as_deref(),
        Some(
            "https://cdimage.debian.org/debian-cd/current/amd64/iso-cd/debian-10.4.0-amd64-netinst.iso"
        )
    );
    assert_eq!(
        si.uuid.as_deref(),
        Some("07a6c9a3-300e-4d0e-b935-624f3dbdff3f")
    );
    assert_eq!(si.written_bytes, Some(0));
}

// ── Cancel import (docs: POST /1.3/storage/{uuid}/import/cancel) ──
// Source: https://developers.upcloud.com/1.3/9-storages/#cancel-import
const CANCEL_IMPORT_RESPONSE_JSON: &str = r#"
    {
        "storage_import": {
            "client_content_length": 0,
            "client_content_type": "",
            "completed": "",
            "created": "2020-06-26T08:51:07Z",
            "error_code": "",
            "error_message": "",
            "md5sum": "",
            "read_bytes": 0,
            "sha256sum": "",
            "source": "http_import",
            "source_location": "https://cdimage.debian.org/debian-cd/current/amd64/iso-cd/debian-10.4.0-amd64-netinst.iso",
            "state": "cancelling",
            "uuid": "07a6c9a3-300e-4d0e-b935-624f3dbdff3f",
            "written_bytes": 0
        }
    }
"#;

#[test]
fn deserialize_cancel_import_response() {
    let resp: StorageImportResponse = serde_json::from_str(CANCEL_IMPORT_RESPONSE_JSON).unwrap();
    let si = resp.storage_import;
    assert_eq!(si.state, Some(StorageImportState::Cancelling));
    assert_eq!(si.client_content_length, Some(0));
    assert_eq!(si.client_content_type.as_deref(), Some(""));
    assert_eq!(si.completed.as_deref(), Some(""));
    assert_eq!(si.created.as_deref(), Some("2020-06-26T08:51:07Z"));
    assert_eq!(si.error_code.as_deref(), Some(""));
    assert_eq!(si.error_message.as_deref(), Some(""));
    assert_eq!(si.md5sum.as_deref(), Some(""));
    assert_eq!(si.read_bytes, Some(0));
    assert_eq!(si.sha256sum.as_deref(), Some(""));
    assert_eq!(si.source.as_deref(), Some("http_import"));
    assert_eq!(
        si.source_location.as_deref(),
        Some(
            "https://cdimage.debian.org/debian-cd/current/amd64/iso-cd/debian-10.4.0-amd64-netinst.iso"
        )
    );
    assert_eq!(
        si.uuid.as_deref(),
        Some("07a6c9a3-300e-4d0e-b935-624f3dbdff3f")
    );
    assert_eq!(si.written_bytes, Some(0));
}

// ─── ListParams query serialization tests ─────────────────────────────────────

#[test]
fn list_params_empty_by_default() {
    let params = ListParams::new();
    assert!(params.is_empty());
    assert!(params.to_query_pairs().is_empty());
}

#[test]
fn list_params_label_filters() {
    let params = ListParams::new()
        .label("env", "prod")
        .label("team", "backend");
    let pairs = params.to_query_pairs();
    assert!(!params.is_empty());
    assert_eq!(
        pairs,
        vec![
            ("label", "env=prod".to_string()),
            ("label", "team=backend".to_string()),
        ]
    );
}
