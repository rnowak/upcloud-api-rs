use super::*;
// Source: https://developers.upcloud.com/1.3/35-load-balancers/#create-service
const LB_PLAN_LIST_JSON: &str = r#"
    [
        {
            "name": "development",
            "per_server_max_sessions": 10000,
            "server_number": 1
        },
        {
            "name": "production",
            "per_server_max_sessions": 50000,
            "server_number": 2
        }
    ]
"#;

#[test]
fn deserialize_lb_plan_list() {
    let plans: Vec<Plan> = serde_json::from_str(LB_PLAN_LIST_JSON).unwrap();
    assert_eq!(plans.len(), 2);
    assert_eq!(plans[0].name, "development");
    assert_eq!(plans[0].per_server_max_sessions, Some(10000));
    assert_eq!(plans[0].server_number, Some(1));
    assert_eq!(plans[0].per_month_price_eur, None);
    assert_eq!(plans[0].per_hour_price_eur, None);
    assert_eq!(plans[0].max_server_count, None);
    assert_eq!(plans[0].max_member_count, None);
    assert_eq!(plans[0].max_backend_count, None);
    assert_eq!(plans[0].max_frontend_count, None);
    assert_eq!(plans[1].name, "production");
    assert_eq!(plans[1].per_server_max_sessions, Some(50000));
    assert_eq!(plans[1].server_number, Some(2));
    assert_eq!(plans[1].per_month_price_eur, None);
    assert_eq!(plans[1].per_hour_price_eur, None);
    assert_eq!(plans[1].max_server_count, None);
    assert_eq!(plans[1].max_member_count, None);
    assert_eq!(plans[1].max_backend_count, None);
    assert_eq!(plans[1].max_frontend_count, None);
}
// Source: https://developers.upcloud.com/1.3/35-load-balancers/#list-services
const LB_LIST_JSON: &str = r#"
    [
        {
            "backends": [
                {
                    "created_at": "2022-10-04T16:11:14.569638Z",
                    "members": [
                        {
                            "backup": false,
                            "created_at": "2022-10-04T16:11:14.569638Z",
                            "enabled": true,
                            "ip": "172.16.1.4",
                            "max_sessions": 1000,
                            "name": "example-member-1",
                            "port": 8000,
                            "type": "static",
                            "weight": 100
                        }
                    ],
                    "name": "example-backend-1",
                    "properties": {
                        "health_check_type": "http",
                        "timeout_server": 10
                    },
                    "updated_at": "2022-10-04T16:11:14.569638Z"
                },
                {
                    "created_at": "2022-10-04T16:11:14.569638Z",
                    "members": [
                        {
                            "enabled": true,
                            "ip": "",
                            "max_sessions": 1000,
                            "name": "member.example.com",
                            "port": 8000,
                            "type": "dynamic",
                            "weight": 100
                        }
                    ],
                    "name": "example-backend-2",
                    "resolver": "example-resolver",
                    "tls_configs": [],
                    "updated_at": "2022-10-04T16:11:14.569638Z"
                }
            ],
            "configured_status": "started",
            "created_at": "2022-10-04T16:11:14.569638Z",
            "dns_name": "lb-0a498284629e4629a55d0415a6e89dda-1.upcloudlb.com",
            "frontends": [
                {
                    "created_at": "2022-10-04T16:11:14.569638Z",
                    "default_backend": "example-backend-1",
                    "mode": "http",
                    "name": "example-frontend",
                    "networks": [
                        {
                            "name": "example-network-1"
                        }
                    ],
                    "port": 443,
                    "properties": {
                        "timeout_client": 5
                    },
                    "rules": [
                        {
                            "actions": [
                                {
                                    "type": "use_backend",
                                    "action_use_backend": {
                                        "backend": "example-backend-2"
                                    }
                                }
                            ],
                            "matchers": [
                                {
                                    "type": "path",
                                    "match_path": {
                                        "method": "exact",
                                        "value": "/app"
                                    }
                                }
                            ],
                            "name": "example-rule-1",
                            "priority": 100
                        }
                    ],
                    "tls_configs": [
                        {
                            "certificate_bundle_uuid": "0a73c569-a2d1-481a-917f-b0fdafd78136",
                            "name": "example-tls-config"
                        }
                    ],
                    "updated_at": "2022-10-04T16:11:14.569638Z"
                }
            ],
            "labels": [
                {
                    "key": "env",
                    "value": "staging"
                },
                {
                    "key": "foo",
                    "value": "bar"
                }
            ],
            "name": "example-service",
            "networks": [
                {
                    "created_at": "2022-10-04T16:11:14.569638Z",
                    "dns_name": "lb-0a498284629e4629a55d0415a6e89dda-1.upcloudlb.com",
                    "family": "IPv4",
                    "name": "example-network-1",
                    "type": "public"
                },
                {
                    "created_at": "2022-10-04T16:11:14.569638Z",
                    "dns_name": "lb-0a498284629e4629a55d0415a6e89dda-2.upcloudlb.com",
                    "family": "IPv4",
                    "name": "example-network-2",
                    "type": "private",
                    "uuid": "03631160-d57a-4926-ad48-a2f828229dcb"
                }
            ],
            "operational_state": "pending",
            "plan": "development",
            "resolvers": [
                {
                    "cache_invalid": 10,
                    "cache_valid": 180,
                    "created_at": "2022-10-04T16:11:14.569638Z",
                    "name": "example-resolver",
                    "nameservers": [
                        "172.16.1.4:53"
                    ],
                    "retries": 5,
                    "timeout": 30,
                    "timeout_retry": 10
                }
            ],
            "updated_at": "2022-10-04T16:36:09.933249Z",
            "uuid": "0a498284-629e-4629-a55d-0415a6e89dda",
            "zone": "fi-hel1"
        }
    ]
"#;

#[test]
fn deserialize_lb_list() {
    let lbs: Vec<LoadBalancer> = serde_json::from_str(LB_LIST_JSON).unwrap();
    assert_eq!(lbs.len(), 1);
    let lb = &lbs[0];
    assert_eq!(lb.uuid, "0a498284-629e-4629-a55d-0415a6e89dda");
    assert_eq!(lb.name, "example-service");
    assert_eq!(lb.plan, "development");
    assert_eq!(lb.zone, "fi-hel1");
    assert_eq!(lb.state, OperationalState::Pending);
    assert_eq!(lb.configured_status.as_deref(), Some("started"));
    assert_eq!(
        lb.created_at.as_deref(),
        Some("2022-10-04T16:11:14.569638Z")
    );
    assert_eq!(
        lb.updated_at.as_deref(),
        Some("2022-10-04T16:36:09.933249Z")
    );
    assert_eq!(
        lb.dns_name.as_deref(),
        Some("lb-0a498284629e4629a55d0415a6e89dda-1.upcloudlb.com")
    );
    assert!(lb.maintenance_dow.is_none());
    assert!(lb.maintenance_time.is_none());
    assert!(lb.nodes.is_none());
    assert!(lb.ip_addresses.is_none());

    // Backends
    assert_eq!(lb.backends.len(), 2);
    let be0 = &lb.backends[0];
    assert_eq!(be0.name, "example-backend-1");
    assert_eq!(
        be0.created_at.as_deref(),
        Some("2022-10-04T16:11:14.569638Z")
    );
    assert_eq!(
        be0.updated_at.as_deref(),
        Some("2022-10-04T16:11:14.569638Z")
    );
    assert!(be0.properties.is_some());
    assert_eq!(
        be0.properties.as_ref().unwrap()["health_check_type"],
        "http"
    );
    assert_eq!(be0.properties.as_ref().unwrap()["timeout_server"], 10);
    assert!(be0.resolver.is_none());
    assert!(be0.mode.is_none());
    assert!(be0.http_cookie_name.is_none());
    assert!(be0.outbound_proxy_protocol.is_none());
    assert!(be0.health_check.is_none());
    assert_eq!(be0.tls_configs.len(), 0);
    assert_eq!(be0.members.len(), 1);
    let m0 = &be0.members[0];
    assert_eq!(m0.name, "example-member-1");
    assert!(m0.enabled);
    assert_eq!(m0.ip, "172.16.1.4");
    assert_eq!(m0.port, 8000);
    assert_eq!(m0.weight, 100);
    assert_eq!(m0.max_sessions, 1000);
    assert_eq!(m0.member_type, "static");
    assert_eq!(m0.backup, Some(false));
    assert_eq!(
        m0.created_at.as_deref(),
        Some("2022-10-04T16:11:14.569638Z")
    );
    assert!(m0.updated_at.is_none());

    let be1 = &lb.backends[1];
    assert_eq!(be1.name, "example-backend-2");
    assert_eq!(be1.resolver.as_deref(), Some("example-resolver"));
    assert_eq!(
        be1.created_at.as_deref(),
        Some("2022-10-04T16:11:14.569638Z")
    );
    assert_eq!(
        be1.updated_at.as_deref(),
        Some("2022-10-04T16:11:14.569638Z")
    );
    assert_eq!(be1.tls_configs.len(), 0);
    assert!(be1.mode.is_none());
    assert!(be1.http_cookie_name.is_none());
    assert!(be1.outbound_proxy_protocol.is_none());
    assert!(be1.health_check.is_none());
    assert!(be1.properties.is_none());
    assert_eq!(be1.members.len(), 1);
    let m1 = &be1.members[0];
    assert_eq!(m1.name, "member.example.com");
    assert!(m1.enabled);
    assert_eq!(m1.ip, "");
    assert_eq!(m1.port, 8000);
    assert_eq!(m1.weight, 100);
    assert_eq!(m1.max_sessions, 1000);
    assert_eq!(m1.member_type, "dynamic");
    assert!(m1.backup.is_none());
    assert!(m1.created_at.is_none());
    assert!(m1.updated_at.is_none());

    // Frontends
    assert_eq!(lb.frontends.len(), 1);
    let fe = &lb.frontends[0];
    assert_eq!(fe.name, "example-frontend");
    assert_eq!(fe.port, 443);
    assert_eq!(fe.mode, "http");
    assert_eq!(fe.default_backend, "example-backend-1");
    assert_eq!(
        fe.created_at.as_deref(),
        Some("2022-10-04T16:11:14.569638Z")
    );
    assert_eq!(
        fe.updated_at.as_deref(),
        Some("2022-10-04T16:11:14.569638Z")
    );
    assert!(fe.properties.is_some());
    assert_eq!(fe.properties.as_ref().unwrap()["timeout_client"], 5);
    assert_eq!(fe.networks.len(), 1);
    assert_eq!(fe.networks[0].name, "example-network-1");
    assert_eq!(fe.rules.len(), 1);
    assert_eq!(fe.rules[0].name, "example-rule-1");
    assert_eq!(fe.rules[0].priority, 100);
    assert_eq!(fe.rules[0].matchers.len(), 1);
    assert_eq!(fe.rules[0].matchers[0]["type"], "path");
    assert_eq!(fe.rules[0].matchers[0]["match_path"]["method"], "exact");
    assert_eq!(fe.rules[0].matchers[0]["match_path"]["value"], "/app");
    assert_eq!(fe.rules[0].actions.len(), 1);
    assert_eq!(fe.rules[0].actions[0]["type"], "use_backend");
    assert_eq!(
        fe.rules[0].actions[0]["action_use_backend"]["backend"],
        "example-backend-2"
    );
    assert!(fe.rules[0].matching_condition.is_none());
    assert!(fe.rules[0].created_at.is_none());
    assert!(fe.rules[0].updated_at.is_none());
    assert_eq!(fe.tls_configs.len(), 1);
    assert_eq!(fe.tls_configs[0].name, "example-tls-config");
    assert_eq!(
        fe.tls_configs[0].certificate_bundle_uuid.as_deref(),
        Some("0a73c569-a2d1-481a-917f-b0fdafd78136")
    );
    assert!(fe.tls_configs[0].certificate_chain.is_none());
    assert!(fe.tls_configs[0].name_match.is_none());
    assert!(fe.tls_configs[0].created_at.is_none());
    assert!(fe.tls_configs[0].updated_at.is_none());

    // Networks
    assert_eq!(lb.networks.len(), 2);
    assert_eq!(lb.networks[0].name, "example-network-1");
    assert_eq!(lb.networks[0].network_type, "public");
    assert_eq!(lb.networks[0].family.as_deref(), Some("IPv4"));
    assert_eq!(
        lb.networks[0].dns_name.as_deref(),
        Some("lb-0a498284629e4629a55d0415a6e89dda-1.upcloudlb.com")
    );
    assert!(lb.networks[0].uuid.is_none());
    assert_eq!(lb.networks[1].name, "example-network-2");
    assert_eq!(lb.networks[1].network_type, "private");
    assert_eq!(lb.networks[1].family.as_deref(), Some("IPv4"));
    assert_eq!(
        lb.networks[1].dns_name.as_deref(),
        Some("lb-0a498284629e4629a55d0415a6e89dda-2.upcloudlb.com")
    );
    assert_eq!(
        lb.networks[1].uuid.as_deref(),
        Some("03631160-d57a-4926-ad48-a2f828229dcb")
    );

    // Resolvers
    assert_eq!(lb.resolvers.len(), 1);
    assert_eq!(lb.resolvers[0].name, "example-resolver");
    assert_eq!(lb.resolvers[0].nameservers, vec!["172.16.1.4:53"]);
    assert_eq!(lb.resolvers[0].retries, 5);
    assert_eq!(lb.resolvers[0].timeout, 30);
    assert_eq!(lb.resolvers[0].timeout_retry, 10);
    assert_eq!(lb.resolvers[0].cache_valid, 180);
    assert_eq!(lb.resolvers[0].cache_invalid, 10);
    assert_eq!(
        lb.resolvers[0].created_at.as_deref(),
        Some("2022-10-04T16:11:14.569638Z")
    );
    assert!(lb.resolvers[0].updated_at.is_none());

    // Labels
    let labels = lb.labels.as_ref().unwrap();
    assert_eq!(labels.len(), 2);
    assert_eq!(labels[0].key, "env");
    assert_eq!(labels[0].value, "staging");
    assert_eq!(labels[1].key, "foo");
    assert_eq!(labels[1].value, "bar");
}

// ─── Test 1: Plan detail ──────────────────────────────────────────────────

// Source: https://developers.upcloud.com/1.3/35-load-balancers/#create-service
const LB_PLAN_DETAIL_JSON: &str =
    r#"
    {
        "name": "development",
        "per_server_max_sessions": 10000,
        "server_number": 1
    }
"#;

#[test]
fn deserialize_lb_plan_detail() {
    let plan: Plan = serde_json::from_str(LB_PLAN_DETAIL_JSON).unwrap();
    assert_eq!(plan.name, "development");
    assert_eq!(plan.per_server_max_sessions, Some(10000));
    assert_eq!(plan.server_number, Some(1));
    assert_eq!(plan.per_month_price_eur, None);
    assert_eq!(plan.per_hour_price_eur, None);
    assert_eq!(plan.max_server_count, None);
    assert_eq!(plan.max_member_count, None);
    assert_eq!(plan.max_backend_count, None);
    assert_eq!(plan.max_frontend_count, None);
}

// Source: https://developers.upcloud.com/1.3/35-load-balancers/#create-service
const CREATE_LB_REQUEST_JSON: &str = r#"
    {
        "name": "example-service",
        "plan": "development",
        "zone": "fi-hel1",
        "configured_status": "started",
        "networks": [
            {
                "name": "example-network-1",
                "type": "public",
                "family": "IPv4"
            },
            {
                "name": "example-network-2",
                "type": "private",
                "family": "IPv4",
                "uuid": "03631160-d57a-4926-ad48-a2f828229dcb"
            }
        ]
    }
"#;

// ─── Test 2: Create LB request ────────────────────────────────────────────

#[test]
fn deserialize_create_lb_request() {
    let val: serde_json::Value = serde_json::from_str(CREATE_LB_REQUEST_JSON).unwrap();
    assert_eq!(val["name"], "example-service");
    assert_eq!(val["plan"], "development");
    assert_eq!(val["zone"], "fi-hel1");
    assert_eq!(val["configured_status"], "started");
    assert_eq!(val["networks"].as_array().unwrap().len(), 2);
    assert_eq!(val["networks"][0]["name"], "example-network-1");
    assert_eq!(val["networks"][0]["type"], "public");
    assert_eq!(val["networks"][0]["family"], "IPv4");
    assert!(val["networks"][0].get("uuid").is_none());
    assert_eq!(val["networks"][1]["name"], "example-network-2");
    assert_eq!(val["networks"][1]["type"], "private");
    assert_eq!(val["networks"][1]["family"], "IPv4");
    assert_eq!(
        val["networks"][1]["uuid"],
        "03631160-d57a-4926-ad48-a2f828229dcb"
    );

    // Also parse as CreateRequest
    let req: CreateRequest = serde_json::from_str(CREATE_LB_REQUEST_JSON).unwrap();
    assert_eq!(req.name, "example-service");
    assert_eq!(req.plan, "development");
    assert_eq!(req.zone, "fi-hel1");
    assert_eq!(req.configured_status.as_deref(), Some("started"));
    assert!(req.labels.is_none());
    assert_eq!(req.networks.len(), 2);
    assert_eq!(req.networks[0].name, "example-network-1");
    assert_eq!(req.networks[0].network_type, "public");
    assert_eq!(req.networks[0].family.as_deref(), Some("IPv4"));
    assert!(req.networks[0].uuid.is_none());
    assert_eq!(req.networks[1].name, "example-network-2");
    assert_eq!(req.networks[1].network_type, "private");
    assert_eq!(req.networks[1].family.as_deref(), Some("IPv4"));
    assert_eq!(
        req.networks[1].uuid.as_deref(),
        Some("03631160-d57a-4926-ad48-a2f828229dcb")
    );
}

// Source: https://developers.upcloud.com/1.3/35-load-balancers/#get-service-details
const LB_SERVICE_DETAIL_JSON: &str = r#"
    {
        "backends": [
            {
                "created_at": "2022-10-04T16:11:14.569638Z",
                "members": [
                    {
                        "backup": false,
                        "created_at": "2022-10-04T16:11:14.569638Z",
                        "enabled": true,
                        "ip": "172.16.1.4",
                        "max_sessions": 1000,
                        "name": "example-member-1",
                        "port": 8000,
                        "type": "static",
                        "updated_at": "2022-10-04T16:11:14.569638Z",
                        "weight": 100
                    }
                ],
                "name": "example-backend-1",
                "properties": {
                    "health_check_type": "http",
                    "timeout_server": 10
                },
                "updated_at": "2022-10-04T16:11:14.569638Z"
            }
        ],
        "configured_status": "started",
        "created_at": "2022-10-04T16:11:14.569638Z",
        "dns_name": "lb-0a498284629e4629a55d0415a6e89dda-1.upcloudlb.com",
        "frontends": [
            {
                "created_at": "2022-10-04T16:11:14.569638Z",
                "default_backend": "example-backend-1",
                "mode": "http",
                "name": "example-frontend",
                "networks": [
                    {
                        "name": "example-network-1"
                    }
                ],
                "port": 443,
                "properties": {
                    "timeout_client": 5
                },
                "rules": [
                    {
                        "actions": [
                            {
                                "type": "use_backend",
                                "action_use_backend": {
                                    "backend": "example-backend-2"
                                }
                            }
                        ],
                        "created_at": "2022-10-04T16:11:14.569638Z",
                        "matchers": [
                            {
                                "type": "path",
                                "inverse": false,
                                "match_path": {
                                    "method": "exact",
                                    "value": "/app"
                                }
                            }
                        ],
                        "name": "example-rule-1",
                        "priority": 100,
                        "updated_at": "2022-10-04T16:11:14.569638Z"
                    }
                ],
                "tls_configs": [
                    {
                        "certificate_bundle_uuid": "0a73c569-a2d1-481a-917f-b0fdafd78136",
                        "created_at": "2022-10-04T16:11:14.569638Z",
                        "name": "example-tls-config",
                        "updated_at": "2022-10-04T16:11:14.569638Z"
                    }
                ],
                "updated_at": "2022-10-04T16:11:14.569638Z"
            }
        ],
        "labels": [
            {
                "key": "env",
                "value": "staging"
            },
            {
                "key": "foo",
                "value": "bar"
            }
        ],
        "name": "example-service",
        "networks": [
            {
                "created_at": "2022-10-04T16:11:14.569638Z",
                "dns_name": "lb-1.upcloudlb.com",
                "family": "IPv4",
                "name": "example-network-1",
                "type": "public"
            },
            {
                "created_at": "2022-10-04T16:11:14.569638Z",
                "dns_name": "lb-2.upcloudlb.com",
                "family": "IPv4",
                "name": "example-network-2",
                "type": "private",
                "uuid": "03631160-d57a-4926-ad48-a2f828229dcb"
            }
        ],
        "operational_state": "pending",
        "plan": "development",
        "resolvers": [
            {
                "cache_invalid": 10,
                "cache_valid": 180,
                "created_at": "2022-10-04T16:11:14.569638Z",
                "name": "example-resolver",
                "nameservers": [
                    "172.16.1.4:53"
                ],
                "retries": 5,
                "timeout": 30,
                "timeout_retry": 10,
                "updated_at": "2022-10-04T16:11:14.569638Z"
            }
        ],
        "updated_at": "2022-10-04T16:36:09.933249Z",
        "uuid": "0a498284-629e-4629-a55d-0415a6e89dda",
        "zone": "fi-hel1"
    }
"#;

// ─── Test 3: LB service details ───────────────────────────────────────────

#[test]
fn deserialize_lb_service_details() {
    let lb: LoadBalancer = serde_json::from_str(LB_SERVICE_DETAIL_JSON).unwrap();
    assert_eq!(lb.uuid, "0a498284-629e-4629-a55d-0415a6e89dda");
    assert_eq!(lb.name, "example-service");
    assert_eq!(lb.plan, "development");
    assert_eq!(lb.zone, "fi-hel1");
    assert_eq!(lb.state, OperationalState::Pending);
    assert_eq!(lb.configured_status.as_deref(), Some("started"));
    assert_eq!(
        lb.dns_name.as_deref(),
        Some("lb-0a498284629e4629a55d0415a6e89dda-1.upcloudlb.com")
    );
    assert_eq!(
        lb.created_at.as_deref(),
        Some("2022-10-04T16:11:14.569638Z")
    );
    assert_eq!(
        lb.updated_at.as_deref(),
        Some("2022-10-04T16:36:09.933249Z")
    );
    assert!(lb.maintenance_dow.is_none());
    assert!(lb.maintenance_time.is_none());
    assert!(lb.nodes.is_none());
    assert!(lb.ip_addresses.is_none());

    // Backends
    assert_eq!(lb.backends.len(), 1);
    let be = &lb.backends[0];
    assert_eq!(be.name, "example-backend-1");
    assert_eq!(
        be.created_at.as_deref(),
        Some("2022-10-04T16:11:14.569638Z")
    );
    assert_eq!(
        be.updated_at.as_deref(),
        Some("2022-10-04T16:11:14.569638Z")
    );
    assert!(be.properties.is_some());
    assert_eq!(be.properties.as_ref().unwrap()["health_check_type"], "http");
    assert_eq!(be.properties.as_ref().unwrap()["timeout_server"], 10);
    assert!(be.resolver.is_none());
    assert!(be.mode.is_none());
    assert!(be.http_cookie_name.is_none());
    assert!(be.outbound_proxy_protocol.is_none());
    assert!(be.health_check.is_none());
    assert_eq!(be.tls_configs.len(), 0);
    assert_eq!(be.members.len(), 1);
    let m = &be.members[0];
    assert_eq!(m.name, "example-member-1");
    assert!(m.enabled);
    assert_eq!(m.ip, "172.16.1.4");
    assert_eq!(m.port, 8000);
    assert_eq!(m.weight, 100);
    assert_eq!(m.max_sessions, 1000);
    assert_eq!(m.member_type, "static");
    assert_eq!(m.backup, Some(false));
    assert_eq!(m.created_at.as_deref(), Some("2022-10-04T16:11:14.569638Z"));
    assert_eq!(m.updated_at.as_deref(), Some("2022-10-04T16:11:14.569638Z"));

    // Frontends
    assert_eq!(lb.frontends.len(), 1);
    let fe = &lb.frontends[0];
    assert_eq!(fe.name, "example-frontend");
    assert_eq!(fe.port, 443);
    assert_eq!(fe.mode, "http");
    assert_eq!(fe.default_backend, "example-backend-1");
    assert_eq!(
        fe.created_at.as_deref(),
        Some("2022-10-04T16:11:14.569638Z")
    );
    assert_eq!(
        fe.updated_at.as_deref(),
        Some("2022-10-04T16:11:14.569638Z")
    );
    assert!(fe.properties.is_some());
    assert_eq!(fe.properties.as_ref().unwrap()["timeout_client"], 5);
    assert_eq!(fe.networks.len(), 1);
    assert_eq!(fe.networks[0].name, "example-network-1");

    // Frontend rules
    assert_eq!(fe.rules.len(), 1);
    assert_eq!(fe.rules[0].name, "example-rule-1");
    assert_eq!(fe.rules[0].priority, 100);
    assert_eq!(
        fe.rules[0].created_at.as_deref(),
        Some("2022-10-04T16:11:14.569638Z")
    );
    assert_eq!(
        fe.rules[0].updated_at.as_deref(),
        Some("2022-10-04T16:11:14.569638Z")
    );
    assert!(fe.rules[0].matching_condition.is_none());
    assert_eq!(fe.rules[0].matchers.len(), 1);
    assert_eq!(fe.rules[0].matchers[0]["type"], "path");
    assert_eq!(fe.rules[0].matchers[0]["inverse"], false);
    assert_eq!(fe.rules[0].matchers[0]["match_path"]["method"], "exact");
    assert_eq!(fe.rules[0].matchers[0]["match_path"]["value"], "/app");
    assert_eq!(fe.rules[0].actions.len(), 1);
    assert_eq!(fe.rules[0].actions[0]["type"], "use_backend");
    assert_eq!(
        fe.rules[0].actions[0]["action_use_backend"]["backend"],
        "example-backend-2"
    );

    // Frontend TLS configs
    assert_eq!(fe.tls_configs.len(), 1);
    assert_eq!(fe.tls_configs[0].name, "example-tls-config");
    assert_eq!(
        fe.tls_configs[0].certificate_bundle_uuid.as_deref(),
        Some("0a73c569-a2d1-481a-917f-b0fdafd78136")
    );
    assert_eq!(
        fe.tls_configs[0].created_at.as_deref(),
        Some("2022-10-04T16:11:14.569638Z")
    );
    assert_eq!(
        fe.tls_configs[0].updated_at.as_deref(),
        Some("2022-10-04T16:11:14.569638Z")
    );
    assert!(fe.tls_configs[0].certificate_chain.is_none());
    assert!(fe.tls_configs[0].name_match.is_none());

    // Networks
    assert_eq!(lb.networks.len(), 2);
    assert_eq!(lb.networks[0].name, "example-network-1");
    assert_eq!(lb.networks[0].network_type, "public");
    assert_eq!(lb.networks[0].family.as_deref(), Some("IPv4"));
    assert_eq!(
        lb.networks[0].dns_name.as_deref(),
        Some("lb-1.upcloudlb.com")
    );
    assert!(lb.networks[0].uuid.is_none());
    assert_eq!(lb.networks[1].name, "example-network-2");
    assert_eq!(lb.networks[1].network_type, "private");
    assert_eq!(lb.networks[1].family.as_deref(), Some("IPv4"));
    assert_eq!(
        lb.networks[1].dns_name.as_deref(),
        Some("lb-2.upcloudlb.com")
    );
    assert_eq!(
        lb.networks[1].uuid.as_deref(),
        Some("03631160-d57a-4926-ad48-a2f828229dcb")
    );

    // Resolvers
    assert_eq!(lb.resolvers.len(), 1);
    let r = &lb.resolvers[0];
    assert_eq!(r.name, "example-resolver");
    assert_eq!(r.retries, 5);
    assert_eq!(r.timeout, 30);
    assert_eq!(r.timeout_retry, 10);
    assert_eq!(r.cache_valid, 180);
    assert_eq!(r.cache_invalid, 10);
    assert_eq!(r.nameservers, vec!["172.16.1.4:53"]);
    assert_eq!(r.created_at.as_deref(), Some("2022-10-04T16:11:14.569638Z"));
    assert_eq!(r.updated_at.as_deref(), Some("2022-10-04T16:11:14.569638Z"));

    // Labels
    let labels = lb.labels.as_ref().unwrap();
    assert_eq!(labels.len(), 2);
    assert_eq!(labels[0].key, "env");
    assert_eq!(labels[0].value, "staging");
    assert_eq!(labels[1].key, "foo");
    assert_eq!(labels[1].value, "bar");
}

// ─── Test 4: Modify LB request ────────────────────────────────────────────

// Source: https://developers.upcloud.com/1.3/35-load-balancers/#modify-service
const MODIFY_LB_REQUEST_JSON: &str =
    r#"
    {
        "name": "example-service",
        "plan": "development",
        "configured_status": "started"
    }
"#;

#[test]
fn deserialize_modify_lb_request() {
    let val: serde_json::Value = serde_json::from_str(MODIFY_LB_REQUEST_JSON).unwrap();
    assert_eq!(val["name"], "example-service");
    assert_eq!(val["plan"], "development");
    assert_eq!(val["configured_status"], "started");

    let req: ModifyRequest = serde_json::from_str(MODIFY_LB_REQUEST_JSON).unwrap();
    assert_eq!(req.name.as_deref(), Some("example-service"));
    assert_eq!(req.plan.as_deref(), Some("development"));
    assert_eq!(req.configured_status.as_deref(), Some("started"));
    assert!(req.labels.is_none());
}

// Source: https://developers.upcloud.com/1.3/35-load-balancers/#list-backends
const LB_BACKEND_LIST_JSON: &str = r#"
    [
        {
            "created_at": "2021-12-07T13:58:30.817272Z",
            "members": [
                {
                    "backup": false,
                    "created_at": "2022-02-11T16:39:55.321306Z",
                    "enabled": true,
                    "ip": "",
                    "max_sessions": 1000,
                    "name": "member.example.com",
                    "port": 8000,
                    "type": "dynamic",
                    "updated_at": "2022-02-11T17:33:08.490581Z",
                    "weight": 100
                }
            ],
            "name": "example-backend-2",
            "properties": {
                "health_check_type": "http",
                "timeout_server": 10
            },
            "resolver": "dns",
            "tls_configs": [],
            "updated_at": "2022-02-11T17:33:08.490581Z"
        }
    ]
"#;

// ─── Test 5: List backends ────────────────────────────────────────────────

#[test]
fn deserialize_lb_backend_list() {
    let backends: Vec<Backend> = serde_json::from_str(LB_BACKEND_LIST_JSON).unwrap();
    assert_eq!(backends.len(), 1);
    let be = &backends[0];
    assert_eq!(be.name, "example-backend-2");
    assert_eq!(be.resolver.as_deref(), Some("dns"));
    assert_eq!(
        be.created_at.as_deref(),
        Some("2021-12-07T13:58:30.817272Z")
    );
    assert_eq!(
        be.updated_at.as_deref(),
        Some("2022-02-11T17:33:08.490581Z")
    );
    assert!(be.properties.is_some());
    assert_eq!(be.properties.as_ref().unwrap()["health_check_type"], "http");
    assert_eq!(be.properties.as_ref().unwrap()["timeout_server"], 10);
    assert!(be.mode.is_none());
    assert!(be.http_cookie_name.is_none());
    assert!(be.outbound_proxy_protocol.is_none());
    assert!(be.health_check.is_none());
    assert_eq!(be.tls_configs.len(), 0);
    assert_eq!(be.members.len(), 1);
    assert_eq!(be.members[0].name, "member.example.com");
    assert_eq!(be.members[0].member_type, "dynamic");
    assert_eq!(be.members[0].ip, "");
    assert!(be.members[0].enabled);
    assert_eq!(be.members[0].port, 8000);
    assert_eq!(be.members[0].weight, 100);
    assert_eq!(be.members[0].max_sessions, 1000);
    assert_eq!(be.members[0].backup, Some(false));
    assert_eq!(
        be.members[0].created_at.as_deref(),
        Some("2022-02-11T16:39:55.321306Z")
    );
    assert_eq!(
        be.members[0].updated_at.as_deref(),
        Some("2022-02-11T17:33:08.490581Z")
    );
}

// Source: https://developers.upcloud.com/1.3/35-load-balancers/#get-backend-details
const LB_BACKEND_DETAIL_JSON: &str = r#"
    {
        "created_at": "2021-12-07T13:58:30.817272Z",
        "members": [
            {
                "backup": false,
                "created_at": "2021-12-07T13:58:30.817272Z",
                "enabled": true,
                "ip": "172.16.1.4",
                "max_sessions": 1000,
                "name": "example-member-1",
                "port": 8000,
                "type": "static",
                "updated_at": "2022-02-11T17:33:08.490581Z",
                "weight": 100
            }
        ],
        "name": "example-backend-1",
        "properties": {
            "health_check_type": "http",
            "health_check_url": "/health",
            "timeout_server": 10
        },
        "resolver": "dns",
        "tls_configs": [
            {
                "certificate_bundle_uuid": "0a98ddbf-29e3-408c-b081-278a67101f76",
                "created_at": "2023-07-28T08:01:11.729382Z",
                "name": "ca",
                "updated_at": "2023-07-28T08:26:30.436412Z"
            }
        ],
        "updated_at": "2022-02-11T17:33:08.490581Z"
    }
"#;

// ─── Test 6: Backend detail ───────────────────────────────────────────────

#[test]
fn deserialize_lb_backend_detail() {
    let be: Backend = serde_json::from_str(LB_BACKEND_DETAIL_JSON).unwrap();
    assert_eq!(be.name, "example-backend-1");
    assert_eq!(be.resolver.as_deref(), Some("dns"));
    assert_eq!(
        be.created_at.as_deref(),
        Some("2021-12-07T13:58:30.817272Z")
    );
    assert_eq!(
        be.updated_at.as_deref(),
        Some("2022-02-11T17:33:08.490581Z")
    );
    assert!(be.mode.is_none());
    assert!(be.http_cookie_name.is_none());
    assert!(be.outbound_proxy_protocol.is_none());
    assert!(be.health_check.is_none());

    // Members
    assert_eq!(be.members.len(), 1);
    let m = &be.members[0];
    assert_eq!(m.name, "example-member-1");
    assert_eq!(m.ip, "172.16.1.4");
    assert_eq!(m.member_type, "static");
    assert!(m.enabled);
    assert_eq!(m.port, 8000);
    assert_eq!(m.weight, 100);
    assert_eq!(m.max_sessions, 1000);
    assert_eq!(m.backup, Some(false));
    assert_eq!(m.created_at.as_deref(), Some("2021-12-07T13:58:30.817272Z"));
    assert_eq!(m.updated_at.as_deref(), Some("2022-02-11T17:33:08.490581Z"));

    // TLS configs
    assert_eq!(be.tls_configs.len(), 1);
    assert_eq!(be.tls_configs[0].name, "ca");
    assert_eq!(
        be.tls_configs[0].certificate_bundle_uuid.as_deref(),
        Some("0a98ddbf-29e3-408c-b081-278a67101f76")
    );
    assert_eq!(
        be.tls_configs[0].created_at.as_deref(),
        Some("2023-07-28T08:01:11.729382Z")
    );
    assert_eq!(
        be.tls_configs[0].updated_at.as_deref(),
        Some("2023-07-28T08:26:30.436412Z")
    );
    assert!(be.tls_configs[0].certificate_chain.is_none());
    assert!(be.tls_configs[0].name_match.is_none());

    // Properties
    let props = be.properties.as_ref().unwrap();
    assert_eq!(props["health_check_type"], "http");
    assert_eq!(props["health_check_url"], "/health");
    assert_eq!(props["timeout_server"], 10);
}

// Source: https://developers.upcloud.com/1.3/35-load-balancers/#list-members
const LB_MEMBER_LIST_JSON: &str = r#"
    [
        {
            "backup": false,
            "created_at": "2022-02-11T16:39:55.321306Z",
            "enabled": true,
            "ip": "",
            "max_sessions": 1000,
            "name": "member.example.com",
            "port": 8000,
            "type": "dynamic",
            "updated_at": "2022-02-11T17:33:08.490581Z",
            "weight": 100
        },
        {
            "backup": false,
            "created_at": "2021-12-07T13:58:30.817272Z",
            "enabled": true,
            "ip": "172.16.1.4",
            "max_sessions": 1000,
            "name": "example-member-1",
            "port": 8000,
            "type": "static",
            "updated_at": "2022-02-11T17:33:08.490581Z",
            "weight": 100
        }
    ]
"#;

// ─── Test 7: List backend members ─────────────────────────────────────────

#[test]
fn deserialize_lb_member_list() {
    let members: Vec<BackendMember> = serde_json::from_str(LB_MEMBER_LIST_JSON).unwrap();
    assert_eq!(members.len(), 2);
    assert_eq!(members[0].name, "member.example.com");
    assert_eq!(members[0].member_type, "dynamic");
    assert_eq!(members[0].ip, "");
    assert!(members[0].enabled);
    assert_eq!(members[0].port, 8000);
    assert_eq!(members[0].weight, 100);
    assert_eq!(members[0].max_sessions, 1000);
    assert_eq!(members[0].backup, Some(false));
    assert_eq!(
        members[0].created_at.as_deref(),
        Some("2022-02-11T16:39:55.321306Z")
    );
    assert_eq!(
        members[0].updated_at.as_deref(),
        Some("2022-02-11T17:33:08.490581Z")
    );
    assert_eq!(members[1].name, "example-member-1");
    assert_eq!(members[1].member_type, "static");
    assert_eq!(members[1].ip, "172.16.1.4");
    assert!(members[1].enabled);
    assert_eq!(members[1].port, 8000);
    assert_eq!(members[1].weight, 100);
    assert_eq!(members[1].max_sessions, 1000);
    assert_eq!(members[1].backup, Some(false));
    assert_eq!(
        members[1].created_at.as_deref(),
        Some("2021-12-07T13:58:30.817272Z")
    );
    assert_eq!(
        members[1].updated_at.as_deref(),
        Some("2022-02-11T17:33:08.490581Z")
    );
}

// Source: https://developers.upcloud.com/1.3/35-load-balancers/#get-member-details
const LB_MEMBER_DETAIL_JSON: &str = r#"
    {
        "backup": false,
        "created_at": "2021-12-07T13:58:30.817272Z",
        "enabled": true,
        "ip": "172.16.1.4",
        "max_sessions": 1000,
        "name": "example-member-1",
        "port": 8000,
        "type": "static",
        "updated_at": "2022-02-11T17:33:08.490581Z",
        "weight": 100
    }
"#;

// ─── Test 8: Member detail ────────────────────────────────────────────────

#[test]
fn deserialize_lb_member_detail() {
    let m: BackendMember = serde_json::from_str(LB_MEMBER_DETAIL_JSON).unwrap();
    assert_eq!(m.name, "example-member-1");
    assert!(m.enabled);
    assert_eq!(m.ip, "172.16.1.4");
    assert_eq!(m.port, 8000);
    assert_eq!(m.weight, 100);
    assert_eq!(m.max_sessions, 1000);
    assert_eq!(m.member_type, "static");
    assert_eq!(m.backup, Some(false));
    assert_eq!(m.created_at.as_deref(), Some("2021-12-07T13:58:30.817272Z"));
    assert_eq!(m.updated_at.as_deref(), Some("2022-02-11T17:33:08.490581Z"));
}

// Source: https://developers.upcloud.com/1.3/35-load-balancers/#list-frontends
const LB_FRONTEND_LIST_JSON: &str = r#"
    [
        {
            "created_at": "2021-12-07T13:58:30.817272Z",
            "default_backend": "example-backend-1",
            "mode": "http",
            "name": "example-frontend",
            "networks": [
                {
                    "name": "example-network-1"
                }
            ],
            "port": 443,
            "properties": {
                "timeout_client": 10
            },
            "rules": [
                {
                    "actions": [
                        {
                            "type": "use_backend",
                            "action_use_backend": {
                                "backend": "example-backend-2"
                            }
                        }
                    ],
                    "created_at": "2021-12-07T13:58:30.817272Z",
                    "matching_condition": "and",
                    "matchers": [
                        {
                            "type": "path",
                            "match_path": {
                                "method": "exact",
                                "value": "/app"
                            }
                        }
                    ],
                    "name": "example-rule-1",
                    "priority": 100,
                    "updated_at": "2022-02-11T17:33:08.490581Z"
                }
            ],
            "tls_configs": [
                {
                    "certificate_bundle_uuid": "0aded5c1-c7a3-498a-b9c8-a871611c47a2",
                    "created_at": "2022-02-11T17:33:08.490581Z",
                    "name": "example-tls-config",
                    "updated_at": "2022-02-11T17:33:08.490581Z"
                }
            ],
            "updated_at": "2022-02-11T17:33:08.490581Z"
        }
    ]
"#;

// ─── Test 9: List frontends ───────────────────────────────────────────────

#[test]
fn deserialize_lb_frontend_list() {
    let frontends: Vec<Frontend> = serde_json::from_str(LB_FRONTEND_LIST_JSON).unwrap();
    assert_eq!(frontends.len(), 1);
    let fe = &frontends[0];
    assert_eq!(fe.name, "example-frontend");
    assert_eq!(fe.port, 443);
    assert_eq!(fe.mode, "http");
    assert_eq!(fe.default_backend, "example-backend-1");
    assert_eq!(
        fe.created_at.as_deref(),
        Some("2021-12-07T13:58:30.817272Z")
    );
    assert_eq!(
        fe.updated_at.as_deref(),
        Some("2022-02-11T17:33:08.490581Z")
    );
    assert_eq!(fe.networks.len(), 1);
    assert_eq!(fe.networks[0].name, "example-network-1");
    assert!(fe.properties.is_some());
    assert_eq!(fe.properties.as_ref().unwrap()["timeout_client"], 10);

    // Rules
    assert_eq!(fe.rules.len(), 1);
    assert_eq!(fe.rules[0].name, "example-rule-1");
    assert_eq!(fe.rules[0].priority, 100);
    assert_eq!(fe.rules[0].matching_condition.as_deref(), Some("and"));
    assert_eq!(
        fe.rules[0].created_at.as_deref(),
        Some("2021-12-07T13:58:30.817272Z")
    );
    assert_eq!(
        fe.rules[0].updated_at.as_deref(),
        Some("2022-02-11T17:33:08.490581Z")
    );
    assert_eq!(fe.rules[0].matchers.len(), 1);
    assert_eq!(fe.rules[0].matchers[0]["type"], "path");
    assert_eq!(fe.rules[0].matchers[0]["match_path"]["method"], "exact");
    assert_eq!(fe.rules[0].matchers[0]["match_path"]["value"], "/app");
    assert_eq!(fe.rules[0].actions.len(), 1);
    assert_eq!(fe.rules[0].actions[0]["type"], "use_backend");
    assert_eq!(
        fe.rules[0].actions[0]["action_use_backend"]["backend"],
        "example-backend-2"
    );

    // TLS configs
    assert_eq!(fe.tls_configs.len(), 1);
    assert_eq!(fe.tls_configs[0].name, "example-tls-config");
    assert_eq!(
        fe.tls_configs[0].certificate_bundle_uuid.as_deref(),
        Some("0aded5c1-c7a3-498a-b9c8-a871611c47a2")
    );
    assert_eq!(
        fe.tls_configs[0].created_at.as_deref(),
        Some("2022-02-11T17:33:08.490581Z")
    );
    assert_eq!(
        fe.tls_configs[0].updated_at.as_deref(),
        Some("2022-02-11T17:33:08.490581Z")
    );
    assert!(fe.tls_configs[0].certificate_chain.is_none());
    assert!(fe.tls_configs[0].name_match.is_none());
}

// Source: https://developers.upcloud.com/1.3/35-load-balancers/#get-frontend-details
const LB_FRONTEND_DETAIL_JSON: &str = r#"
    {
        "created_at": "2021-12-07T13:58:30.817272Z",
        "default_backend": "example-backend-1",
        "mode": "http",
        "name": "example-frontend",
        "networks": [
            {
                "name": "example-network-1"
            }
        ],
        "port": 443,
        "properties": {
            "timeout_client": 10
        },
        "rules": [
            {
                "actions": [
                    {
                        "type": "use_backend",
                        "action_use_backend": {
                            "backend": "example-backend-2"
                        }
                    }
                ],
                "created_at": "2021-12-07T13:58:30.817272Z",
                "matching_condition": "and",
                "matchers": [
                    {
                        "type": "path",
                        "match_path": {
                            "method": "exact",
                            "value": "/app"
                        }
                    }
                ],
                "name": "example-rule-1",
                "priority": 100,
                "updated_at": "2022-02-11T17:33:08.490581Z"
            }
        ],
        "tls_configs": [
            {
                "certificate_bundle_uuid": "0aded5c1-c7a3-498a-b9c8-a871611c47a2",
                "created_at": "2022-02-11T17:33:08.490581Z",
                "name": "example-tls-config",
                "updated_at": "2022-02-11T17:33:08.490581Z"
            }
        ],
        "updated_at": "2022-02-11T17:33:08.490581Z"
    }
"#;

// ─── Test 10: Frontend detail ─────────────────────────────────────────────

#[test]
fn deserialize_lb_frontend_detail() {
    let fe: Frontend = serde_json::from_str(LB_FRONTEND_DETAIL_JSON).unwrap();
    assert_eq!(fe.name, "example-frontend");
    assert_eq!(fe.port, 443);
    assert_eq!(fe.mode, "http");
    assert_eq!(fe.default_backend, "example-backend-1");
    assert_eq!(
        fe.created_at.as_deref(),
        Some("2021-12-07T13:58:30.817272Z")
    );
    assert_eq!(
        fe.updated_at.as_deref(),
        Some("2022-02-11T17:33:08.490581Z")
    );
    assert_eq!(fe.networks.len(), 1);
    assert_eq!(fe.networks[0].name, "example-network-1");
    assert!(fe.properties.is_some());
    assert_eq!(fe.properties.as_ref().unwrap()["timeout_client"], 10);
    assert_eq!(fe.rules.len(), 1);
    assert_eq!(fe.rules[0].name, "example-rule-1");
    assert_eq!(fe.rules[0].priority, 100);
    assert_eq!(fe.rules[0].matching_condition.as_deref(), Some("and"));
    assert_eq!(
        fe.rules[0].created_at.as_deref(),
        Some("2021-12-07T13:58:30.817272Z")
    );
    assert_eq!(
        fe.rules[0].updated_at.as_deref(),
        Some("2022-02-11T17:33:08.490581Z")
    );
    assert_eq!(fe.rules[0].matchers.len(), 1);
    assert_eq!(fe.rules[0].matchers[0]["type"], "path");
    assert_eq!(fe.rules[0].matchers[0]["match_path"]["method"], "exact");
    assert_eq!(fe.rules[0].matchers[0]["match_path"]["value"], "/app");
    assert_eq!(fe.rules[0].actions.len(), 1);
    assert_eq!(fe.rules[0].actions[0]["type"], "use_backend");
    assert_eq!(
        fe.rules[0].actions[0]["action_use_backend"]["backend"],
        "example-backend-2"
    );
    assert_eq!(fe.tls_configs.len(), 1);
    assert_eq!(fe.tls_configs[0].name, "example-tls-config");
    assert_eq!(
        fe.tls_configs[0].certificate_bundle_uuid.as_deref(),
        Some("0aded5c1-c7a3-498a-b9c8-a871611c47a2")
    );
    assert_eq!(
        fe.tls_configs[0].created_at.as_deref(),
        Some("2022-02-11T17:33:08.490581Z")
    );
    assert_eq!(
        fe.tls_configs[0].updated_at.as_deref(),
        Some("2022-02-11T17:33:08.490581Z")
    );
    assert!(fe.tls_configs[0].certificate_chain.is_none());
    assert!(fe.tls_configs[0].name_match.is_none());
}

// Source: https://developers.upcloud.com/1.3/35-load-balancers/#list-frontend-tls-configs
const LB_FRONTEND_TLS_CONFIG_LIST_JSON: &str = r#"
    [
        {
            "certificate_bundle_uuid": "0aded5c1-c7a3-498a-b9c8-a871611c47a2",
            "created_at": "2022-02-11T17:33:08.490581Z",
            "name": "example-tls-config-1",
            "updated_at": "2022-02-11T17:33:08.490581Z"
        },
        {
            "certificate_bundle_uuid": "c136c429-d18f-4192-b55e-e7b5079dee40",
            "created_at": "2021-11-11T11:21:51.121789Z",
            "name": "example-tls-config-2",
            "updated_at": "2021-11-11T11:21:51.121789Z"
        }
    ]
"#;

// ─── Test 11: Frontend TLS config list ────────────────────────────────────

#[test]
fn deserialize_lb_frontend_tls_config_list() {
    let configs: Vec<TLSConfig> = serde_json::from_str(LB_FRONTEND_TLS_CONFIG_LIST_JSON).unwrap();
    assert_eq!(configs.len(), 2);
    assert_eq!(configs[0].name, "example-tls-config-1");
    assert_eq!(
        configs[0].certificate_bundle_uuid.as_deref(),
        Some("0aded5c1-c7a3-498a-b9c8-a871611c47a2")
    );
    assert_eq!(
        configs[0].created_at.as_deref(),
        Some("2022-02-11T17:33:08.490581Z")
    );
    assert_eq!(
        configs[0].updated_at.as_deref(),
        Some("2022-02-11T17:33:08.490581Z")
    );
    assert!(configs[0].certificate_chain.is_none());
    assert!(configs[0].name_match.is_none());
    assert_eq!(configs[1].name, "example-tls-config-2");
    assert_eq!(
        configs[1].certificate_bundle_uuid.as_deref(),
        Some("c136c429-d18f-4192-b55e-e7b5079dee40")
    );
    assert_eq!(
        configs[1].created_at.as_deref(),
        Some("2021-11-11T11:21:51.121789Z")
    );
    assert_eq!(
        configs[1].updated_at.as_deref(),
        Some("2021-11-11T11:21:51.121789Z")
    );
    assert!(configs[1].certificate_chain.is_none());
    assert!(configs[1].name_match.is_none());
}

// Source: https://developers.upcloud.com/1.3/35-load-balancers/#get-frontend-tls-config-details
const LB_FRONTEND_TLS_CONFIG_DETAIL_JSON: &str = r#"
    {
        "certificate_bundle_uuid": "0aded5c1-c7a3-498a-b9c8-a871611c47a2",
        "created_at": "2022-02-11T17:33:08.490581Z",
        "name": "example-tls-config",
        "updated_at": "2022-02-11T17:33:08.490581Z"
    }
"#;

// ─── Test 12: Frontend TLS config detail ──────────────────────────────────

#[test]
fn deserialize_lb_frontend_tls_config_detail() {
    let config: TLSConfig = serde_json::from_str(LB_FRONTEND_TLS_CONFIG_DETAIL_JSON).unwrap();
    assert_eq!(config.name, "example-tls-config");
    assert_eq!(
        config.certificate_bundle_uuid.as_deref(),
        Some("0aded5c1-c7a3-498a-b9c8-a871611c47a2")
    );
    assert_eq!(
        config.created_at.as_deref(),
        Some("2022-02-11T17:33:08.490581Z")
    );
    assert_eq!(
        config.updated_at.as_deref(),
        Some("2022-02-11T17:33:08.490581Z")
    );
    assert_eq!(config.certificate_chain, None);
    assert_eq!(config.name_match, None);
}

// Source: https://developers.upcloud.com/1.3/35-load-balancers/#list-rules
const LB_RULE_LIST_JSON: &str = r#"
    [
        {
            "actions": [
                {
                    "type": "use_backend",
                    "action_use_backend": {
                        "backend": "example-backend-2"
                    }
                }
            ],
            "created_at": "2021-12-07T13:58:30.817272Z",
            "matching_condition": "and",
            "matchers": [
                {
                    "type": "path",
                    "match_path": {
                        "method": "exact",
                        "value": "/app"
                    }
                }
            ],
            "name": "example-rule-1",
            "priority": 100,
            "updated_at": "2022-02-11T17:33:08.490581Z"
        }
    ]
"#;

// ─── Test 13: Frontend rule list ──────────────────────────────────────────

#[test]
fn deserialize_lb_rule_list() {
    let rules: Vec<FrontendRule> = serde_json::from_str(LB_RULE_LIST_JSON).unwrap();
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].name, "example-rule-1");
    assert_eq!(rules[0].priority, 100);
    assert_eq!(rules[0].matching_condition.as_deref(), Some("and"));
    assert_eq!(
        rules[0].created_at.as_deref(),
        Some("2021-12-07T13:58:30.817272Z")
    );
    assert_eq!(
        rules[0].updated_at.as_deref(),
        Some("2022-02-11T17:33:08.490581Z")
    );
    assert_eq!(rules[0].matchers.len(), 1);
    assert_eq!(rules[0].actions.len(), 1);
    assert_eq!(rules[0].actions[0]["type"], "use_backend");
    assert_eq!(
        rules[0].actions[0]["action_use_backend"]["backend"],
        "example-backend-2"
    );
    assert_eq!(rules[0].matchers[0]["type"], "path");
    assert_eq!(rules[0].matchers[0]["match_path"]["method"], "exact");
    assert_eq!(rules[0].matchers[0]["match_path"]["value"], "/app");
}

// Source: https://developers.upcloud.com/1.3/35-load-balancers/#get-rule-details
const LB_RULE_DETAIL_JSON: &str = r#"
    {
        "actions": [
            {
                "type": "use_backend",
                "action_use_backend": {
                    "backend": "example-backend-2"
                }
            }
        ],
        "created_at": "2021-12-07T13:58:30.817272Z",
        "matching_condition": "and",
        "matchers": [
            {
                "type": "path",
                "match_path": {
                    "method": "exact",
                    "value": "/app"
                }
            }
        ],
        "name": "example-rule-1",
        "priority": 100,
        "updated_at": "2022-02-11T17:33:08.490581Z"
    }
"#;

// ─── Test 14: Rule detail ─────────────────────────────────────────────────

#[test]
fn deserialize_lb_rule_detail() {
    let rule: FrontendRule = serde_json::from_str(LB_RULE_DETAIL_JSON).unwrap();
    assert_eq!(rule.name, "example-rule-1");
    assert_eq!(rule.priority, 100);
    assert_eq!(rule.matching_condition.as_deref(), Some("and"));
    assert_eq!(
        rule.created_at.as_deref(),
        Some("2021-12-07T13:58:30.817272Z")
    );
    assert_eq!(
        rule.updated_at.as_deref(),
        Some("2022-02-11T17:33:08.490581Z")
    );
    assert_eq!(rule.matchers.len(), 1);
    assert_eq!(rule.matchers[0]["type"], "path");
    assert_eq!(rule.matchers[0]["match_path"]["method"], "exact");
    assert_eq!(rule.matchers[0]["match_path"]["value"], "/app");
    assert_eq!(rule.actions.len(), 1);
    assert_eq!(rule.actions[0]["type"], "use_backend");
    assert_eq!(
        rule.actions[0]["action_use_backend"]["backend"],
        "example-backend-2"
    );
}

// Source: https://developers.upcloud.com/1.3/35-load-balancers/#list-resolvers
const LB_RESOLVER_LIST_JSON: &str = r#"
    [
        {
            "cache_invalid": 30,
            "cache_valid": 180,
            "created_at": "2022-02-11T16:39:55.321306Z",
            "name": "example-resolver",
            "nameservers": [
                "172.16.1.250"
            ],
            "retries": 5,
            "timeout": 30,
            "timeout_retry": 10,
            "updated_at": "2022-02-11T17:33:08.490581Z"
        }
    ]
"#;

// ─── Test 15: List resolvers ──────────────────────────────────────────────

#[test]
fn deserialize_lb_resolver_list() {
    let resolvers: Vec<Resolver> = serde_json::from_str(LB_RESOLVER_LIST_JSON).unwrap();
    assert_eq!(resolvers.len(), 1);
    let r = &resolvers[0];
    assert_eq!(r.name, "example-resolver");
    assert_eq!(r.nameservers, vec!["172.16.1.250"]);
    assert_eq!(r.retries, 5);
    assert_eq!(r.timeout, 30);
    assert_eq!(r.timeout_retry, 10);
    assert_eq!(r.cache_valid, 180);
    assert_eq!(r.cache_invalid, 30);
    assert_eq!(r.created_at.as_deref(), Some("2022-02-11T16:39:55.321306Z"));
    assert_eq!(r.updated_at.as_deref(), Some("2022-02-11T17:33:08.490581Z"));
}

// Source: https://developers.upcloud.com/1.3/35-load-balancers/#get-resolver-details
const LB_RESOLVER_DETAIL_JSON: &str = r#"
    {
        "cache_invalid": 30,
        "cache_valid": 180,
        "created_at": "2022-02-11T16:39:55.321306Z",
        "name": "example-resolver",
        "nameservers": [
            "172.16.1.250"
        ],
        "retries": 5,
        "timeout": 30,
        "timeout_retry": 10,
        "updated_at": "2022-02-11T17:33:08.490581Z"
    }
"#;

// ─── Test 16: Resolver detail ─────────────────────────────────────────────

#[test]
fn deserialize_lb_resolver_detail() {
    let r: Resolver = serde_json::from_str(LB_RESOLVER_DETAIL_JSON).unwrap();
    assert_eq!(r.name, "example-resolver");
    assert_eq!(r.nameservers, vec!["172.16.1.250"]);
    assert_eq!(r.retries, 5);
    assert_eq!(r.timeout, 30);
    assert_eq!(r.timeout_retry, 10);
    assert_eq!(r.cache_valid, 180);
    assert_eq!(r.cache_invalid, 30);
    assert_eq!(r.created_at.as_deref(), Some("2022-02-11T16:39:55.321306Z"));
    assert_eq!(r.updated_at.as_deref(), Some("2022-02-11T17:33:08.490581Z"));
}

// Source: https://developers.upcloud.com/1.3/35-load-balancers/#create-resolver
const CREATE_RESOLVER_REQUEST_JSON: &str = r#"
    {
        "name": "example-resolver",
        "nameservers": [
            "172.16.1.4:53"
        ],
        "retries": 5,
        "timeout": 30,
        "timeout_retry": 10,
        "cache_valid": 180,
        "cache_invalid": 10
    }
"#;

// ─── Test 17: Create resolver request ─────────────────────────────────────

#[test]
fn deserialize_create_resolver_request() {
    let req: CreateResolverRequest = serde_json::from_str(CREATE_RESOLVER_REQUEST_JSON).unwrap();
    assert_eq!(req.name, "example-resolver");
    assert_eq!(req.nameservers, vec!["172.16.1.4:53"]);
    assert_eq!(req.retries, Some(5));
    assert_eq!(req.timeout, Some(30));
    assert_eq!(req.timeout_retry, Some(10));
    assert_eq!(req.cache_valid, Some(180));
    assert_eq!(req.cache_invalid, Some(10));
}

// Source: https://developers.upcloud.com/1.3/35-load-balancers/#create-backend
const CREATE_BACKEND_REQUEST_JSON: &str = r#"
    {
        "name": "example-backend-2",
        "resolver": "example-resolver",
        "members": [
            {
                "name": "member1.example.com",
                "port": 8000,
                "type": "dynamic",
                "max_sessions": 1000,
                "weight": 100,
                "enabled": true
            }
        ],
        "properties": {
            "timeout_server": 30,
            "health_check_type": "tcp"
        }
    }
"#;

// ─── Test 18: Create backend request ──────────────────────────────────────

#[test]
fn deserialize_create_backend_request() {
    let val: serde_json::Value = serde_json::from_str(CREATE_BACKEND_REQUEST_JSON).unwrap();
    assert_eq!(val["name"], "example-backend-2");
    assert_eq!(val["resolver"], "example-resolver");
    assert_eq!(val["members"].as_array().unwrap().len(), 1);
    assert_eq!(val["members"][0]["name"], "member1.example.com");
    assert_eq!(val["members"][0]["type"], "dynamic");
    assert_eq!(val["members"][0]["port"], 8000);
    assert_eq!(val["members"][0]["max_sessions"], 1000);
    assert_eq!(val["members"][0]["weight"], 100);
    assert_eq!(val["members"][0]["enabled"], true);
    assert_eq!(val["properties"]["health_check_type"], "tcp");
    assert_eq!(val["properties"]["timeout_server"], 30);
}

// Source: https://developers.upcloud.com/1.3/35-load-balancers/#create-frontend
const CREATE_FRONTEND_REQUEST_JSON: &str = r#"
    {
        "name": "example-frontend",
        "mode": "http",
        "port": 443,
        "default_backend": "example-backend",
        "networks": [
            {
                "name": "example-network-1"
            }
        ],
        "properties": {
            "timeout_client": 5
        }
    }
"#;

// ─── Test 19: Create frontend request ─────────────────────────────────────

#[test]
fn deserialize_create_frontend_request() {
    let req: CreateFrontendRequest = serde_json::from_str(CREATE_FRONTEND_REQUEST_JSON).unwrap();
    assert_eq!(req.name, "example-frontend");
    assert_eq!(req.mode, "http");
    assert_eq!(req.port, 443);
    assert_eq!(req.default_backend, "example-backend");
    assert_eq!(req.networks.len(), 1);
    assert_eq!(req.networks[0].name, "example-network-1");
    assert!(req.properties.is_some());
    assert_eq!(req.properties.as_ref().unwrap()["timeout_client"], 5);
}

// Source: https://developers.upcloud.com/1.3/35-load-balancers/#create-rule
const CREATE_FRONTEND_RULE_REQUEST_JSON: &str = r#"
    {
        "name": "example-rule-1",
        "priority": 100,
        "matchers": [
            {
                "type": "path",
                "match_path": {
                    "method": "exact",
                    "value": "/app"
                }
            }
        ],
        "actions": [
            {
                "type": "use_backend",
                "action_use_backend": {
                    "backend": "example-backend-2"
                }
            }
        ]
    }
"#;

// ─── Test 20: Create frontend rule request ────────────────────────────────

#[test]
fn deserialize_create_frontend_rule_request() {
    let req: CreateFrontendRuleRequest =
        serde_json::from_str(CREATE_FRONTEND_RULE_REQUEST_JSON).unwrap();
    assert_eq!(req.name, "example-rule-1");
    assert_eq!(req.priority, 100);
    assert_eq!(req.matchers.len(), 1);
    assert_eq!(req.matchers[0]["type"], "path");
    assert_eq!(req.matchers[0]["match_path"]["method"], "exact");
    assert_eq!(req.matchers[0]["match_path"]["value"], "/app");
    assert_eq!(req.actions.len(), 1);
    assert_eq!(req.actions[0]["type"], "use_backend");
    assert_eq!(
        req.actions[0]["action_use_backend"]["backend"],
        "example-backend-2"
    );
}

// Source: https://developers.upcloud.com/1.3/35-load-balancers/#list-backend-tls-configs
const LB_BACKEND_TLS_CONFIG_LIST_JSON: &str = r#"
    [
        {
            "certificate_bundle_uuid": "0aded5c1-c7a3-498a-b9c8-a871611c47a2",
            "created_at": "2022-02-11T17:33:08.490581Z",
            "name": "example-backend-tls-1",
            "updated_at": "2022-02-11T17:33:08.490581Z"
        },
        {
            "certificate_bundle_uuid": "c136c429-d18f-4192-b55e-e7b5079dee40",
            "created_at": "2021-11-11T11:21:51.121789Z",
            "name": "example-backend-tls-2",
            "updated_at": "2021-11-11T11:21:51.121789Z"
        }
    ]
"#;

// ─── Test 21: Backend TLS config list ─────────────────────────────────────

#[test]
fn deserialize_lb_backend_tls_config_list() {
    let configs: Vec<TLSConfig> = serde_json::from_str(LB_BACKEND_TLS_CONFIG_LIST_JSON).unwrap();
    assert_eq!(configs.len(), 2);
    assert_eq!(configs[0].name, "example-backend-tls-1");
    assert_eq!(
        configs[0].certificate_bundle_uuid.as_deref(),
        Some("0aded5c1-c7a3-498a-b9c8-a871611c47a2")
    );
    assert_eq!(
        configs[0].created_at.as_deref(),
        Some("2022-02-11T17:33:08.490581Z")
    );
    assert_eq!(
        configs[0].updated_at.as_deref(),
        Some("2022-02-11T17:33:08.490581Z")
    );
    assert!(configs[0].certificate_chain.is_none());
    assert!(configs[0].name_match.is_none());
    assert_eq!(configs[1].name, "example-backend-tls-2");
    assert_eq!(
        configs[1].certificate_bundle_uuid.as_deref(),
        Some("c136c429-d18f-4192-b55e-e7b5079dee40")
    );
    assert_eq!(
        configs[1].created_at.as_deref(),
        Some("2021-11-11T11:21:51.121789Z")
    );
    assert_eq!(
        configs[1].updated_at.as_deref(),
        Some("2021-11-11T11:21:51.121789Z")
    );
    assert!(configs[1].certificate_chain.is_none());
    assert!(configs[1].name_match.is_none());
}

// Source: https://developers.upcloud.com/1.3/35-load-balancers/#create-backend-tls-config
const CREATE_TLS_CONFIG_REQUEST_JSON: &str = r#"
    {
        "name": "example-tls-config",
        "certificate_bundle_uuid": "0aded5c1-c7a3-498a-b9c8-a871611c47a2"
    }
"#;

// ─── Test 22: Create TLS config request ───────────────────────────────────

#[test]
fn deserialize_create_tls_config_request() {
    let val: serde_json::Value = serde_json::from_str(CREATE_TLS_CONFIG_REQUEST_JSON).unwrap();
    assert_eq!(val["name"], "example-tls-config");
    assert_eq!(
        val["certificate_bundle_uuid"],
        "0aded5c1-c7a3-498a-b9c8-a871611c47a2"
    );

    let req: CreateTLSConfigRequest = serde_json::from_str(CREATE_TLS_CONFIG_REQUEST_JSON).unwrap();
    assert_eq!(req.name, "example-tls-config");
    assert!(req.certificate_chain.is_none());
    assert!(req.private_key.is_none());
    assert!(req.name_match.is_none());
}

// ─── Test 23: Certificate bundle detail from docs ─────────────────────────
// Real JSON from UpCloud API 1.3 docs: "Get certificate bundle details"
// Source: https://developers.upcloud.com/1.3/35-load-balancers/#get-certificate-bundle-details
const CERTIFICATE_BUNDLE_DETAIL_JSON: &str = r#"
    {
        "certificate": "LStLS1CRUdJTiBDRVJUSUZJQ0FURS0tLS0t",
        "created_at": "2022-02-11T17:31:38.202398Z",
        "hostnames": [
            "web.example.com"
        ],
        "intermediates": "LS0tS1CRUdJTiBDRVJUSUZJQ0FURS0tLS0t",
        "key_type": "rsa",
        "name": "example-dynamic-certificate",
        "not_after": "2022-02-13T05:33:43Z",
        "not_before": "2022-02-12T05:32:43Z",
        "operational_state": "idle",
        "type": "dynamic",
        "updated_at": "2022-02-12T08:13:47.877562Z",
        "uuid": "0aded5c1-c7a3-498a-b9c8-a871611c47a2"
    }
"#;

#[test]
fn deserialize_certificate_bundle_detail() {
    let cb: CertificateBundle = serde_json::from_str(CERTIFICATE_BUNDLE_DETAIL_JSON).unwrap();
    assert_eq!(cb.uuid, "0aded5c1-c7a3-498a-b9c8-a871611c47a2");
    assert_eq!(cb.name, "example-dynamic-certificate");
    assert_eq!(
        cb.certificate.as_deref(),
        Some("LStLS1CRUdJTiBDRVJUSUZJQ0FURS0tLS0t")
    );
    assert_eq!(
        cb.intermediates.as_deref(),
        Some("LS0tS1CRUdJTiBDRVJUSUZJQ0FURS0tLS0t")
    );
    assert_eq!(cb.key_type.as_deref(), Some("rsa"));
    assert_eq!(cb.operational_state.as_deref(), Some("idle"));
    assert_eq!(cb.not_after.as_deref(), Some("2022-02-13T05:33:43Z"));
    assert_eq!(cb.not_before.as_deref(), Some("2022-02-12T05:32:43Z"));
    assert_eq!(cb.bundle_type.as_deref(), Some("dynamic"));
    assert_eq!(
        cb.created_at.as_deref(),
        Some("2022-02-11T17:31:38.202398Z")
    );
    assert_eq!(
        cb.updated_at.as_deref(),
        Some("2022-02-12T08:13:47.877562Z")
    );
}

// ─── Test 24: Certificate bundle list from docs ──────────────────────────
// Real JSON from UpCloud API 1.3 docs: "List certificate bundles"
// Source: https://developers.upcloud.com/1.3/35-load-balancers/#list-certificate-bundles
const CERTIFICATE_BUNDLE_LIST_JSON: &str = r#"
    [
        {
            "certificate": "LS0LS1CRUdJTiBDRVJUSUZJQ0FURS0tLS0t",
            "created_at": "2021-11-09T08:07:39.749472Z",
            "hostnames": [
                "application.example.com"
            ],
            "key_type": "ecdsa",
            "name": "example-manual-certificate",
            "not_after": "2031-10-20T12:10:36Z",
            "not_before": "2021-10-22T12:10:36Z",
            "operational_state": "idle",
            "type": "manual",
            "updated_at": "2021-11-09T08:07:39.749472Z",
            "uuid": "bf571589-7378-41f8-879e-5505613b070d"
        },
        {
            "certificate": "LStLS1CRUdJTiBDRVJUSUZJQ0FURS0tLS0t",
            "created_at": "2022-02-11T17:31:38.202398Z",
            "hostnames": [
                "web.example.com"
            ],
            "intermediates": "LS0tS1CRUdJTiBDRVJUSUZJQ0FURS0tLS0t",
            "key_type": "rsa",
            "name": "example-dynamic-certificate",
            "not_after": "2022-02-13T05:33:43Z",
            "not_before": "2022-02-12T05:32:43Z",
            "operational_state": "idle",
            "type": "dynamic",
            "updated_at": "2022-02-12T08:13:47.877562Z",
            "uuid": "0aded5c1-c7a3-498a-b9c8-a871611c47a2"
        }
    ]
"#;

#[test]
fn deserialize_certificate_bundle_list() {
    let bundles: Vec<CertificateBundle> =
        serde_json::from_str(CERTIFICATE_BUNDLE_LIST_JSON).unwrap();
    assert_eq!(bundles.len(), 2);
    assert_eq!(bundles[0].uuid, "bf571589-7378-41f8-879e-5505613b070d");
    assert_eq!(bundles[0].name, "example-manual-certificate");
    assert_eq!(bundles[0].bundle_type.as_deref(), Some("manual"));
    assert_eq!(bundles[0].key_type.as_deref(), Some("ecdsa"));
    assert_eq!(
        bundles[0].not_after.as_deref(),
        Some("2031-10-20T12:10:36Z")
    );
    assert_eq!(bundles[0].operational_state.as_deref(), Some("idle"));
    assert!(bundles[0].intermediates.is_none());
    assert_eq!(bundles[1].uuid, "0aded5c1-c7a3-498a-b9c8-a871611c47a2");
    assert_eq!(bundles[1].name, "example-dynamic-certificate");
    assert_eq!(bundles[1].bundle_type.as_deref(), Some("dynamic"));
    assert_eq!(bundles[1].key_type.as_deref(), Some("rsa"));
    assert!(bundles[1].intermediates.is_some());
}

// ─── Test 25: Frontend detail from docs (with rules + tls_configs) ───────
// Real JSON from UpCloud API 1.3 docs: "Get frontend details"
// Source: https://developers.upcloud.com/1.3/35-load-balancers/#get-frontend-details
const FRONTEND_DETAIL_FROM_DOCS_JSON: &str = r#"
    {
        "created_at": "2021-12-07T13:58:30.817272Z",
        "default_backend": "example-backend-1",
        "mode": "http",
        "name": "example-frontend",
        "networks": [
            {
                "name": "example-network-1"
            }
        ],
        "port": 443,
        "properties": {
            "timeout_client": 10
        },
        "rules": [
            {
                "actions": [
                    {
                        "type": "use_backend",
                        "action_use_backend": {
                            "backend": "example-backend-2"
                        }
                    }
                ],
                "created_at": "2021-12-07T13:58:30.817272Z",
                "matching_condition": "and",
                "matchers": [
                    {
                        "type": "path",
                        "match_path": {
                            "method": "exact",
                            "value": "/app"
                        }
                    }
                ],
                "name": "example-rule-1",
                "priority": 100,
                "updated_at": "2022-02-11T17:33:08.490581Z"
            }
        ],
        "tls_configs": [
            {
                "certificate_bundle_uuid": "0aded5c1-c7a3-498a-b9c8-a871611c47a2",
                "created_at": "2022-02-11T17:33:08.490581Z",
                "name": "example-tls-config",
                "updated_at": "2022-02-11T17:33:08.490581Z"
            }
        ],
        "updated_at": "2022-02-11T17:33:08.490581Z"
    }
"#;

#[test]
fn deserialize_frontend_detail_from_docs() {
    let fe: Frontend = serde_json::from_str(FRONTEND_DETAIL_FROM_DOCS_JSON).unwrap();
    assert_eq!(fe.name, "example-frontend");
    assert_eq!(fe.port, 443);
    assert_eq!(fe.mode, "http");
    assert_eq!(fe.default_backend, "example-backend-1");
    assert_eq!(
        fe.created_at.as_deref(),
        Some("2021-12-07T13:58:30.817272Z")
    );
    assert_eq!(
        fe.updated_at.as_deref(),
        Some("2022-02-11T17:33:08.490581Z")
    );
    assert_eq!(fe.networks.len(), 1);
    assert_eq!(fe.networks[0].name, "example-network-1");
    assert!(fe.properties.is_some());
    assert_eq!(fe.properties.as_ref().unwrap()["timeout_client"], 10);
    assert_eq!(fe.rules.len(), 1);
    assert_eq!(fe.rules[0].name, "example-rule-1");
    assert_eq!(fe.rules[0].priority, 100);
    assert_eq!(fe.rules[0].matching_condition.as_deref(), Some("and"));
    assert_eq!(fe.rules[0].matchers.len(), 1);
    assert_eq!(fe.rules[0].actions.len(), 1);
    assert_eq!(fe.tls_configs.len(), 1);
    assert_eq!(fe.tls_configs[0].name, "example-tls-config");
    assert_eq!(
        fe.tls_configs[0].certificate_bundle_uuid.as_deref(),
        Some("0aded5c1-c7a3-498a-b9c8-a871611c47a2")
    );
}

// ─── Test 26: ModifyBackendMemberRequest serialization ────────────────────
// Uses realistic values from the docs "Modify static member" example.

#[test]
fn serialize_modify_backend_member_request() {
    let req = ModifyBackendMemberRequest {
        name: Some("example-member-1".to_string()),
        weight: Some(100),
        max_sessions: Some(1000),
        enabled: Some(true),
        ip: Some("172.16.1.4".to_string()),
        port: Some(8000),
    };
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["name"], "example-member-1");
    assert_eq!(json["weight"], 100);
    assert_eq!(json["max_sessions"], 1000);
    assert_eq!(json["enabled"], true);
    assert_eq!(json["ip"], "172.16.1.4");
    assert_eq!(json["port"], 8000);
}

// ─── Test 27: ModifyFrontendRequest serialization ─────────────────────────
// Uses realistic values from the docs "Modify frontend" example.

#[test]
fn serialize_modify_frontend_request() {
    let req = ModifyFrontendRequest {
        name: Some("example-frontend".to_string()),
        default_backend: Some("example-backend".to_string()),
        mode: Some("http".to_string()),
        networks: Some(vec![FrontendNetwork {
            name: "example-network-2".to_string(),
        }]),
        properties: Some(
            serde_json::json!({"timeout_client": 10, "inbound_proxy_protocol": false}),
        ),
    };
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["name"], "example-frontend");
    assert_eq!(json["default_backend"], "example-backend");
    assert_eq!(json["mode"], "http");
    assert_eq!(json["networks"][0]["name"], "example-network-2");
    assert_eq!(json["properties"]["timeout_client"], 10);
}

// ─── Test 28: ModifyResolverRequest serialization ─────────────────────────
// Uses realistic values from the docs "Modify resolver" example.

#[test]
fn serialize_modify_resolver_request() {
    let req = ModifyResolverRequest {
        name: Some("example-resolver".to_string()),
        nameservers: Some(vec!["172.16.1.4:53".to_string()]),
        retries: Some(5),
        timeout: Some(30),
        timeout_retry: Some(10),
        cache_valid: Some(180),
        cache_invalid: Some(10),
    };
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["name"], "example-resolver");
    assert_eq!(json["nameservers"], serde_json::json!(["172.16.1.4:53"]));
    assert_eq!(json["retries"], 5);
    assert_eq!(json["timeout"], 30);
    assert_eq!(json["timeout_retry"], 10);
    assert_eq!(json["cache_valid"], 180);
    assert_eq!(json["cache_invalid"], 10);
}

// ─── Test 29: ReplaceFrontendRuleRequest serialization ────────────────────
// Uses realistic values from the docs "Replace rule" example.

#[test]
fn serialize_replace_frontend_rule_request() {
    let req = ReplaceFrontendRuleRequest {
        name: "example-rule-1".to_string(),
        priority: 100,
        matchers: vec![serde_json::json!({
            "type": "path",
            "match_path": {"method": "exact", "value": "/app"}
        })],
        actions: vec![serde_json::json!({
            "type": "use_backend",
            "action_use_backend": {"backend": "example-backend-2"}
        })],
        matching_condition: Some("and".to_string()),
    };
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["name"], "example-rule-1");
    assert_eq!(json["priority"], 100);
    assert_eq!(json["matching_condition"], "and");
    assert_eq!(json["matchers"][0]["type"], "path");
    assert_eq!(json["actions"][0]["type"], "use_backend");
}

// ─── Test 30: CreateCertificateBundleRequest serialization ────────────────
// Uses realistic values from the docs "Create manual certificate bundle".

#[test]
fn serialize_create_certificate_bundle_request() {
    let req = CreateCertificateBundleRequest {
        name: "example-manual-certificate".to_string(),
        certificate: Some("LS0LS1CRUdJTiBDRVJUSUZJQ0FURS0tLS0t".to_string()),
        intermediates: Some("LS0tS1CRUdJTiBDRVJUSUZJQ0FURS0tLS0t".to_string()),
        private_key: Some("LS0tL1CRUdJTiBQUklWQVRFIEtFWS0tLS0t".to_string()),
        bundle_type: Some("manual".to_string()),
    };
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["name"], "example-manual-certificate");
    assert!(json["certificate"].is_string());
    assert!(json["intermediates"].is_string());
    assert!(json["private_key"].is_string());
    assert_eq!(json["type"], "manual");
}

// ─── Test 31: ModifyCertificateBundleRequest omits None fields ────────────

#[test]
fn serialize_modify_certificate_bundle_request() {
    let req = ModifyCertificateBundleRequest {
        name: Some("example-dynamic-certificate".to_string()),
        certificate: None,
        intermediates: None,
        private_key: None,
        bundle_type: None,
    };
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["name"], "example-dynamic-certificate");
    assert!(json.get("certificate").is_none());
    assert!(json.get("intermediates").is_none());
    assert!(json.get("private_key").is_none());
    assert!(json.get("type").is_none());
}

// ─── Test 32: ModifyLoadBalancerNetworkRequest serialization ──────────────
// Uses realistic values from the docs "Modify network" example.

#[test]
fn serialize_modify_network_request() {
    let req = ModifyLoadBalancerNetworkRequest {
        name: Some("example-network-3".to_string()),
    };
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["name"], "example-network-3");

    let empty_req = ModifyLoadBalancerNetworkRequest { name: None };
    let json2 = serde_json::to_value(&empty_req).unwrap();
    assert!(json2.get("name").is_none());
}

// ─── Test 33: Node deserialization from docs ──────────────────────────────
// Real JSON from the "List services" response `nodes` array.
// Source: https://developers.upcloud.com/1.3/35-load-balancers/
const NODE_DETAIL_JSON: &str = r#"
    {
        "networks": [
            {
                "ip_addresses": [
                    {
                        "address": "100.127.5.146",
                        "listen": true
                    }
                ],
                "name": "example-network-1",
                "type": "public"
            },
            {
                "ip_addresses": [
                    {
                        "address": "172.16.1.44",
                        "listen": false
                    },
                    {
                        "address": "172.16.1.45",
                        "listen": true
                    }
                ],
                "name": "example-network-2",
                "type": "private"
            }
        ],
        "operational_state": "running"
    }
"#;

#[test]
fn deserialize_node() {
    let node: Node = serde_json::from_str(NODE_DETAIL_JSON).unwrap();
    assert_eq!(node.operational_state.as_deref(), Some("running"));
    assert_eq!(node.networks.len(), 2);
    assert_eq!(node.networks[0].name, "example-network-1");
    assert_eq!(node.networks[0].network_type, "public");
    assert_eq!(node.networks[0].ip_addresses.len(), 1);
    assert_eq!(node.networks[0].ip_addresses[0].address, "100.127.5.146");
    assert_eq!(node.networks[1].name, "example-network-2");
    assert_eq!(node.networks[1].network_type, "private");
    assert_eq!(node.networks[1].ip_addresses.len(), 2);
    assert_eq!(node.networks[1].ip_addresses[0].address, "172.16.1.44");
    assert_eq!(node.networks[1].ip_addresses[1].address, "172.16.1.45");
}
