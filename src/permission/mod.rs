//! Permissions — `GET /1.3/permission`, `POST /1.3/permission/grant|revoke`

mod types;
pub use types::*;

use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::Error;

// ─── Response wrappers ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct PermissionListResponse {
    permissions: PermissionWrapper,
}

#[derive(Debug, Deserialize)]
struct PermissionWrapper {
    permission: Vec<Permission>,
}

// ─── Handler ──────────────────────────────────────────────────────────────────

pub struct PermissionHandler<'a> {
    client: &'a Client,
}

impl<'a> PermissionHandler<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Returns a list of all permissions for the current account.
    pub async fn list(&self) -> Result<Vec<Permission>, Error> {
        let resp: PermissionListResponse = self.client.get("/1.3/permission").await?;
        Ok(resp.permissions.permission)
    }

    /// Grants a permission to a subaccount.
    pub async fn grant(&self, req: GrantRequest) -> Result<(), Error> {
        #[derive(Serialize)]
        struct Body {
            permission: GrantRequest,
        }
        self.client
            .post_no_response("/1.3/permission/grant", &Body { permission: req })
            .await
    }

    /// Revokes a permission from a subaccount.
    pub async fn revoke(&self, req: RevokeRequest) -> Result<(), Error> {
        #[derive(Serialize)]
        struct Body {
            permission: RevokeRequest,
        }
        self.client
            .post_no_response("/1.3/permission/revoke", &Body { permission: req })
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PERMISSION_LIST_JSON: &str = r#"{
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
}"#;

    #[test]
    fn deserialize_permission_list() {
        let resp: PermissionListResponse =
            serde_json::from_str(PERMISSION_LIST_JSON).unwrap();
        let perms = resp.permissions.permission;
        assert_eq!(perms.len(), 7);

        // perm[0] — managed_loadbalancer
        assert_eq!(perms[0].target_identifier, "0ad9408c-8563-4abf-b862-dbde5b581123");
        assert_eq!(perms[0].target_type, TargetType::ManagedLoadbalancer);
        assert_eq!(perms[0].user, "sub_account_user1");
        assert!(perms[0].options.is_none());

        // perm[1] — managed_database
        assert_eq!(perms[1].target_identifier, "0973a187-3ede-4jze-133e-85ea3e61b5bc");
        assert_eq!(perms[1].target_type, TargetType::ManagedDatabase);
        assert_eq!(perms[1].user, "sub_account_user1");
        assert!(perms[1].options.is_none());

        // perm[2] — managed_object_storage
        assert_eq!(perms[2].target_identifier, "1203a187-3ede-4aae-883e-85ea3e69babc");
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

    // ─── Additional deserialization tests ──────────────────────────────────

    #[test]
    fn deserialize_permission_list_full() {
        let json = r#"{
  "permissions": {
    "permission": [
      {"target_identifier": "0ad9408c-8563-4abf-b862-dbde5b581123", "target_type": "managed_loadbalancer", "user": "sub_account_user1"},
      {"target_identifier": "0973a187-3ede-4jze-133e-85ea3e61b5bc", "target_type": "managed_database", "user": "sub_account_user1"},
      {"target_identifier": "1203a187-3ede-4aae-883e-85ea3e69babc", "target_type": "managed_object_storage", "user": "sub_account_user1"},
      {"options": {"storage": "yes"}, "target_identifier": "tag1", "target_type": "tag_access", "user": "sub_account_user1"},
      {"target_identifier": "*", "target_type": "router", "user": "main_account_user1"},
      {"target_identifier": "*", "target_type": "server", "user": "main_account_user1"},
      {"target_identifier": "*", "target_type": "storage", "user": "main_account_user1"},
      {"target_identifier": "*", "target_type": "managed_database", "user": "main_account_user1"},
      {"target_identifier": "*", "target_type": "managed_loadbalancer", "user": "main_account_user1"},
      {"target_identifier": "*", "target_type": "tag_access", "user": "main_account_user1"},
      {"target_identifier": "*", "target_type": "managed_object_storage", "user": "main_account_user1"},
      {"target_identifier": "*", "target_type": "network", "user": "main_account_user1"}
    ]
  }
}"#;
        let resp: PermissionListResponse = serde_json::from_str(json).unwrap();
        let perms = resp.permissions.permission;
        assert_eq!(perms.len(), 12);

        // sub_account entries
        assert_eq!(perms[0].target_identifier, "0ad9408c-8563-4abf-b862-dbde5b581123");
        assert_eq!(perms[0].target_type, TargetType::ManagedLoadbalancer);
        assert_eq!(perms[0].user, "sub_account_user1");

        assert_eq!(perms[1].target_identifier, "0973a187-3ede-4jze-133e-85ea3e61b5bc");
        assert_eq!(perms[1].target_type, TargetType::ManagedDatabase);
        assert_eq!(perms[1].user, "sub_account_user1");

        assert_eq!(perms[2].target_identifier, "1203a187-3ede-4aae-883e-85ea3e69babc");
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

    #[test]
    fn deserialize_permission_list_subaccount() {
        let json = r#"{
  "permissions": {
    "permission": [
      {"target_identifier": "0ad9408c-8563-4abf-b862-dbde5b581123", "target_type": "managed_loadbalancer", "user": "sub_account_user1"},
      {"target_identifier": "1203a187-3ede-4lwe-884v-25ea3e69babc", "target_type": "managed_object_storage", "user": "sub_account_user1"},
      {"options": {"storage": "yes"}, "target_identifier": "tag1", "target_type": "tag_access", "user": "sub_account_user1"}
    ]
  }
}"#;
        let resp: PermissionListResponse = serde_json::from_str(json).unwrap();
        let perms = resp.permissions.permission;
        assert_eq!(perms.len(), 3);

        assert_eq!(perms[0].target_identifier, "0ad9408c-8563-4abf-b862-dbde5b581123");
        assert_eq!(perms[0].target_type, TargetType::ManagedLoadbalancer);
        assert_eq!(perms[0].user, "sub_account_user1");
        assert!(perms[0].options.is_none());

        assert_eq!(perms[1].target_identifier, "1203a187-3ede-4lwe-884v-25ea3e69babc");
        assert_eq!(perms[1].target_type, TargetType::ManagedObjectStorage);
        assert_eq!(perms[1].user, "sub_account_user1");
        assert!(perms[1].options.is_none());

        assert_eq!(perms[2].target_identifier, "tag1");
        assert_eq!(perms[2].target_type, TargetType::TagAccess);
        assert_eq!(perms[2].user, "sub_account_user1");
        assert!(perms[2].options.is_some());
        assert_eq!(perms[2].options.as_ref().unwrap()["storage"], "yes");
    }

    #[test]
    fn deserialize_grant_permission_request() {
        let json = r#"{
  "permission": {
    "options": {},
    "target_identifier": "0ad9408c-8563-4abf-b862-dbde5b581123",
    "target_type": "managed_loadbalancer",
    "user": "sub_account_user1"
  }
}"#;
        let body: GrantPermissionBody = serde_json::from_str(json).unwrap();
        let perm = body.permission;
        assert_eq!(perm.target_identifier, "0ad9408c-8563-4abf-b862-dbde5b581123");
        assert_eq!(perm.target_type, TargetType::ManagedLoadbalancer);
        assert_eq!(perm.user, "sub_account_user1");
        assert!(perm.options.is_some());
    }

    #[test]
    fn deserialize_grant_permission_response() {
        let json = r#"{
  "permission": {
    "options": {},
    "target_identifier": "0ad9408c-8563-4abf-b862-dbde5b581123",
    "target_type": "managed_loadbalancer",
    "user": "sub_account_user1"
  }
}"#;
        let body: PermissionBody = serde_json::from_str(json).unwrap();
        let perm = body.permission;
        assert_eq!(perm.target_identifier, "0ad9408c-8563-4abf-b862-dbde5b581123");
        assert_eq!(perm.target_type, TargetType::ManagedLoadbalancer);
        assert_eq!(perm.user, "sub_account_user1");
        assert!(perm.options.is_some());
    }

    #[test]
    fn deserialize_revoke_permission_request() {
        let json = r#"{
  "permission": {
    "target_identifier": "0ad9408c-8563-4abf-b862-dbde5b581123",
    "target_type": "managed_loadbalancer",
    "user": "sub_account_user1"
  }
}"#;
        let body: RevokePermissionBody = serde_json::from_str(json).unwrap();
        let perm = body.permission;
        assert_eq!(perm.target_identifier, "0ad9408c-8563-4abf-b862-dbde5b581123");
        assert_eq!(perm.target_type, TargetType::ManagedLoadbalancer);
        assert_eq!(perm.user, "sub_account_user1");
    }

    #[test]
    fn deserialize_empty_permission_list() {
        let json = r#"{"permissions": {"permission": []}}"#;
        let resp: PermissionListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.permissions.permission.len(), 0);
    }
}
