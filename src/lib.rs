//! # UpCloud API SDK
//!
//! An idiomatic Rust SDK for the [UpCloud API v1.3](https://developers.upcloud.com/1.3/).
//!
//! ## Authentication
//!
//! ```rust,no_run
//! use upcloud_sdk::Client;
//!
//! // Bearer token (recommended)
//! let client = Client::with_bearer_token("ucat_...").unwrap();
//!
//! // Basic auth (legacy)
//! let client = Client::with_basic_auth("username", "password").unwrap();
//! ```
//!
//! ## Usage
//!
//! ```rust,no_run
//! # use upcloud_sdk::Client;
//! # async fn example() -> Result<(), upcloud_sdk::Error> {
//! let client = Client::with_bearer_token("ucat_...").unwrap();
//!
//! let servers = client.list_servers().await?;
//! let zones = client.list_zones().await?;
//! # Ok(())
//! # }
//! ```

pub mod api;
pub mod client;
pub mod config;
pub mod error;
pub mod types;

pub use client::{Client, ClientBuilder};
pub use config::{AuthMode, ClientConfig};
pub use error::{ApiError, Error};

/// Convenience re-exports of the most commonly used types.
pub mod prelude {
    pub use crate::client::{Client, ClientBuilder};
    pub use crate::config::AuthMode;
    pub use crate::error::{ApiError, Error};
    pub use crate::types::common::{Label, Labels, PageParams, YesNo};

    // Account
    pub use crate::api::account::{
        Account, AccountEntry, ApiToken, BillingSummaryResponse, CreateSubaccountRequest,
        CreateTokenRequest, CreateTokenResponse, CurrentNetworkUsage,
        DeprecatedBillingWrapper, NetworkTransferStatsResponse, ResourceBillingWrapper,
        ResourceTransferStat, ServerAccessEntry, UpdateAccountDetailsRequest,
    };

    // Pricing / zones / plans
    pub use crate::api::pricing::PriceZone;
    pub use crate::api::plans::{Plan, ServerSize};
    pub use crate::api::zones::Zone;

    // Servers
    pub use crate::api::servers::{
        CreateServerGroupRequest, CreateServerRequest, ModifyServerGroupRequest,
        ModifyServerRequest, RebuildServerRequest, RestartServerRequest, Server, ServerGroup,
        StopServerRequest,
    };

    // Storages
    pub use crate::api::storages::{
        BackupRule, CloneStorageRequest, CreateBackupRequest, CreateStorageRequest,
        ModifyStorageRequest, Storage,
    };

    // IP addresses
    pub use crate::api::ip_addresses::{
        AssignIpAddressRequest, IpAddress, IpAddressFamily, ModifyIpAddressRequest,
    };

    // Firewall
    pub use crate::api::firewall::{
        CreateFirewallRuleRequest, FirewallAction, FirewallDirection, FirewallProtocol,
        FirewallRule,
    };

    // Networks
    pub use crate::api::networks::{
        CreateNetworkRequest, CreatePeeringRequest, CreateRouterRequest, ModifyNetworkRequest,
        ModifyPeeringRequest, ModifyRouterRequest, Network, NetworkPeering, Router,
    };

    // Tags
    pub use crate::api::tags::{CreateTagRequest, ModifyTagRequest, Tag};

    // Hosts
    pub use crate::api::hosts::{Host, ModifyHostRequest};

    // Managed databases
    pub use crate::api::managed_database::{
        CreateDbUserRequest, CreateLogicalDatabaseRequest, CreateManagedDatabaseRequest,
        DbLogicalDatabase, DbPlan, DbUser, ManagedDatabase, ModifyManagedDatabaseRequest,
    };

    // Load balancer
    pub use crate::api::load_balancer::{
        CreateLoadBalancerRequest, LbBackend, LbFrontend, LbPlan, LoadBalancer,
        ModifyLoadBalancerRequest,
    };

    // Kubernetes
    pub use crate::api::kubernetes::{
        CreateClusterRequest, CreateNodeGroupRequest, K8sPlan, KubernetesCluster, NodeGroup,
    };

    // Managed object storage
    pub use crate::api::managed_object_storage::{
        CreateMosServiceRequest, ManagedObjectStorage, MosCredential, MosPolicy, MosRegion,
        MosUser,
    };

    // Gateway
    pub use crate::api::gateway::{
        CreateGatewayRequest, Gateway, GatewayConnection, GatewayPlan, GatewayTunnel,
    };

    // Permissions
    pub use crate::api::permissions::{
        GrantPermissionRequest, Permission, RevokePermissionRequest,
    };

    // Partners
    pub use crate::api::partners::{CreatePartnerAccountRequest, PartnerAccount};

    // Audit log
    pub use crate::api::audit_log::{AuditEvent, AuditEventPage, AuditListParams};

    // File storage
    pub use crate::api::file_storage::{
        CreateFileStorageRequest, CreateShareRequest, FileStorage, FileStoragePlan,
        FileStorageShare, ModifyFileStorageRequest,
    };
}
