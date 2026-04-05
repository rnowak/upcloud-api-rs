use super::*;

const GATEWAY_LIST_JSON: &str = r#"[
  {
"addresses": [{"address": "100.127.0.241", "name": "public-ip-1"}],
"configured_status": "started",
"created_at": "2022-12-01T09:04:08.529138Z",
"features": ["nat", "vpn"],
"connections": [
  {
    "uuid": "1085d477-8d8f-4c97-9bef-731933187538",
    "name": "example-connection",
    "type": "ipsec",
    "local_routes": [{"name": "upcloud-example-route", "type": "static", "static_network": "10.0.0.0/24"}],
    "remote_routes": [{"name": "remote-example-route", "type": "static", "static_network": "10.0.1.0/24"}],
    "tunnels": [
      {
        "local_address": {"name": "public-ip-1"},
        "name": "example-tunnel-1",
        "uuid": "1085d477-8d8f-4c97-9bef-731933187777",
        "remote_address": {"address": "100.10.0.111"},
        "ipsec": {
          "authentication": {"authentication": "psk"},
          "child_rekey_time": 1440,
          "dpd_delay": 30,
          "dpd_timeout": 120,
          "ike_lifetime": 86400,
          "phase1_algorithms": ["aes128gcm128", "aes256gcm128"],
          "phase1_dh_group_numbers": [14, 16, 18, 19, 20, 21],
          "phase1_integrity_algorithms": ["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"],
          "phase2_algorithms": ["aes128gcm128", "aes256gcm128"],
          "phase2_dh_group_numbers": [14, 16, 18, 19, 20, 21],
          "phase2_integrity_algorithms": ["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"],
          "rekey_time": 14400
        },
        "tunnel_healthy": true,
        "tunnel_up": true,
        "tunnel_internal_ip": "169.254.17.1",
        "internal_peer_ping_interval": 0,
        "operational_state": "established",
        "created_at": "2022-12-01T09:04:08.529138Z",
        "updated_at": "2022-12-01T19:04:08.529138Z"
      }
    ],
    "created_at": "2022-12-01T09:04:08.529138Z",
    "updated_at": "2022-12-01T19:04:08.529138Z"
  }
],
"name": "example-gateway",
"operational_state": "running",
"plan": "production",
"routers": [{"created_at": "2022-12-01T09:04:08.529138Z", "uuid": "1085d477-8d8f-4c97-9bef-731933187538"}],
"labels": [{"key": "env", "value": "testing"}],
"automatic_tunnel_internal_ip_allocation": true,
"updated_at": "2022-12-01T19:04:08.529138Z",
"uuid": "10c153e0-12e4-4dea-8748-4f34850ff55d",
"zone": "fi-hel1"
  }
]"#;

const GATEWAY_PLAN_LIST_JSON: &str = r#"[
  {
"name": "advanced",
"per_gateway_bandwidth_mbps": 10000,
"per_gateway_max_connections": 100000,
"server_number": 2,
"supported_features": ["nat", "vpn"],
"vpn_tunnel_amount": 10
  },
  {
"name": "development",
"per_gateway_bandwidth_mbps": 10,
"per_gateway_max_connections": 10000,
"server_number": 1,
"supported_features": ["nat"],
"vpn_tunnel_amount": 0
  }
]"#;

#[test]
fn deserialize_gateway_list() {
    let gateways: Vec<Gateway> = serde_json::from_str(GATEWAY_LIST_JSON).unwrap();
    assert_eq!(gateways.len(), 1);
    let gw = &gateways[0];
    assert_eq!(gw.uuid, "10c153e0-12e4-4dea-8748-4f34850ff55d");
    assert_eq!(gw.name, "example-gateway");
    assert_eq!(gw.zone, "fi-hel1");
    assert_eq!(gw.plan, "production");
    assert_eq!(gw.operational_state.as_deref(), Some("running"));
    assert_eq!(gw.configured_status.as_deref(), Some("started"));
    assert_eq!(gw.automatic_tunnel_internal_ip_allocation, Some(true));
    assert_eq!(gw.created_at.as_deref(), Some("2022-12-01T09:04:08.529138Z"));
    assert_eq!(gw.updated_at.as_deref(), Some("2022-12-01T19:04:08.529138Z"));

    // features
    assert_eq!(gw.features.len(), 2);
    assert_eq!(gw.features[0], Feature::Nat);
    assert_eq!(gw.features[1], Feature::Vpn);

    // addresses
    assert_eq!(gw.addresses.len(), 1);
    assert_eq!(gw.addresses[0].name, "public-ip-1");
    assert_eq!(gw.addresses[0].address.as_deref(), Some("100.127.0.241"));

    // routers
    assert_eq!(gw.routers.len(), 1);
    assert_eq!(gw.routers[0].uuid, "1085d477-8d8f-4c97-9bef-731933187538");
    assert_eq!(gw.routers[0].created_at.as_deref(), Some("2022-12-01T09:04:08.529138Z"));

    // labels
    let labels = gw.labels.as_ref().unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0].key, "env");
    assert_eq!(labels[0].value, "testing");

    // connections
    assert_eq!(gw.connections.len(), 1);
    let conn = &gw.connections[0];
    assert_eq!(conn.name, "example-connection");
    assert_eq!(conn.uuid.as_deref(), Some("1085d477-8d8f-4c97-9bef-731933187538"));
    assert_eq!(conn.connection_type, "ipsec");
    assert_eq!(conn.created_at.as_deref(), Some("2022-12-01T09:04:08.529138Z"));
    assert_eq!(conn.updated_at.as_deref(), Some("2022-12-01T19:04:08.529138Z"));

    // local_routes
    assert_eq!(conn.local_routes.len(), 1);
    assert_eq!(conn.local_routes[0].name, "upcloud-example-route");
    assert_eq!(conn.local_routes[0].route_type, "static");
    assert_eq!(conn.local_routes[0].static_network, "10.0.0.0/24");

    // remote_routes
    assert_eq!(conn.remote_routes.len(), 1);
    assert_eq!(conn.remote_routes[0].name, "remote-example-route");
    assert_eq!(conn.remote_routes[0].route_type, "static");
    assert_eq!(conn.remote_routes[0].static_network, "10.0.1.0/24");

    // tunnels
    assert_eq!(conn.tunnels.len(), 1);
    let tunnel = &conn.tunnels[0];
    assert_eq!(tunnel.name, "example-tunnel-1");
    assert_eq!(tunnel.uuid.as_deref(), Some("1085d477-8d8f-4c97-9bef-731933187777"));
    assert_eq!(tunnel.tunnel_healthy, Some(true));
    assert_eq!(tunnel.tunnel_up, Some(true));
    assert_eq!(tunnel.tunnel_internal_ip.as_deref(), Some("169.254.17.1"));
    assert_eq!(tunnel.internal_peer_ping_interval, Some(0));
    assert_eq!(tunnel.operational_state.as_deref(), Some("established"));
    assert_eq!(tunnel.created_at.as_deref(), Some("2022-12-01T09:04:08.529138Z"));
    assert_eq!(tunnel.updated_at.as_deref(), Some("2022-12-01T19:04:08.529138Z"));

    // tunnel local_address / remote_address
    let local_addr = tunnel.local_address.as_ref().unwrap();
    assert_eq!(local_addr.name.as_deref(), Some("public-ip-1"));
    assert!(local_addr.address.is_none());
    let remote_addr = tunnel.remote_address.as_ref().unwrap();
    assert_eq!(remote_addr.address.as_deref(), Some("100.10.0.111"));
    assert!(remote_addr.name.is_none());

    // ipsec
    let ipsec = tunnel.ipsec.as_ref().unwrap();
    assert_eq!(ipsec.authentication.as_ref().unwrap().authentication, "psk");
    assert!(ipsec.authentication.as_ref().unwrap().psk.is_none());
    assert_eq!(ipsec.child_rekey_time, Some(1440));
    assert_eq!(ipsec.dpd_delay, Some(30));
    assert_eq!(ipsec.dpd_timeout, Some(120));
    assert_eq!(ipsec.ike_lifetime, Some(86400));
    assert_eq!(ipsec.rekey_time, Some(14400));

    let p1_alg = ipsec.phase1_algorithms.as_ref().unwrap();
    assert_eq!(p1_alg.len(), 2);
    assert_eq!(p1_alg[0], "aes128gcm128");
    assert_eq!(p1_alg[1], "aes256gcm128");

    let p1_dh = ipsec.phase1_dh_group_numbers.as_ref().unwrap();
    assert_eq!(p1_dh.len(), 6);
    assert_eq!(p1_dh, &[14, 16, 18, 19, 20, 21]);

    let p1_int = ipsec.phase1_integrity_algorithms.as_ref().unwrap();
    assert_eq!(p1_int.len(), 5);
    assert_eq!(p1_int, &["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"]);

    let p2_alg = ipsec.phase2_algorithms.as_ref().unwrap();
    assert_eq!(p2_alg.len(), 2);
    assert_eq!(p2_alg[0], "aes128gcm128");
    assert_eq!(p2_alg[1], "aes256gcm128");

    let p2_dh = ipsec.phase2_dh_group_numbers.as_ref().unwrap();
    assert_eq!(p2_dh.len(), 6);
    assert_eq!(p2_dh, &[14, 16, 18, 19, 20, 21]);

    let p2_int = ipsec.phase2_integrity_algorithms.as_ref().unwrap();
    assert_eq!(p2_int.len(), 5);
    assert_eq!(p2_int, &["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"]);
}

#[test]
fn deserialize_gateway_plan_list() {
    let plans: Vec<Plan> = serde_json::from_str(GATEWAY_PLAN_LIST_JSON).unwrap();
    assert_eq!(plans.len(), 2);

    // Entry 0: advanced
    assert_eq!(plans[0].name, "advanced");
    assert_eq!(plans[0].per_gateway_bandwidth_mbps, Some(10000));
    assert_eq!(plans[0].per_gateway_max_connections, Some(100000));
    assert_eq!(plans[0].server_number, Some(2));
    assert_eq!(plans[0].vpn_tunnel_amount, Some(10));
    let features_0 = plans[0].supported_features.as_ref().unwrap();
    assert_eq!(features_0.len(), 2);
    assert_eq!(features_0[0], Feature::Nat);
    assert_eq!(features_0[1], Feature::Vpn);

    // Entry 1: development
    assert_eq!(plans[1].name, "development");
    assert_eq!(plans[1].per_gateway_bandwidth_mbps, Some(10));
    assert_eq!(plans[1].per_gateway_max_connections, Some(10000));
    assert_eq!(plans[1].server_number, Some(1));
    assert_eq!(plans[1].vpn_tunnel_amount, Some(0));
    let features_1 = plans[1].supported_features.as_ref().unwrap();
    assert_eq!(features_1.len(), 1);
    assert_eq!(features_1[0], Feature::Nat);
}

const GATEWAY_PLAN_DETAIL_JSON: &str = r#"{
  "name": "advanced",
  "per_gateway_bandwidth_mbps": 10000,
  "per_gateway_max_connections": 100000,
  "server_number": 2,
  "supported_features": ["nat", "vpn"],
  "vpn_tunnel_amount": 10
}"#;

const GATEWAY_DETAIL_JSON: &str = r#"{
  "addresses": [{"address": "100.127.0.241", "name": "public-ip-1"}],
  "configured_status": "started",
  "created_at": "2022-12-01T09:04:08.529138Z",
  "features": ["nat", "vpn"],
  "connections": [
{
  "name": "example-connection",
  "uuid": "1085d477-8d8f-4c97-9bef-731933187538",
  "type": "ipsec",
  "local_routes": [{"name": "upcloud-example-route", "type": "static", "static_network": "10.0.0.0/24"}],
  "remote_routes": [{"name": "remote-example-route", "type": "static", "static_network": "10.0.1.0/24"}],
  "tunnels": [
    {
      "local_address": {"name": "public-ip-1"},
      "name": "example-tunnel-1",
      "uuid": "1085d477-8d8f-4c97-9bef-731933187777",
      "remote_address": {"address": "100.10.0.111"},
      "ipsec": {
        "authentication": {"authentication": "psk"},
        "child_rekey_time": 1440,
        "dpd_delay": 30,
        "dpd_timeout": 120,
        "ike_lifetime": 86400,
        "phase1_algorithms": ["aes128gcm128", "aes256gcm128"],
        "phase1_dh_group_numbers": [14, 16, 18, 19, 20, 21],
        "phase1_integrity_algorithms": ["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"],
        "phase2_algorithms": ["aes128gcm128", "aes256gcm128"],
        "phase2_dh_group_numbers": [14, 16, 18, 19, 20, 21],
        "phase2_integrity_algorithms": ["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"],
        "rekey_time": 14400
      },
      "tunnel_healthy": true,
      "tunnel_up": true,
      "tunnel_internal_ip": "169.254.17.1",
      "internal_peer_ping_interval": 0,
      "operational_state": "established",
      "created_at": "2022-12-01T09:04:08.529138Z",
      "updated_at": "2022-12-01T19:04:08.529138Z"
    }
  ],
  "created_at": "2022-12-01T09:04:08.529138Z",
  "updated_at": "2022-12-01T19:04:08.529138Z"
}
  ],
  "name": "example-gateway",
  "operational_state": "running",
  "plan": "production",
  "routers": [{"created_at": "2022-12-01T09:04:08.529138Z", "uuid": "1085d477-8d8f-4c97-9bef-731933187538"}],
  "labels": [{"key": "env", "value": "testing"}],
  "automatic_tunnel_internal_ip_allocation": true,
  "updated_at": "2022-12-01T19:04:08.529138Z",
  "uuid": "10c153e0-12e4-4dea-8748-4f34850ff55d",
  "zone": "fi-hel1"
}"#;

const GATEWAY_CREATE_REQ_JSON: &str = r#"{
  "name": "example-gateway",
  "zone": "fi-hel1",
  "features": ["nat", "vpn"],
  "plan": "production",
  "routers": [{"uuid": "1085d477-8d8f-4c97-9bef-731933187538"}],
  "addresses": [{"name": "my-public-ip"}],
  "connections": [
{
  "name": "example-connection",
  "type": "ipsec",
  "local_routes": [{"name": "upcloud-example-route", "type": "static", "static_network": "10.0.0.0/24"}],
  "remote_routes": [{"name": "remote-example-route", "type": "static", "static_network": "10.0.1.0/24"}],
  "tunnels": [
    {
      "name": "example-tunnel-1",
      "local_address": {"name": "public-ip-1"},
      "remote_address": {"address": "100.10.0.111"},
      "ipsec": {
        "authentication": {"authentication": "psk", "psk": "secretkey123"}
      },
      "tunnel_internal_ip": "169.254.17.1",
      "internal_peer_ping_interval": 5
    }
  ]
}
  ],
  "labels": [{"key": "env", "value": "testing"}],
  "configured_status": "started",
  "automatic_tunnel_internal_ip_allocation": false
}"#;

const GATEWAY_MODIFY_REQ_JSON: &str = r#"{
  "name": "example-gateway",
  "configured_status": "started",
  "labels": [{"key": "env", "value": "testing"}]
}"#;

const CONNECTION_CREATE_REQ_JSON: &str = r#"{
  "name": "example-connection",
  "type": "ipsec",
  "local_routes": [{"name": "upcloud-example-route", "type": "static", "static_network": "10.0.0.0/24"}],
  "remote_routes": [{"name": "remote-example-route", "type": "static", "static_network": "10.0.1.0/24"}],
  "tunnels": [
{
  "name": "example-tunnel-1",
  "local_address": {"name": "public-ip-1"},
  "remote_address": {"address": "100.10.0.111"},
  "ipsec": {
    "authentication": {"authentication": "psk", "psk": "secretkey123"}
  },
  "tunnel_internal_ip": "169.254.17.1",
  "internal_peer_ping_interval": 0
}
  ]
}"#;

const CONNECTION_DETAIL_JSON: &str = r#"{
  "name": "example-connection",
  "uuid": "1085d477-8d8f-4c97-9bef-731933187538",
  "type": "ipsec",
  "local_routes": [{"name": "upcloud-example-route", "type": "static", "static_network": "10.0.0.0/24"}],
  "remote_routes": [{"name": "remote-example-route", "type": "static", "static_network": "10.0.1.0/24"}],
  "tunnels": [
{
  "local_address": {"name": "public-ip-1"},
  "name": "example-tunnel-1",
  "uuid": "1085d477-8d8f-4c97-9bef-731933187777",
  "remote_address": {"address": "100.10.0.111"},
  "ipsec": {
    "authentication": {"authentication": "psk"},
    "child_rekey_time": 1440,
    "dpd_delay": 30,
    "dpd_timeout": 120,
    "ike_lifetime": 86400,
    "phase1_algorithms": ["aes128gcm128", "aes256gcm128"],
    "phase1_dh_group_numbers": [14, 16, 18, 19, 20, 21],
    "phase1_integrity_algorithms": ["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"],
    "phase2_algorithms": ["aes128gcm128", "aes256gcm128"],
    "phase2_dh_group_numbers": [14, 16, 18, 19, 20, 21],
    "phase2_integrity_algorithms": ["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"],
    "rekey_time": 14400
  },
  "tunnel_healthy": true,
  "tunnel_up": true,
  "tunnel_internal_ip": "169.254.17.1",
  "internal_peer_ping_interval": 0,
  "operational_state": "established",
  "created_at": "2022-12-01T09:04:08.529138Z",
  "updated_at": "2022-12-01T19:04:08.529138Z"
}
  ],
  "created_at": "2022-12-01T09:04:08.529138Z",
  "updated_at": "2022-12-01T19:04:08.529138Z"
}"#;

const CONNECTION_MODIFY_REQ_JSON: &str = r#"{
  "local_routes": [{"name": "modified-routes", "type": "static", "static_network": "10.0.22.0/24"}]
}"#;

const TUNNEL_LIST_JSON: &str = r#"[
  {
"local_address": {"name": "public-ip-1"},
"name": "example-tunnel-1",
"uuid": "1085d477-8d8f-4c97-9bef-731933187538",
"remote_address": {"address": "100.10.0.111"},
"ipsec": {
  "authentication": {"authentication": "psk"},
  "child_rekey_time": 1440,
  "dpd_delay": 30,
  "dpd_timeout": 120,
  "ike_lifetime": 86400,
  "phase1_algorithms": ["aes128gcm128", "aes256gcm128"],
  "phase1_dh_group_numbers": [14, 16, 18, 19, 20, 21],
  "phase1_integrity_algorithms": ["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"],
  "phase2_algorithms": ["aes128gcm128", "aes256gcm128"],
  "phase2_dh_group_numbers": [14, 16, 18, 19, 20, 21],
  "phase2_integrity_algorithms": ["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"],
  "rekey_time": 14400
},
"tunnel_healthy": true,
"tunnel_up": true,
"tunnel_internal_ip": "169.254.17.1",
"internal_peer_ping_interval": 0,
"operational_state": "established",
"created_at": "2022-12-01T09:04:08.529138Z",
"updated_at": "2022-12-01T19:04:08.529138Z"
  }
]"#;

const TUNNEL_CREATE_REQ_JSON: &str = r#"{
  "name": "example-tunnel-2",
  "local_address": {"name": "public-ip-1"},
  "remote_address": {"address": "100.10.0.111"},
  "ipsec": {
"authentication": {"authentication": "psk", "psk": "secretkey123"}
  },
  "tunnel_internal_ip": "169.254.17.1",
  "internal_peer_ping_interval": 0
}"#;

const TUNNEL_MODIFY_REQ_JSON: &str = r#"{
  "remote_address": {"address": "100.10.0.222"}
}"#;

const GATEWAY_REPLACE_REQ_JSON: &str = r#"{
  "name": "example-gateway",
  "configured_status": "started"
}"#;

const GATEWAY_LABEL_LIST_JSON: &str = r#"[
  {"key": "env", "value": "staging"},
  {"key": "foo", "value": "bar"}
]"#;

const GATEWAY_LABEL_DETAIL_JSON: &str = r#"{"key": "env", "value": "staging"}"#;

const GATEWAY_LABEL_CREATE_JSON: &str = r#"{"key": "env", "value": "production"}"#;

const CONNECTION_LIST_JSON: &str = r#"[
  {
"name": "example-connection",
"uuid": "1085d477-8d8f-4c97-9bef-731933187538",
"type": "ipsec",
"local_routes": [{"name": "upcloud-example-route", "type": "static", "static_network": "10.0.0.0/24"}],
"remote_routes": [{"name": "remote-example-route", "type": "static", "static_network": "10.0.1.0/24"}],
"tunnels": [
  {
    "local_address": {"name": "public-ip-1"},
    "name": "example-tunnel-1",
    "uuid": "1085d477-8d8f-4c97-9bef-731933187777",
    "remote_address": {"address": "100.10.0.111"},
    "ipsec": {
      "authentication": {"authentication": "psk"},
      "child_rekey_time": 1440,
      "dpd_delay": 30,
      "dpd_timeout": 120,
      "ike_lifetime": 86400,
      "phase1_algorithms": ["aes128gcm128", "aes256gcm128"],
      "phase1_dh_group_numbers": [14, 16, 18, 19, 20, 21],
      "phase1_integrity_algorithms": ["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"],
      "phase2_algorithms": ["aes128gcm128", "aes256gcm128"],
      "phase2_dh_group_numbers": [14, 16, 18, 19, 20, 21],
      "phase2_integrity_algorithms": ["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"],
      "rekey_time": 14400
    },
    "tunnel_healthy": true,
    "tunnel_up": true,
    "tunnel_internal_ip": "169.254.17.1",
    "internal_peer_ping_interval": 0,
    "operational_state": "established",
    "created_at": "2022-12-01T09:04:08.529138Z",
    "updated_at": "2022-12-01T19:04:08.529138Z"
  }
],
"created_at": "2022-12-01T09:04:08.529138Z",
"updated_at": "2022-12-01T19:04:08.529138Z"
  }
]"#;

#[test]
fn deserialize_gateway_plan_detail() {
    let plan: Plan = serde_json::from_str(GATEWAY_PLAN_DETAIL_JSON).unwrap();
    assert_eq!(plan.name, "advanced");
    assert_eq!(plan.per_gateway_bandwidth_mbps, Some(10000));
    assert_eq!(plan.per_gateway_max_connections, Some(100000));
    assert_eq!(plan.server_number, Some(2));
    let features = plan.supported_features.unwrap();
    assert_eq!(features.len(), 2);
    assert_eq!(features[0], Feature::Nat);
    assert_eq!(features[1], Feature::Vpn);
    assert_eq!(plan.vpn_tunnel_amount, Some(10));
}

#[test]
fn deserialize_gateway_detail() {
    let gw: Gateway = serde_json::from_str(GATEWAY_DETAIL_JSON).unwrap();
    assert_eq!(gw.uuid, "10c153e0-12e4-4dea-8748-4f34850ff55d");
    assert_eq!(gw.name, "example-gateway");
    assert_eq!(gw.zone, "fi-hel1");
    assert_eq!(gw.plan, "production");
    assert_eq!(gw.operational_state.as_deref(), Some("running"));
    assert_eq!(gw.configured_status.as_deref(), Some("started"));
    assert_eq!(gw.automatic_tunnel_internal_ip_allocation, Some(true));
    assert_eq!(gw.created_at.as_deref(), Some("2022-12-01T09:04:08.529138Z"));
    assert_eq!(gw.updated_at.as_deref(), Some("2022-12-01T19:04:08.529138Z"));

    // features
    assert_eq!(gw.features.len(), 2);
    assert_eq!(gw.features[0], Feature::Nat);
    assert_eq!(gw.features[1], Feature::Vpn);

    // addresses
    assert_eq!(gw.addresses.len(), 1);
    assert_eq!(gw.addresses[0].name, "public-ip-1");
    assert_eq!(gw.addresses[0].address.as_deref(), Some("100.127.0.241"));

    // routers
    assert_eq!(gw.routers.len(), 1);
    assert_eq!(gw.routers[0].uuid, "1085d477-8d8f-4c97-9bef-731933187538");
    assert_eq!(gw.routers[0].created_at.as_deref(), Some("2022-12-01T09:04:08.529138Z"));

    // labels
    let labels = gw.labels.as_ref().unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0].key, "env");
    assert_eq!(labels[0].value, "testing");

    // connections
    assert_eq!(gw.connections.len(), 1);
    let conn = &gw.connections[0];
    assert_eq!(conn.name, "example-connection");
    assert_eq!(conn.uuid.as_deref(), Some("1085d477-8d8f-4c97-9bef-731933187538"));
    assert_eq!(conn.connection_type, "ipsec");
    assert_eq!(conn.created_at.as_deref(), Some("2022-12-01T09:04:08.529138Z"));
    assert_eq!(conn.updated_at.as_deref(), Some("2022-12-01T19:04:08.529138Z"));

    assert_eq!(conn.local_routes.len(), 1);
    assert_eq!(conn.local_routes[0].name, "upcloud-example-route");
    assert_eq!(conn.local_routes[0].route_type, "static");
    assert_eq!(conn.local_routes[0].static_network, "10.0.0.0/24");

    assert_eq!(conn.remote_routes.len(), 1);
    assert_eq!(conn.remote_routes[0].name, "remote-example-route");
    assert_eq!(conn.remote_routes[0].route_type, "static");
    assert_eq!(conn.remote_routes[0].static_network, "10.0.1.0/24");

    // tunnel
    assert_eq!(conn.tunnels.len(), 1);
    let tunnel = &conn.tunnels[0];
    assert_eq!(tunnel.name, "example-tunnel-1");
    assert_eq!(tunnel.uuid.as_deref(), Some("1085d477-8d8f-4c97-9bef-731933187777"));
    assert_eq!(tunnel.tunnel_healthy, Some(true));
    assert_eq!(tunnel.tunnel_up, Some(true));
    assert_eq!(tunnel.tunnel_internal_ip.as_deref(), Some("169.254.17.1"));
    assert_eq!(tunnel.internal_peer_ping_interval, Some(0));
    assert_eq!(tunnel.operational_state.as_deref(), Some("established"));
    assert_eq!(tunnel.created_at.as_deref(), Some("2022-12-01T09:04:08.529138Z"));
    assert_eq!(tunnel.updated_at.as_deref(), Some("2022-12-01T19:04:08.529138Z"));

    let local_addr = tunnel.local_address.as_ref().unwrap();
    assert_eq!(local_addr.name.as_deref(), Some("public-ip-1"));
    assert!(local_addr.address.is_none());
    let remote_addr = tunnel.remote_address.as_ref().unwrap();
    assert_eq!(remote_addr.address.as_deref(), Some("100.10.0.111"));
    assert!(remote_addr.name.is_none());

    // ipsec
    let ipsec = tunnel.ipsec.as_ref().unwrap();
    assert_eq!(ipsec.authentication.as_ref().unwrap().authentication, "psk");
    assert!(ipsec.authentication.as_ref().unwrap().psk.is_none());
    assert_eq!(ipsec.child_rekey_time, Some(1440));
    assert_eq!(ipsec.dpd_delay, Some(30));
    assert_eq!(ipsec.dpd_timeout, Some(120));
    assert_eq!(ipsec.ike_lifetime, Some(86400));
    assert_eq!(ipsec.rekey_time, Some(14400));
    assert_eq!(ipsec.phase1_algorithms.as_ref().unwrap(), &["aes128gcm128", "aes256gcm128"]);
    assert_eq!(ipsec.phase1_dh_group_numbers.as_ref().unwrap(), &[14, 16, 18, 19, 20, 21]);
    assert_eq!(ipsec.phase1_integrity_algorithms.as_ref().unwrap(), &["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"]);
    assert_eq!(ipsec.phase2_algorithms.as_ref().unwrap(), &["aes128gcm128", "aes256gcm128"]);
    assert_eq!(ipsec.phase2_dh_group_numbers.as_ref().unwrap(), &[14, 16, 18, 19, 20, 21]);
    assert_eq!(ipsec.phase2_integrity_algorithms.as_ref().unwrap(), &["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"]);
}

#[test]
fn deserialize_create_gateway_request() {
    let req: CreateRequest =
        serde_json::from_str(GATEWAY_CREATE_REQ_JSON).unwrap();
    assert_eq!(req.name, "example-gateway");
    assert_eq!(req.zone, "fi-hel1");
    assert_eq!(req.configured_status, "started");
    assert_eq!(req.plan, Some("production".to_string()));
    assert_eq!(req.automatic_tunnel_internal_ip_allocation, Some(false));

    // features
    assert_eq!(req.features.len(), 2);
    assert_eq!(req.features[0], Feature::Nat);
    assert_eq!(req.features[1], Feature::Vpn);

    // routers
    assert_eq!(req.routers.len(), 1);
    assert_eq!(req.routers[0].uuid, "1085d477-8d8f-4c97-9bef-731933187538");

    // addresses
    let addrs = req.addresses.as_ref().unwrap();
    assert_eq!(addrs.len(), 1);
    assert_eq!(addrs[0].name, "my-public-ip");

    // labels
    let labels = req.labels.as_ref().unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0].key, "env");
    assert_eq!(labels[0].value, "testing");

    // connections
    let conns = req.connections.as_ref().unwrap();
    assert_eq!(conns.len(), 1);
    assert_eq!(conns[0].name, "example-connection");
    assert_eq!(conns[0].connection_type, "ipsec");

    // connection local_routes
    assert_eq!(conns[0].local_routes.len(), 1);
    assert_eq!(conns[0].local_routes[0].name, "upcloud-example-route");
    assert_eq!(conns[0].local_routes[0].route_type, "static");
    assert_eq!(conns[0].local_routes[0].static_network, "10.0.0.0/24");

    // connection remote_routes
    assert_eq!(conns[0].remote_routes.len(), 1);
    assert_eq!(conns[0].remote_routes[0].name, "remote-example-route");
    assert_eq!(conns[0].remote_routes[0].route_type, "static");
    assert_eq!(conns[0].remote_routes[0].static_network, "10.0.1.0/24");

    // tunnels
    let tunnels = conns[0].tunnels.as_ref().unwrap();
    assert_eq!(tunnels.len(), 1);
    assert_eq!(tunnels[0].name, "example-tunnel-1");
    assert_eq!(tunnels[0].tunnel_internal_ip.as_deref(), Some("169.254.17.1"));
    assert_eq!(tunnels[0].internal_peer_ping_interval, Some(5));

    // tunnel addresses
    assert_eq!(tunnels[0].local_address.name.as_deref(), Some("public-ip-1"));
    assert!(tunnels[0].local_address.address.is_none());
    assert_eq!(tunnels[0].remote_address.address.as_deref(), Some("100.10.0.111"));
    assert!(tunnels[0].remote_address.name.is_none());

    // tunnel ipsec
    let ipsec = tunnels[0].ipsec.as_ref().unwrap();
    let auth = ipsec.authentication.as_ref().unwrap();
    assert_eq!(auth.authentication, "psk");
    assert_eq!(auth.psk.as_deref(), Some("secretkey123"));
    assert!(ipsec.child_rekey_time.is_none());
    assert!(ipsec.dpd_delay.is_none());
    assert!(ipsec.dpd_timeout.is_none());
    assert!(ipsec.ike_lifetime.is_none());
    assert!(ipsec.rekey_time.is_none());
    assert!(ipsec.phase1_algorithms.is_none());
    assert!(ipsec.phase1_dh_group_numbers.is_none());
    assert!(ipsec.phase1_integrity_algorithms.is_none());
    assert!(ipsec.phase2_algorithms.is_none());
    assert!(ipsec.phase2_dh_group_numbers.is_none());
    assert!(ipsec.phase2_integrity_algorithms.is_none());
}

#[test]
fn deserialize_modify_gateway_request() {
    let req: ModifyRequest =
        serde_json::from_str(GATEWAY_MODIFY_REQ_JSON).unwrap();
    assert_eq!(req.name.as_deref(), Some("example-gateway"));
    assert_eq!(req.configured_status.as_deref(), Some("started"));
    let labels = req.labels.as_ref().unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0].key, "env");
    assert_eq!(labels[0].value, "testing");
    assert!(req.features.is_none());
    assert!(req.plan.is_none());
    assert!(req.connections.is_none());
    assert!(req.automatic_tunnel_internal_ip_allocation.is_none());
}

#[test]
fn deserialize_create_connection_request() {
    let req: CreateConnectionRequest =
        serde_json::from_str(CONNECTION_CREATE_REQ_JSON).unwrap();
    assert_eq!(req.name, "example-connection");
    assert_eq!(req.connection_type, "ipsec");

    // local_routes
    assert_eq!(req.local_routes.len(), 1);
    assert_eq!(req.local_routes[0].name, "upcloud-example-route");
    assert_eq!(req.local_routes[0].route_type, "static");
    assert_eq!(req.local_routes[0].static_network, "10.0.0.0/24");

    // remote_routes
    assert_eq!(req.remote_routes.len(), 1);
    assert_eq!(req.remote_routes[0].name, "remote-example-route");
    assert_eq!(req.remote_routes[0].route_type, "static");
    assert_eq!(req.remote_routes[0].static_network, "10.0.1.0/24");

    // tunnels
    let tunnels = req.tunnels.as_ref().unwrap();
    assert_eq!(tunnels.len(), 1);
    assert_eq!(tunnels[0].name, "example-tunnel-1");
    assert_eq!(tunnels[0].tunnel_internal_ip.as_deref(), Some("169.254.17.1"));
    assert_eq!(tunnels[0].internal_peer_ping_interval, Some(0));

    // tunnel addresses
    assert_eq!(tunnels[0].local_address.name.as_deref(), Some("public-ip-1"));
    assert!(tunnels[0].local_address.address.is_none());
    assert_eq!(tunnels[0].remote_address.address.as_deref(), Some("100.10.0.111"));
    assert!(tunnels[0].remote_address.name.is_none());

    // tunnel ipsec
    let ipsec = tunnels[0].ipsec.as_ref().unwrap();
    let auth = ipsec.authentication.as_ref().unwrap();
    assert_eq!(auth.authentication, "psk");
    assert_eq!(auth.psk.as_deref(), Some("secretkey123"));
    assert!(ipsec.child_rekey_time.is_none());
    assert!(ipsec.dpd_delay.is_none());
    assert!(ipsec.dpd_timeout.is_none());
    assert!(ipsec.ike_lifetime.is_none());
    assert!(ipsec.rekey_time.is_none());
    assert!(ipsec.phase1_algorithms.is_none());
    assert!(ipsec.phase1_dh_group_numbers.is_none());
    assert!(ipsec.phase1_integrity_algorithms.is_none());
    assert!(ipsec.phase2_algorithms.is_none());
    assert!(ipsec.phase2_dh_group_numbers.is_none());
    assert!(ipsec.phase2_integrity_algorithms.is_none());
}

#[test]
fn deserialize_connection_detail() {
    let conn: Connection =
        serde_json::from_str(CONNECTION_DETAIL_JSON).unwrap();
    assert_eq!(conn.name, "example-connection");
    assert_eq!(conn.uuid.as_deref(), Some("1085d477-8d8f-4c97-9bef-731933187538"));
    assert_eq!(conn.connection_type, "ipsec");
    assert_eq!(conn.created_at.as_deref(), Some("2022-12-01T09:04:08.529138Z"));
    assert_eq!(conn.updated_at.as_deref(), Some("2022-12-01T19:04:08.529138Z"));

    // local_routes
    assert_eq!(conn.local_routes.len(), 1);
    assert_eq!(conn.local_routes[0].name, "upcloud-example-route");
    assert_eq!(conn.local_routes[0].route_type, "static");
    assert_eq!(conn.local_routes[0].static_network, "10.0.0.0/24");

    // remote_routes
    assert_eq!(conn.remote_routes.len(), 1);
    assert_eq!(conn.remote_routes[0].name, "remote-example-route");
    assert_eq!(conn.remote_routes[0].route_type, "static");
    assert_eq!(conn.remote_routes[0].static_network, "10.0.1.0/24");

    // tunnel
    assert_eq!(conn.tunnels.len(), 1);
    let tunnel = &conn.tunnels[0];
    assert_eq!(tunnel.name, "example-tunnel-1");
    assert_eq!(tunnel.uuid.as_deref(), Some("1085d477-8d8f-4c97-9bef-731933187777"));
    assert_eq!(tunnel.operational_state.as_deref(), Some("established"));
    assert_eq!(tunnel.tunnel_healthy, Some(true));
    assert_eq!(tunnel.tunnel_up, Some(true));
    assert_eq!(tunnel.tunnel_internal_ip.as_deref(), Some("169.254.17.1"));
    assert_eq!(tunnel.internal_peer_ping_interval, Some(0));
    assert_eq!(tunnel.created_at.as_deref(), Some("2022-12-01T09:04:08.529138Z"));
    assert_eq!(tunnel.updated_at.as_deref(), Some("2022-12-01T19:04:08.529138Z"));

    // tunnel addresses
    let local_addr = tunnel.local_address.as_ref().unwrap();
    assert_eq!(local_addr.name.as_deref(), Some("public-ip-1"));
    assert!(local_addr.address.is_none());
    let remote_addr = tunnel.remote_address.as_ref().unwrap();
    assert_eq!(remote_addr.address.as_deref(), Some("100.10.0.111"));
    assert!(remote_addr.name.is_none());

    // ipsec
    let ipsec = tunnel.ipsec.as_ref().unwrap();
    assert_eq!(ipsec.authentication.as_ref().unwrap().authentication, "psk");
    assert!(ipsec.authentication.as_ref().unwrap().psk.is_none());
    assert_eq!(ipsec.child_rekey_time, Some(1440));
    assert_eq!(ipsec.dpd_delay, Some(30));
    assert_eq!(ipsec.dpd_timeout, Some(120));
    assert_eq!(ipsec.ike_lifetime, Some(86400));
    assert_eq!(ipsec.rekey_time, Some(14400));
    assert_eq!(ipsec.phase1_algorithms.as_ref().unwrap(), &["aes128gcm128", "aes256gcm128"]);
    assert_eq!(ipsec.phase1_dh_group_numbers.as_ref().unwrap(), &[14, 16, 18, 19, 20, 21]);
    assert_eq!(ipsec.phase1_integrity_algorithms.as_ref().unwrap(), &["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"]);
    assert_eq!(ipsec.phase2_algorithms.as_ref().unwrap(), &["aes128gcm128", "aes256gcm128"]);
    assert_eq!(ipsec.phase2_dh_group_numbers.as_ref().unwrap(), &[14, 16, 18, 19, 20, 21]);
    assert_eq!(ipsec.phase2_integrity_algorithms.as_ref().unwrap(), &["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"]);
}

#[test]
fn deserialize_modify_connection_request() {
    let val: serde_json::Value =
        serde_json::from_str(CONNECTION_MODIFY_REQ_JSON).unwrap();
    let obj = val.as_object().unwrap();
    assert_eq!(obj.len(), 1);
    let routes = val["local_routes"].as_array().unwrap();
    assert_eq!(routes.len(), 1);
    assert_eq!(routes[0]["name"], "modified-routes");
    assert_eq!(routes[0]["type"], "static");
    assert_eq!(routes[0]["static_network"], "10.0.22.0/24");
}

#[test]
fn deserialize_tunnel_list() {
    let tunnels: Vec<Tunnel> =
        serde_json::from_str(TUNNEL_LIST_JSON).unwrap();
    assert_eq!(tunnels.len(), 1);
    let t = &tunnels[0];
    assert_eq!(t.name, "example-tunnel-1");
    assert_eq!(t.uuid.as_deref(), Some("1085d477-8d8f-4c97-9bef-731933187538"));
    assert_eq!(t.tunnel_healthy, Some(true));
    assert_eq!(t.tunnel_up, Some(true));
    assert_eq!(t.tunnel_internal_ip.as_deref(), Some("169.254.17.1"));
    assert_eq!(t.internal_peer_ping_interval, Some(0));
    assert_eq!(t.operational_state.as_deref(), Some("established"));
    assert_eq!(t.created_at.as_deref(), Some("2022-12-01T09:04:08.529138Z"));
    assert_eq!(t.updated_at.as_deref(), Some("2022-12-01T19:04:08.529138Z"));

    // tunnel addresses
    let local_addr = t.local_address.as_ref().unwrap();
    assert_eq!(local_addr.name.as_deref(), Some("public-ip-1"));
    assert!(local_addr.address.is_none());
    let remote_addr = t.remote_address.as_ref().unwrap();
    assert_eq!(remote_addr.address.as_deref(), Some("100.10.0.111"));
    assert!(remote_addr.name.is_none());

    // ipsec
    let ipsec = t.ipsec.as_ref().unwrap();
    assert_eq!(ipsec.authentication.as_ref().unwrap().authentication, "psk");
    assert!(ipsec.authentication.as_ref().unwrap().psk.is_none());
    assert_eq!(ipsec.child_rekey_time, Some(1440));
    assert_eq!(ipsec.dpd_delay, Some(30));
    assert_eq!(ipsec.dpd_timeout, Some(120));
    assert_eq!(ipsec.ike_lifetime, Some(86400));
    assert_eq!(ipsec.rekey_time, Some(14400));
    assert_eq!(ipsec.phase1_algorithms.as_ref().unwrap(), &["aes128gcm128", "aes256gcm128"]);
    assert_eq!(ipsec.phase1_dh_group_numbers.as_ref().unwrap(), &[14, 16, 18, 19, 20, 21]);
    assert_eq!(ipsec.phase1_integrity_algorithms.as_ref().unwrap(), &["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"]);
    assert_eq!(ipsec.phase2_algorithms.as_ref().unwrap(), &["aes128gcm128", "aes256gcm128"]);
    assert_eq!(ipsec.phase2_dh_group_numbers.as_ref().unwrap(), &[14, 16, 18, 19, 20, 21]);
    assert_eq!(ipsec.phase2_integrity_algorithms.as_ref().unwrap(), &["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"]);
}

#[test]
fn deserialize_tunnel_detail() {
    // Same structure as tunnel_list[0], parsed as a single Tunnel
    let tunnels: Vec<Tunnel> =
        serde_json::from_str(TUNNEL_LIST_JSON).unwrap();
    let json_str = serde_json::to_string(&tunnels[0]).unwrap();
    let tunnel: Tunnel = serde_json::from_str(&json_str).unwrap();
    assert_eq!(tunnel.name, "example-tunnel-1");
    assert_eq!(tunnel.uuid.as_deref(), Some("1085d477-8d8f-4c97-9bef-731933187538"));
    assert_eq!(tunnel.tunnel_healthy, Some(true));
    assert_eq!(tunnel.tunnel_up, Some(true));
    assert_eq!(tunnel.tunnel_internal_ip.as_deref(), Some("169.254.17.1"));
    assert_eq!(tunnel.internal_peer_ping_interval, Some(0));
    assert_eq!(tunnel.operational_state.as_deref(), Some("established"));
    assert_eq!(tunnel.created_at.as_deref(), Some("2022-12-01T09:04:08.529138Z"));
    assert_eq!(tunnel.updated_at.as_deref(), Some("2022-12-01T19:04:08.529138Z"));

    let local = tunnel.local_address.as_ref().unwrap();
    assert_eq!(local.name.as_deref(), Some("public-ip-1"));
    assert!(local.address.is_none());
    let remote = tunnel.remote_address.as_ref().unwrap();
    assert_eq!(remote.address.as_deref(), Some("100.10.0.111"));
    assert!(remote.name.is_none());

    // ipsec
    let ipsec = tunnel.ipsec.as_ref().unwrap();
    assert_eq!(ipsec.authentication.as_ref().unwrap().authentication, "psk");
    assert!(ipsec.authentication.as_ref().unwrap().psk.is_none());
    assert_eq!(ipsec.child_rekey_time, Some(1440));
    assert_eq!(ipsec.dpd_delay, Some(30));
    assert_eq!(ipsec.dpd_timeout, Some(120));
    assert_eq!(ipsec.ike_lifetime, Some(86400));
    assert_eq!(ipsec.rekey_time, Some(14400));
    assert_eq!(ipsec.phase1_algorithms.as_ref().unwrap(), &["aes128gcm128", "aes256gcm128"]);
    assert_eq!(ipsec.phase1_dh_group_numbers.as_ref().unwrap(), &[14, 16, 18, 19, 20, 21]);
    assert_eq!(ipsec.phase1_integrity_algorithms.as_ref().unwrap(), &["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"]);
    assert_eq!(ipsec.phase2_algorithms.as_ref().unwrap(), &["aes128gcm128", "aes256gcm128"]);
    assert_eq!(ipsec.phase2_dh_group_numbers.as_ref().unwrap(), &[14, 16, 18, 19, 20, 21]);
    assert_eq!(ipsec.phase2_integrity_algorithms.as_ref().unwrap(), &["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"]);
}

#[test]
fn deserialize_create_tunnel_request() {
    let req: CreateTunnelRequest =
        serde_json::from_str(TUNNEL_CREATE_REQ_JSON).unwrap();
    assert_eq!(req.name, "example-tunnel-2");
    assert_eq!(req.tunnel_internal_ip.as_deref(), Some("169.254.17.1"));
    assert_eq!(req.internal_peer_ping_interval, Some(0));

    // local_address
    assert_eq!(req.local_address.name.as_deref(), Some("public-ip-1"));
    assert!(req.local_address.address.is_none());

    // remote_address
    assert_eq!(req.remote_address.address.as_deref(), Some("100.10.0.111"));
    assert!(req.remote_address.name.is_none());

    // ipsec
    let ipsec = req.ipsec.as_ref().unwrap();
    let auth = ipsec.authentication.as_ref().unwrap();
    assert_eq!(auth.authentication, "psk");
    assert_eq!(auth.psk.as_deref(), Some("secretkey123"));
    assert!(ipsec.child_rekey_time.is_none());
    assert!(ipsec.dpd_delay.is_none());
    assert!(ipsec.dpd_timeout.is_none());
    assert!(ipsec.ike_lifetime.is_none());
    assert!(ipsec.rekey_time.is_none());
    assert!(ipsec.phase1_algorithms.is_none());
    assert!(ipsec.phase1_dh_group_numbers.is_none());
    assert!(ipsec.phase1_integrity_algorithms.is_none());
    assert!(ipsec.phase2_algorithms.is_none());
    assert!(ipsec.phase2_dh_group_numbers.is_none());
    assert!(ipsec.phase2_integrity_algorithms.is_none());
}

#[test]
fn deserialize_modify_tunnel_request() {
    let val: serde_json::Value =
        serde_json::from_str(TUNNEL_MODIFY_REQ_JSON).unwrap();
    let obj = val.as_object().unwrap();
    assert_eq!(obj.len(), 1);
    let remote = val["remote_address"].as_object().unwrap();
    assert_eq!(remote.len(), 1);
    assert_eq!(val["remote_address"]["address"], "100.10.0.222");
}

#[test]
fn deserialize_replace_gateway_request() {
    let req: ModifyRequest =
        serde_json::from_str(GATEWAY_REPLACE_REQ_JSON).unwrap();
    assert_eq!(req.name.as_deref(), Some("example-gateway"));
    assert_eq!(req.configured_status.as_deref(), Some("started"));
    assert!(req.labels.is_none());
    assert!(req.features.is_none());
    assert!(req.plan.is_none());
    assert!(req.connections.is_none());
    assert!(req.automatic_tunnel_internal_ip_allocation.is_none());
}

#[test]
fn deserialize_gateway_label_list() {
    let labels: Vec<crate::types::common::Label> =
        serde_json::from_str(GATEWAY_LABEL_LIST_JSON).unwrap();
    assert_eq!(labels.len(), 2);
    assert_eq!(labels[0].key, "env");
    assert_eq!(labels[0].value, "staging");
    assert_eq!(labels[1].key, "foo");
    assert_eq!(labels[1].value, "bar");
}

#[test]
fn deserialize_gateway_label_detail() {
    let label: crate::types::common::Label =
        serde_json::from_str(GATEWAY_LABEL_DETAIL_JSON).unwrap();
    assert_eq!(label.key, "env");
    assert_eq!(label.value, "staging");
}

#[test]
fn deserialize_gateway_label_create() {
    let label: crate::types::common::Label =
        serde_json::from_str(GATEWAY_LABEL_CREATE_JSON).unwrap();
    assert_eq!(label.key, "env");
    assert_eq!(label.value, "production");
}

#[test]
fn deserialize_connection_list() {
    let conns: Vec<Connection> =
        serde_json::from_str(CONNECTION_LIST_JSON).unwrap();
    assert_eq!(conns.len(), 1);
    let conn = &conns[0];
    assert_eq!(conn.name, "example-connection");
    assert_eq!(conn.uuid.as_deref(), Some("1085d477-8d8f-4c97-9bef-731933187538"));
    assert_eq!(conn.connection_type, "ipsec");
    assert_eq!(conn.created_at.as_deref(), Some("2022-12-01T09:04:08.529138Z"));
    assert_eq!(conn.updated_at.as_deref(), Some("2022-12-01T19:04:08.529138Z"));

    // local_routes
    assert_eq!(conn.local_routes.len(), 1);
    assert_eq!(conn.local_routes[0].name, "upcloud-example-route");
    assert_eq!(conn.local_routes[0].route_type, "static");
    assert_eq!(conn.local_routes[0].static_network, "10.0.0.0/24");

    // remote_routes
    assert_eq!(conn.remote_routes.len(), 1);
    assert_eq!(conn.remote_routes[0].name, "remote-example-route");
    assert_eq!(conn.remote_routes[0].route_type, "static");
    assert_eq!(conn.remote_routes[0].static_network, "10.0.1.0/24");

    // tunnel
    assert_eq!(conn.tunnels.len(), 1);
    let tunnel = &conn.tunnels[0];
    assert_eq!(tunnel.name, "example-tunnel-1");
    assert_eq!(tunnel.uuid.as_deref(), Some("1085d477-8d8f-4c97-9bef-731933187777"));
    assert_eq!(tunnel.tunnel_healthy, Some(true));
    assert_eq!(tunnel.tunnel_up, Some(true));
    assert_eq!(tunnel.tunnel_internal_ip.as_deref(), Some("169.254.17.1"));
    assert_eq!(tunnel.internal_peer_ping_interval, Some(0));
    assert_eq!(tunnel.operational_state.as_deref(), Some("established"));
    assert_eq!(tunnel.created_at.as_deref(), Some("2022-12-01T09:04:08.529138Z"));
    assert_eq!(tunnel.updated_at.as_deref(), Some("2022-12-01T19:04:08.529138Z"));

    // tunnel addresses
    let local_addr = tunnel.local_address.as_ref().unwrap();
    assert_eq!(local_addr.name.as_deref(), Some("public-ip-1"));
    assert!(local_addr.address.is_none());
    let remote_addr = tunnel.remote_address.as_ref().unwrap();
    assert_eq!(remote_addr.address.as_deref(), Some("100.10.0.111"));
    assert!(remote_addr.name.is_none());

    // ipsec
    let ipsec = tunnel.ipsec.as_ref().unwrap();
    assert_eq!(ipsec.authentication.as_ref().unwrap().authentication, "psk");
    assert!(ipsec.authentication.as_ref().unwrap().psk.is_none());
    assert_eq!(ipsec.child_rekey_time, Some(1440));
    assert_eq!(ipsec.dpd_delay, Some(30));
    assert_eq!(ipsec.dpd_timeout, Some(120));
    assert_eq!(ipsec.ike_lifetime, Some(86400));
    assert_eq!(ipsec.rekey_time, Some(14400));
    assert_eq!(ipsec.phase1_algorithms.as_ref().unwrap(), &["aes128gcm128", "aes256gcm128"]);
    assert_eq!(ipsec.phase1_dh_group_numbers.as_ref().unwrap(), &[14, 16, 18, 19, 20, 21]);
    assert_eq!(ipsec.phase1_integrity_algorithms.as_ref().unwrap(), &["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"]);
    assert_eq!(ipsec.phase2_algorithms.as_ref().unwrap(), &["aes128gcm128", "aes256gcm128"]);
    assert_eq!(ipsec.phase2_dh_group_numbers.as_ref().unwrap(), &[14, 16, 18, 19, 20, 21]);
    assert_eq!(ipsec.phase2_integrity_algorithms.as_ref().unwrap(), &["aes128gmac", "aes256gmac", "sha256", "sha384", "sha512"]);
}
