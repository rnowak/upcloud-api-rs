use serde::{Deserialize, Serialize};

// ─── Enums ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OperationalState {
    Pending,
    SetupAgent,
    SetupServer,
    SetupNetwork,
    SetupLb,
    SetupDns,
    Checkup,
    Running,
    DeleteDns,
    DeleteNetwork,
    DeleteServer,
    DeleteService,
}

// ─── Plan ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_month_price_eur: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_hour_price_eur: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_server_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_member_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_backend_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_frontend_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_server_max_sessions: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_number: Option<u32>,
}

// ─── Load Balancer ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPAddress {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen_only: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    pub name: String,
    #[serde(rename = "type")]
    pub network_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancer {
    pub uuid: String,
    pub name: String,
    pub plan: String,
    pub zone: String,
    #[serde(alias = "operational_state")]
    pub state: OperationalState,
    #[serde(default)]
    pub backends: Vec<Backend>,
    #[serde(default)]
    pub frontends: Vec<Frontend>,
    #[serde(default)]
    pub resolvers: Vec<Resolver>,
    #[serde(default)]
    pub networks: Vec<Network>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_dow: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<Vec<IPAddress>>,
}

// ─── Node ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeNetwork {
    pub name: String,
    #[serde(rename = "type")]
    pub network_type: String,
    #[serde(default)]
    pub ip_addresses: Vec<IPAddress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_state: Option<String>,
    #[serde(default)]
    pub networks: Vec<NodeNetwork>,
}

// ─── Certificate Bundle ───────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateBundle {
    pub uuid: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intermediates: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub bundle_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

// ─── Backend ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub interval: u32,
    #[serde(rename = "type")]
    pub check_type: String,
    pub timeout: u32,
    pub unhealthy_threshold: u32,
    pub healthy_threshold: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_verify: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendMember {
    pub name: String,
    pub enabled: bool,
    pub ip: String,
    pub port: u16,
    pub weight: u32,
    pub max_sessions: u32,
    #[serde(rename = "type")]
    pub member_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TLSConfig {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_match: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_bundle_uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Backend {
    pub name: String,
    pub resolver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_cookie_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_proxy_protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<HealthCheck>,
    #[serde(default)]
    pub members: Vec<BackendMember>,
    #[serde(default)]
    pub tls_configs: Vec<TLSConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

// ─── Frontend ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontendNetwork {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontendRule {
    pub name: String,
    pub priority: u32,
    #[serde(default)]
    pub matchers: Vec<serde_json::Value>,
    #[serde(default)]
    pub actions: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matching_condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frontend {
    pub name: String,
    pub port: u16,
    pub mode: String,
    pub default_backend: String,
    #[serde(default)]
    pub rules: Vec<FrontendRule>,
    #[serde(default)]
    pub tls_configs: Vec<TLSConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(default)]
    pub networks: Vec<FrontendNetwork>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

// ─── Resolver ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resolver {
    pub name: String,
    #[serde(default)]
    pub nameservers: Vec<String>,
    pub retries: u32,
    pub timeout: u32,
    pub timeout_retry: u32,
    pub cache_valid: u32,
    pub cache_invalid: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

// ─── Request types ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequest {
    pub name: String,
    pub plan: String,
    pub zone: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub networks: Vec<NetworkInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInput {
    pub name: String,
    #[serde(rename = "type")]
    pub network_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBackendRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<HealthCheck>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBackendMemberRequest {
    pub name: String,
    pub ip: String,
    pub port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_sessions: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFrontendRequest {
    pub name: String,
    pub port: u16,
    pub mode: String,
    pub default_backend: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub networks: Vec<FrontendNetwork>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFrontendRuleRequest {
    pub name: String,
    pub priority: u32,
    #[serde(default)]
    pub matchers: Vec<serde_json::Value>,
    #[serde(default)]
    pub actions: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateResolverRequest {
    pub name: String,
    pub nameservers: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_retry: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_valid: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_invalid: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTLSConfigRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_match: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyFrontendRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_backend: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<FrontendNetwork>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyBackendMemberRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_sessions: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyResolverRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nameservers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_retry: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_valid: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_invalid: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyFrontendRuleRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matchers: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matching_condition: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplaceFrontendRuleRequest {
    pub name: String,
    pub priority: u32,
    #[serde(default)]
    pub matchers: Vec<serde_json::Value>,
    #[serde(default)]
    pub actions: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matching_condition: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCertificateBundleRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intermediates: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub bundle_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyCertificateBundleRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intermediates: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub bundle_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyLoadBalancerNetworkRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
