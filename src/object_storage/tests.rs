use super::*;

const MOS_SERVICE_LIST_JSON: &str = r#"[
  {
"configured_status": "started",
"created_at": "2025-08-22T06:46:24.483024Z",
"custom_domains": [],
"endpoints": [
  {
    "domain_name": "h7fao.upcloudobjects.com",
    "iam_url": "https://h7fao.upcloudobjects.com:4443/iam",
    "mode": "api",
    "sts_url": "https://h7fao.upcloudobjects.com:4443/sts",
    "type": "public"
  }
],
"labels": [],
"name": "SE-OBJ1",
"networks": [
  {
    "family": "IPv4",
    "name": "my-public-network",
    "type": "public"
  }
],
"operational_state": "running",
"region": "europe-3",
"state_messages": [],
"termination_protection": false,
"updated_at": "2026-04-04T01:25:47.939667Z",
"usage": {
  "total_objects": 0,
  "total_size_bytes": 0
},
"uuid": "1289deac-1cd1-459d-a467-a6fc420d4502"
  }
]"#;

const MOS_BUCKET_LIST_JSON: &str = r#"[
  {"name": "example-bucket-1", "total_objects": 157, "total_size_bytes": 5293771},
  {"name": "example-bucket-2", "total_objects": 12, "total_size_bytes": 832226}
]"#;

#[test]
fn deserialize_mos_service_list() {
    let services: Vec<Service> =
        serde_json::from_str(MOS_SERVICE_LIST_JSON).unwrap();
    assert_eq!(services.len(), 1);
    let svc = &services[0];
    assert_eq!(svc.uuid, "1289deac-1cd1-459d-a467-a6fc420d4502");
    assert_eq!(svc.name, "SE-OBJ1");
    assert_eq!(svc.region, "europe-3");
    assert_eq!(svc.configured_status.as_deref(), Some("started"));
    assert_eq!(svc.operational_state.as_deref(), Some("running"));
    assert_eq!(svc.termination_protection, Some(false));
    assert_eq!(
        svc.created_at.as_deref(),
        Some("2025-08-22T06:46:24.483024Z")
    );
    assert_eq!(
        svc.updated_at.as_deref(),
        Some("2026-04-04T01:25:47.939667Z")
    );
    assert!(svc.custom_domains.is_empty());
    assert!(svc.state_messages.is_empty());
    assert!(svc.static_websites.is_empty());
    assert!(svc.users.is_none());

    let labels = svc.labels.as_ref().unwrap();
    assert!(labels.is_empty());

    let usage = svc.usage.as_ref().unwrap();
    assert_eq!(usage.total_objects, Some(0));
    assert_eq!(usage.total_size_bytes, Some(0));

    let endpoints = svc.endpoints.as_ref().unwrap();
    assert_eq!(endpoints.len(), 1);
    let ep = &endpoints[0];
    assert_eq!(ep.domain_name, "h7fao.upcloudobjects.com");
    assert_eq!(ep.endpoint_type, "public");
    assert_eq!(ep.mode.as_deref(), Some("api"));
    assert_eq!(
        ep.iam_url.as_deref(),
        Some("https://h7fao.upcloudobjects.com:4443/iam")
    );
    assert_eq!(
        ep.sts_url.as_deref(),
        Some("https://h7fao.upcloudobjects.com:4443/sts")
    );

    let networks = svc.networks.as_ref().unwrap();
    assert_eq!(networks.len(), 1);
    assert_eq!(networks[0].family, "IPv4");
    assert_eq!(networks[0].name, "my-public-network");
    assert_eq!(networks[0].network_type, "public");
    assert!(networks[0].uuid.is_none());
}

#[test]
fn deserialize_mos_bucket_list() {
    let buckets: Vec<Bucket> = serde_json::from_str(MOS_BUCKET_LIST_JSON).unwrap();
    assert_eq!(buckets.len(), 2);
    assert_eq!(buckets[0].name, "example-bucket-1");
    assert_eq!(buckets[0].total_objects, Some(157));
    assert_eq!(buckets[0].total_size_bytes, Some(5293771));
    assert!(buckets[0].created_at.is_none());
    assert_eq!(buckets[1].name, "example-bucket-2");
    assert_eq!(buckets[1].total_objects, Some(12));
    assert_eq!(buckets[1].total_size_bytes, Some(832226));
    assert!(buckets[1].created_at.is_none());
}

// ─── New test data (official docs, page 21) ──────────────────────────────

const MOS_REGION_LIST_JSON: &str = r#"[
{
    "name": "europe-1",
    "primary_zone": "fi-hel1",
    "zones": [
        { "name": "fi-hel1" },
        { "name": "de-fra1" }
    ]
}
]"#;

const MOS_REGION_DETAIL_JSON: &str = r#"{
"name": "europe-1",
"primary_zone": "fi-hel1",
"zones": [
    { "name": "fi-hel1" },
    { "name": "de-fra1" }
]
}"#;

const MOS_CREATE_SERVICE_REQUEST_JSON: &str = r#"{
"name": "example-service",
"region": "example",
"configured_status": "started",
"termination_protection": false,
"networks": [
    {
        "name": "example-public-network",
        "type": "public",
        "family": "IPv4"
    },
    {
        "name": "example-private-network",
        "uuid": "03aa7245-2ff9-49c8-9f0e-7ca0270d71a4",
        "type": "private",
        "family": "IPv4"
    }
]
}"#;

const MOS_SERVICE_DETAIL_JSON: &str = r#"{
"configured_status": "started",
"created_at": "2023-05-07T15:55:24.655776Z",
"custom_domains": [],
"endpoints": [
    {
        "domain_name": "7mf5k.upcloudobjects.com",
        "iam_url": "https://7mf5k.upcloudobjects.com:4443/iam",
        "mode": "api",
        "sts_url": "https://7mf5k.upcloudobjects.com:4443/sts",
        "type": "public"
    },
    {
        "domain_name": "7mf5k-private.upcloudobjects.com",
        "iam_url": "https://7mf5k-private.upcloudobjects.com:4443/iam",
        "mode": "api",
        "sts_url": "https://7mf5k-private.upcloudobjects.com:4443/sts",
        "type": "private"
    },
    {
        "domain_name": "7mf5k-static.upcloudobjects.com",
        "mode": "static-website",
        "type": "public"
    }
],
"labels": [
  { "key": "example-key", "value": "example-value" }
],
"name": "example-service",
"networks": [
    {
        "family": "IPv4",
        "name": "example-public-network",
        "type": "public"
    },
    {
        "family": "IPv4",
        "name": "example-private-network",
        "type": "private",
        "uuid": "03aa7245-2ff9-49c8-9f0e-7ca0270d71a4"
    }
],
"operational_state": "setup-checkup",
"region": "europe-1",
"state_messages": [
  {
    "code": "waiting_certificate_issuing",
    "created_at": "2025-01-16T11:20:40.372611Z",
    "message": "Certificate issuing is in progress for domains: 7mf5k-private.upbucket.com, *.7mf5k-private.upbucket.com.",
    "operational_state": "setup-checkup",
    "updated_at": "2025-01-16T11:20:40.372611Z"
  }
],
"static_websites": [
  {
    "bucket_name": "my-website",
    "bucket_prefix": "",
    "created_at": "2023-05-07T16:00:00.000000Z",
    "domain_name": "7mf5k-static.upcloudobjects.com",
    "enabled": true,
    "error_pages": [],
    "index_document": "index.html",
    "spa_mode": false,
    "updated_at": "2023-05-07T16:00:00.000000Z"
  }
],
"termination_protection": false,
"updated_at": "2023-05-07T21:38:15.757405Z",
"usage": {
  "total_objects": 310499,
  "total_size_bytes": 32414921734
},
"users": [
    {
        "access_keys": [
            {
                "access_key_id": "AKIA63F41D01345BB477",
                "created_at": "2023-05-07T20:52:19.705405Z",
                "last_used_at": "2023-05-07T20:52:17Z",
                "status": "Active"
            }
        ],
        "arn": "urn:ecs:iam::123bbb5c6a4240409e07f7d89fe28891:user/example_user",
        "created_at": "2023-05-07T15:55:24.655776Z",
        "policies": [
          { "arn": "urn:ecs:iam:::policy/ECSS3FullAccess", "name": "ECSS3FullAccess" },
          { "arn": "urn:ecs:iam:::policy/IAMFullAccess", "name": "IAMFullAccess" }
        ],
        "username": "example_user"
    }
],
"uuid": "1200ecde-db95-4d1c-9133-6508f3232567"
}"#;

const MOS_REPLACE_SERVICE_REQUEST_JSON: &str = r#"{
"name": "example-service",
"configured_status": "started",
"termination_protection": false,
"networks": [
    {
        "name": "example-public-network",
        "type": "public",
        "family": "IPv4"
    },
    {
        "name": "example-private-network",
        "uuid": "03aa7245-2ff9-49c8-9f0e-7ca0270d71a4",
        "type": "private",
        "family": "IPv4"
    }
],
"labels": [
    { "key": "example-label-key", "value": "example-label-value" }
]
}"#;

const MOS_MODIFY_SERVICE_REQUEST_JSON: &str = r#"{
"name": "example-service",
"configured_status": "started",
"termination_protection": false,
"networks": [
    {
        "name": "example-private-network",
        "uuid": "03aa7245-2ff9-49c8-9f0e-7ca0270d71a4",
        "type": "private",
        "family": "IPv4"
    }
],
"labels": null
}"#;

const MOS_USAGE_JSON: &str = r#"{
"total_objects": 1287,
"total_size_bytes": 48277239
}"#;

const MOS_CREATE_BUCKET_RESPONSE_JSON: &str = r#"{
"name": "my-bucket-1",
"total_objects": 0,
"total_size_bytes": 0
}"#;

const MOS_NETWORK_LIST_JSON: &str = r#"[
{
    "family": "IPv4",
    "name": "example-public-network",
    "type": "public"
},
{
    "family": "IPv4",
    "name": "example-private-network",
    "type": "private",
    "uuid": "03aa7245-2ff9-49c8-9f0e-7ca0270d71a4"
}
]"#;

const MOS_NETWORK_DETAIL_JSON: &str = r#"{
"family": "IPv4",
"name": "example-private-network",
"type": "private",
"uuid": "03bec0ad-85c3-459e-824d-710f8f24f740"
}"#;

const MOS_PUBLIC_NETWORK_JSON: &str = r#"{
"name": "example-public-network",
"type": "public",
"family": "IPv4"
}"#;

const MOS_PRIVATE_NETWORK_INPUT_JSON: &str = r#"{
"name": "example-private-network",
"uuid": "03bec0ad-85c3-459e-824d-710f8f24f740",
"type": "private",
"family": "IPv4"
}"#;

const MOS_METRICS_SERIES_JSON: &str = r#"[
{
    "bytes_created": 530000,
    "bytes_deleted": 244000,
    "bytes_received": 530000,
    "bytes_total": 7502000,
    "bytes_transmitted": 0,
    "end_at": "2023-10-31T12:00:00Z",
    "objects_created": 79,
    "objects_deleted": 40,
    "objects_total": 1237,
    "start_at": "2023-10-31T11:00:00Z"
}
]"#;

// ─── New tests ───────────────────────────────────────────────────────────

#[test]
fn deserialize_mos_region_list() {
    let regions: Vec<Region> = serde_json::from_str(MOS_REGION_LIST_JSON).unwrap();
    assert_eq!(regions.len(), 1);
    let r = &regions[0];
    assert_eq!(r.name, "europe-1");
    assert_eq!(r.primary_zone.as_deref(), Some("fi-hel1"));
    assert!(r.endpoint_url.is_none());
    let zones = r.zones.as_ref().unwrap();
    assert_eq!(zones.len(), 2);
    assert_eq!(zones[0].name, "fi-hel1");
    assert_eq!(zones[1].name, "de-fra1");
}

#[test]
fn deserialize_mos_region_detail() {
    let r: Region = serde_json::from_str(MOS_REGION_DETAIL_JSON).unwrap();
    assert_eq!(r.name, "europe-1");
    assert_eq!(r.primary_zone.as_deref(), Some("fi-hel1"));
    assert!(r.endpoint_url.is_none());
    let zones = r.zones.as_ref().unwrap();
    assert_eq!(zones.len(), 2);
    assert_eq!(zones[0].name, "fi-hel1");
    assert_eq!(zones[1].name, "de-fra1");
}

#[test]
fn deserialize_mos_create_service_request() {
    let req: CreateRequest =
        serde_json::from_str(MOS_CREATE_SERVICE_REQUEST_JSON).unwrap();
    assert_eq!(req.name, "example-service");
    assert_eq!(req.region, "example");
    assert_eq!(req.configured_status.as_deref(), Some("started"));
    assert!(req.labels.is_none());
    assert_eq!(req.networks.len(), 2);
    assert_eq!(req.networks[0].name, "example-public-network");
    assert_eq!(req.networks[0].network_type, "public");
    assert_eq!(req.networks[0].family, "IPv4");
    assert_eq!(req.networks[0].uuid, None);
    assert_eq!(req.networks[1].name, "example-private-network");
    assert_eq!(req.networks[1].network_type, "private");
    assert_eq!(req.networks[1].family, "IPv4");
    assert_eq!(
        req.networks[1].uuid.as_deref(),
        Some("03aa7245-2ff9-49c8-9f0e-7ca0270d71a4")
    );
}

#[test]
fn deserialize_mos_service_detail() {
    let svc: Service =
        serde_json::from_str(MOS_SERVICE_DETAIL_JSON).unwrap();
    assert_eq!(svc.uuid, "1200ecde-db95-4d1c-9133-6508f3232567");
    assert_eq!(svc.name, "example-service");
    assert_eq!(svc.region, "europe-1");
    assert_eq!(svc.configured_status.as_deref(), Some("started"));
    assert_eq!(svc.operational_state.as_deref(), Some("setup-checkup"));
    assert_eq!(svc.termination_protection, Some(false));
    assert_eq!(
        svc.created_at.as_deref(),
        Some("2023-05-07T15:55:24.655776Z")
    );
    assert_eq!(
        svc.updated_at.as_deref(),
        Some("2023-05-07T21:38:15.757405Z")
    );
    assert!(svc.custom_domains.is_empty());

    // endpoints
    let endpoints = svc.endpoints.as_ref().unwrap();
    assert_eq!(endpoints.len(), 3);
    assert_eq!(endpoints[0].domain_name, "7mf5k.upcloudobjects.com");
    assert_eq!(endpoints[0].endpoint_type, "public");
    assert_eq!(endpoints[0].mode.as_deref(), Some("api"));
    assert_eq!(
        endpoints[0].iam_url.as_deref(),
        Some("https://7mf5k.upcloudobjects.com:4443/iam")
    );
    assert_eq!(
        endpoints[0].sts_url.as_deref(),
        Some("https://7mf5k.upcloudobjects.com:4443/sts")
    );
    assert_eq!(
        endpoints[1].domain_name,
        "7mf5k-private.upcloudobjects.com"
    );
    assert_eq!(endpoints[1].endpoint_type, "private");
    assert_eq!(endpoints[1].mode.as_deref(), Some("api"));
    assert_eq!(
        endpoints[1].iam_url.as_deref(),
        Some("https://7mf5k-private.upcloudobjects.com:4443/iam")
    );
    assert_eq!(
        endpoints[1].sts_url.as_deref(),
        Some("https://7mf5k-private.upcloudobjects.com:4443/sts")
    );
    assert_eq!(
        endpoints[2].domain_name,
        "7mf5k-static.upcloudobjects.com"
    );
    assert_eq!(endpoints[2].endpoint_type, "public");
    assert_eq!(endpoints[2].mode.as_deref(), Some("static-website"));
    assert!(endpoints[2].iam_url.is_none());
    assert!(endpoints[2].sts_url.is_none());

    // labels
    let labels = svc.labels.as_ref().unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0].key, "example-key");
    assert_eq!(labels[0].value, "example-value");

    // networks
    let networks = svc.networks.as_ref().unwrap();
    assert_eq!(networks.len(), 2);
    assert_eq!(networks[0].family, "IPv4");
    assert_eq!(networks[0].name, "example-public-network");
    assert_eq!(networks[0].network_type, "public");
    assert!(networks[0].uuid.is_none());
    assert_eq!(networks[1].family, "IPv4");
    assert_eq!(networks[1].name, "example-private-network");
    assert_eq!(networks[1].network_type, "private");
    assert_eq!(
        networks[1].uuid.as_deref(),
        Some("03aa7245-2ff9-49c8-9f0e-7ca0270d71a4")
    );

    // state_messages
    assert_eq!(svc.state_messages.len(), 1);
    assert_eq!(
        svc.state_messages[0]["code"],
        "waiting_certificate_issuing"
    );
    assert_eq!(
        svc.state_messages[0]["operational_state"],
        "setup-checkup"
    );

    // static_websites
    assert_eq!(svc.static_websites.len(), 1);
    assert_eq!(svc.static_websites[0]["bucket_name"], "my-website");
    assert_eq!(svc.static_websites[0]["enabled"], true);
    assert_eq!(svc.static_websites[0]["index_document"], "index.html");
    assert_eq!(svc.static_websites[0]["spa_mode"], false);

    // usage
    let usage = svc.usage.as_ref().unwrap();
    assert_eq!(usage.total_objects, Some(310499));
    assert_eq!(usage.total_size_bytes, Some(32414921734));

    // users
    let users = svc.users.as_ref().unwrap();
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].username, "example_user");
}

#[test]
fn deserialize_mos_replace_service_request() {
    let v: serde_json::Value =
        serde_json::from_str(MOS_REPLACE_SERVICE_REQUEST_JSON).unwrap();
    assert_eq!(v["name"], "example-service");
    assert_eq!(v["configured_status"], "started");
    assert_eq!(v["termination_protection"], false);

    let networks = v["networks"].as_array().unwrap();
    assert_eq!(networks.len(), 2);
    assert_eq!(networks[0]["name"], "example-public-network");
    assert_eq!(networks[0]["type"], "public");
    assert_eq!(networks[0]["family"], "IPv4");
    assert_eq!(networks[1]["name"], "example-private-network");
    assert_eq!(networks[1]["type"], "private");
    assert_eq!(networks[1]["family"], "IPv4");
    assert_eq!(
        networks[1]["uuid"],
        "03aa7245-2ff9-49c8-9f0e-7ca0270d71a4"
    );

    let labels = v["labels"].as_array().unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0]["key"], "example-label-key");
    assert_eq!(labels[0]["value"], "example-label-value");
}

#[test]
fn deserialize_mos_modify_service_request() {
    let v: serde_json::Value =
        serde_json::from_str(MOS_MODIFY_SERVICE_REQUEST_JSON).unwrap();
    assert_eq!(v["name"], "example-service");
    assert_eq!(v["configured_status"], "started");
    assert_eq!(v["termination_protection"], false);
    assert!(v["labels"].is_null());

    let networks = v["networks"].as_array().unwrap();
    assert_eq!(networks.len(), 1);
    assert_eq!(networks[0]["name"], "example-private-network");
    assert_eq!(networks[0]["type"], "private");
    assert_eq!(networks[0]["family"], "IPv4");
    assert_eq!(
        networks[0]["uuid"],
        "03aa7245-2ff9-49c8-9f0e-7ca0270d71a4"
    );
}

#[test]
fn deserialize_mos_usage() {
    let u: Usage = serde_json::from_str(MOS_USAGE_JSON).unwrap();
    assert_eq!(u.total_objects, Some(1287));
    assert_eq!(u.total_size_bytes, Some(48277239));
}

#[test]
fn deserialize_mos_create_bucket_response() {
    let b: Bucket = serde_json::from_str(MOS_CREATE_BUCKET_RESPONSE_JSON).unwrap();
    assert_eq!(b.name, "my-bucket-1");
    assert_eq!(b.total_objects, Some(0));
    assert_eq!(b.total_size_bytes, Some(0));
    assert!(b.created_at.is_none());
}

#[test]
fn deserialize_mos_network_list() {
    let nets: Vec<Network> = serde_json::from_str(MOS_NETWORK_LIST_JSON).unwrap();
    assert_eq!(nets.len(), 2);
    assert_eq!(nets[0].family, "IPv4");
    assert_eq!(nets[0].name, "example-public-network");
    assert_eq!(nets[0].network_type, "public");
    assert!(nets[0].uuid.is_none());
    assert_eq!(nets[1].family, "IPv4");
    assert_eq!(nets[1].name, "example-private-network");
    assert_eq!(nets[1].network_type, "private");
    assert_eq!(
        nets[1].uuid.as_deref(),
        Some("03aa7245-2ff9-49c8-9f0e-7ca0270d71a4")
    );
}

#[test]
fn deserialize_mos_network_detail() {
    let n: Network = serde_json::from_str(MOS_NETWORK_DETAIL_JSON).unwrap();
    assert_eq!(n.family, "IPv4");
    assert_eq!(n.name, "example-private-network");
    assert_eq!(n.network_type, "private");
    assert_eq!(
        n.uuid.as_deref(),
        Some("03bec0ad-85c3-459e-824d-710f8f24f740")
    );
}

#[test]
fn deserialize_mos_public_network_value() {
    let v: serde_json::Value = serde_json::from_str(MOS_PUBLIC_NETWORK_JSON).unwrap();
    assert_eq!(v["name"], "example-public-network");
    assert_eq!(v["type"], "public");
    assert_eq!(v["family"], "IPv4");
}

#[test]
fn deserialize_mos_private_network_input() {
    let n: NetworkInput =
        serde_json::from_str(MOS_PRIVATE_NETWORK_INPUT_JSON).unwrap();
    assert_eq!(n.name, "example-private-network");
    assert_eq!(n.network_type, "private");
    assert_eq!(n.family, "IPv4");
    assert_eq!(
        n.uuid.as_deref(),
        Some("03bec0ad-85c3-459e-824d-710f8f24f740")
    );
}

#[test]
fn deserialize_mos_metrics_series() {
    let series: Vec<serde_json::Value> =
        serde_json::from_str(MOS_METRICS_SERIES_JSON).unwrap();
    assert_eq!(series.len(), 1);
    let m = &series[0];
    assert_eq!(m["bytes_created"], 530000);
    assert_eq!(m["bytes_deleted"], 244000);
    assert_eq!(m["bytes_received"], 530000);
    assert_eq!(m["bytes_total"], 7502000);
    assert_eq!(m["bytes_transmitted"], 0);
    assert_eq!(m["end_at"], "2023-10-31T12:00:00Z");
    assert_eq!(m["objects_created"], 79);
    assert_eq!(m["objects_deleted"], 40);
    assert_eq!(m["objects_total"], 1237);
    assert_eq!(m["start_at"], "2023-10-31T11:00:00Z");
}

// ─── MOS policy tests (official docs, page 21) ───────────────────────────

const MOS_POLICY_LIST_JSON: &str = r#"[
  {
"arn": "urn:ecs:iam:::policy/ECSDenyAll",
"attachment_count": 0,
"created_at": "2024-02-05T11:06:17Z",
"default_version_id": "v1",
"description": "Deny all access.",
"name": "ECSDenyAll",
"system": true,
"updated_at": "2024-02-05T11:06:17Z"
  },
  {
"arn": "urn:ecs:iam:::policy/ECSS3FullAccess",
"attachment_count": 1,
"created_at": "2024-02-05T11:06:16Z",
"default_version_id": "v1",
"description": "Provides full access to all buckets.",
"name": "ECSS3FullAccess",
"system": true,
"updated_at": "2024-02-05T11:06:16Z"
  },
  {
"arn": "urn:ecs:iam:::policy/ECSS3ReadOnlyAccess",
"attachment_count": 0,
"created_at": "2024-02-05T11:06:16Z",
"default_version_id": "v1",
"description": "Provides read only access to all buckets.",
"name": "ECSS3ReadOnlyAccess",
"system": true,
"updated_at": "2024-02-05T11:06:16Z"
  },
  {
"arn": "urn:ecs:iam:::policy/IAMFullAccess",
"attachment_count": 1,
"created_at": "2024-02-05T11:06:16Z",
"default_version_id": "v1",
"description": "Provides full access to IAM.",
"name": "IAMFullAccess",
"system": true,
"updated_at": "2024-02-05T11:06:16Z"
  },
  {
"arn": "urn:ecs:iam:::policy/IAMReadOnlyAccess",
"attachment_count": 0,
"created_at": "2024-02-05T11:06:16Z",
"default_version_id": "v1",
"description": "Provides read only access to IAM.",
"name": "IAMReadOnlyAccess",
"system": true,
"updated_at": "2024-02-05T11:06:16Z"
  }
]"#;

const MOS_POLICY_DETAIL_JSON: &str = r#"{
  "arn": "urn:ecs:iam:::policy/ECSS3FullAccess",
  "attachment_count": 0,
  "created_at": "2024-02-05T11:06:16Z",
  "default_version_id": "v1",
  "description": "Provides full access to all buckets.",
  "document": "%7B%0A++++%22Version%22%3A+%222012-10-17%22%2C%0A++++%22Statement%22%3A+%5B%0A++++++++%7B%0A++++++++++++%22Effect%22%3A+%22Allow%22%2C%0A++++++++++++%22Action%22%3A+%22s3%3A*%22%2C%0A++++++++++++%22Resource%22%3A+%22*%22%0A++++++++%7D%0A++++%5D%0A%7D",
  "name": "ECSS3FullAccess",
  "system": true,
  "updated_at": "2024-02-05T11:06:16Z"
}"#;

#[test]
fn deserialize_mos_policy_list() {
    let policies: Vec<Policy> = serde_json::from_str(MOS_POLICY_LIST_JSON).unwrap();
    assert_eq!(policies.len(), 5);

    let p0 = &policies[0];
    assert_eq!(p0.name, "ECSDenyAll");
    assert_eq!(p0.arn.as_deref(), Some("urn:ecs:iam:::policy/ECSDenyAll"));
    assert_eq!(p0.attachment_count, Some(0));
    assert_eq!(p0.created_at.as_deref(), Some("2024-02-05T11:06:17Z"));
    assert_eq!(p0.default_version_id.as_deref(), Some("v1"));
    assert_eq!(p0.description.as_deref(), Some("Deny all access."));
    assert_eq!(p0.system, Some(true));
    assert_eq!(p0.updated_at.as_deref(), Some("2024-02-05T11:06:17Z"));
    assert!(p0.document.is_none());

    let p1 = &policies[1];
    assert_eq!(p1.name, "ECSS3FullAccess");
    assert_eq!(p1.arn.as_deref(), Some("urn:ecs:iam:::policy/ECSS3FullAccess"));
    assert_eq!(p1.attachment_count, Some(1));
    assert_eq!(p1.description.as_deref(), Some("Provides full access to all buckets."));
    assert_eq!(p1.system, Some(true));

    let p2 = &policies[2];
    assert_eq!(p2.name, "ECSS3ReadOnlyAccess");
    assert_eq!(p2.attachment_count, Some(0));
    assert_eq!(p2.description.as_deref(), Some("Provides read only access to all buckets."));

    let p3 = &policies[3];
    assert_eq!(p3.name, "IAMFullAccess");
    assert_eq!(p3.attachment_count, Some(1));
    assert_eq!(p3.description.as_deref(), Some("Provides full access to IAM."));

    let p4 = &policies[4];
    assert_eq!(p4.name, "IAMReadOnlyAccess");
    assert_eq!(p4.attachment_count, Some(0));
    assert_eq!(p4.description.as_deref(), Some("Provides read only access to IAM."));
}

#[test]
fn deserialize_mos_policy_detail() {
    let p: Policy = serde_json::from_str(MOS_POLICY_DETAIL_JSON).unwrap();
    assert_eq!(p.name, "ECSS3FullAccess");
    assert_eq!(p.arn.as_deref(), Some("urn:ecs:iam:::policy/ECSS3FullAccess"));
    assert_eq!(p.attachment_count, Some(0));
    assert_eq!(p.created_at.as_deref(), Some("2024-02-05T11:06:16Z"));
    assert_eq!(p.default_version_id.as_deref(), Some("v1"));
    assert_eq!(p.description.as_deref(), Some("Provides full access to all buckets."));
    assert!(p.document.is_some());
    assert!(p.document.as_deref().unwrap().contains("%22Version%22"));
    assert_eq!(p.system, Some(true));
    assert_eq!(p.updated_at.as_deref(), Some("2024-02-05T11:06:16Z"));
}

// ─── MOS user tests (official docs, page 21) ─────────────────────────────

const MOS_USER_LIST_JSON: &str = r#"[
{
    "access_keys": [
        {
            "access_key_id": "AKIA63F41D01345BB477",
            "created_at": "2023-05-07T20:52:19.705405Z",
            "last_used_at": "2023-05-07T20:52:17Z",
            "status": "Active"
        }
    ],
    "arn": "urn:ecs:iam::123bbb5c6a4240409e07f7d89fe28891:user/example_user",
    "created_at": "2023-05-07T15:55:24.655776Z",
    "policies": [
        {
            "arn": "urn:ecs:iam:::policy/ECSS3FullAccess",
            "name": "ECSS3FullAccess"
        },
        {
            "arn": "urn:ecs:iam:::policy/IAMFullAccess",
            "name": "IAMFullAccess"
        }
    ],
    "username": "example_user"
}
]"#;

const MOS_USER_DETAIL_JSON: &str = r#"{
"access_keys": [
    {
        "access_key_id": "AKIA63F41D01345BB477",
        "created_at": "2023-05-07T20:52:19.705405Z",
        "last_used_at": "2023-05-07T20:52:17Z",
        "status": "Active"
    }
],
"arn": "urn:ecs:iam::123bbb5c6a4240409e07f7d89fe28891:user/example_user",
"created_at": "2023-05-07T15:55:24.655776Z",
"policies": [
    {
        "arn": "urn:ecs:iam:::policy/ECSS3FullAccess",
        "name": "ECSS3FullAccess"
    },
    {
        "arn": "urn:ecs:iam:::policy/IAMFullAccess",
        "name": "IAMFullAccess"
    }
],
"username": "example_user"
}"#;

const MOS_USER_CREATE_RESPONSE_JSON: &str = r#"{
"access_keys": [],
"created_at": "2023-05-07T18:19:16.507681Z",
"policies": [],
"username": "example_user"
}"#;

#[test]
fn deserialize_mos_user_list() {
    let users: Vec<User> = serde_json::from_str(MOS_USER_LIST_JSON).unwrap();
    assert_eq!(users.len(), 1);
    let u = &users[0];
    assert_eq!(u.username, "example_user");
    assert_eq!(u.arn.as_deref(), Some("urn:ecs:iam::123bbb5c6a4240409e07f7d89fe28891:user/example_user"));
    assert_eq!(u.created_at.as_deref(), Some("2023-05-07T15:55:24.655776Z"));

    let keys = u.access_keys.as_ref().unwrap();
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0].access_key_id, "AKIA63F41D01345BB477");
    assert_eq!(keys[0].created_at.as_deref(), Some("2023-05-07T20:52:19.705405Z"));
    assert_eq!(keys[0].last_used_at.as_deref(), Some("2023-05-07T20:52:17Z"));
    assert_eq!(keys[0].status.as_deref(), Some("Active"));
    assert!(keys[0].secret_access_key.is_none());

    let policies = u.policies.as_ref().unwrap();
    assert_eq!(policies.len(), 2);
    assert_eq!(policies[0].name, "ECSS3FullAccess");
    assert_eq!(policies[0].arn.as_deref(), Some("urn:ecs:iam:::policy/ECSS3FullAccess"));
    assert_eq!(policies[1].name, "IAMFullAccess");
    assert_eq!(policies[1].arn.as_deref(), Some("urn:ecs:iam:::policy/IAMFullAccess"));
}

#[test]
fn deserialize_mos_user_detail() {
    let u: User = serde_json::from_str(MOS_USER_DETAIL_JSON).unwrap();
    assert_eq!(u.username, "example_user");
    assert_eq!(u.arn.as_deref(), Some("urn:ecs:iam::123bbb5c6a4240409e07f7d89fe28891:user/example_user"));
    assert_eq!(u.created_at.as_deref(), Some("2023-05-07T15:55:24.655776Z"));

    let keys = u.access_keys.as_ref().unwrap();
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0].access_key_id, "AKIA63F41D01345BB477");
    assert_eq!(keys[0].created_at.as_deref(), Some("2023-05-07T20:52:19.705405Z"));
    assert_eq!(keys[0].last_used_at.as_deref(), Some("2023-05-07T20:52:17Z"));
    assert_eq!(keys[0].status.as_deref(), Some("Active"));
    assert!(keys[0].secret_access_key.is_none());

    let policies = u.policies.as_ref().unwrap();
    assert_eq!(policies.len(), 2);
    assert_eq!(policies[0].name, "ECSS3FullAccess");
    assert_eq!(policies[0].arn.as_deref(), Some("urn:ecs:iam:::policy/ECSS3FullAccess"));
    assert_eq!(policies[1].name, "IAMFullAccess");
    assert_eq!(policies[1].arn.as_deref(), Some("urn:ecs:iam:::policy/IAMFullAccess"));
}

#[test]
fn deserialize_mos_user_create_response() {
    let u: User = serde_json::from_str(MOS_USER_CREATE_RESPONSE_JSON).unwrap();
    assert_eq!(u.username, "example_user");
    assert_eq!(u.created_at.as_deref(), Some("2023-05-07T18:19:16.507681Z"));
    assert!(u.arn.is_none());
    let keys = u.access_keys.as_ref().unwrap();
    assert!(keys.is_empty());
    let policies = u.policies.as_ref().unwrap();
    assert!(policies.is_empty());
}

// ─── MOS access key tests (official docs, page 21) ───────────────────────

const MOS_ACCESS_KEY_LIST_JSON: &str = r#"[
{
    "access_key_id": "AKIA63F41D01345BB477",
    "created_at": "2023-05-07T20:52:19.705405Z",
    "last_used_at": "2023-05-07T20:52:17Z",
    "status": "Active"
}
]"#;

const MOS_ACCESS_KEY_CREATE_RESPONSE_JSON: &str = r#"{
"access_key_id": "AKIA589142A152F5E423",
"created_at": "2023-05-07T22:58:26.239729Z",
"last_used_at": "2023-05-07T22:58:26.239729Z",
"secret_access_key": "xbINHFALkXjFjmxhAYL8mJnODRDX91OM7cCd2+1Y",
"status": "Active"
}"#;

const MOS_ACCESS_KEY_DETAIL_JSON: &str = r#"{
"access_key_id": "AKIA63F41D01345BB477",
"created_at": "2023-05-07T20:52:19.705405Z",
"last_used_at": "2023-05-07T20:52:17Z",
"status": "Active"
}"#;

#[test]
fn deserialize_mos_access_key_list() {
    let keys: Vec<UserAccessKey> = serde_json::from_str(MOS_ACCESS_KEY_LIST_JSON).unwrap();
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0].access_key_id, "AKIA63F41D01345BB477");
    assert_eq!(keys[0].created_at.as_deref(), Some("2023-05-07T20:52:19.705405Z"));
    assert_eq!(keys[0].last_used_at.as_deref(), Some("2023-05-07T20:52:17Z"));
    assert_eq!(keys[0].status.as_deref(), Some("Active"));
    assert!(keys[0].secret_access_key.is_none());
}

#[test]
fn deserialize_mos_access_key_create_response() {
    let key: UserAccessKey =
        serde_json::from_str(MOS_ACCESS_KEY_CREATE_RESPONSE_JSON).unwrap();
    assert_eq!(key.access_key_id, "AKIA589142A152F5E423");
    assert_eq!(key.created_at.as_deref(), Some("2023-05-07T22:58:26.239729Z"));
    assert_eq!(key.last_used_at.as_deref(), Some("2023-05-07T22:58:26.239729Z"));
    assert_eq!(key.secret_access_key.as_deref(), Some("xbINHFALkXjFjmxhAYL8mJnODRDX91OM7cCd2+1Y"));
    assert_eq!(key.status.as_deref(), Some("Active"));
}

#[test]
fn deserialize_mos_access_key_detail() {
    let key: UserAccessKey =
        serde_json::from_str(MOS_ACCESS_KEY_DETAIL_JSON).unwrap();
    assert_eq!(key.access_key_id, "AKIA63F41D01345BB477");
    assert_eq!(key.created_at.as_deref(), Some("2023-05-07T20:52:19.705405Z"));
    assert_eq!(key.last_used_at.as_deref(), Some("2023-05-07T20:52:17Z"));
    assert_eq!(key.status.as_deref(), Some("Active"));
    assert!(key.secret_access_key.is_none());
}

// ─── Custom domain tests ─────────────────────────────────────────────────

const MOS_CUSTOM_DOMAIN_JSON: &str = r#"{
    "name": "my-custom-domain",
    "domain_name": "objects.example.com",
    "status": "active",
    "created_at": "2024-03-15T10:00:00Z"
}"#;

const MOS_CUSTOM_DOMAIN_LIST_JSON: &str = r#"[
    {
        "name": "my-custom-domain",
        "domain_name": "objects.example.com",
        "status": "active",
        "created_at": "2024-03-15T10:00:00Z"
    },
    {
        "name": "second-domain",
        "domain_name": "files.example.com",
        "status": "pending",
        "created_at": "2024-03-16T12:00:00Z"
    }
]"#;

#[test]
fn deserialize_custom_domain() {
    let cd: CustomDomain = serde_json::from_str(MOS_CUSTOM_DOMAIN_JSON).unwrap();
    assert_eq!(cd.name, "my-custom-domain");
    assert_eq!(cd.domain_name, "objects.example.com");
    assert_eq!(cd.status.as_deref(), Some("active"));
    assert_eq!(cd.created_at.as_deref(), Some("2024-03-15T10:00:00Z"));
}

#[test]
fn deserialize_custom_domain_list() {
    let list: Vec<CustomDomain> = serde_json::from_str(MOS_CUSTOM_DOMAIN_LIST_JSON).unwrap();
    assert_eq!(list.len(), 2);
    assert_eq!(list[0].name, "my-custom-domain");
    assert_eq!(list[0].domain_name, "objects.example.com");
    assert_eq!(list[1].name, "second-domain");
    assert_eq!(list[1].domain_name, "files.example.com");
    assert_eq!(list[1].status.as_deref(), Some("pending"));
}

// ─── Policy version tests ────────────────────────────────────────────────

const MOS_POLICY_VERSION_JSON: &str = r#"{
    "version_id": "v1",
    "document": "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Effect\":\"Allow\",\"Action\":\"s3:*\",\"Resource\":\"*\"}]}",
    "is_default_version": true,
    "created_at": "2024-02-05T11:06:16Z"
}"#;

const MOS_POLICY_VERSION_LIST_JSON: &str = r#"[
    {
        "version_id": "v1",
        "document": "{\"Version\":\"2012-10-17\"}",
        "is_default_version": true,
        "created_at": "2024-02-05T11:06:16Z"
    },
    {
        "version_id": "v2",
        "document": "{\"Version\":\"2012-10-17\",\"Statement\":[]}",
        "is_default_version": false,
        "created_at": "2024-03-10T09:00:00Z"
    }
]"#;

#[test]
fn deserialize_policy_version() {
    let pv: PolicyVersion = serde_json::from_str(MOS_POLICY_VERSION_JSON).unwrap();
    assert_eq!(pv.version_id, "v1");
    assert!(pv.document.contains("2012-10-17"));
    assert_eq!(pv.is_default_version, Some(true));
    assert_eq!(pv.created_at.as_deref(), Some("2024-02-05T11:06:16Z"));
}

#[test]
fn deserialize_policy_version_list() {
    let list: Vec<PolicyVersion> =
        serde_json::from_str(MOS_POLICY_VERSION_LIST_JSON).unwrap();
    assert_eq!(list.len(), 2);
    assert_eq!(list[0].version_id, "v1");
    assert_eq!(list[0].is_default_version, Some(true));
    assert_eq!(list[1].version_id, "v2");
    assert_eq!(list[1].is_default_version, Some(false));
}

// ─── Enum round-trip tests ───────────────────────────────────────────────

#[test]
fn operational_state_round_trip() {
    let states = vec![
        (OperationalState::Pending, "\"pending\""),
        (OperationalState::SetupDns, "\"setup-dns\""),
        (OperationalState::SetupNetwork, "\"setup-network\""),
        (OperationalState::SetupServer, "\"setup-server\""),
        (OperationalState::SetupService, "\"setup-service\""),
        (OperationalState::Running, "\"running\""),
        (OperationalState::DeleteDns, "\"delete-dns\""),
        (OperationalState::DeleteNetwork, "\"delete-network\""),
        (OperationalState::DeleteServer, "\"delete-server\""),
        (OperationalState::DeleteService, "\"delete-service\""),
    ];
    for (variant, json) in states {
        let serialized = serde_json::to_string(&variant).unwrap();
        assert_eq!(serialized, json);
        let deserialized: OperationalState = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, variant);
    }
}

#[test]
fn configured_status_round_trip() {
    let statuses = vec![
        (ConfiguredStatus::Started, "\"started\""),
        (ConfiguredStatus::Stopped, "\"stopped\""),
    ];
    for (variant, json) in statuses {
        let serialized = serde_json::to_string(&variant).unwrap();
        assert_eq!(serialized, json);
        let deserialized: ConfiguredStatus = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, variant);
    }
}

#[test]
fn user_access_key_status_round_trip() {
    let statuses = vec![
        (UserAccessKeyStatus::Active, "\"Active\""),
        (UserAccessKeyStatus::Inactive, "\"Inactive\""),
    ];
    for (variant, json) in statuses {
        let serialized = serde_json::to_string(&variant).unwrap();
        assert_eq!(serialized, json);
        let deserialized: UserAccessKeyStatus = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, variant);
    }
}

// ─── Request serialization tests ─────────────────────────────────────────

#[test]
fn serialize_create_custom_domain_request() {
    let req = CreateCustomDomainRequest {
        domain_name: "objects.example.com".to_string(),
    };
    let json: serde_json::Value = serde_json::to_value(&req).unwrap();
    assert_eq!(json["domain_name"], "objects.example.com");
}

#[test]
fn serialize_modify_custom_domain_request() {
    let req = ModifyCustomDomainRequest {
        domain_name: Some("new.example.com".to_string()),
    };
    let json: serde_json::Value = serde_json::to_value(&req).unwrap();
    assert_eq!(json["domain_name"], "new.example.com");

    let empty_req = ModifyCustomDomainRequest::default();
    let json2: serde_json::Value = serde_json::to_value(&empty_req).unwrap();
    assert!(json2.get("domain_name").is_none());
}

#[test]
fn serialize_create_bucket_request() {
    let req = CreateBucketRequest {
        name: "my-bucket".to_string(),
    };
    let json: serde_json::Value = serde_json::to_value(&req).unwrap();
    assert_eq!(json["name"], "my-bucket");
}

#[test]
fn serialize_create_policy_version_request() {
    let doc = "{\"Version\":\"2012-10-17\"}";
    let req = CreatePolicyVersionRequest {
        document: doc.to_string(),
    };
    let json: serde_json::Value = serde_json::to_value(&req).unwrap();
    assert_eq!(json["document"], doc);
}

#[test]
fn serialize_modify_user_access_key_request() {
    let req = ModifyUserAccessKeyRequest {
        status: UserAccessKeyStatus::Inactive,
    };
    let json: serde_json::Value = serde_json::to_value(&req).unwrap();
    assert_eq!(json["status"], "Inactive");

    let req2 = ModifyUserAccessKeyRequest {
        status: UserAccessKeyStatus::Active,
    };
    let json2: serde_json::Value = serde_json::to_value(&req2).unwrap();
    assert_eq!(json2["status"], "Active");
}
