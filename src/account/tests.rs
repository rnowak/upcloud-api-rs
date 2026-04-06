use super::*;

use serde::Deserialize;

const TOKEN_LIST_JSON: &str = r#"[
  {
"id": "0c848138-2559-4562-9520-b1f22c4ae751",
"name": "token",
"created_at": "2025-03-25T13:46:19.502878Z",
"expires_at": "2025-03-25T14:16:19.502644Z",
"can_create_tokens": true,
"allowed_ip_ranges": ["1.2.3.4"],
"gui": false
  },
  {
"id": "0cc50c9d-6a81-494b-af8c-f7271cfd4d22",
"name": "token",
"created_at": "2025-03-25T14:12:06.534789Z",
"expires_at": "2025-03-25T14:42:06.534342Z",
"last_used_at": "2025-03-25T14:12:33.000692Z",
"can_create_tokens": true,
"allowed_ip_ranges": ["1.2.3.4"],
"gui": false
  }
]"#;

const TOKEN_CREATE_JSON: &str = r#"{
  "token": "ucat_01DQE3AJDEBFEKECFM558TGH2F",
  "id": "0c8bc50b-968e-4dc3-b9c2-1a4a5298b652",
  "name": "token",
  "created_at": "2025-04-02T11:24:25.10281Z",
  "expires_at": "2025-11-30T08:03:15.944005Z",
  "can_create_tokens": true,
  "allowed_ip_ranges": ["0.0.0.0/0", "::/0"],
  "gui": false
}"#;

#[test]
fn deserialize_token_list() {
    let tokens: Vec<ApiToken> = serde_json::from_str(TOKEN_LIST_JSON).unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].id, "0c848138-2559-4562-9520-b1f22c4ae751");
    assert!(tokens[0].can_create_tokens);
    assert!(!tokens[0].gui);
    assert_eq!(tokens[0].allowed_ip_ranges, vec!["1.2.3.4"]);
    assert!(tokens[1].last_used_at.is_some());
}

#[test]
fn deserialize_token_create() {
    let resp: CreateTokenResponse = serde_json::from_str(TOKEN_CREATE_JSON).unwrap();
    assert_eq!(resp.token, "ucat_01DQE3AJDEBFEKECFM558TGH2F");
    assert_eq!(resp.id, "0c8bc50b-968e-4dc3-b9c2-1a4a5298b652");
    assert!(resp.can_create_tokens);
    assert_eq!(resp.allowed_ip_ranges.len(), 2);
}

const TOKEN_DETAIL_JSON: &str = r#"{
  "id": "0c0f1d0e-ccba-48bf-a2c2-b267e8a58597",
  "name": "token",
  "created_at": "2025-04-04T08:27:27.603019Z",
  "expires_at": "2025-11-30T08:03:15.944017Z",
  "last_used_at": "2025-04-04T08:38:20.890805Z",
  "can_create_tokens": true,
  "allowed_ip_ranges": [
"0.0.0.0/0",
"::/0"
  ],
  "gui": false
}"#;

#[test]
fn deserialize_token_detail() {
    let token: ApiToken = serde_json::from_str(TOKEN_DETAIL_JSON).unwrap();
    assert_eq!(token.id, "0c0f1d0e-ccba-48bf-a2c2-b267e8a58597");
    assert!(token.last_used_at.is_some());
    assert_eq!(
        token.last_used_at.as_deref(),
        Some("2025-04-04T08:38:20.890805Z")
    );
    assert!(token.can_create_tokens);
    assert_eq!(token.allowed_ip_ranges.len(), 2);
}

const CREATE_TOKEN_REQUEST_JSON: &str = r#"{
  "name": "token",
  "expires_at": "2025-11-30T08:03:15.944Z",
  "allowed_ip_ranges": [
"0.0.0.0/0",
"::/0"
  ]
}"#;

#[test]
fn deserialize_create_token_request() {
    let req: CreateTokenRequest = serde_json::from_str(CREATE_TOKEN_REQUEST_JSON).unwrap();
    assert_eq!(req.name, "token");
    assert_eq!(req.expires_at.as_deref(), Some("2025-11-30T08:03:15.944Z"));
    let ranges = req.allowed_ip_ranges.unwrap();
    assert_eq!(ranges.len(), 2);
    assert_eq!(ranges[0], "0.0.0.0/0");
    assert_eq!(ranges[1], "::/0");
}

// ─── Account info (GET /1.3/account) ─────────────────────────────────────

const GET_ACCOUNT_INFO_JSON: &str = r#"{
  "account": {
"credits": 9972.2324,
"username": "username",
"resource_limits": {
  "cores": 200,
  "detached_floating_ips": 10,
  "load_balancers": 50,
  "managed_databases": 100,
  "managed_kubernetes": 100,
  "managed_object_storages": 100,
  "memory": 1048576,
  "network_gateways": 100,
  "network_peerings": 100,
  "networks": 100,
  "ntp_excess_gib": 20000,
  "public_ipv4": 100,
  "public_ipv6": 100,
  "routers": 100,
  "storage_hdd": 10240,
  "storage_maxiops": 10240,
  "storage_standard": 10240,
  "storage_total": 30720
}
  }
}"#;

#[test]
fn deserialize_get_account_info() {
    let resp: AccountResponse = serde_json::from_str(GET_ACCOUNT_INFO_JSON).unwrap();
    let acct = resp.account;
    assert_eq!(acct.username, "username");
    assert_eq!(acct.credits, Some(9972.2324));
    let limits = acct.resource_limits.unwrap();
    assert_eq!(limits.cores, Some(200));
    assert_eq!(limits.load_balancers, Some(50));
    assert_eq!(limits.managed_databases, Some(100));
    assert_eq!(limits.storage_total, Some(30720));
    assert_eq!(limits.ntp_excess_gib, Some(20000));
    assert_eq!(limits.routers, Some(100));
}

// ─── Account list (GET /1.3/account/list) ────────────────────────────────

const GET_ACCOUNT_LIST_JSON: &str = r#"{
  "accounts": {
"account": [
  {
    "labels": [],
    "roles": {
      "role": ["technical"]
    },
    "type": "mymain",
    "username": "test"
  },
  {
    "labels": [],
    "roles": {
      "role": ["technical"]
    },
    "type": "sub",
    "username": "my_sub_account"
  },
  {
    "labels": [
      {
        "key": "to_be_removed",
        "value": "after 2022-31-12"
      }
    ],
    "roles": {
      "role": []
    },
    "type": "sub",
    "username": "my_temp_account"
  },
  {
    "labels": [],
    "roles": {
      "role": ["billing"]
    },
    "type": "sub",
    "username": "my_billing_account"
  }
]
  }
}"#;

#[test]
fn deserialize_get_account_list() {
    let resp: AccountListResponse = serde_json::from_str(GET_ACCOUNT_LIST_JSON).unwrap();
    let accounts = resp.accounts.account;
    assert_eq!(accounts.len(), 4);

    assert_eq!(accounts[0].username, "test");
    assert_eq!(accounts[0].roles.as_ref().unwrap().role, vec!["technical"]);
    assert!(accounts[0].labels.as_ref().unwrap().is_empty());

    assert_eq!(accounts[2].username, "my_temp_account");
    let labels = accounts[2].labels.as_ref().unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0].key, "to_be_removed");

    assert_eq!(accounts[3].username, "my_billing_account");
}

// ─── Account details (GET /1.3/account/details/{username}) ───────────────

const GET_ACCOUNT_DETAILS_JSON: &str = r#"{
  "account": {
"main_account": "mymain",
"type": "sub",
"username": "my_sub_account",
"first_name": "first",
"last_name": "last",
"company": "UpCloud Ltd",
"address": "my address",
"postal_code": "00130",
"city": "Helsinki",
"state": "",
"country": "FIN",
"currency": "EUR",
"language": "fi",
"phone": "+358.31245434",
"email": "test@example.com",
"vat_number": "FI24315605",
"timezone": "UTC",
"campaigns": {
  "campaign": []
},
"roles": {
  "role": []
},
"allow_api": "yes",
"allow_gui": "no",
"enable_3rd_party_services": "yes",
"network_access": {
  "network": []
},
"server_access": {
  "server": [
    {
      "storage": "no",
      "uuid": "*"
    }
  ]
},
"storage_access": {
  "storage": [
    "*"
  ]
},
"tag_access": {
  "tag": []
},
"ip_filters": {
  "ip_filter": []
},
"labels": []
  }
}"#;

#[test]
fn deserialize_get_account_details() {
    let resp: AccountResponse = serde_json::from_str(GET_ACCOUNT_DETAILS_JSON).unwrap();
    let acct = resp.account;
    assert_eq!(acct.username, "my_sub_account");
    assert_eq!(acct.main_account.as_deref(), Some("mymain"));
    assert_eq!(acct.account_type.as_deref(), Some("sub"));
    assert_eq!(acct.first_name.as_deref(), Some("first"));
    assert_eq!(acct.timezone.as_deref(), Some("UTC"));
    assert_eq!(acct.allow_api, Some(true));
    assert_eq!(acct.allow_gui, Some(false));
    assert_eq!(acct.enable_3rd_party_services.as_deref(), Some("yes"));

    let servers = acct.server_access.unwrap();
    assert_eq!(servers.server.len(), 1);
    assert_eq!(servers.server[0].uuid, "*");
    assert_eq!(servers.server[0].storage.as_deref(), Some("no"));

    let storages = acct.storage_access.unwrap();
    assert_eq!(storages.storage, vec!["*"]);

    let ip_filters = acct.ip_filter_access.unwrap();
    assert!(ip_filters.ip_filter.is_empty());

    assert!(acct.labels.unwrap().is_empty());
}

// ─── Modify account details (PUT request body) ───────────────────────────

const MODIFY_ACCOUNT_JSON: &str = r#"{
  "account": {
"first_name": "first",
"last_name": "last",
"company": "my company name",
"address": "my address",
"postal_code": "00130",
"city": "Helsinki",
"state": "",
"country": "FIN",
"currency": "EUR",
"language": "en",
"phone": "+358.31245434",
"email": "test@example.com",
"vat_number": "my vat number",
"timezone": "Europe/Helsinki",
"roles": {
  "role": [
    "billing",
    "aux_billing",
    "technical"
  ]
},
"allow_api": "yes",
"allow_gui": "no",
"enable_3rd_party_services": "yes",
"network_access": {
  "network": []
},
"server_access": {
  "server": [
    {
      "uuid": "*",
      "storage": "no"
    }
  ]
},
"storage_access": {
  "storage": [
    "*"
  ]
},
"tag_access": {
  "tag": [
    {
      "name": "mytag",
      "storage": "yes"
    },
    {
      "name": "mytag2",
      "storage": "no"
    }
  ]
},
"ip_filters": {
  "ip_filter": []
},
"labels": [
  {
    "key": "role",
    "value": "staging"
  }
]
  }
}"#;

#[test]
fn deserialize_modify_account_request() {
    #[derive(Debug, Deserialize)]
    struct Body {
        account: ModifyAccountDetailsRequest,
    }
    let body: Body = serde_json::from_str(MODIFY_ACCOUNT_JSON).unwrap();
    let req = body.account;
    assert_eq!(req.first_name.as_deref(), Some("first"));
    assert_eq!(req.timezone.as_deref(), Some("Europe/Helsinki"));
    assert_eq!(req.allow_api, Some(true));
    assert_eq!(req.allow_gui, Some(false));

    let roles = req.roles.unwrap();
    assert_eq!(roles.role.len(), 3);

    let servers = req.server_access.unwrap();
    assert_eq!(servers.server[0].uuid, "*");

    let labels = req.labels.unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0].key, "role");
}

// ─── Add subaccount (POST request body) ──────────────────────────────────

const ADD_SUBACCOUNT_JSON: &str = r#"{
  "sub_account": {
"first_name": "first",
"last_name": "last",
"company": "my company name",
"address": "my address",
"postal_code": "00130",
"city": "Helsinki",
"state": "",
"country": "FIN",
"phone": "+358.31245434",
"email": "test@example.com",
"vat_number": "my vat number",
"timezone": "Europe/Helsinki",
"username": "myusername",
"password": "mysecr3tPassword",
"currency": "EUR",
"language": "en",
"roles": {
    "role": ["technical", "billing", "aux_billing"]
},
"allow_gui": "no",
"allow_api": "yes",
"network_access": {
    "network": []
},
"server_access": {
  "server": []
},
"storage_access": {
  "storage": []
},
"tag_access": {
  "tag": []
},
"ip_filters": {
  "ip_filter": []
},
"labels": [
  {
    "key": "department",
    "value": "it"
  }
]
  }
}"#;

#[test]
fn deserialize_add_subaccount_request() {
    #[derive(Debug, Deserialize)]
    struct Body {
        sub_account: CreateSubaccountRequest,
    }
    let body: Body = serde_json::from_str(ADD_SUBACCOUNT_JSON).unwrap();
    let req = body.sub_account;
    assert_eq!(req.username, "myusername");
    assert_eq!(req.password, "mysecr3tPassword");
    assert_eq!(req.first_name.as_deref(), Some("first"));
    assert_eq!(req.timezone.as_deref(), Some("Europe/Helsinki"));
    assert_eq!(req.allow_gui, Some(false));
    assert_eq!(req.allow_api, Some(true));

    let roles = req.roles.unwrap();
    assert_eq!(roles.role.len(), 3);

    let labels = req.labels.unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0].key, "department");
    assert_eq!(labels[0].value, "it");
}

// ─── Monthly billing summary (new endpoint) ─────────────────────────────

const BILLING_SUMMARY_JSON: &str = r#"{
  "currency": "EUR",
  "managed_databases": {
"managed_database": {
  "resources": [
    {
      "amount": 15.10223,
      "details": [
        {
          "amount": 15.10223,
          "hours": 420,
          "plan": "1x1xCPU-2GB-25GB",
          "zone": "fi-hel2"
        }
      ],
      "hours": 420,
      "resource_id": "09001a4d-b525-4ab8-835d-000000000000"
    }
  ],
  "total_amount": 15.10223
},
"total_amount": 15.10223
  },
  "managed_object_storages": {
"managed_object_storage": {
  "resources": [
    {
      "amount": 5.03384,
      "details": [
        {
          "amount": 5.03384,
          "billable_size_gib": 500,
          "hours": 420,
          "zone": "fi-hel2"
        }
      ],
      "hours": 420,
      "resource_id": "127ee42a-8304-477c-84e1-000000000000"
    }
  ],
  "total_amount": 5.03384
},
"total_amount": 5.03384
  },
  "servers": {
"server": {
  "resources": [
    {
      "amount": 16.18091,
      "details": [
        {
          "amount": 16.18091,
          "hours": 420,
          "labels": [
            {
              "key": "test",
              "value": ""
            }
          ],
          "plan": "2xCPU-4GB",
          "zone": "fi-hel2"
        }
      ],
      "hours": 420,
      "resource_id": "001c2caa-00dc-4eff-9321-000000000000"
    },
    {
      "amount": 16.10386,
      "details": [
        {
          "amount": 16.10386,
          "hours": 418,
          "labels": [
            {
              "key": "test2",
              "value": ""
            }
          ],
          "plan": "2xCPU-4GB",
          "zone": "fi-hel2"
        },
        {
          "amount": 0.9288,
          "hours": 12,
          "labels": [
            {
              "key": "test2",
              "value": ""
            }
          ],
          "plan": "4xCPU-8GB",
          "zone": "fi-hel2"
        }
      ],
      "hours": 430,
      "resource_id": "0010cf04-3608-4512-b4de-000000000000"
    }
  ],
  "total_amount": 33.21357
},
"total_amount": 33.21357
  },
  "total_amount": 53.34964
}"#;

#[test]
fn deserialize_billing_summary() {
    let resp: BillingSummaryResponse = serde_json::from_str(BILLING_SUMMARY_JSON).unwrap();
    assert_eq!(resp.currency.as_deref(), Some("EUR"));
    assert_eq!(resp.total_amount, Some(53.34964));
    assert!(resp.managed_databases.is_some());
    assert!(resp.managed_object_storages.is_some());
    assert!(resp.servers.is_some());
    assert!(resp.storages.is_none());
    assert!(resp.networks.is_none());

    // Drill into managed_databases
    let mdb = resp.managed_databases.unwrap();
    assert_eq!(mdb["total_amount"], 15.10223);
    assert_eq!(mdb["managed_database"]["total_amount"], 15.10223);
    let mdb_resources = &mdb["managed_database"]["resources"];
    assert_eq!(mdb_resources.as_array().unwrap().len(), 1);
    assert_eq!(mdb_resources[0]["amount"], 15.10223);
    assert_eq!(
        mdb_resources[0]["resource_id"],
        "09001a4d-b525-4ab8-835d-000000000000"
    );
    assert_eq!(mdb_resources[0]["hours"], 420);

    // Drill into managed_object_storages
    let mos = resp.managed_object_storages.unwrap();
    assert_eq!(mos["total_amount"], 5.03384);
    assert_eq!(
        mos["managed_object_storage"]["resources"][0]["resource_id"],
        "127ee42a-8304-477c-84e1-000000000000"
    );

    // Drill into servers
    let srv = resp.servers.unwrap();
    assert_eq!(srv["total_amount"], 33.21357);
    let srv_resources = &srv["server"]["resources"];
    assert_eq!(srv_resources.as_array().unwrap().len(), 2);
    assert_eq!(
        srv_resources[0]["resource_id"],
        "001c2caa-00dc-4eff-9321-000000000000"
    );
    assert_eq!(srv_resources[0]["amount"], 16.18091);
    assert_eq!(srv_resources[0]["hours"], 420);
    assert_eq!(
        srv_resources[1]["resource_id"],
        "0010cf04-3608-4512-b4de-000000000000"
    );
    assert_eq!(srv_resources[1]["amount"], 16.10386);
    assert_eq!(srv_resources[1]["hours"], 430);
}

// ─── Monthly billing summary deprecated ─────────────────────────────────

const DEPRECATED_BILLING_JSON: &str = r#"{
  "billing": {
"currency": "EUR",
"networks": {
  "bandwidth": {
    "total_amount": 52.136
  },
  "total_amount": 52.136
},
"managed_databases": {
  "managed_database": {
    "total_amount": 80.13127
  },
  "total_amount": 80.13127
},
"managed_object_storages": {
  "managed_object_storage": {
    "total_amount": 2.2253
  },
  "total_amount": 2.2253
},
"servers": {
  "server": {
    "total_amount": 28.05132
  },
  "total_amount": 28.05132
},
"storages": {
  "storage": {
    "total_amount": 3.6224
  },
  "total_amount": 3.6224
},
"total_amount": 166.16629
  }
}"#;

#[test]
fn deserialize_deprecated_billing_summary() {
    let resp: DeprecatedBillingWrapper = serde_json::from_str(DEPRECATED_BILLING_JSON).unwrap();
    assert_eq!(resp.billing.currency.as_deref(), Some("EUR"));
    assert_eq!(resp.billing.total_amount, Some(166.16629));
    assert!(resp.billing.networks.is_some());
    assert!(resp.billing.servers.is_some());
    assert!(resp.billing.storages.is_some());
    assert!(resp.billing.managed_databases.is_some());
    assert!(resp.billing.managed_object_storages.is_some());

    let networks = resp.billing.networks.unwrap();
    assert_eq!(networks["total_amount"], 52.136);
    assert_eq!(networks["bandwidth"]["total_amount"], 52.136);

    let mdb = resp.billing.managed_databases.unwrap();
    assert_eq!(mdb["total_amount"], 80.13127);

    let mos = resp.billing.managed_object_storages.unwrap();
    assert_eq!(mos["total_amount"], 2.2253);

    let srv = resp.billing.servers.unwrap();
    assert_eq!(srv["total_amount"], 28.05132);

    let sto = resp.billing.storages.unwrap();
    assert_eq!(sto["total_amount"], 3.6224);
}

// ─── Detailed monthly billing summary deprecated ────────────────────────

const DETAILED_BILLING_JSON: &str = r#"{
  "billing": {
"currency": "EUR",
"networks": {
  "bandwidth": {
    "resources": [
      {
        "amount": 52.136,
        "bandwidth_out": 953344,
        "bandwidth_type": "public_ipv4",
        "hours": 0,
        "resource_id": "00ce9363-9065-4c2a-9d66-3b0c762f556b"
      }
    ],
    "total_amount": 52.136
  },
  "total_amount": 52.136
},
"managed_databases": {
  "managed_database": {
    "resources": [
      {
        "amount": 15.83323,
        "hours": 114,
        "plan": "2x2xCPU-4GB-50GB",
        "resource_id": "09de34f1-c87e-48f5-9c99-2e1152b953e1"
      },
      {
        "amount": 33.29854,
        "hours": 137,
        "plan": "3x2xCPU-4GB-100GB",
        "resource_id": "09f858ad-aa13-4ab1-98d4-2366b32c0027"
      },
      {
        "amount": 30.9995,
        "hours": 744,
        "plan": "1x1xCPU-2GB-25GB",
        "resource_id": "095d8803-d6e7-4b03-add1-3938a4a32345"
      }
    ],
    "total_amount": 80.13127
  },
  "total_amount": 80.13127
},
"managed_object_storages": {
  "managed_object_storage": {
    "resources": [
      {
        "amount": 2.2253,
        "hours": 2,
        "resource_id": "1292388b-0b4a-46e7-ac58-d31c9effed64",
        "size": 500
      }
    ],
    "total_amount": 2.2253
  },
  "total_amount": 2.2253
},
"servers": {
  "server": {
    "resources": [
      {
        "amount": 7.66668,
        "firewall": 2.8,
        "hours": 553,
        "plan": "1xCPU-1GB",
        "resource_id": "008c1044-208e-4344-b03b-fc16e9e27a09"
      },
      {
        "amount": 6.16032,
        "hours": 552,
        "plan": "1xCPU-1GB",
        "resource_id": "001b9e08-362f-4004-8318-43d0e61358f6"
      },
      {
        "amount": 14.22432,
        "hours": 552,
        "plan": "1xCPU-1GB",
        "resource_id": "00ce9363-9065-4c2a-9d66-3b0c762f556b"
      }
    ],
    "total_amount": 28.05132
  },
  "total_amount": 28.05132
},
"storages": {
  "storage": {
    "resources": [
      {
        "amount": 3.4224,
        "hours": 552,
        "resource_id": "01af6d71-43d4-433c-8342-0c9bc4068dda",
        "size": 20
      }
    ],
    "template": [
      {
        "amount": 0.1000,
        "resource_id": "01bfe607-dbac-44a3-9143-313d9c285e59",
        "size": 10
      }
    ],
    "backup": [
      {
        "amount": 0.1000,
        "resource_id": "01bfe607-76ac-42a3-914a-237dcc285ab2",
        "size": 10
      }
    ],
    "total_amount": 3.6224
  },
  "total_amount": 3.6224
},
"total_amount": 166.16629
  }
}"#;

#[test]
fn deserialize_detailed_billing_summary() {
    let resp: DeprecatedBillingWrapper = serde_json::from_str(DETAILED_BILLING_JSON).unwrap();
    assert_eq!(resp.billing.currency.as_deref(), Some("EUR"));
    assert_eq!(resp.billing.total_amount, Some(166.16629));
    assert!(resp.billing.networks.is_some());
    assert!(resp.billing.managed_databases.is_some());
    assert!(resp.billing.managed_object_storages.is_some());
    assert!(resp.billing.servers.is_some());
    assert!(resp.billing.storages.is_some());

    // Networks with bandwidth resources
    let networks = resp.billing.networks.as_ref().unwrap();
    assert_eq!(networks["total_amount"], 52.136);
    let bw_resources = &networks["bandwidth"]["resources"];
    assert_eq!(bw_resources.as_array().unwrap().len(), 1);
    assert_eq!(
        bw_resources[0]["resource_id"],
        "00ce9363-9065-4c2a-9d66-3b0c762f556b"
    );
    assert_eq!(bw_resources[0]["amount"], 52.136);
    assert_eq!(bw_resources[0]["bandwidth_type"], "public_ipv4");

    // Managed databases with multiple resources
    let mdb = resp.billing.managed_databases.as_ref().unwrap();
    assert_eq!(mdb["total_amount"], 80.13127);
    let mdb_resources = &mdb["managed_database"]["resources"];
    assert_eq!(mdb_resources.as_array().unwrap().len(), 3);
    assert_eq!(
        mdb_resources[0]["resource_id"],
        "09de34f1-c87e-48f5-9c99-2e1152b953e1"
    );
    assert_eq!(mdb_resources[0]["amount"], 15.83323);
    assert_eq!(mdb_resources[0]["plan"], "2x2xCPU-4GB-50GB");
    assert_eq!(
        mdb_resources[2]["resource_id"],
        "095d8803-d6e7-4b03-add1-3938a4a32345"
    );
    assert_eq!(mdb_resources[2]["hours"], 744);

    // Managed object storages
    let mos = resp.billing.managed_object_storages.as_ref().unwrap();
    assert_eq!(mos["total_amount"], 2.2253);
    assert_eq!(
        mos["managed_object_storage"]["resources"][0]["resource_id"],
        "1292388b-0b4a-46e7-ac58-d31c9effed64"
    );

    // Servers
    let srv = resp.billing.servers.as_ref().unwrap();
    assert_eq!(srv["total_amount"], 28.05132);
    let srv_resources = &srv["server"]["resources"];
    assert_eq!(srv_resources.as_array().unwrap().len(), 3);
    assert_eq!(
        srv_resources[0]["resource_id"],
        "008c1044-208e-4344-b03b-fc16e9e27a09"
    );
    assert_eq!(srv_resources[0]["plan"], "1xCPU-1GB");
    assert_eq!(srv_resources[0]["firewall"], 2.8);

    // Storages with storage, template, and backup
    let sto = resp.billing.storages.as_ref().unwrap();
    assert_eq!(sto["total_amount"], 3.6224);
    assert_eq!(sto["storage"]["resources"].as_array().unwrap().len(), 1);
    assert_eq!(
        sto["storage"]["resources"][0]["resource_id"],
        "01af6d71-43d4-433c-8342-0c9bc4068dda"
    );
    assert_eq!(sto["storage"]["template"].as_array().unwrap().len(), 1);
    assert_eq!(sto["storage"]["backup"].as_array().unwrap().len(), 1);
}

// ─── Resource billing summary ───────────────────────────────────────────

const RESOURCE_BILLING_JSON: &str = r#"{
  "billing": {
"daily_sums": {
  "2019-12-02": 3.33684,
  "2019-12-03": 52.77344,
  "2019-12-04": 1.07424,
  "2019-12-05": 1.07424,
  "2019-12-06": 1.07424,
  "2019-12-07": 1.07424,
  "2019-12-08": 1.07424,
  "2019-12-09": 1.07424,
  "2019-12-12": 3.22272,
  "2019-12-13": 0.58188
},
"details": {
  "hours": 792,
  "plan": "1xCPU-1GB",
  "resource_id": "00ce9363-9065-4c2a-9d66-3b0c762f556b",
  "title": "My Glorious Server",
  "type": "server",
  "zone": "fi-hel1"
},
"total_amount": 66.36032
  }
}"#;

#[test]
fn deserialize_resource_billing_summary() {
    let resp: ResourceBillingWrapper = serde_json::from_str(RESOURCE_BILLING_JSON).unwrap();
    assert_eq!(resp.billing.total_amount, Some(66.36032));
    assert!(resp.billing.daily_sums.is_some());

    // Verify daily_sums content
    let sums = resp.billing.daily_sums.as_ref().unwrap();
    assert_eq!(sums["2019-12-02"], 3.33684);
    assert_eq!(sums["2019-12-03"], 52.77344);
    assert_eq!(sums["2019-12-13"], 0.58188);

    let details = resp.billing.details.unwrap();
    assert_eq!(details.hours, Some(792));
    assert_eq!(details.plan.as_deref(), Some("1xCPU-1GB"));
    assert_eq!(
        details.resource_id.as_deref(),
        Some("00ce9363-9065-4c2a-9d66-3b0c762f556b")
    );
    assert_eq!(details.title.as_deref(), Some("My Glorious Server"));
    assert_eq!(details.resource_type.as_deref(), Some("server"));
    assert_eq!(details.zone.as_deref(), Some("fi-hel1"));
}

// ─── Network transfer statistics ────────────────────────────────────────

const NETWORK_TRANSFER_STATS_JSON: &str = r#"{
  "stats": {
"stat": [
  {
    "accumulated_quota_bytes": 6352371508013,
    "projected_monthly_quota_bytes": 6362243967561,
    "quota_increase_bytes": 9872459548,
    "sent_bytes": 1254,
    "start_time": "2020-09-30T22:00:00Z",
    "total_sent_bytes": 225149331390
  },
  {
    "accumulated_quota_bytes": 6362243967561,
    "projected_monthly_quota_bytes": 6362243967561,
    "quota_increase_bytes": 9872459548,
    "sent_bytes": 1254,
    "start_time": "2020-09-30T23:00:00Z",
    "total_sent_bytes": 225149332644
  },
  {
    "accumulated_quota_bytes": 9872459548,
    "projected_monthly_quota_bytes": 7345109903712,
    "quota_increase_bytes": 9872459548,
    "sent_bytes": 906,
    "start_time": "2020-10-01T00:00:00Z",
    "total_sent_bytes": 906
  }
]
  }
}"#;

#[test]
fn deserialize_network_transfer_stats() {
    let resp: NetworkTransferStatsResponse =
        serde_json::from_str(NETWORK_TRANSFER_STATS_JSON).unwrap();
    let stats = &resp.stats.stat;
    assert_eq!(stats.len(), 3);

    // Stat 0
    assert_eq!(stats[0].accumulated_quota_bytes, Some(6352371508013));
    assert_eq!(stats[0].projected_monthly_quota_bytes, Some(6362243967561));
    assert_eq!(stats[0].quota_increase_bytes, Some(9872459548));
    assert_eq!(stats[0].sent_bytes, Some(1254));
    assert_eq!(stats[0].start_time.as_deref(), Some("2020-09-30T22:00:00Z"));
    assert_eq!(stats[0].total_sent_bytes, Some(225149331390));

    // Stat 1
    assert_eq!(stats[1].accumulated_quota_bytes, Some(6362243967561));
    assert_eq!(stats[1].projected_monthly_quota_bytes, Some(6362243967561));
    assert_eq!(stats[1].quota_increase_bytes, Some(9872459548));
    assert_eq!(stats[1].sent_bytes, Some(1254));
    assert_eq!(stats[1].start_time.as_deref(), Some("2020-09-30T23:00:00Z"));
    assert_eq!(stats[1].total_sent_bytes, Some(225149332644));

    // Stat 2
    assert_eq!(stats[2].accumulated_quota_bytes, Some(9872459548));
    assert_eq!(stats[2].projected_monthly_quota_bytes, Some(7345109903712));
    assert_eq!(stats[2].quota_increase_bytes, Some(9872459548));
    assert_eq!(stats[2].sent_bytes, Some(906));
    assert_eq!(stats[2].start_time.as_deref(), Some("2020-10-01T00:00:00Z"));
    assert_eq!(stats[2].total_sent_bytes, Some(906));
}

// ─── Resource transfer statistics ───────────────────────────────────────

const RESOURCE_TRANSFER_STATS_JSON: &str = r#"{
  "stats": {
"stat": [
  {
    "resource_id": "00777436-a6f8-43af-9daf-89e8e6833a7a",
    "sent_bytes": 10094,
    "service": "server_public",
    "start_time": "2020-09-07T00:00:00Z",
    "zone": "de-fra1"
  },
  {
    "resource_id": "00777436-a6f8-43af-9daf-89e8e6833a7a",
    "sent_bytes": 35174050,
    "service": "server_public",
    "start_time": "2020-09-08T00:00:00Z",
    "zone": "de-fra1"
  },
  {
    "resource_id": "00777436-a6f8-43af-9daf-89e8e6833a7a",
    "sent_bytes": 32790622,
    "service": "server_public",
    "start_time": "2020-09-09T00:00:00Z",
    "zone": "de-fra1"
  }
]
  }
}"#;

#[test]
fn deserialize_resource_transfer_stats() {
    #[derive(Debug, Deserialize)]
    struct StatsResponse {
        stats: StatsWrapper,
    }
    #[derive(Debug, Deserialize)]
    struct StatsWrapper {
        stat: Vec<ResourceTransferStat>,
    }
    let resp: StatsResponse = serde_json::from_str(RESOURCE_TRANSFER_STATS_JSON).unwrap();
    assert_eq!(resp.stats.stat.len(), 3);

    // Stat 0
    assert_eq!(
        resp.stats.stat[0].resource_id.as_deref(),
        Some("00777436-a6f8-43af-9daf-89e8e6833a7a")
    );
    assert_eq!(resp.stats.stat[0].sent_bytes, Some(10094));
    assert_eq!(resp.stats.stat[0].service.as_deref(), Some("server_public"));
    assert_eq!(
        resp.stats.stat[0].start_time.as_deref(),
        Some("2020-09-07T00:00:00Z")
    );
    assert_eq!(resp.stats.stat[0].zone.as_deref(), Some("de-fra1"));

    // Stat 1
    assert_eq!(
        resp.stats.stat[1].resource_id.as_deref(),
        Some("00777436-a6f8-43af-9daf-89e8e6833a7a")
    );
    assert_eq!(resp.stats.stat[1].sent_bytes, Some(35174050));
    assert_eq!(resp.stats.stat[1].service.as_deref(), Some("server_public"));
    assert_eq!(
        resp.stats.stat[1].start_time.as_deref(),
        Some("2020-09-08T00:00:00Z")
    );
    assert_eq!(resp.stats.stat[1].zone.as_deref(), Some("de-fra1"));

    // Stat 2
    assert_eq!(
        resp.stats.stat[2].resource_id.as_deref(),
        Some("00777436-a6f8-43af-9daf-89e8e6833a7a")
    );
    assert_eq!(resp.stats.stat[2].sent_bytes, Some(32790622));
    assert_eq!(resp.stats.stat[2].service.as_deref(), Some("server_public"));
    assert_eq!(
        resp.stats.stat[2].start_time.as_deref(),
        Some("2020-09-09T00:00:00Z")
    );
    assert_eq!(resp.stats.stat[2].zone.as_deref(), Some("de-fra1"));
}

// ─── Current network usage ──────────────────────────────────────────────

const CURRENT_NETWORK_USAGE_JSON: &str = r#"{
  "stats": {
"current_network_usage": {
  "accumulated_quota_bytes": 3546328504778,
  "hourly_quota_increase_bytes": 10577635230,
  "projected_monthly_quota_bytes": 7777382596778,
  "total_sent_bytes": 13682943146,
  "updated": "2020-10-15T07:00:00Z"
}
  }
}"#;

#[test]
fn deserialize_current_network_usage() {
    let resp: NetworkTransferStatsResponse =
        serde_json::from_str(CURRENT_NETWORK_USAGE_JSON).unwrap();
    let usage = resp.stats.current_network_usage.unwrap();
    assert_eq!(usage.accumulated_quota_bytes, Some(3546328504778));
    assert_eq!(usage.hourly_quota_increase_bytes, Some(10577635230));
    assert_eq!(usage.projected_monthly_quota_bytes, Some(7777382596778));
    assert_eq!(usage.total_sent_bytes, Some(13682943146));
    assert_eq!(usage.updated.as_deref(), Some("2020-10-15T07:00:00Z"));
}
