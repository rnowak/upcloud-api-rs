#![doc = include_str!("../README.md")]

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
pub use error::{ApiError, Error, ErrorCode};

/// Re-export of [`reqwest`] to guarantee version compatibility when using
/// [`ClientBuilder::http_client()`] with a custom `reqwest::Client`.
pub use reqwest;

/// Convenience re-exports of the most commonly used types.
pub mod prelude {
    pub use crate::client::{Client, ClientBuilder};
    pub use crate::config::AuthMode;
    pub use crate::error::{ApiError, Error, ErrorCode};
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
