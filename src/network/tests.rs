use super::*;

// Source: https://developers.upcloud.com/1.3/13-networks/#list-all-networks
const LIST_NETWORKS_JSON: &str = r#"
    {
        "networks": {
            "network": [
                {
                    "ip_networks": {
                        "ip_network": [
                            {
                                "address": "80.69.172.0/22",
                                "dhcp": "yes",
                                "dhcp_default_route": "yes",
                                "dhcp_dns": [
                                    "94.237.127.9",
                                    "94.237.40.9"
                                ],
                                "family": "IPv4",
                                "gateway": "80.69.172.1"
                            }
                        ]
                    },
                    "labels": [],
                    "name": "Public 80.69.172.0/22",
                    "type": "public",
                    "uuid": "03000000-0000-4000-8001-000000000000",
                    "zone": "fi-hel1",
                    "servers": {
                        "server": [
                            {
                                "uuid": "007e3200-268f-4848-8b45-bd88c44555d2",
                                "title": "Helsinki server #1"
                            }
                        ]
                    }
                }
            ]
        }
    }
"#;

// ─── List all networks ──────────────────────────────────────────────────

#[test]
fn deserialize_network_list() {
    let resp: NetworkListResponse = serde_json::from_str(LIST_NETWORKS_JSON).unwrap();
    let networks = resp.networks.network;
    assert_eq!(networks.len(), 1);

    let net = &networks[0];
    assert_eq!(net.name, "Public 80.69.172.0/22");
    assert_eq!(net.network_type, NetworkType::Public);
    assert_eq!(net.zone, "fi-hel1");

    let ip_nets = net.ip_networks.as_ref().unwrap();
    assert_eq!(ip_nets.ip_network.len(), 1);
    assert_eq!(ip_nets.ip_network[0].address, "80.69.172.0/22");

    let servers = net.servers.as_ref().unwrap();
    assert_eq!(servers.server.len(), 1);
    assert_eq!(
        servers.server[0].uuid,
        "007e3200-268f-4848-8b45-bd88c44555d2"
    );
    assert!(servers.server[0].ip_address.is_none());

    assert_eq!(net.uuid, "03000000-0000-4000-8001-000000000000");
    assert!(net.router.is_none());

    assert_eq!(ip_nets.ip_network[0].dhcp.as_deref(), Some("yes"));
    assert_eq!(
        ip_nets.ip_network[0].dhcp_default_route.as_deref(),
        Some("yes")
    );
    let dhcp_dns = ip_nets.ip_network[0].dhcp_dns.as_ref().unwrap();
    assert_eq!(dhcp_dns.len(), 2);
    assert_eq!(dhcp_dns[0], "94.237.127.9");
    assert_eq!(dhcp_dns[1], "94.237.40.9");
    assert_eq!(ip_nets.ip_network[0].family, "IPv4");
    assert_eq!(
        ip_nets.ip_network[0].gateway.as_deref(),
        Some("80.69.172.1")
    );

    assert!(net.labels.as_ref().unwrap().is_empty());

    assert_eq!(
        servers.server[0].title.as_deref(),
        Some("Helsinki server #1")
    );
}

// Source: https://developers.upcloud.com/1.3/13-networks/#list-networks-within-zone
const LIST_NETWORKS_ZONE_JSON: &str = r#"
    {
        "networks": {
            "network": [
                {
                    "ip_networks": {
                        "ip_network": [
                            {
                                "address": "80.69.172.0/22",
                                "dhcp": "yes",
                                "dhcp_default_route": "yes",
                                "dhcp_dns": [
                                    "94.237.127.9",
                                    "94.237.40.9"
                                ],
                                "dhcp_routes_configuration": {
                                    "effective_routes_auto_population": {
                                        "enabled": "yes",
                                        "exclude_by_source": [
                                            "router-connected-network"
                                        ],
                                        "filter_by_route_type": [
                                            "user"
                                        ]
                                    }
                                },
                                "dhcp_effective_routes": [
                                    {
                                        "auto_populated": "yes",
                                        "route": "192.168.0.1/24",
                                        "nexthop": "80.69.172.1"
                                    }
                                ],
                                "family": "IPv4",
                                "gateway": "80.69.172.1"
                            }
                        ]
                    },
                    "effective_routes": [
                        {
                            "source": "static-route",
                            "route": "192.168.0.1/24",
                            "nexthop": "80.69.172.1",
                            "type": "user"
                        }
                    ],
                    "labels": [],
                    "name": "Public 80.69.172.0/22",
                    "type": "public",
                    "uuid": "03000000-0000-4000-8001-000000000000",
                    "zone": "fi-hel1"
                }
            ]
        }
    }
"#;

// ─── List networks within zone ──────────────────────────────────────────

#[test]
fn deserialize_network_list_within_zone() {
    let resp: NetworkListResponse = serde_json::from_str(LIST_NETWORKS_ZONE_JSON).unwrap();
    let net = &resp.networks.network[0];
    assert_eq!(net.zone, "fi-hel1");

    let ip_net = &net.ip_networks.as_ref().unwrap().ip_network[0];
    assert!(ip_net.dhcp_routes_configuration.is_some());
    assert!(ip_net.dhcp_effective_routes.is_some());
    assert_eq!(ip_net.dhcp_effective_routes.as_ref().unwrap().len(), 1);

    assert_eq!(net.name, "Public 80.69.172.0/22");
    assert_eq!(net.network_type, NetworkType::Public);
    assert_eq!(net.uuid, "03000000-0000-4000-8001-000000000000");
    assert!(net.labels.as_ref().unwrap().is_empty());

    assert_eq!(ip_net.address, "80.69.172.0/22");
    assert_eq!(ip_net.dhcp.as_deref(), Some("yes"));
    assert_eq!(ip_net.dhcp_default_route.as_deref(), Some("yes"));
    let dhcp_dns = ip_net.dhcp_dns.as_ref().unwrap();
    assert_eq!(dhcp_dns.len(), 2);
    assert_eq!(dhcp_dns[0], "94.237.127.9");
    assert_eq!(dhcp_dns[1], "94.237.40.9");
    assert_eq!(ip_net.family, "IPv4");
    assert_eq!(ip_net.gateway.as_deref(), Some("80.69.172.1"));

    let dhcp_eff = &ip_net.dhcp_effective_routes.as_ref().unwrap()[0];
    assert_eq!(dhcp_eff["auto_populated"], "yes");
    assert_eq!(dhcp_eff["route"], "192.168.0.1/24");
    assert_eq!(dhcp_eff["nexthop"], "80.69.172.1");

    let eff_routes = net.effective_routes.as_ref().unwrap();
    assert_eq!(eff_routes.len(), 1);
    assert_eq!(eff_routes[0]["source"], "static-route");
    assert_eq!(eff_routes[0]["route"], "192.168.0.1/24");
    assert_eq!(eff_routes[0]["nexthop"], "80.69.172.1");
    assert_eq!(eff_routes[0]["type"], "user");
}

// Source: https://developers.upcloud.com/1.3/13-networks/#get-network-details
const GET_NETWORK_JSON: &str = r#"
    {
        "network": {
            "name": "Test private net",
            "type": "private",
            "uuid": "034c12bc-cf15-4b19-97b2-0ab4e51bb98d",
            "zone": "uk-lon1",
            "ip_networks": {
                "ip_network": [
                    {
                        "address": "172.16.0.0/22",
                        "dhcp": "yes",
                        "dhcp_default_route": "no",
                        "dhcp_dns": [
                            "172.16.0.10",
                            "172.16.1.10"
                        ],
                        "family": "IPv4",
                        "gateway": "172.16.0.1"
                    }
                ]
            },
            "labels": [
                {
                    "key": "test",
                    "value": ""
                }
            ]
        }
    }
"#;

// ─── Get network details ────────────────────────────────────────────────

#[test]
fn deserialize_network_detail() {
    let resp: NetworkResponse = serde_json::from_str(GET_NETWORK_JSON).unwrap();
    let net = resp.network;
    assert_eq!(net.network_type, NetworkType::Private);
    assert_eq!(net.uuid, "034c12bc-cf15-4b19-97b2-0ab4e51bb98d");
    assert_eq!(net.labels.as_ref().unwrap().len(), 1);
    assert_eq!(net.labels.as_ref().unwrap()[0].key, "test");

    assert_eq!(net.name, "Test private net");
    assert_eq!(net.zone, "uk-lon1");
    assert!(net.router.is_none());
    assert!(net.servers.is_none());

    let ip_nets = net.ip_networks.as_ref().unwrap();
    assert_eq!(ip_nets.ip_network[0].address, "172.16.0.0/22");
    assert_eq!(ip_nets.ip_network[0].dhcp.as_deref(), Some("yes"));
    assert_eq!(
        ip_nets.ip_network[0].dhcp_default_route.as_deref(),
        Some("no")
    );
    let dhcp_dns = ip_nets.ip_network[0].dhcp_dns.as_ref().unwrap();
    assert_eq!(dhcp_dns.len(), 2);
    assert_eq!(dhcp_dns[0], "172.16.0.10");
    assert_eq!(dhcp_dns[1], "172.16.1.10");
    assert_eq!(ip_nets.ip_network[0].family, "IPv4");
    assert_eq!(ip_nets.ip_network[0].gateway.as_deref(), Some("172.16.0.1"));

    assert_eq!(net.labels.as_ref().unwrap()[0].value, "");
}

// Source: https://developers.upcloud.com/1.3/13-networks/#create-sdn-private-network
const CREATE_NETWORK_REQUEST_JSON: &str = r#"
    {
        "network": {
            "name": "Test private net",
            "zone": "uk-lon1",
            "router": "04c0df35-2658-4b0c-8ac7-962090f4e92a",
            "ip_networks": {
                "ip_network": [
                    {
                        "address": "172.16.0.0/22",
                        "dhcp": "yes",
                        "dhcp_default_route": "no",
                        "dhcp_dns": [
                            "172.16.0.10",
                            "172.16.1.10"
                        ],
                        "dhcp_routes_configuration": {
                            "effective_routes_auto_population": {
                                "enabled": "yes",
                                "exclude_by_source": [
                                    "static-route"
                                ]
                            }
                        },
                        "dhcp_routes": [
                            "192.168.0.0/24-nexthop=10.0.1.100",
                            "192.168.100.100/32"
                        ],
                        "family": "IPv4",
                        "gateway": "172.16.0.1"
                    }
                ]
            },
            "labels": [
                {
                    "key": "test",
                    "value": ""
                }
            ]
        }
    }
"#;

// ─── Create SDN private network request ─────────────────────────────────

#[test]
fn deserialize_create_network_request() {
    #[derive(Debug, Deserialize)]
    struct Body {
        network: CreateNetworkRequest,
    }
    let body: Body = serde_json::from_str(CREATE_NETWORK_REQUEST_JSON).unwrap();
    assert_eq!(body.network.name, "Test private net");
    assert_eq!(body.network.zone, "uk-lon1");
    assert_eq!(
        body.network.router.as_deref(),
        Some("04c0df35-2658-4b0c-8ac7-962090f4e92a")
    );

    let ip_net = &body.network.ip_networks.unwrap().ip_network[0];
    assert_eq!(ip_net.dhcp_routes.as_ref().unwrap().len(), 2);
    assert!(ip_net.dhcp_routes_configuration.is_some());

    assert_eq!(ip_net.address, "172.16.0.0/22");
    assert_eq!(ip_net.dhcp.as_deref(), Some("yes"));
    assert_eq!(ip_net.dhcp_default_route.as_deref(), Some("no"));
    let dhcp_dns = ip_net.dhcp_dns.as_ref().unwrap();
    assert_eq!(dhcp_dns.len(), 2);
    assert_eq!(dhcp_dns[0], "172.16.0.10");
    assert_eq!(dhcp_dns[1], "172.16.1.10");
    assert_eq!(ip_net.family, "IPv4");
    assert_eq!(ip_net.gateway.as_deref(), Some("172.16.0.1"));
    let dhcp_routes = ip_net.dhcp_routes.as_ref().unwrap();
    assert_eq!(dhcp_routes[0], "192.168.0.0/24-nexthop=10.0.1.100");
    assert_eq!(dhcp_routes[1], "192.168.100.100/32");

    let labels = body.network.labels.unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0].key, "test");
    assert_eq!(labels[0].value, "");
}

// Source: https://developers.upcloud.com/1.3/13-networks/#create-sdn-private-network
const CREATE_NETWORK_RESPONSE_JSON: &str = r#"
    {
        "network": {
            "name": "Test private net",
            "type": "private",
            "uuid": "034c12bc-cf15-4b19-97b2-0ab4e51bb98d",
            "zone": "uk-lon1",
            "router": "04c0df35-2658-4b0c-8ac7-962090f4e92a",
            "ip_networks": {
                "ip_network": [
                    {
                        "address": "172.16.0.0/22",
                        "dhcp": "yes",
                        "dhcp_default_route": "no",
                        "dhcp_dns": [
                            "172.16.0.10",
                            "172.16.1.10"
                        ],
                        "dhcp_routes": [
                            "192.168.0.0/24-nexthop=10.0.1.100",
                            "192.168.100.100/32"
                        ],
                        "dhcp_routes_configuration": {
                            "effective_routes_auto_population": {
                                "enabled": "yes",
                                "exclude_by_source": [
                                    "static-route"
                                ]
                            }
                        },
                        "dhcp_effective_routes": [
                            {
                                "auto_populated": "no",
                                "route": "192.168.0.0/24",
                                "nexthop": "10.0.1.100"
                            },
                            {
                                "auto_populated": "no",
                                "route": "192.168.100.100/32",
                                "nexthop": "172.16.0.1"
                            },
                            {
                                "auto_populated": "yes",
                                "route": "10.0.0.0/24",
                                "nexthop": "172.16.0.1"
                            }
                        ],
                        "family": "IPv4",
                        "gateway": "172.16.0.1"
                    }
                ]
            },
            "effective_routes": [
                {
                    "source": "router-connected-network",
                    "route": "10.0.0.0/24",
                    "source_resource_id": "034c12bc-cf15-4b19-97b2-0ab4e51bb98d"
                }
            ],
            "labels": [
                {
                    "key": "test",
                    "value": ""
                }
            ]
        }
    }
"#;

// ─── Create SDN private network response ────────────────────────────────

#[test]
fn deserialize_create_network_response() {
    let resp: NetworkResponse = serde_json::from_str(CREATE_NETWORK_RESPONSE_JSON).unwrap();
    let net = resp.network;
    assert_eq!(net.name, "Test private net");
    assert_eq!(net.network_type, NetworkType::Private);
    assert_eq!(
        net.router.as_deref(),
        Some("04c0df35-2658-4b0c-8ac7-962090f4e92a")
    );

    assert_eq!(net.uuid, "034c12bc-cf15-4b19-97b2-0ab4e51bb98d");
    assert_eq!(net.zone, "uk-lon1");

    let ip_net = &net.ip_networks.unwrap().ip_network[0];
    assert_eq!(ip_net.dhcp_effective_routes.as_ref().unwrap().len(), 3);
    assert!(ip_net.dhcp_routes_configuration.is_some());

    assert_eq!(ip_net.address, "172.16.0.0/22");
    assert_eq!(ip_net.dhcp.as_deref(), Some("yes"));
    assert_eq!(ip_net.dhcp_default_route.as_deref(), Some("no"));
    let dhcp_dns = ip_net.dhcp_dns.as_ref().unwrap();
    assert_eq!(dhcp_dns.len(), 2);
    assert_eq!(dhcp_dns[0], "172.16.0.10");
    assert_eq!(dhcp_dns[1], "172.16.1.10");
    assert_eq!(ip_net.family, "IPv4");
    assert_eq!(ip_net.gateway.as_deref(), Some("172.16.0.1"));
    assert_eq!(ip_net.dhcp_routes.as_ref().unwrap().len(), 2);

    let eff = net.effective_routes.unwrap();
    assert_eq!(eff.len(), 1);
    assert_eq!(eff[0]["source"], "router-connected-network");
    assert_eq!(eff[0]["route"], "10.0.0.0/24");
    assert_eq!(
        eff[0]["source_resource_id"],
        "034c12bc-cf15-4b19-97b2-0ab4e51bb98d"
    );

    let labels = net.labels.unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0].key, "test");
    assert_eq!(labels[0].value, "");
}

// Source: https://developers.upcloud.com/1.3/13-networks/#modify-network-details
const MODIFY_NETWORK_REQUEST_JSON: &str = r#"
    {
        "network": {
            "name": "My private network",
            "ip_networks": {
                "ip_network": [
                    {
                        "address": "172.16.0.0/22",
                        "dhcp": "yes",
                        "dhcp_default_route": "no",
                        "dhcp_dns": [
                            "172.16.0.10",
                            "172.16.1.10"
                        ],
                        "dhcp_routes": [
                            "192.168.0.0/24",
                            "192.168.100.100/32"
                        ],
                        "dhcp_routes_configuration": {
                            "effective_routes_auto_population": {
                                "enabled": "no"
                            }
                        },
                        "family": "IPv4",
                        "gateway": "172.16.0.1"
                    }
                ]
            },
            "labels": [
                {
                    "key": "purpose",
                    "value": "databases"
                }
            ]
        }
    }
"#;

// ─── Modify network request ─────────────────────────────────────────────

#[test]
fn deserialize_modify_network_request() {
    #[derive(Debug, Deserialize)]
    struct Body {
        network: ModifyNetworkRequest,
    }
    let body: Body = serde_json::from_str(MODIFY_NETWORK_REQUEST_JSON).unwrap();
    assert_eq!(body.network.name.as_deref(), Some("My private network"));

    assert!(body.network.router.is_none());

    let ip_net = &body.network.ip_networks.unwrap().ip_network[0];
    assert_eq!(ip_net.dhcp_routes.as_ref().unwrap().len(), 2);
    assert!(ip_net.dhcp_routes_configuration.is_some());

    assert_eq!(ip_net.address, "172.16.0.0/22");
    assert_eq!(ip_net.dhcp.as_deref(), Some("yes"));
    assert_eq!(ip_net.dhcp_default_route.as_deref(), Some("no"));
    let dhcp_dns = ip_net.dhcp_dns.as_ref().unwrap();
    assert_eq!(dhcp_dns.len(), 2);
    assert_eq!(dhcp_dns[0], "172.16.0.10");
    assert_eq!(dhcp_dns[1], "172.16.1.10");
    assert_eq!(ip_net.family, "IPv4");
    assert_eq!(ip_net.gateway.as_deref(), Some("172.16.0.1"));
    let dhcp_routes = ip_net.dhcp_routes.as_ref().unwrap();
    assert_eq!(dhcp_routes[0], "192.168.0.0/24");
    assert_eq!(dhcp_routes[1], "192.168.100.100/32");

    let labels = body.network.labels.unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0].key, "purpose");
    assert_eq!(labels[0].value, "databases");
}

// Source: https://developers.upcloud.com/1.3/13-networks/#modify-network-details
const MODIFY_NETWORK_ROUTER_JSON: &str = r#"
    {
        "network": {
            "router": "04c0df35-2658-4b0c-8ac7-962090f4e92a"
        }
    }
"#;

// ─── Modify network — attach to router ──────────────────────────────────

#[test]
fn deserialize_modify_network_attach_router() {
    #[derive(Debug, Deserialize)]
    struct Body {
        network: ModifyNetworkRequest,
    }
    let body: Body = serde_json::from_str(MODIFY_NETWORK_ROUTER_JSON).unwrap();
    assert_eq!(
        body.network.router.as_deref(),
        Some("04c0df35-2658-4b0c-8ac7-962090f4e92a")
    );
    assert!(body.network.name.is_none());
    assert!(body.network.ip_networks.is_none());
    assert!(body.network.labels.is_none());
}

// Source: https://developers.upcloud.com/1.3/13-networks/#modify-network-details
const MODIFY_NETWORK_RESPONSE_JSON: &str = r#"
    {
        "network": {
            "name": "My private network",
            "type": "private",
            "uuid": "034c12bc-cf15-4b19-97b2-0ab4e51bb98d",
            "zone": "uk-lon1",
            "ip_networks": {
                "ip_network": [
                    {
                        "address": "172.16.0.0/22",
                        "dhcp": "yes",
                        "dhcp_default_route": "no",
                        "dhcp_dns": [
                            "172.16.0.10",
                            "172.16.1.10"
                        ],
                        "dhcp_routes": [
                            "192.168.0.0/24",
                            "192.168.100.100/32"
                        ],
                        "dhcp_routes_configuration": {
                            "effective_routes_auto_population": {
                                "enabled": "no"
                            }
                        },
                        "dhcp_effective_routes": [],
                        "family": "IPv4",
                        "gateway": "172.16.0.1"
                    }
                ]
            },
            "effective_routes": [
                {
                    "source": "network-peering",
                    "route": "172.16.10.0/24",
                    "source_resource_id": "03126dc1-a69f-4bc2-8b24-e31c22d64712"
                }
            ],
            "labels": [
                {
                    "key": "purpose",
                    "value": "databases"
                }
            ]
        }
    }
"#;

// ─── Modify network response ────────────────────────────────────────────

#[test]
fn deserialize_modify_network_response() {
    let resp: NetworkResponse = serde_json::from_str(MODIFY_NETWORK_RESPONSE_JSON).unwrap();
    let net = resp.network;
    assert_eq!(net.name, "My private network");
    assert_eq!(net.network_type, NetworkType::Private);
    assert_eq!(net.uuid, "034c12bc-cf15-4b19-97b2-0ab4e51bb98d");

    assert_eq!(net.zone, "uk-lon1");

    let ip_net = &net.ip_networks.unwrap().ip_network[0];
    assert!(ip_net.dhcp_effective_routes.as_ref().unwrap().is_empty());
    assert!(ip_net.dhcp_routes_configuration.is_some());

    assert_eq!(ip_net.address, "172.16.0.0/22");
    assert_eq!(ip_net.dhcp.as_deref(), Some("yes"));
    assert_eq!(ip_net.dhcp_default_route.as_deref(), Some("no"));
    let dhcp_dns = ip_net.dhcp_dns.as_ref().unwrap();
    assert_eq!(dhcp_dns.len(), 2);
    assert_eq!(dhcp_dns[0], "172.16.0.10");
    assert_eq!(dhcp_dns[1], "172.16.1.10");
    assert_eq!(ip_net.family, "IPv4");
    assert_eq!(ip_net.gateway.as_deref(), Some("172.16.0.1"));
    let dhcp_routes = ip_net.dhcp_routes.as_ref().unwrap();
    assert_eq!(dhcp_routes.len(), 2);
    assert_eq!(dhcp_routes[0], "192.168.0.0/24");
    assert_eq!(dhcp_routes[1], "192.168.100.100/32");

    let eff = net.effective_routes.unwrap();
    assert_eq!(eff.len(), 1);
    assert_eq!(eff[0]["source"], "network-peering");
    assert_eq!(eff[0]["route"], "172.16.10.0/24");
    assert_eq!(
        eff[0]["source_resource_id"],
        "03126dc1-a69f-4bc2-8b24-e31c22d64712"
    );

    let labels = net.labels.unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0].key, "purpose");
    assert_eq!(labels[0].value, "databases");
}

// Source: https://developers.upcloud.com/1.3/13-networks/#list-routers
const LIST_ROUTERS_JSON: &str = r#"
    {
        "routers": {
            "router": [
                {
                    "attached_networks": {
                        "network": []
                    },
                    "labels": [],
                    "name": "Example router",
                    "static_routes": [],
                    "type": "normal",
                    "uuid": "04c0df35-2658-4b0c-8ac7-962090f4e92a"
                }
            ]
        }
    }
"#;

// ─── List routers ───────────────────────────────────────────────────────

#[test]
fn deserialize_router_list() {
    let resp: RouterListResponse = serde_json::from_str(LIST_ROUTERS_JSON).unwrap();
    let routers = resp.routers.router;
    assert_eq!(routers.len(), 1);
    assert_eq!(routers[0].name, "Example router");
    assert_eq!(routers[0].uuid, "04c0df35-2658-4b0c-8ac7-962090f4e92a");
    assert!(
        routers[0]
            .attached_networks
            .as_ref()
            .unwrap()
            .network
            .is_empty()
    );
    assert!(routers[0].static_routes.is_empty());
    assert_eq!(routers[0].router_type.as_deref(), Some("normal"));
    assert!(routers[0].labels.as_ref().unwrap().is_empty());
}

// Source: https://developers.upcloud.com/1.3/13-networks/#get-router-details
const GET_ROUTER_JSON: &str = r#"
    {
        "router": {
            "attached_networks": {
                "network": []
            },
            "labels": [],
            "name": "Example router",
            "static_routes": [],
            "type": "normal",
            "uuid": "04c0df35-2658-4b0c-8ac7-962090f4e92a"
        }
    }
"#;

// ─── Get router details ─────────────────────────────────────────────────

#[test]
fn deserialize_router_detail() {
    let resp: RouterResponse = serde_json::from_str(GET_ROUTER_JSON).unwrap();
    let router = resp.router;
    assert_eq!(router.name, "Example router");
    assert_eq!(router.uuid, "04c0df35-2658-4b0c-8ac7-962090f4e92a");
    assert!(router.labels.as_ref().unwrap().is_empty());
    assert_eq!(router.router_type.as_deref(), Some("normal"));
    assert!(
        router
            .attached_networks
            .as_ref()
            .unwrap()
            .network
            .is_empty()
    );
    assert!(router.static_routes.is_empty());
}

// Source: https://developers.upcloud.com/1.3/13-networks/#create-router
const CREATE_ROUTER_REQUEST_JSON: &str = r#"
    {
        "router": {
            "name": "Example router",
            "static_routes": [
                {
                    "route": "0.0.0.0/0",
                    "nexthop": "10.0.0.100",
                    "name": "static_route_0"
                }
            ]
        }
    }
"#;

// ─── Create router request ──────────────────────────────────────────────

#[test]
fn deserialize_create_router_request() {
    #[derive(Debug, Deserialize)]
    struct Body {
        router: CreateRouterRequest,
    }
    let body: Body = serde_json::from_str(CREATE_ROUTER_REQUEST_JSON).unwrap();
    assert_eq!(body.router.name, "Example router");
    assert_eq!(body.router.static_routes.len(), 1);
    assert_eq!(body.router.static_routes[0].route, "0.0.0.0/0");
    assert_eq!(body.router.static_routes[0].nexthop, "10.0.0.100");
    assert_eq!(
        body.router.static_routes[0].name.as_deref(),
        Some("static_route_0")
    );
    assert!(body.router.labels.is_none());
    assert!(body.router.static_routes[0].route_type.is_none());
}

// Source: https://developers.upcloud.com/1.3/13-networks/#create-router
const CREATE_ROUTER_RESPONSE_JSON: &str = r#"
    {
        "router": {
            "attached_networks": {
                "network": []
            },
            "labels": [],
            "name": "Example router",
            "static_routes": [
                {
                    "name": "static_route_0",
                    "route": "0.0.0.0/0",
                    "nexthop": "10.0.0.100",
                    "type": "user"
                }
            ],
            "type": "normal",
            "uuid": "04c0df35-2658-4b0c-8ac7-962090f4e92a"
        }
    }
"#;

// ─── Create router response ─────────────────────────────────────────────

#[test]
fn deserialize_create_router_response() {
    let resp: RouterResponse = serde_json::from_str(CREATE_ROUTER_RESPONSE_JSON).unwrap();
    let router = resp.router;
    assert_eq!(router.name, "Example router");
    assert_eq!(router.static_routes.len(), 1);
    assert_eq!(
        router.static_routes[0].route_type,
        Some(RouterStaticRouteType::User)
    );
    assert_eq!(router.static_routes[0].route, "0.0.0.0/0");
    assert!(router.labels.as_ref().unwrap().is_empty());
    assert_eq!(router.uuid, "04c0df35-2658-4b0c-8ac7-962090f4e92a");
    assert_eq!(router.router_type.as_deref(), Some("normal"));
    assert!(
        router
            .attached_networks
            .as_ref()
            .unwrap()
            .network
            .is_empty()
    );
    assert_eq!(
        router.static_routes[0].name.as_deref(),
        Some("static_route_0")
    );
    assert_eq!(router.static_routes[0].nexthop, "10.0.0.100");
}

// Source: https://developers.upcloud.com/1.3/13-networks/#modify-router
const MODIFY_ROUTER_REQUEST_JSON: &str = r#"
    {
        "router": {
            "name": "Example router",
            "labels": [
                {
                    "key": "service",
                    "value": "postgres cluster connectivity"
                }
            ],
            "static_routes": []
        }
    }
"#;

// ─── Modify router request ──────────────────────────────────────────────

#[test]
fn deserialize_modify_router_request() {
    #[derive(Debug, Deserialize)]
    struct Body {
        router: ModifyRouterRequest,
    }
    let body: Body = serde_json::from_str(MODIFY_ROUTER_REQUEST_JSON).unwrap();
    assert_eq!(body.router.name.as_deref(), Some("Example router"));
    assert!(body.router.static_routes.as_ref().unwrap().is_empty());
    let labels = body.router.labels.unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0].key, "service");
    assert_eq!(labels[0].value, "postgres cluster connectivity");
}

// Source: https://developers.upcloud.com/1.3/13-networks/#modify-router
const MODIFY_ROUTER_RESPONSE_JSON: &str = r#"
    {
        "router": {
            "attached_networks": {
                "network": []
            },
            "labels": [
                {
                    "key": "service",
                    "value": "postgres cluster connectivity"
                }
            ],
            "name": "Example router",
            "static_routes": [],
            "type": "normal",
            "uuid": "04c0df35-2658-4b0c-8ac7-962090f4e92a"
        }
    }
"#;

// ─── Modify router response ─────────────────────────────────────────────

#[test]
fn deserialize_modify_router_response() {
    let resp: RouterResponse = serde_json::from_str(MODIFY_ROUTER_RESPONSE_JSON).unwrap();
    let router = resp.router;
    assert_eq!(router.name, "Example router");
    assert!(router.static_routes.is_empty());
    let labels = router.labels.unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0].key, "service");
    assert_eq!(labels[0].value, "postgres cluster connectivity");
    assert_eq!(router.uuid, "04c0df35-2658-4b0c-8ac7-962090f4e92a");
    assert_eq!(router.router_type.as_deref(), Some("normal"));
    assert!(
        router
            .attached_networks
            .as_ref()
            .unwrap()
            .network
            .is_empty()
    );
}

// Source: https://developers.upcloud.com/1.3/13-networks/#list-network-peerings
const LIST_PEERINGS_JSON: &str = r#"
    {
        "network_peerings": [
            {
                "uuid": "0f5ab9c7-959a-4bbf-9994-8322517fc359",
                "name": "my-peering-1",
                "configured_status": "active",
                "state": "active",
                "network": {
                    "uuid": "034c12bc-cf15-4b19-97b2-0ab4e51bb98d"
                },
                "peer_network": {
                    "uuid": "03e3d3ab-ac72-4e39-8264-2bc0f2e4e872"
                },
                "labels": []
            }
        ]
    }
"#;

// ─── List network peerings ──────────────────────────────────────────────

#[test]
fn deserialize_peering_list() {
    let resp: PeeringListResponse = serde_json::from_str(LIST_PEERINGS_JSON).unwrap();
    assert_eq!(resp.network_peerings.len(), 1);
    assert_eq!(resp.network_peerings[0].name, "my-peering-1");
    assert_eq!(resp.network_peerings[0].state, PeeringState::Active);
    assert_eq!(
        resp.network_peerings[0].configured_status,
        PeeringConfiguredStatus::Active
    );

    let p = &resp.network_peerings[0];
    assert_eq!(p.uuid, "0f5ab9c7-959a-4bbf-9994-8322517fc359");
    assert!(p.labels.as_ref().unwrap().is_empty());

    let net = p.network.as_ref().unwrap();
    assert_eq!(net.uuid, "034c12bc-cf15-4b19-97b2-0ab4e51bb98d");
    assert!(net.ip_networks.is_none());

    let peer = p.peer_network.as_ref().unwrap();
    assert_eq!(peer.uuid, "03e3d3ab-ac72-4e39-8264-2bc0f2e4e872");
    assert!(peer.ip_networks.is_none());
}

// Source: https://developers.upcloud.com/1.3/13-networks/#list-network-peerings
const LIST_PEERINGS_DOCS_JSON: &str = r#"
    {
        "network_peerings": {
            "network_peering": [
                {
                    "configured_status": "active",
                    "labels": [
                        {
                            "key": "env",
                            "value": "prod"
                        }
                    ],
                    "name": "Peering A->B",
                    "network": {
                        "ip_networks": {
                            "ip_network": [
                                {
                                    "address": "192.168.0.0/24",
                                    "family": "IPv4"
                                },
                                {
                                    "address": "fc02:c4f3::/64",
                                    "family": "IPv6"
                                }
                            ]
                        },
                        "uuid": "03126dc1-a69f-4bc2-8b24-e31c22d64712"
                    },
                    "peer_network": {
                        "ip_networks": {
                            "ip_network": [
                                {
                                    "address": "192.168.99.0/24",
                                    "family": "IPv4"
                                },
                                {
                                    "address": "fc02:c4f3:99::/64",
                                    "family": "IPv6"
                                }
                            ]
                        },
                        "uuid": "03585987-bf7d-4544-8e9b-5a1b4d74a333"
                    },
                    "state": "active",
                    "uuid": "0f7984bc-5d72-4aaf-b587-90e6a8f32efc"
                }
            ]
        }
    }
"#;

// ─── List peerings (docs wrapper format) ────────────────────────────────

#[test]
fn deserialize_peering_list_docs_format() {
    #[derive(Debug, Deserialize)]
    struct DocPeeringListResponse {
        network_peerings: DocPeeringWrapper,
    }
    #[derive(Debug, Deserialize)]
    struct DocPeeringWrapper {
        network_peering: Vec<NetworkPeering>,
    }
    let resp: DocPeeringListResponse = serde_json::from_str(LIST_PEERINGS_DOCS_JSON).unwrap();
    let peerings = resp.network_peerings.network_peering;
    assert_eq!(peerings.len(), 1);
    assert_eq!(peerings[0].name, "Peering A->B");
    assert_eq!(peerings[0].state, PeeringState::Active);

    let net = peerings[0].network.as_ref().unwrap();
    let ip_nets = net.ip_networks.as_ref().unwrap();
    assert_eq!(ip_nets.ip_network.len(), 2);
    assert_eq!(ip_nets.ip_network[0].address, "192.168.0.0/24");

    assert_eq!(peerings[0].uuid, "0f7984bc-5d72-4aaf-b587-90e6a8f32efc");
    assert_eq!(
        peerings[0].configured_status,
        PeeringConfiguredStatus::Active
    );

    assert_eq!(net.uuid, "03126dc1-a69f-4bc2-8b24-e31c22d64712");
    assert_eq!(ip_nets.ip_network[1].address, "fc02:c4f3::/64");
    assert_eq!(ip_nets.ip_network[1].family, "IPv6");

    let peer = peerings[0].peer_network.as_ref().unwrap();
    assert_eq!(peer.uuid, "03585987-bf7d-4544-8e9b-5a1b4d74a333");
    let peer_ips = peer.ip_networks.as_ref().unwrap();
    assert_eq!(peer_ips.ip_network.len(), 2);
    assert_eq!(peer_ips.ip_network[0].address, "192.168.99.0/24");
    assert_eq!(peer_ips.ip_network[0].family, "IPv4");
    assert_eq!(peer_ips.ip_network[1].address, "fc02:c4f3:99::/64");
    assert_eq!(peer_ips.ip_network[1].family, "IPv6");

    let labels = peerings[0].labels.as_ref().unwrap();
    assert_eq!(labels[0].key, "env");
    assert_eq!(labels[0].value, "prod");
}

// Source: https://developers.upcloud.com/1.3/13-networks/#get-network-peering-details
const GET_PEERING_DETAILS_JSON: &str = r#"
    {
        "network_peering": {
            "configured_status": "active",
            "labels": [
                {
                    "key": "env",
                    "value": "prod"
                }
            ],
            "name": "Peering A->B",
            "network": {
                "ip_networks": {
                    "ip_network": [
                        {
                            "address": "192.168.0.0/24",
                            "family": "IPv4"
                        },
                        {
                            "address": "fc02:c4f3::/64",
                            "family": "IPv6"
                        }
                    ]
                },
                "uuid": "03126dc1-a69f-4bc2-8b24-e31c22d64712"
            },
            "peer_network": {
                "ip_networks": {
                    "ip_network": [
                        {
                            "address": "192.168.99.0/24",
                            "family": "IPv4"
                        },
                        {
                            "address": "fc02:c4f3:99::/64",
                            "family": "IPv6"
                        }
                    ]
                },
                "uuid": "03585987-bf7d-4544-8e9b-5a1b4d74a333"
            },
            "state": "active",
            "uuid": "0f7984bc-5d72-4aaf-b587-90e6a8f32efc"
        }
    }
"#;

// ─── Get network peering details ────────────────────────────────────────

#[test]
fn deserialize_peering_details() {
    let resp: PeeringResponse = serde_json::from_str(GET_PEERING_DETAILS_JSON).unwrap();
    let p = resp.network_peering;
    assert_eq!(p.name, "Peering A->B");
    assert_eq!(p.state, PeeringState::Active);
    assert_eq!(p.configured_status, PeeringConfiguredStatus::Active);
    assert_eq!(p.uuid, "0f7984bc-5d72-4aaf-b587-90e6a8f32efc");

    let net = p.network.unwrap();
    assert_eq!(net.uuid, "03126dc1-a69f-4bc2-8b24-e31c22d64712");
    let ip_nets = net.ip_networks.unwrap();
    assert_eq!(ip_nets.ip_network.len(), 2);

    let labels = p.labels.as_ref().unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0].key, "env");
    assert_eq!(labels[0].value, "prod");

    assert_eq!(ip_nets.ip_network[0].address, "192.168.0.0/24");
    assert_eq!(ip_nets.ip_network[0].family, "IPv4");
    assert_eq!(ip_nets.ip_network[1].address, "fc02:c4f3::/64");
    assert_eq!(ip_nets.ip_network[1].family, "IPv6");

    let peer = p.peer_network.unwrap();
    assert_eq!(peer.uuid, "03585987-bf7d-4544-8e9b-5a1b4d74a333");
    let peer_ips = peer.ip_networks.unwrap();
    assert_eq!(peer_ips.ip_network[0].address, "192.168.99.0/24");
    assert_eq!(peer_ips.ip_network[0].family, "IPv4");
    assert_eq!(peer_ips.ip_network[1].address, "fc02:c4f3:99::/64");
    assert_eq!(peer_ips.ip_network[1].family, "IPv6");
}

// Source: https://developers.upcloud.com/1.3/13-networks/#create-network-peering
const CREATE_PEERING_REQUEST_JSON: &str = r#"
    {
        "network_peering": {
            "configured_status": "active",
            "name": "Peering A->B",
            "network": {
                "uuid": "03126dc1-a69f-4bc2-8b24-e31c22d64712"
            },
            "peer_network": {
                "uuid": "03585987-bf7d-4544-8e9b-5a1b4d74a333"
            },
            "labels": [
                {
                    "key": "env",
                    "value": "staging"
                },
                {
                    "key": "foo",
                    "value": "bar"
                }
            ]
        }
    }
"#;

// ─── Create peering request ─────────────────────────────────────────────

#[test]
fn deserialize_create_peering_request() {
    #[derive(Debug, Deserialize)]
    struct Body {
        network_peering: CreatePeeringRequest,
    }
    let body: Body = serde_json::from_str(CREATE_PEERING_REQUEST_JSON).unwrap();
    assert_eq!(body.network_peering.name, "Peering A->B");
    assert_eq!(
        body.network_peering.network.uuid,
        "03126dc1-a69f-4bc2-8b24-e31c22d64712"
    );
    assert_eq!(
        body.network_peering.peer_network.uuid,
        "03585987-bf7d-4544-8e9b-5a1b4d74a333"
    );
    assert_eq!(
        body.network_peering.configured_status,
        Some(PeeringConfiguredStatus::Active)
    );

    let labels = body.network_peering.labels.unwrap();
    assert_eq!(labels.len(), 2);
    assert_eq!(labels[0].key, "env");
    assert_eq!(labels[0].value, "staging");
    assert_eq!(labels[1].key, "foo");
    assert_eq!(labels[1].value, "bar");
}

// Source: https://developers.upcloud.com/1.3/13-networks/#create-network-peering
const CREATE_PEERING_RESPONSE_JSON: &str = r#"
    {
        "network_peering": {
            "configured_status": "active",
            "labels": [
                {
                    "key": "env",
                    "value": "prod"
                }
            ],
            "name": "Peering A->B",
            "network": {
                "ip_networks": {
                    "ip_network": [
                        {
                            "address": "192.168.0.0/24",
                            "family": "IPv4"
                        },
                        {
                            "address": "fc02:c4f3::/64",
                            "family": "IPv6"
                        }
                    ]
                },
                "uuid": "03126dc1-a69f-4bc2-8b24-e31c22d64712"
            },
            "peer_network": {
                "uuid": "03585987-bf7d-4544-8e9b-5a1b4d74a333"
            },
            "state": "pending-peer",
            "uuid": "0f7984bc-5d72-4aaf-b587-90e6a8f32efc"
        }
    }
"#;

// ─── Create peering response ────────────────────────────────────────────

#[test]
fn deserialize_create_peering_response() {
    let resp: PeeringResponse = serde_json::from_str(CREATE_PEERING_RESPONSE_JSON).unwrap();
    let p = resp.network_peering;
    assert_eq!(p.name, "Peering A->B");
    assert_eq!(p.state, PeeringState::PendingPeer);
    assert_eq!(p.uuid, "0f7984bc-5d72-4aaf-b587-90e6a8f32efc");

    assert_eq!(p.configured_status, PeeringConfiguredStatus::Active);

    let labels = p.labels.as_ref().unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0].key, "env");
    assert_eq!(labels[0].value, "prod");

    let net = p.network.unwrap();
    assert_eq!(net.uuid, "03126dc1-a69f-4bc2-8b24-e31c22d64712");
    let ip_networks = net.ip_networks.unwrap();
    assert_eq!(ip_networks.ip_network.len(), 2);
    assert_eq!(ip_networks.ip_network[0].address, "192.168.0.0/24");
    assert_eq!(ip_networks.ip_network[0].family, "IPv4");
    assert_eq!(ip_networks.ip_network[1].address, "fc02:c4f3::/64");
    assert_eq!(ip_networks.ip_network[1].family, "IPv6");

    let peer = p.peer_network.unwrap();
    assert_eq!(peer.uuid, "03585987-bf7d-4544-8e9b-5a1b4d74a333");
    assert!(peer.ip_networks.is_none());
}

// Source: https://developers.upcloud.com/1.3/13-networks/#modify-network-peering
const MODIFY_PEERING_REQUEST_JSON: &str = r#"
    {
        "network_peering": {
            "configured_status": "disabled",
            "name": "Peering A->B modified",
            "labels": [
                {
                    "key": "env",
                    "value": "staging"
                },
                {
                    "key": "foo",
                    "value": "bar"
                }
            ]
        }
    }
"#;

// ─── Modify peering request ─────────────────────────────────────────────

#[test]
fn deserialize_modify_peering_request() {
    #[derive(Debug, Deserialize)]
    struct Body {
        network_peering: ModifyPeeringRequest,
    }
    let body: Body = serde_json::from_str(MODIFY_PEERING_REQUEST_JSON).unwrap();
    assert_eq!(
        body.network_peering.name.as_deref(),
        Some("Peering A->B modified")
    );
    assert_eq!(
        body.network_peering.configured_status,
        Some(PeeringConfiguredStatus::Disabled)
    );
    let labels = body.network_peering.labels.unwrap();
    assert_eq!(labels.len(), 2);
    assert_eq!(labels[0].key, "env");
    assert_eq!(labels[0].value, "staging");
    assert_eq!(labels[1].key, "foo");
    assert_eq!(labels[1].value, "bar");
}

// Source: https://developers.upcloud.com/1.3/13-networks/#modify-network-peering
const MODIFY_PEERING_RESPONSE_JSON: &str = r#"
    {
        "network_peering": {
            "configured_status": "disabled",
            "labels": [
                {
                    "key": "env",
                    "value": "prod"
                }
            ],
            "name": "Peering A->B modified",
            "network": {
                "ip_networks": {
                    "ip_network": [
                        {
                            "address": "192.168.0.0/24",
                            "family": "IPv4"
                        },
                        {
                            "address": "fc02:c4f3::/64",
                            "family": "IPv6"
                        }
                    ]
                },
                "uuid": "03126dc1-a69f-4bc2-8b24-e31c22d64712"
            },
            "peer_network": {
                "ip_networks": {
                    "ip_network": [
                        {
                            "address": "192.168.99.0/24",
                            "family": "IPv4"
                        },
                        {
                            "address": "fc02:c4f3:99::/64",
                            "family": "IPv6"
                        }
                    ]
                },
                "uuid": "03585987-bf7d-4544-8e9b-5a1b4d74a333"
            },
            "state": "provisioning",
            "uuid": "0f7984bc-5d72-4aaf-b587-90e6a8f32efc"
        }
    }
"#;

// ─── Modify peering response ────────────────────────────────────────────

#[test]
fn deserialize_modify_peering_response() {
    let resp: PeeringResponse = serde_json::from_str(MODIFY_PEERING_RESPONSE_JSON).unwrap();
    let p = resp.network_peering;
    assert_eq!(p.name, "Peering A->B modified");
    assert_eq!(p.configured_status, PeeringConfiguredStatus::Disabled);
    assert_eq!(p.state, PeeringState::Provisioning);

    assert_eq!(p.uuid, "0f7984bc-5d72-4aaf-b587-90e6a8f32efc");

    let labels = p.labels.as_ref().unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0].key, "env");
    assert_eq!(labels[0].value, "prod");

    let net = p.network.unwrap();
    assert_eq!(net.uuid, "03126dc1-a69f-4bc2-8b24-e31c22d64712");
    let net_ips = net.ip_networks.unwrap();
    assert_eq!(net_ips.ip_network.len(), 2);
    assert_eq!(net_ips.ip_network[0].address, "192.168.0.0/24");
    assert_eq!(net_ips.ip_network[0].family, "IPv4");
    assert_eq!(net_ips.ip_network[1].address, "fc02:c4f3::/64");
    assert_eq!(net_ips.ip_network[1].family, "IPv6");

    let peer = p.peer_network.unwrap();
    assert_eq!(peer.uuid, "03585987-bf7d-4544-8e9b-5a1b4d74a333");
    let peer_ips = peer.ip_networks.unwrap();
    assert_eq!(peer_ips.ip_network.len(), 2);
    assert_eq!(peer_ips.ip_network[0].address, "192.168.99.0/24");
    assert_eq!(peer_ips.ip_network[0].family, "IPv4");
    assert_eq!(peer_ips.ip_network[1].address, "fc02:c4f3:99::/64");
    assert_eq!(peer_ips.ip_network[1].family, "IPv6");
}
