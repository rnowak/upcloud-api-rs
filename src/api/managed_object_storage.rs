//! Managed Object Storage — `GET/POST/PATCH/DELETE /1.3/object-storage-2` + sub-resources.
//!
//! NOTE: The base path is `/object-storage-2` (not `/object-storage`).
//! The legacy Object Storage was at `/object-storage`.

use serde::{Deserialize, Serialize};

use crate::{client::Client, error::Error, types::common::PageParams};

// ─── Types ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MosRegionZone {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MosRegion {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zones: Option<Vec<MosRegionZone>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedObjectStorage {
    pub uuid: String,
    pub name: String,
    pub region: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<MosEndpoint>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<MosNetwork>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<MosUserSummary>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<MosUsage>,
    #[serde(default)]
    pub state_messages: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protection: Option<bool>,
    #[serde(default)]
    pub custom_domains: Vec<serde_json::Value>,
    #[serde(default)]
    pub static_websites: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MosUsage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_objects: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size_bytes: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MosEndpoint {
    pub domain_name: String,
    #[serde(rename = "type")]
    pub endpoint_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sts_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MosNetwork {
    pub family: String,
    pub name: String,
    #[serde(rename = "type")]
    pub network_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MosUserSummary {
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MosUser {
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_keys: Option<Vec<MosCredential>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<MosUserPolicy>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MosUserPolicy {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MosCredential {
    pub access_key_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MosPolicy {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MosBucket {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_objects: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MosStats {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_objects: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buckets: Option<Vec<MosBucketStats>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MosBucketStats {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objects: Option<u64>,
}

// ─── Request types ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMosServiceRequest {
    pub name: String,
    pub region: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub networks: Vec<MosNetworkInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MosNetworkInput {
    pub family: String,
    pub name: String,
    #[serde(rename = "type")]
    pub network_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyMosServiceRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMosUserRequest {
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMosPolicyRequest {
    pub name: String,
    pub document: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

// ─── Client methods ───────────────────────────────────────────────────────────

impl Client {
    // ─── Regions ──────────────────────────────────────────────────────────────

    /// Returns a list of available managed object storage regions.
    pub async fn list_mos_regions(&self) -> Result<Vec<MosRegion>, Error> {
        self.get("/1.3/object-storage-2/regions").await
    }

    /// Returns details for a specific managed object storage region.
    pub async fn get_mos_region(&self, name: &str) -> Result<MosRegion, Error> {
        self.get(&format!("/1.3/object-storage-2/regions/{name}"))
            .await
    }

    // ─── Services ─────────────────────────────────────────────────────────────

    /// Returns a list of all managed object storage services.
    pub async fn list_mos_services(&self, params: PageParams) -> Result<Vec<ManagedObjectStorage>, Error> {
        self.get_with_query("/1.3/object-storage-2", &params).await
    }

    /// Returns details for a specific managed object storage service.
    pub async fn get_mos_service(&self, uuid: &str) -> Result<ManagedObjectStorage, Error> {
        self.get(&format!("/1.3/object-storage-2/{uuid}")).await
    }

    /// Creates a new managed object storage service.
    pub async fn create_mos_service(
        &self,
        req: CreateMosServiceRequest,
    ) -> Result<ManagedObjectStorage, Error> {
        self.post("/1.3/object-storage-2", &req).await
    }

    /// Modifies a managed object storage service.
    pub async fn modify_mos_service(
        &self,
        uuid: &str,
        req: ModifyMosServiceRequest,
    ) -> Result<ManagedObjectStorage, Error> {
        self.patch(&format!("/1.3/object-storage-2/{uuid}"), &req).await
    }

    /// Deletes a managed object storage service.
    pub async fn delete_mos_service(&self, uuid: &str) -> Result<(), Error> {
        self.delete(&format!("/1.3/object-storage-2/{uuid}")).await
    }

    /// Returns usage statistics for a managed object storage service.
    pub async fn get_mos_stats(&self, uuid: &str) -> Result<MosStats, Error> {
        self.get(&format!("/1.3/object-storage-2/{uuid}/stats")).await
    }

    // ─── Users ────────────────────────────────────────────────────────────────

    /// Returns a list of users for a managed object storage service.
    pub async fn list_mos_users(&self, uuid: &str) -> Result<Vec<MosUser>, Error> {
        self.get(&format!("/1.3/object-storage-2/{uuid}/users")).await
    }

    /// Returns details for a specific user.
    pub async fn get_mos_user(&self, uuid: &str, username: &str) -> Result<MosUser, Error> {
        self.get(&format!("/1.3/object-storage-2/{uuid}/users/{username}"))
            .await
    }

    /// Creates a user for a managed object storage service.
    pub async fn create_mos_user(
        &self,
        uuid: &str,
        req: CreateMosUserRequest,
    ) -> Result<MosUser, Error> {
        self.post(&format!("/1.3/object-storage-2/{uuid}/users"), &req)
            .await
    }

    /// Deletes a user from a managed object storage service.
    pub async fn delete_mos_user(&self, uuid: &str, username: &str) -> Result<(), Error> {
        self.delete(&format!("/1.3/object-storage-2/{uuid}/users/{username}"))
            .await
    }

    // ─── User credentials ─────────────────────────────────────────────────────

    /// Returns a list of access keys for a user.
    pub async fn list_mos_user_credentials(
        &self,
        uuid: &str,
        username: &str,
    ) -> Result<Vec<MosCredential>, Error> {
        self.get(&format!(
            "/1.3/object-storage-2/{uuid}/users/{username}/access-keys"
        ))
        .await
    }

    /// Creates an access key for a user.
    pub async fn create_mos_user_credential(
        &self,
        uuid: &str,
        username: &str,
    ) -> Result<MosCredential, Error> {
        self.post_empty(&format!(
            "/1.3/object-storage-2/{uuid}/users/{username}/access-keys"
        ))
        .await
    }

    /// Deletes an access key for a user.
    pub async fn delete_mos_user_credential(
        &self,
        uuid: &str,
        username: &str,
        access_key_id: &str,
    ) -> Result<(), Error> {
        self.delete(&format!(
            "/1.3/object-storage-2/{uuid}/users/{username}/access-keys/{access_key_id}"
        ))
        .await
    }

    // ─── Policies ─────────────────────────────────────────────────────────────

    /// Returns a list of policies for a managed object storage service.
    pub async fn list_mos_policies(&self, uuid: &str) -> Result<Vec<MosPolicy>, Error> {
        self.get(&format!("/1.3/object-storage-2/{uuid}/policies"))
            .await
    }

    /// Creates a policy for a managed object storage service.
    pub async fn create_mos_policy(
        &self,
        uuid: &str,
        req: CreateMosPolicyRequest,
    ) -> Result<MosPolicy, Error> {
        self.post(&format!("/1.3/object-storage-2/{uuid}/policies"), &req)
            .await
    }

    /// Deletes a policy.
    pub async fn delete_mos_policy(&self, uuid: &str, policy_name: &str) -> Result<(), Error> {
        self.delete(&format!(
            "/1.3/object-storage-2/{uuid}/policies/{policy_name}"
        ))
        .await
    }

    /// Returns policies attached to a user.
    pub async fn list_mos_user_policies(
        &self,
        uuid: &str,
        username: &str,
    ) -> Result<Vec<MosPolicy>, Error> {
        self.get(&format!(
            "/1.3/object-storage-2/{uuid}/users/{username}/policies"
        ))
        .await
    }

    /// Attaches a policy to a user.
    pub async fn attach_mos_user_policy(
        &self,
        uuid: &str,
        username: &str,
        policy_name: &str,
    ) -> Result<(), Error> {
        #[derive(Serialize)]
        struct Body {
            name: String,
        }
        self.post_no_response(
            &format!("/1.3/object-storage-2/{uuid}/users/{username}/policies"),
            &Body {
                name: policy_name.to_string(),
            },
        )
        .await
    }

    /// Detaches a policy from a user.
    pub async fn detach_mos_user_policy(
        &self,
        uuid: &str,
        username: &str,
        policy_name: &str,
    ) -> Result<(), Error> {
        self.delete(&format!(
            "/1.3/object-storage-2/{uuid}/users/{username}/policies/{policy_name}"
        ))
        .await
    }

    // ─── Buckets ──────────────────────────────────────────────────────────────

    /// Returns a list of buckets in a managed object storage service.
    pub async fn list_mos_buckets(&self, uuid: &str) -> Result<Vec<MosBucket>, Error> {
        self.get(&format!("/1.3/object-storage-2/{uuid}/buckets"))
            .await
    }

    /// Returns CORS configurations for buckets.
    pub async fn list_mos_bucket_cors(&self, uuid: &str) -> Result<serde_json::Value, Error> {
        self.get(&format!("/1.3/object-storage-2/{uuid}/buckets/cors"))
            .await
    }

    // ─── Networks ─────────────────────────────────────────────────────────────

    /// Returns a list of networks for a managed object storage service.
    pub async fn list_mos_networks(&self, uuid: &str) -> Result<Vec<MosNetwork>, Error> {
        self.get(&format!("/1.3/object-storage-2/{uuid}/networks"))
            .await
    }

    /// Creates a network for a managed object storage service.
    pub async fn create_mos_network(
        &self,
        uuid: &str,
        req: MosNetworkInput,
    ) -> Result<MosNetwork, Error> {
        self.post(&format!("/1.3/object-storage-2/{uuid}/networks"), &req)
            .await
    }

    /// Deletes a network from a managed object storage service.
    pub async fn delete_mos_network(&self, uuid: &str, network_name: &str) -> Result<(), Error> {
        self.delete(&format!(
            "/1.3/object-storage-2/{uuid}/networks/{network_name}"
        ))
        .await
    }

    // ─── Metrics ──────────────────────────────────────────────────────────────

    /// Returns service-level metrics (total objects, total size).
    pub async fn get_mos_metrics(&self, uuid: &str) -> Result<MosUsage, Error> {
        self.get(&format!("/1.3/object-storage-2/{uuid}/metrics"))
            .await
    }
}

#[cfg(test)]
mod tests {
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
        let services: Vec<ManagedObjectStorage> =
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
        let buckets: Vec<MosBucket> = serde_json::from_str(MOS_BUCKET_LIST_JSON).unwrap();
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
        let regions: Vec<MosRegion> = serde_json::from_str(MOS_REGION_LIST_JSON).unwrap();
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
        let r: MosRegion = serde_json::from_str(MOS_REGION_DETAIL_JSON).unwrap();
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
        let req: CreateMosServiceRequest =
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
        let svc: ManagedObjectStorage =
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
        let u: MosUsage = serde_json::from_str(MOS_USAGE_JSON).unwrap();
        assert_eq!(u.total_objects, Some(1287));
        assert_eq!(u.total_size_bytes, Some(48277239));
    }

    #[test]
    fn deserialize_mos_create_bucket_response() {
        let b: MosBucket = serde_json::from_str(MOS_CREATE_BUCKET_RESPONSE_JSON).unwrap();
        assert_eq!(b.name, "my-bucket-1");
        assert_eq!(b.total_objects, Some(0));
        assert_eq!(b.total_size_bytes, Some(0));
        assert!(b.created_at.is_none());
    }

    #[test]
    fn deserialize_mos_network_list() {
        let nets: Vec<MosNetwork> = serde_json::from_str(MOS_NETWORK_LIST_JSON).unwrap();
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
        let n: MosNetwork = serde_json::from_str(MOS_NETWORK_DETAIL_JSON).unwrap();
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
        let n: MosNetworkInput =
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
        let policies: Vec<MosPolicy> = serde_json::from_str(MOS_POLICY_LIST_JSON).unwrap();
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
        let p: MosPolicy = serde_json::from_str(MOS_POLICY_DETAIL_JSON).unwrap();
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
        let users: Vec<MosUser> = serde_json::from_str(MOS_USER_LIST_JSON).unwrap();
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
        let u: MosUser = serde_json::from_str(MOS_USER_DETAIL_JSON).unwrap();
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
        let u: MosUser = serde_json::from_str(MOS_USER_CREATE_RESPONSE_JSON).unwrap();
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
        let keys: Vec<MosCredential> = serde_json::from_str(MOS_ACCESS_KEY_LIST_JSON).unwrap();
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].access_key_id, "AKIA63F41D01345BB477");
        assert_eq!(keys[0].created_at.as_deref(), Some("2023-05-07T20:52:19.705405Z"));
        assert_eq!(keys[0].last_used_at.as_deref(), Some("2023-05-07T20:52:17Z"));
        assert_eq!(keys[0].status.as_deref(), Some("Active"));
        assert!(keys[0].secret_access_key.is_none());
    }

    #[test]
    fn deserialize_mos_access_key_create_response() {
        let key: MosCredential =
            serde_json::from_str(MOS_ACCESS_KEY_CREATE_RESPONSE_JSON).unwrap();
        assert_eq!(key.access_key_id, "AKIA589142A152F5E423");
        assert_eq!(key.created_at.as_deref(), Some("2023-05-07T22:58:26.239729Z"));
        assert_eq!(key.last_used_at.as_deref(), Some("2023-05-07T22:58:26.239729Z"));
        assert_eq!(key.secret_access_key.as_deref(), Some("xbINHFALkXjFjmxhAYL8mJnODRDX91OM7cCd2+1Y"));
        assert_eq!(key.status.as_deref(), Some("Active"));
    }

    #[test]
    fn deserialize_mos_access_key_detail() {
        let key: MosCredential =
            serde_json::from_str(MOS_ACCESS_KEY_DETAIL_JSON).unwrap();
        assert_eq!(key.access_key_id, "AKIA63F41D01345BB477");
        assert_eq!(key.created_at.as_deref(), Some("2023-05-07T20:52:19.705405Z"));
        assert_eq!(key.last_used_at.as_deref(), Some("2023-05-07T20:52:17Z"));
        assert_eq!(key.status.as_deref(), Some("Active"));
        assert!(key.secret_access_key.is_none());
    }
}
