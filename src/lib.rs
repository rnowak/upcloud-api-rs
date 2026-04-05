//! # UpCloud API SDK
//!
//! An idiomatic Rust SDK for the [UpCloud API v1.3](https://developers.upcloud.com/1.3/).
//!
//! ## Authentication
//!
//! ```rust,no_run
//! use upcloud_api_rs::Client;
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
//! # use upcloud_api_rs::Client;
//! # async fn example() -> Result<(), upcloud_api_rs::Error> {
//! let client = Client::with_bearer_token("ucat_...").unwrap();
//!
//! // Use domain-scoped handlers
//! let servers = client.servers().list(&Default::default()).await?;
//! let zones = client.zones().list().await?;
//! let dbs = client.managed_database().list().await?;
//! # Ok(())
//! # }
//! ```

pub mod client;
pub mod config;
pub mod error;
pub mod types;

// Domain modules
pub mod account;
pub mod audit_log;
pub mod file_storage;
pub mod gateway;
pub mod host;
pub mod ip_address;
pub mod kubernetes;
pub mod load_balancer;
pub mod managed_database;
pub mod network;
pub mod object_storage;
pub mod partner;
pub mod permission;
pub mod plan;
pub mod pricing;
pub mod server;
pub mod storage;
pub mod tag;
pub mod timezone;
pub mod zone;

pub use client::{Client, ClientBuilder};
pub use config::{AuthMode, ClientConfig};
pub use error::{ApiError, Error};

/// Re-export of [`reqwest`] to guarantee version compatibility when using
/// [`ClientBuilder::http_client()`] with a custom `reqwest::Client`.
pub use reqwest;

/// Convenience re-exports of the most commonly used types.
pub mod prelude {
    pub use crate::client::{Client, ClientBuilder};
    pub use crate::config::AuthMode;
    pub use crate::error::{ApiError, Error};
    pub use crate::types::common::{Label, Labels, PageParams, SortOrder};

    // Primary resource types from each domain module
    pub use crate::account::Account;
    pub use crate::file_storage::Service as FileStorageService;
    pub use crate::gateway::Gateway;
    pub use crate::host::Host;
    pub use crate::ip_address::IpAddress;
    pub use crate::kubernetes::Cluster;
    pub use crate::load_balancer::LoadBalancer;
    pub use crate::managed_database::Database;
    pub use crate::network::{Network, Router};
    pub use crate::object_storage::Service as ObjectStorageService;
    pub use crate::plan::Plan;
    pub use crate::pricing::PriceZone;
    pub use crate::server::{Server, ServerDetails};
    pub use crate::storage::{Storage, StorageDetails};
    pub use crate::tag::Tag;
    pub use crate::zone::Zone;
}
