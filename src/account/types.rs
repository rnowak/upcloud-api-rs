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
    /// Account username.
    pub username: String,
    /// First name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// Street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Postal/zip code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// City.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// U.S. state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// ISO 3166-1 three character country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Currency code (EUR/GBP/USD/SGD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Language preference (fi/en).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Phone number in international format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// E-mail address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Valid VAT number.
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
    /// Time zone in Continent/Location format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Account campaigns.
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
    /// Whether 3rd party services are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_3rd_party_services: Option<String>,
    /// Whether simple backup is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_backup: Option<bool>,
    /// Labels describing the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
}

/// Maximum resource quotas for an account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ResourceLimits {
    /// Maximum number of CPU cores.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cores: Option<u32>,
    /// Maximum number of detached floating IP addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detached_floating_ips: Option<u32>,
    /// Maximum amount of memory in MiB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<u64>,
    /// Maximum number of networks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<u32>,
    /// Maximum number of IPv4 addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ipv4: Option<u32>,
    /// Maximum number of IPv6 addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ipv6: Option<u32>,
    /// Maximum amount of Archive tier storage space in MiB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_hdd: Option<u64>,
    /// Maximum amount of MaxIOPS storage space in MiB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_maxiops: Option<u64>,
    /// Maximum amount of encrypted storage space in MiB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encypted: Option<u64>,
    /// Maximum number of Load Balancer services.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<u32>,
    /// Maximum number of managed databases.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_databases: Option<u32>,
    /// Maximum number of managed Kubernetes clusters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_kubernetes: Option<u32>,
    /// Maximum number of managed object storages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_object_storages: Option<u32>,
    /// Maximum number of network gateways.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_gateways: Option<u32>,
    /// Maximum number of network peerings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_peerings: Option<u32>,
    /// Network Transfer Pool excess transfer limit in GiB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ntp_excess_gib: Option<u64>,
    /// Maximum number of routers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routers: Option<u32>,
    /// Maximum amount of Standard tier storage space in MiB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_standard: Option<u64>,
    /// Maximum total storage space in MiB.
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
    /// Server UUID or `*` for all servers.
    pub uuid: String,
    /// Storage access mode for this server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
}

/// List of server access entries.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ServerAccessList {
    /// Server access entries.
    #[serde(default)]
    pub server: Vec<ServerAccessEntry>,
}

/// List of storage UUIDs the account can access.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StorageAccessList {
    /// Storage UUIDs the account can manage (`*` means all).
    #[serde(default)]
    pub storage: Vec<String>,
}

/// List of network UUIDs the account can access.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NetworkAccessList {
    /// Network UUIDs the account can manage (`*` means all).
    #[serde(default)]
    pub network: Vec<String>,
}

/// IP filter allowlist for account access.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpFilterList {
    /// IP address restrictions on API access (CIDR, ranges, or single IPs).
    #[serde(default)]
    pub ip_filter: Vec<String>,
}

/// Simple account entry from the list endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountEntry {
    /// Account username.
    pub username: String,
    /// Account roles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<AccountRoles>,
    /// Account type (`mymain` or `sub`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// Labels assigned to the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
}

/// Request body for modifying account details.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyAccountDetailsRequest {
    /// First name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// Street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Postal/zip code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// City.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// U.S. state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// ISO 3166-1 three character country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Currency code (EUR/GBP/USD/SGD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Language preference (fi/en).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Phone number in international format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// E-mail address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Valid VAT number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_number: Option<String>,
    /// Resource quotas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_limits: Option<ResourceLimits>,
    /// Account roles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<AccountRoles>,
    /// Time zone in Continent/Location format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Whether API access via HTTP Basic is enabled.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_api: Option<bool>,
    /// Whether control panel access is enabled.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_gui: Option<bool>,
    /// Whether 3rd party services are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_3rd_party_services: Option<String>,
    /// Server access control list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_access: Option<ServerAccessList>,
    /// Storage access control list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_access: Option<StorageAccessList>,
    /// Network access control list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_access: Option<NetworkAccessList>,
    /// Tag-based server/storage access permissions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_access: Option<serde_json::Value>,
    /// IP address restrictions on API access.
    #[serde(skip_serializing_if = "Option::is_none", alias = "ip_filters")]
    pub ip_filter_access: Option<IpFilterList>,
    /// Labels describing the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
}

/// Request body for creating a subaccount.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateSubaccountRequest {
    /// Username (4–64 characters).
    pub username: String,
    /// Account password.
    pub password: String,
    /// First name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// E-mail address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
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
    /// IP address restrictions on API access.
    #[serde(skip_serializing_if = "Option::is_none", alias = "ip_filters")]
    pub ip_filter_access: Option<IpFilterList>,
    /// Company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// Street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Postal/zip code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// City.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// U.S. state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// ISO 3166-1 three character country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Currency code (EUR/GBP/USD/SGD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Language preference (fi/en).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Phone number in international format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Valid VAT number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_number: Option<String>,
    /// Time zone in Continent/Location format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Whether API access via HTTP Basic is enabled.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_api: Option<bool>,
    /// Whether control panel access is enabled.
    #[serde_as(as = "Option<YesNo>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_gui: Option<bool>,
    /// Tag-based server/storage access permissions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_access: Option<serde_json::Value>,
    /// Labels describing the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
}

// ─── API Token types ──────────────────────────────────────────────────────────

/// An API token.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ApiToken {
    /// Unique identifier for the token.
    pub id: String,
    /// Token name.
    pub name: String,
    /// Creation timestamp (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Expiry timestamp (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Last used timestamp (ISO 8601).
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
    /// Unique identifier for the token.
    pub id: String,
    /// Token name.
    pub name: String,
    /// Creation timestamp (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Expiry timestamp (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
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
    /// Currency code for all amounts.
    #[serde(default)]
    pub currency: Option<String>,
    /// Managed database billing details.
    #[serde(default)]
    pub managed_databases: Option<serde_json::Value>,
    /// Managed object storage billing details.
    #[serde(default)]
    pub managed_object_storages: Option<serde_json::Value>,
    /// Server billing details.
    #[serde(default)]
    pub servers: Option<serde_json::Value>,
    /// Storage billing details.
    #[serde(default)]
    pub storages: Option<serde_json::Value>,
    /// Network billing details.
    #[serde(default)]
    pub networks: Option<serde_json::Value>,
    /// Cumulative charge from the beginning of the month.
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
    /// Currency code for all amounts.
    #[serde(default)]
    pub currency: Option<String>,
    /// Network billing details.
    #[serde(default)]
    pub networks: Option<serde_json::Value>,
    /// Managed database billing details.
    #[serde(default)]
    pub managed_databases: Option<serde_json::Value>,
    /// Managed object storage billing details.
    #[serde(default)]
    pub managed_object_storages: Option<serde_json::Value>,
    /// Server billing details.
    #[serde(default)]
    pub servers: Option<serde_json::Value>,
    /// Storage billing details.
    #[serde(default)]
    pub storages: Option<serde_json::Value>,
    /// Cumulative charge from the beginning of the month.
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
    /// Cumulative charge from the beginning of the month.
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
    /// Resource plan.
    #[serde(default)]
    pub plan: Option<String>,
    /// Resource identifier.
    #[serde(default)]
    pub resource_id: Option<String>,
    /// Short description.
    #[serde(default)]
    pub title: Option<String>,
    /// Resource type.
    #[serde(rename = "type", default)]
    pub resource_type: Option<String>,
    /// Data center zone.
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
    /// Quota increase in bytes.
    #[serde(default)]
    pub quota_increase_bytes: Option<u64>,
    /// Bytes sent in this period.
    #[serde(default)]
    pub sent_bytes: Option<u64>,
    /// Period start timestamp.
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
    /// Resource identifier.
    #[serde(default)]
    pub resource_id: Option<String>,
    /// Bytes sent in this period.
    #[serde(default)]
    pub sent_bytes: Option<u64>,
    /// Service type.
    #[serde(default)]
    pub service: Option<String>,
    /// Period start timestamp.
    #[serde(default)]
    pub start_time: Option<String>,
    /// Data center zone.
    #[serde(default)]
    pub zone: Option<String>,
}

/// Response for current network usage.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct CurrentNetworkUsage {
    /// Accumulated bandwidth quota in bytes.
    #[serde(default)]
    pub accumulated_quota_bytes: Option<u64>,
    /// Hourly quota increase in bytes.
    #[serde(default)]
    pub hourly_quota_increase_bytes: Option<u64>,
    /// Projected monthly quota in bytes.
    #[serde(default)]
    pub projected_monthly_quota_bytes: Option<u64>,
    /// Total bytes sent.
    #[serde(default)]
    pub total_sent_bytes: Option<u64>,
    /// Last updated timestamp.
    #[serde(default)]
    pub updated: Option<String>,
}
