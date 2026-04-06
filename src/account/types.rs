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

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_network_access: Option<String>,
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
    pub tag_access: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credits: Option<f64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none", default)]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaigns: Option<serde_json::Value>,
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_api: Option<bool>,
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_gui: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_3rd_party_services: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_backup: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountRoles {
    #[serde(default)]
    pub role: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerAccessEntry {
    pub uuid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerAccessList {
    #[serde(default)]
    pub server: Vec<ServerAccessEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageAccessList {
    #[serde(default)]
    pub storage: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkAccessList {
    #[serde(default)]
    pub network: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpFilterList {
    #[serde(default)]
    pub ip_filter: Vec<String>,
}

/// Simple account entry from the list endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountEntry {
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<AccountRoles>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiToken {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<String>,
    #[serde(default)]
    pub can_create_tokens: bool,
    #[serde(default)]
    pub allowed_ip_ranges: Vec<String>,
    #[serde(default)]
    pub gui: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTokenRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_create_tokens: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_ip_ranges: Option<Vec<String>>,
}

/// Response from creating a token — includes the token value (only shown once).
/// The response is flat: `{token, id, name, created_at, ...}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Response for `GET /1.3/account/billing/summary/{yearmonth}`.
#[derive(Debug, Clone, Deserialize)]
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
pub struct DeprecatedBillingWrapper {
    pub billing: DeprecatedBillingSummary,
}

#[derive(Debug, Clone, Deserialize)]
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
pub struct ResourceBillingWrapper {
    pub billing: ResourceBillingSummary,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResourceBillingSummary {
    #[serde(default)]
    pub daily_sums: Option<serde_json::Value>,
    #[serde(default)]
    pub details: Option<ResourceBillingDetails>,
    #[serde(default)]
    pub total_amount: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResourceBillingDetails {
    #[serde(default)]
    pub hours: Option<u64>,
    #[serde(default)]
    pub plan: Option<String>,
    #[serde(default)]
    pub resource_id: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(rename = "type", default)]
    pub resource_type: Option<String>,
    #[serde(default)]
    pub zone: Option<String>,
}

/// Response for network transfer statistics.
#[derive(Debug, Clone, Deserialize)]
pub struct NetworkTransferStatsResponse {
    pub stats: NetworkTransferStatsWrapper,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NetworkTransferStatsWrapper {
    #[serde(default)]
    pub stat: Vec<NetworkTransferStat>,
    #[serde(default)]
    pub current_network_usage: Option<CurrentNetworkUsage>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NetworkTransferStat {
    #[serde(default)]
    pub accumulated_quota_bytes: Option<u64>,
    #[serde(default)]
    pub projected_monthly_quota_bytes: Option<u64>,
    #[serde(default)]
    pub quota_increase_bytes: Option<u64>,
    #[serde(default)]
    pub sent_bytes: Option<u64>,
    #[serde(default)]
    pub start_time: Option<String>,
    #[serde(default)]
    pub total_sent_bytes: Option<u64>,
}

/// Stat entry for resource-level transfer stats.
#[derive(Debug, Clone, Deserialize)]
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
