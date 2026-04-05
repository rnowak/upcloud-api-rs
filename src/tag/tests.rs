use super::*;

const LIST_TAGS_JSON: &str = r#"{
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
}"#;

const CREATE_TAG_RESPONSE_JSON: &str = r#"{
  "tag": {
"name": "DEV",
"description": "Development servers",
"servers": {
  "server": [
    "0077fa3d-32db-4b09-9f5f-30d9e9afb565"
  ]
}
  }
}"#;

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

// ─── Create tag request ─────────────────────────────────────────────────

const CREATE_TAG_REQUEST_JSON: &str = r#"{
  "tag": {
"name": "DEV",
"description": "Development servers",
"servers": {
  "server": [
    "0077fa3d-32db-4b09-9f5f-30d9e9afb565"
  ]
}
  }
}"#;

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

// ─── Create tag response ────────────────────────────────────────────────

const CREATE_TAG_RESP_JSON: &str = r#"{
  "tag": {
"name": "DEV",
"description": "Development servers",
"servers": {
  "server": [
    "0077fa3d-32db-4b09-9f5f-30d9e9afb565"
  ]
}
  }
}"#;

#[test]
fn deserialize_create_tag_response() {
    let resp: TagResponse = serde_json::from_str(CREATE_TAG_RESP_JSON).unwrap();
    assert_eq!(resp.tag.name, "DEV");
    assert_eq!(resp.tag.servers.server.len(), 1);
    assert_eq!(
        resp.tag.servers.server[0],
        "0077fa3d-32db-4b09-9f5f-30d9e9afb565"
    );
    assert_eq!(
        resp.tag.description.as_deref(),
        Some("Development servers")
    );
}

// ─── Modify tag request ─────────────────────────────────────────────────

const MODIFY_TAG_REQUEST_JSON: &str = r#"{
  "tag": {
"name": "PROD",
"description": "Production servers",
"servers": {
  "server": [
    "0077fa3d-32db-4b09-9f5f-30d9e9afb565"
  ]
}
  }
}"#;

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

// ─── Modify tag response ────────────────────────────────────────────────

const MODIFY_TAG_RESP_JSON: &str = r#"{
  "tag": {
"name": "PROD",
"description": "Production servers",
"servers": {
  "server": [
    "0077fa3d-32db-4b09-9f5f-30d9e9afb565"
  ]
}
  }
}"#;

#[test]
fn deserialize_modify_tag_response() {
    let resp: TagResponse = serde_json::from_str(MODIFY_TAG_RESP_JSON).unwrap();
    assert_eq!(resp.tag.name, "PROD");
    assert_eq!(resp.tag.servers.server.len(), 1);
    assert_eq!(
        resp.tag.description.as_deref(),
        Some("Production servers")
    );
    assert_eq!(
        resp.tag.servers.server[0],
        "0077fa3d-32db-4b09-9f5f-30d9e9afb565"
    );
}

// ─── Assign tag to server response ──────────────────────────────────────

const ASSIGN_TAG_SERVER_JSON: &str = r#"{
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
}"#;

#[test]
fn deserialize_assign_tag_server_response() {
    let v: serde_json::Value = serde_json::from_str(ASSIGN_TAG_SERVER_JSON).unwrap();
    let server = &v["server"];
    assert_eq!(server["hostname"], "debian.example.com");
    assert_eq!(server["uuid"], "00c78863-db86-44ea-af70-d6edc4d162bf");
    assert_eq!(server["state"], "started");
    assert_eq!(server["plan"], "2xCPU-4GB");

    let tags = server["tags"]["tag"].as_array().unwrap();
    assert_eq!(tags.len(), 3);
    assert_eq!(tags[0], "DEV");
    assert_eq!(tags[1], "private");
    assert_eq!(tags[2], "RHEL");

    let ips = server["ip_addresses"]["ip_address"].as_array().unwrap();
    assert_eq!(ips.len(), 3);
    assert_eq!(ips[0]["access"], "utility");
    assert_eq!(ips[0]["address"], "10.0.0.0");
    assert_eq!(ips[0]["family"], "IPv4");
    assert_eq!(ips[1]["access"], "public");
    assert_eq!(ips[1]["address"], "x.x.x.x");
    assert_eq!(ips[1]["family"], "IPv4");
    assert_eq!(ips[2]["access"], "public");
    assert_eq!(ips[2]["family"], "IPv6");

    assert_eq!(server["boot_order"], "disk");
    assert_eq!(server["core_number"], "2");
    assert_eq!(server["firewall"], "on");
    assert_eq!(server["license"], 0);
    assert_eq!(server["memory_amount"], "4096");
    assert_eq!(server["nic_model"], "virtio");
    assert_eq!(server["plan_ipv4_bytes"], "345346754423");
    assert_eq!(server["plan_ipv6_bytes"], "676300234");
    assert_eq!(server["timezone"], "UTC");
    assert_eq!(server["title"], "My Debian server");
    assert_eq!(server["video_model"], "cirrus");
    assert_eq!(server["vnc"], "off");
    assert_eq!(server["vnc_password"], "aabbccdd");
    assert_eq!(server["zone"], "fi-hel1");

    let storages = server["storage_devices"]["storage_device"]
        .as_array()
        .unwrap();
    assert_eq!(storages.len(), 1);
    assert_eq!(storages[0]["storage_size"], 50);
    assert_eq!(storages[0]["address"], "virtio:0");
    assert_eq!(storages[0]["part_of_plan"], "yes");
    assert_eq!(storages[0]["storage"], "0169b4f8-051c-4a86-9484-f5b798249949");
    assert_eq!(storages[0]["storage_title"], "Debian from a template");
    assert_eq!(storages[0]["type"], "disk");
}

// ─── Remove tag from server response ────────────────────────────────────

const REMOVE_TAG_SERVER_JSON: &str = r#"{
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
}"#;

#[test]
fn deserialize_remove_tag_server_response() {
    let v: serde_json::Value = serde_json::from_str(REMOVE_TAG_SERVER_JSON).unwrap();
    let server = &v["server"];
    assert_eq!(server["uuid"], "00c78863-db86-44ea-af70-d6edc4d162bf");

    let tags = server["tags"]["tag"].as_array().unwrap();
    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0], "DEV");
    assert_eq!(tags[1], "RHEL");

    assert_eq!(server["boot_order"], "disk");
    assert_eq!(server["core_number"], "2");
    assert_eq!(server["firewall"], "on");
    assert_eq!(server["hostname"], "debian.example.com");
    assert_eq!(server["state"], "started");
    assert_eq!(server["plan"], "2xCPU-4GB");
    assert_eq!(server["license"], 0);
    assert_eq!(server["memory_amount"], "4096");
    assert_eq!(server["nic_model"], "virtio");
    assert_eq!(server["plan_ipv4_bytes"], "345346754423");
    assert_eq!(server["plan_ipv6_bytes"], "676300234");
    assert_eq!(server["timezone"], "UTC");
    assert_eq!(server["title"], "My Debian server");
    assert_eq!(server["video_model"], "cirrus");
    assert_eq!(server["vnc"], "off");
    assert_eq!(server["vnc_password"], "aabbccdd");
    assert_eq!(server["zone"], "fi-hel1");

    let ips = server["ip_addresses"]["ip_address"].as_array().unwrap();
    assert_eq!(ips.len(), 3);

    let storages = server["storage_devices"]["storage_device"]
        .as_array()
        .unwrap();
    assert_eq!(storages.len(), 1);
    assert_eq!(storages[0]["storage_size"], 50);
}
