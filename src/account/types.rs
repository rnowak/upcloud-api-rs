//! Types for accounts, subaccounts, and API tokens.

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::types::common::{Label, YesNo};

// ─── Enums ───────────────────────────────────────────────────────────────────

/// The type of an UpCloud account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccountType {
    Main,
    Sub,
}

/// The type of an API token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TokenType {
    Pat,
    Workspace,
}

// ─── Account types ────────────────────────────────────────────────────────────

/// An UpCloud account with profile, limits, and access settings.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Account {
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_number: Option<String>,
    /// Whether network-level access is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_network_access: Option<String>,
    /// Resource quotas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_limits: Option<ResourceLimits>,
    /// Account roles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<AccountRoles>,
    /// Server access control list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_access: Option<ServerAccessList>,
    /// Storage access control list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_access: Option<StorageAccessList>,
    /// Network access control list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_access: Option<NetworkAccessList>,
    /// IP filter access control list.
    #[serde(skip_serializing_if = "Option::is_none", alias = "ip_filters")]
    pub ip_filter_access: Option<IpFilterList>,
    /// Tag access control list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_access: Option<serde_json::Value>,
    /// Account credit balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credits: Option<f64>,
    /// Account type string.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none", default)]
    pub account_type: Option<String>,
    /// Username of the parent main account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaigns: Option<serde_json::Value>,
    /// Whether API access is enabled.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_api: Option<bool>,
    /// Whether control panel access is enabled.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_gui: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_3rd_party_services: Option<String>,
    /// Whether simple backup is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_backup: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
}

/// Maximum resource quotas for an account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ResourceLimits {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cores: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detached_floating_ips: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ipv4: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ipv6: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_hdd: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_maxiops: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encypted: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_databases: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_kubernetes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_object_storages: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_gateways: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_peerings: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ntp_excess_gib: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routers: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_standard: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_total: Option<u64>,
}

/// Account roles wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountRoles {
    /// List of role names.
    #[serde(default)]
    pub role: Vec<String>,
}

/// Server-level access control entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ServerAccessEntry {
    pub uuid: String,
    /// Storage access mode for this server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
}

/// List of server access entries.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ServerAccessList {
    #[serde(default)]
    pub server: Vec<ServerAccessEntry>,
}

/// List of storage UUIDs the account can access.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StorageAccessList {
    #[serde(default)]
    pub storage: Vec<String>,
}

/// List of network UUIDs the account can access.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NetworkAccessList {
    #[serde(default)]
    pub network: Vec<String>,
}

/// IP filter allowlist for account access.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpFilterList {
    #[serde(default)]
    pub ip_filter: Vec<String>,
}

/// Simple account entry from the list endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountEntry {
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<AccountRoles>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
}

/// Request body for modifying account details.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyAccountDetailsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_limits: Option<ResourceLimits>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<AccountRoles>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_api: Option<bool>,
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_gui: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_3rd_party_services: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_access: Option<ServerAccessList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_access: Option<StorageAccessList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_access: Option<NetworkAccessList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_access: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "ip_filters")]
    pub ip_filter_access: Option<IpFilterList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
}

/// Request body for creating a subaccount.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateSubaccountRequest {
    pub username: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_limits: Option<ResourceLimits>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<AccountRoles>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_access: Option<ServerAccessList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_access: Option<StorageAccessList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_access: Option<NetworkAccessList>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "ip_filters")]
    pub ip_filter_access: Option<IpFilterList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_api: Option<bool>,
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_gui: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_access: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
}

// ─── API Token types ──────────────────────────────────────────────────────────

/// An API token.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ApiToken {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<String>,
    /// Whether this token can create other tokens.
    #[serde(default)]
    pub can_create_tokens: bool,
    /// CIDRs allowed to use this token.
    #[serde(default)]
    pub allowed_ip_ranges: Vec<String>,
    /// Whether this token grants control panel access.
    #[serde(default)]
    pub gui: bool,
}

/// Request body for creating an API token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTokenRequest {
    pub name: String,
    /// Absolute expiry timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Relative expiry duration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<String>,
    /// Allow token creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_create_tokens: Option<bool>,
    /// Allowed CIDRs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_ip_ranges: Option<Vec<String>>,
}

/// Response from creating a token — includes the token value (only shown once).
/// The response is flat: `{token, id, name, created_at, ...}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateTokenResponse {
    /// The bearer token value — only returned at creation time.
    pub token: String,
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(default)]
    pub can_create_tokens: bool,
    #[serde(default)]
    pub allowed_ip_ranges: Vec<String>,
    #[serde(default)]
    pub gui: bool,
}

// ─── Billing / Statistics types ───────────────────────────────────────────────

/// Optional query parameters for `GET /1.3/account/billing/summary/{yearmonth}`.
#[derive(Debug, Clone, Default)]
pub struct BillingSummaryParams {
    /// Filter by resource ID.
    pub resource_id: Option<String>,
    /// Filter by username.
    pub username: Option<String>,
}

impl BillingSummaryParams {
    /// Build a URL-encoded query string from the non-`None` fields.
    pub fn to_query_string(&self) -> String {
        let mut pairs = url::form_urlencoded::Serializer::new(String::new());
        if let Some(ref v) = self.resource_id {
            pairs.append_pair("resource_id", v);
        }
        if let Some(ref v) = self.username {
            pairs.append_pair("username", v);
        }
        pairs.finish()
    }
}

/// Response for `GET /1.3/account/billing/summary/{yearmonth}`.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct BillingSummaryResponse {
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub managed_databases: Option<serde_json::Value>,
    #[serde(default)]
    pub managed_object_storages: Option<serde_json::Value>,
    #[serde(default)]
    pub servers: Option<serde_json::Value>,
    #[serde(default)]
    pub storages: Option<serde_json::Value>,
    #[serde(default)]
    pub networks: Option<serde_json::Value>,
    #[serde(default)]
    pub total_amount: Option<f64>,
}

/// Wrapper for deprecated billing endpoints.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct DeprecatedBillingWrapper {
    pub billing: DeprecatedBillingSummary,
}

/// Billing summary from the deprecated endpoint.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct DeprecatedBillingSummary {
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub networks: Option<serde_json::Value>,
    #[serde(default)]
    pub managed_databases: Option<serde_json::Value>,
    #[serde(default)]
    pub managed_object_storages: Option<serde_json::Value>,
    #[serde(default)]
    pub servers: Option<serde_json::Value>,
    #[serde(default)]
    pub storages: Option<serde_json::Value>,
    #[serde(default)]
    pub total_amount: Option<f64>,
}

/// Response for `GET /1.3/account/resource_billing_summary/{id}/{yearmonth}`.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ResourceBillingWrapper {
    pub billing: ResourceBillingSummary,
}

/// Per-resource billing summary.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ResourceBillingSummary {
    /// Daily billing amounts.
    #[serde(default)]
    pub daily_sums: Option<serde_json::Value>,
    /// Resource billing details.
    #[serde(default)]
    pub details: Option<ResourceBillingDetails>,
    #[serde(default)]
    pub total_amount: Option<f64>,
}

/// Details of a billed resource.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ResourceBillingDetails {
    /// Billed hours.
    #[serde(default)]
    pub hours: Option<u64>,
    #[serde(default)]
    pub plan: Option<String>,
    #[serde(default)]
    pub resource_id: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    /// Resource type.
    #[serde(rename = "type", default)]
    pub resource_type: Option<String>,
    #[serde(default)]
    pub zone: Option<String>,
}

/// Response for network transfer statistics.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct NetworkTransferStatsResponse {
    pub stats: NetworkTransferStatsWrapper,
}

/// Wrapper for network transfer statistics.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct NetworkTransferStatsWrapper {
    /// Transfer statistics entries.
    #[serde(default)]
    pub stat: Vec<NetworkTransferStat>,
    /// Current usage snapshot.
    #[serde(default)]
    pub current_network_usage: Option<CurrentNetworkUsage>,
}

/// A single network transfer statistics entry.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct NetworkTransferStat {
    /// Accumulated bandwidth quota.
    #[serde(default)]
    pub accumulated_quota_bytes: Option<u64>,
    /// Projected monthly quota.
    #[serde(default)]
    pub projected_monthly_quota_bytes: Option<u64>,
    /// Quota increase.
    #[serde(default)]
    pub quota_increase_bytes: Option<u64>,
    /// Bytes sent in period.
    #[serde(default)]
    pub sent_bytes: Option<u64>,
    #[serde(default)]
    pub start_time: Option<String>,
    /// Total bytes sent.
    #[serde(default)]
    pub total_sent_bytes: Option<u64>,
}

/// Stat entry for resource-level transfer stats.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ResourceTransferStat {
    #[serde(default)]
    pub resource_id: Option<String>,
    #[serde(default)]
    pub sent_bytes: Option<u64>,
    #[serde(default)]
    pub service: Option<String>,
    #[serde(default)]
    pub start_time: Option<String>,
    #[serde(default)]
    pub zone: Option<String>,
}

/// Response for current network usage.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct CurrentNetworkUsage {
    #[serde(default)]
    pub accumulated_quota_bytes: Option<u64>,
    #[serde(default)]
    pub hourly_quota_increase_bytes: Option<u64>,
    #[serde(default)]
    pub projected_monthly_quota_bytes: Option<u64>,
    #[serde(default)]
    pub total_sent_bytes: Option<u64>,
    #[serde(default)]
    pub updated: Option<String>,
}
