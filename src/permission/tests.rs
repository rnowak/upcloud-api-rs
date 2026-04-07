use super::*;
// Source: https://developers.upcloud.com/1.3/18-permissions/#get-permissions
const PERMISSION_LIST_JSON: &str = r#"
    {
        "permissions": {
            "permission": [
                {
                    "target_identifier": "0ad9408c-8563-4abf-b862-dbde5b581123",
                    "target_type": "managed_loadbalancer",
                    "user": "sub_account_user1"
                },
                {
                    "target_identifier": "0973a187-3ede-4jze-133e-85ea3e61b5bc",
                    "target_type": "managed_database",
                    "user": "sub_account_user1"
                },
                {
                    "target_identifier": "1203a187-3ede-4aae-883e-85ea3e69babc",
                    "target_type": "managed_object_storage",
                    "user": "sub_account_user1"
                },
                {
                    "options": {
                        "storage": "yes"
                    },
                    "target_identifier": "tag1",
                    "target_type": "tag_access",
                    "user": "sub_account_user1"
                },
                {
                    "target_identifier": "*",
                    "target_type": "server",
                    "user": "main_account_user1"
                },
                {
                    "target_identifier": "*",
                    "target_type": "storage",
                    "user": "main_account_user1"
                },
                {
                    "target_identifier": "*",
                    "target_type": "network",
                    "user": "main_account_user1"
                }
            ]
        }
    }
"#;

#[test]
fn deserialize_permission_list() {
    let resp: PermissionListResponse = serde_json::from_str(PERMISSION_LIST_JSON).unwrap();
    let perms = resp.permissions.permission;
    assert_eq!(perms.len(), 7);

    // perm[0] — managed_loadbalancer
    assert_eq!(
        perms[0].target_identifier,
        "0ad9408c-8563-4abf-b862-dbde5b581123"
    );
    assert_eq!(perms[0].target_type, TargetType::ManagedLoadbalancer);
    assert_eq!(perms[0].user, "sub_account_user1");
    assert!(perms[0].options.is_none());

    // perm[1] — managed_database
    assert_eq!(
        perms[1].target_identifier,
        "0973a187-3ede-4jze-133e-85ea3e61b5bc"
    );
    assert_eq!(perms[1].target_type, TargetType::ManagedDatabase);
    assert_eq!(perms[1].user, "sub_account_user1");
    assert!(perms[1].options.is_none());

    // perm[2] — managed_object_storage
    assert_eq!(
        perms[2].target_identifier,
        "1203a187-3ede-4aae-883e-85ea3e69babc"
    );
    assert_eq!(perms[2].target_type, TargetType::ManagedObjectStorage);
    assert_eq!(perms[2].user, "sub_account_user1");
    assert!(perms[2].options.is_none());

    // perm[3] — tag_access with options
    assert_eq!(perms[3].target_identifier, "tag1");
    assert_eq!(perms[3].target_type, TargetType::TagAccess);
    assert_eq!(perms[3].user, "sub_account_user1");
    assert!(perms[3].options.is_some());
    assert_eq!(perms[3].options.as_ref().unwrap()["storage"], "yes");

    // perm[4] — server wildcard
    assert_eq!(perms[4].target_identifier, "*");
    assert_eq!(perms[4].target_type, TargetType::Server);
    assert_eq!(perms[4].user, "main_account_user1");
    assert!(perms[4].options.is_none());

    // perm[5] — storage wildcard
    assert_eq!(perms[5].target_identifier, "*");
    assert_eq!(perms[5].target_type, TargetType::Storage);
    assert_eq!(perms[5].user, "main_account_user1");
    assert!(perms[5].options.is_none());

    // perm[6] — network wildcard
    assert_eq!(perms[6].target_identifier, "*");
    assert_eq!(perms[6].target_type, TargetType::Network);
    assert_eq!(perms[6].user, "main_account_user1");
    assert!(perms[6].options.is_none());
}

// ─── Wrapper structs for request/response body deserialization ─────────

#[derive(Debug, Deserialize)]
struct PermissionBody {
    permission: Permission,
}

#[derive(Debug, Deserialize)]
struct GrantPermissionBody {
    permission: GrantRequest,
}

#[derive(Debug, Deserialize)]
struct RevokePermissionBody {
    permission: RevokeRequest,
}

// Source: https://developers.upcloud.com/1.3/18-permissions/#get-permissions
const PERMISSION_LIST_FULL_JSON: &str = r#"
    {
        "permissions": {
            "permission": [
                {
                    "target_identifier": "0ad9408c-8563-4abf-b862-dbde5b581123",
                    "target_type": "managed_loadbalancer",
                    "user": "sub_account_user1"
                },
                {
                    "target_identifier": "0973a187-3ede-4jze-133e-85ea3e61b5bc",
                    "target_type": "managed_database",
                    "user": "sub_account_user1"
                },
                {
                    "target_identifier": "1203a187-3ede-4aae-883e-85ea3e69babc",
                    "target_type": "managed_object_storage",
                    "user": "sub_account_user1"
                },
                {
                    "options": {
                        "storage": "yes"
                    },
                    "target_identifier": "tag1",
                    "target_type": "tag_access",
                    "user": "sub_account_user1"
                },
                {
                    "target_identifier": "*",
                    "target_type": "router",
                    "user": "main_account_user1"
                },
                {
                    "target_identifier": "*",
                    "target_type": "server",
                    "user": "main_account_user1"
                },
                {
                    "target_identifier": "*",
                    "target_type": "storage",
                    "user": "main_account_user1"
                },
                {
                    "target_identifier": "*",
                    "target_type": "managed_database",
                    "user": "main_account_user1"
                },
                {
                    "target_identifier": "*",
                    "target_type": "managed_loadbalancer",
                    "user": "main_account_user1"
                },
                {
                    "target_identifier": "*",
                    "target_type": "tag_access",
                    "user": "main_account_user1"
                },
                {
                    "target_identifier": "*",
                    "target_type": "managed_object_storage",
                    "user": "main_account_user1"
                },
                {
                    "target_identifier": "*",
                    "target_type": "network",
                    "user": "main_account_user1"
                }
            ]
        }
    }
"#;

// ─── Additional deserialization tests ──────────────────────────────────

#[test]
fn deserialize_permission_list_full() {
    let resp: PermissionListResponse = serde_json::from_str(PERMISSION_LIST_FULL_JSON).unwrap();
    let perms = resp.permissions.permission;
    assert_eq!(perms.len(), 12);

    // sub_account entries
    assert_eq!(
        perms[0].target_identifier,
        "0ad9408c-8563-4abf-b862-dbde5b581123"
    );
    assert_eq!(perms[0].target_type, TargetType::ManagedLoadbalancer);
    assert_eq!(perms[0].user, "sub_account_user1");

    assert_eq!(
        perms[1].target_identifier,
        "0973a187-3ede-4jze-133e-85ea3e61b5bc"
    );
    assert_eq!(perms[1].target_type, TargetType::ManagedDatabase);
    assert_eq!(perms[1].user, "sub_account_user1");

    assert_eq!(
        perms[2].target_identifier,
        "1203a187-3ede-4aae-883e-85ea3e69babc"
    );
    assert_eq!(perms[2].target_type, TargetType::ManagedObjectStorage);
    assert_eq!(perms[2].user, "sub_account_user1");

    assert_eq!(perms[3].target_identifier, "tag1");
    assert_eq!(perms[3].target_type, TargetType::TagAccess);
    assert_eq!(perms[3].user, "sub_account_user1");
    assert!(perms[3].options.is_some());
    assert_eq!(perms[3].options.as_ref().unwrap()["storage"], "yes");

    // main_account wildcard entries
    assert_eq!(perms[4].target_type, TargetType::Router);
    assert_eq!(perms[4].target_identifier, "*");
    assert_eq!(perms[4].user, "main_account_user1");

    assert_eq!(perms[5].target_type, TargetType::Server);
    assert_eq!(perms[5].target_identifier, "*");
    assert_eq!(perms[5].user, "main_account_user1");

    assert_eq!(perms[6].target_type, TargetType::Storage);
    assert_eq!(perms[6].target_identifier, "*");
    assert_eq!(perms[6].user, "main_account_user1");

    assert_eq!(perms[7].target_type, TargetType::ManagedDatabase);
    assert_eq!(perms[7].target_identifier, "*");
    assert_eq!(perms[7].user, "main_account_user1");

    assert_eq!(perms[8].target_type, TargetType::ManagedLoadbalancer);
    assert_eq!(perms[8].target_identifier, "*");
    assert_eq!(perms[8].user, "main_account_user1");

    assert_eq!(perms[9].target_type, TargetType::TagAccess);
    assert_eq!(perms[9].target_identifier, "*");
    assert_eq!(perms[9].user, "main_account_user1");

    assert_eq!(perms[10].target_type, TargetType::ManagedObjectStorage);
    assert_eq!(perms[10].target_identifier, "*");
    assert_eq!(perms[10].user, "main_account_user1");

    assert_eq!(perms[11].target_type, TargetType::Network);
    assert_eq!(perms[11].target_identifier, "*");
    assert_eq!(perms[11].user, "main_account_user1");
}
// Source: https://developers.upcloud.com/1.3/18-permissions/#get-permissions
const PERMISSION_LIST_SUBACCOUNT_JSON: &str = r#"
    {
        "permissions": {
            "permission": [
                {
                    "target_identifier": "0ad9408c-8563-4abf-b862-dbde5b581123",
                    "target_type": "managed_loadbalancer",
                    "user": "sub_account_user1"
                },
                {
                    "target_identifier": "1203a187-3ede-4lwe-884v-25ea3e69babc",
                    "target_type": "managed_object_storage",
                    "user": "sub_account_user1"
                },
                {
                    "options": {
                        "storage": "yes"
                    },
                    "target_identifier": "tag1",
                    "target_type": "tag_access",
                    "user": "sub_account_user1"
                }
            ]
        }
    }
"#;

#[test]
fn deserialize_permission_list_subaccount() {
    let resp: PermissionListResponse =
        serde_json::from_str(PERMISSION_LIST_SUBACCOUNT_JSON).unwrap();
    let perms = resp.permissions.permission;
    assert_eq!(perms.len(), 3);

    assert_eq!(
        perms[0].target_identifier,
        "0ad9408c-8563-4abf-b862-dbde5b581123"
    );
    assert_eq!(perms[0].target_type, TargetType::ManagedLoadbalancer);
    assert_eq!(perms[0].user, "sub_account_user1");
    assert!(perms[0].options.is_none());

    assert_eq!(
        perms[1].target_identifier,
        "1203a187-3ede-4lwe-884v-25ea3e69babc"
    );
    assert_eq!(perms[1].target_type, TargetType::ManagedObjectStorage);
    assert_eq!(perms[1].user, "sub_account_user1");
    assert!(perms[1].options.is_none());

    assert_eq!(perms[2].target_identifier, "tag1");
    assert_eq!(perms[2].target_type, TargetType::TagAccess);
    assert_eq!(perms[2].user, "sub_account_user1");
    assert!(perms[2].options.is_some());
    assert_eq!(perms[2].options.as_ref().unwrap()["storage"], "yes");
}
// Source: https://developers.upcloud.com/1.3/18-permissions/#grant-permission
const GRANT_PERMISSION_REQUEST_JSON: &str = r#"
    {
        "permission": {
            "options": {},
            "target_identifier": "0ad9408c-8563-4abf-b862-dbde5b581123",
            "target_type": "managed_loadbalancer",
            "user": "sub_account_user1"
        }
    }
"#;

#[test]
fn deserialize_grant_permission_request() {
    let body: GrantPermissionBody = serde_json::from_str(GRANT_PERMISSION_REQUEST_JSON).unwrap();
    let perm = body.permission;
    assert_eq!(
        perm.target_identifier,
        "0ad9408c-8563-4abf-b862-dbde5b581123"
    );
    assert_eq!(perm.target_type, TargetType::ManagedLoadbalancer);
    assert_eq!(perm.user, "sub_account_user1");
    assert!(perm.options.is_some());
}
// Source: https://developers.upcloud.com/1.3/18-permissions/#grant-permission
const GRANT_PERMISSION_RESPONSE_JSON: &str = r#"
    {
        "permission": {
            "options": {},
            "target_identifier": "0ad9408c-8563-4abf-b862-dbde5b581123",
            "target_type": "managed_loadbalancer",
            "user": "sub_account_user1"
        }
    }
"#;

#[test]
fn deserialize_grant_permission_response() {
    let body: PermissionBody = serde_json::from_str(GRANT_PERMISSION_RESPONSE_JSON).unwrap();
    let perm = body.permission;
    assert_eq!(
        perm.target_identifier,
        "0ad9408c-8563-4abf-b862-dbde5b581123"
    );
    assert_eq!(perm.target_type, TargetType::ManagedLoadbalancer);
    assert_eq!(perm.user, "sub_account_user1");
    assert!(perm.options.is_some());
}
// Source: https://developers.upcloud.com/1.3/18-permissions/#revoke-permission
const REVOKE_PERMISSION_REQUEST_JSON: &str = r#"
    {
        "permission": {
            "target_identifier": "0ad9408c-8563-4abf-b862-dbde5b581123",
            "target_type": "managed_loadbalancer",
            "user": "sub_account_user1"
        }
    }
"#;

#[test]
fn deserialize_revoke_permission_request() {
    let body: RevokePermissionBody = serde_json::from_str(REVOKE_PERMISSION_REQUEST_JSON).unwrap();
    let perm = body.permission;
    assert_eq!(
        perm.target_identifier,
        "0ad9408c-8563-4abf-b862-dbde5b581123"
    );
    assert_eq!(perm.target_type, TargetType::ManagedLoadbalancer);
    assert_eq!(perm.user, "sub_account_user1");
}
// Source: https://developers.upcloud.com/1.3/18-permissions/#get-permissions
const EMPTY_PERMISSION_LIST_JSON: &str = r#"
    {
        "permissions": {
            "permission": []
        }
    }
"#;

#[test]
fn deserialize_empty_permission_list() {
    let resp: PermissionListResponse = serde_json::from_str(EMPTY_PERMISSION_LIST_JSON).unwrap();
    assert_eq!(resp.permissions.permission.len(), 0);
}
