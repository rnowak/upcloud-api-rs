use serde::{Deserialize, Serialize};

// ─── Enums ────────────────────────────────────────────────────────────────────

/// Load balancer operational state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OperationalState {
    /// Service creation is pending.
    Pending,
    /// Agent is being set up.
    SetupAgent,
    /// Server is being provisioned.
    SetupServer,
    /// Network is being configured.
    SetupNetwork,
    /// Load balancer is being configured.
    SetupLb,
    /// DNS records are being created.
    SetupDns,
    /// Service is performing a health check.
    Checkup,
    /// Service is running normally.
    Running,
    /// DNS records are being removed.
    DeleteDns,
    /// Network is being torn down.
    DeleteNetwork,
    /// Server is being deleted.
    DeleteServer,
    /// Service is being deleted.
    DeleteService,
}

// ─── Plan ─────────────────────────────────────────────────────────────────────

/// A load balancer service plan with pricing and capacity limits.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Plan {
    pub name: String,
    /// Monthly price in EUR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_month_price_eur: Option<f64>,
    /// Hourly price in EUR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_hour_price_eur: Option<f64>,
    /// Maximum number of compute nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_server_count: Option<u32>,
    /// Maximum number of backend members across all backends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_member_count: Option<u32>,
    /// Maximum number of backends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_backend_count: Option<u32>,
    /// Maximum number of frontends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_frontend_count: Option<u32>,
    /// Maximum concurrent sessions per server node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_server_max_sessions: Option<u32>,
    /// Number of server nodes included in this plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_number: Option<u32>,
}

// ─── Load Balancer ────────────────────────────────────────────────────────────

/// An IP address assigned to the load balancer.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IPAddress {
    pub address: String,
    /// The name of the network where the IP address is attached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    /// ISO 8601 creation timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// ISO 8601 last update timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// A network attached to the load balancer.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Network {
    pub name: String,
    /// Network type (e.g. `"public"`, `"private"`, `"utility"`).
    #[serde(rename = "type")]
    pub network_type: String,
    /// Address family (`"IPv4"` or `"IPv6"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// UUID of the SDN private network (required when type is `"private"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    /// Auto-generated DNS name for this network attachment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
}

/// A managed load balancer instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LoadBalancer {
    pub uuid: String,
    pub name: String,
    /// Service plan name (e.g. `"development"`, `"production-small"`).
    pub plan: String,
    /// Zone where the load balancer is deployed (e.g. `"fi-hel1"`).
    pub zone: String,
    /// Current operational state of the load balancer.
    #[serde(alias = "operational_state")]
    pub state: OperationalState,
    /// Backend server pools.
    #[serde(default)]
    pub backends: Vec<Backend>,
    /// Frontend listeners.
    #[serde(default)]
    pub frontends: Vec<Frontend>,
    /// DNS resolvers used by backends.
    #[serde(default)]
    pub resolvers: Vec<Resolver>,
    /// Networks attached to the load balancer.
    #[serde(default)]
    pub networks: Vec<Network>,
    /// Key-value labels attached to the service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
    /// ISO 8601 creation timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// ISO 8601 last update timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Desired service status (`"started"` or `"stopped"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_status: Option<String>,
    /// Auto-generated DNS name for the load balancer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    /// Preferred day of week for maintenance (e.g. `"monday"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_dow: Option<String>,
    /// Preferred time for maintenance in UTC (e.g. `"06:00:00"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_time: Option<String>,
    /// Compute nodes backing this load balancer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<Node>>,
    /// IP addresses assigned to the load balancer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<Vec<IPAddress>>,
}

// ─── Node ─────────────────────────────────────────────────────────────────

/// A network attached to a load balancer compute node.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NodeNetwork {
    pub name: String,
    /// Network type (e.g. `"public"`, `"private"`).
    #[serde(rename = "type")]
    pub network_type: String,
    /// IP addresses assigned on this network.
    #[serde(default)]
    pub ip_addresses: Vec<IPAddress>,
}

/// A compute node backing the load balancer.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Node {
    /// Current operational state of the node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_state: Option<String>,
    /// Networks attached to this node.
    #[serde(default)]
    pub networks: Vec<NodeNetwork>,
}

// ─── Certificate Bundle ───────────────────────────────────────────────────

/// A TLS certificate bundle for frontend or backend TLS termination.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CertificateBundle {
    pub uuid: String,
    pub name: String,
    /// PEM-encoded server certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// PEM-encoded intermediate CA certificates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intermediates: Option<String>,
    /// Key type (e.g. `"RSA"`, `"ECDSA"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
    /// Operational state of the certificate bundle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_state: Option<String>,
    /// Certificate validity end date (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_after: Option<String>,
    /// Certificate validity start date (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<String>,
    /// Bundle type (e.g. `"manual"`, `"dynamic"`, `"authority"`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub bundle_type: Option<String>,
    /// ISO 8601 creation timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// ISO 8601 last update timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

// ─── Backend ─────────────────────────────────────────────────────────────────

/// Health check configuration for a load balancer backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HealthCheck {
    /// Interval between health checks in seconds.
    pub interval: u32,
    /// Health check type (e.g. `"tcp"`, `"http"`).
    #[serde(rename = "type")]
    pub check_type: String,
    /// Timeout for a single health check in seconds.
    pub timeout: u32,
    /// Number of consecutive failures before marking a member as down.
    pub unhealthy_threshold: u32,
    /// Number of consecutive successes before marking a member as up.
    pub healthy_threshold: u32,
    /// HTTP path to request for HTTP health checks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_path: Option<String>,
    /// Whether to verify TLS certificates during health checks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_verify: Option<bool>,
}

/// An upstream server (member) in a load balancer backend pool.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BackendMember {
    pub name: String,
    /// Whether this member accepts traffic.
    pub enabled: bool,
    /// IP address of the upstream server.
    pub ip: String,
    /// Port on the upstream server.
    pub port: u16,
    /// Load balancing weight (higher receives more traffic).
    pub weight: u32,
    /// Maximum concurrent sessions for this member.
    pub max_sessions: u32,
    /// Member type (e.g. `"static"`, `"dynamic"`).
    #[serde(rename = "type")]
    pub member_type: String,
    /// Whether this member is a backup (only receives traffic when primary members are down).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup: Option<bool>,
    /// ISO 8601 creation timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// ISO 8601 last update timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// TLS configuration for backend connections.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TLSConfig {
    pub name: String,
    /// PEM-encoded certificate chain for backend TLS verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
    /// Server name patterns to match for TLS SNI routing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_match: Option<Vec<String>>,
    /// UUID of the certificate bundle to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_bundle_uuid: Option<String>,
    /// ISO 8601 creation timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// ISO 8601 last update timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// A load balancer backend — the upstream server pool.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Backend {
    pub name: String,
    /// Name of the DNS resolver used for backend member hostnames.
    pub resolver: Option<String>,
    /// Backend mode (e.g. `"tcp"`, `"http"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Cookie name used for cookie-based session persistence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_cookie_name: Option<String>,
    /// PROXY protocol version for outbound connections (e.g. `"v1"`, `"v2"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_proxy_protocol: Option<String>,
    /// Health check configuration for backend members.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<HealthCheck>,
    /// Upstream server members in this backend pool.
    #[serde(default)]
    pub members: Vec<BackendMember>,
    /// TLS configurations for backend connections.
    #[serde(default)]
    pub tls_configs: Vec<TLSConfig>,
    /// Additional backend properties as free-form JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    /// ISO 8601 creation timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// ISO 8601 last update timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

// ─── Frontend ─────────────────────────────────────────────────────────────────

/// A network reference attached to a frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FrontendNetwork {
    pub name: String,
}

/// A rule attached to a frontend, consisting of matchers and actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FrontendRule {
    pub name: String,
    /// Evaluation priority (lower values are evaluated first).
    pub priority: u32,
    /// Conditions that must match for the rule to trigger.
    #[serde(default)]
    pub matchers: Vec<serde_json::Value>,
    /// Actions to execute when matchers are satisfied.
    #[serde(default)]
    pub actions: Vec<serde_json::Value>,
    /// How multiple matchers combine (`"and"` or `"or"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matching_condition: Option<String>,
    /// ISO 8601 creation timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// ISO 8601 last update timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// A load balancer frontend — the client-facing listener.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Frontend {
    pub name: String,
    /// Port the frontend listens on.
    pub port: u16,
    /// Frontend mode (e.g. `"tcp"`, `"http"`).
    pub mode: String,
    /// Name of the backend that receives traffic by default.
    pub default_backend: String,
    /// Traffic rules applied to this frontend.
    #[serde(default)]
    pub rules: Vec<FrontendRule>,
    /// TLS configurations for frontend connections.
    #[serde(default)]
    pub tls_configs: Vec<TLSConfig>,
    /// Additional frontend properties as free-form JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    /// Networks this frontend is attached to.
    #[serde(default)]
    pub networks: Vec<FrontendNetwork>,
    /// ISO 8601 creation timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// ISO 8601 last update timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

// ─── Resolver ─────────────────────────────────────────────────────────────────

/// A DNS resolver used by backends for member hostname resolution.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Resolver {
    pub name: String,
    /// DNS nameserver IP addresses.
    #[serde(default)]
    pub nameservers: Vec<String>,
    /// Number of retry attempts for failed DNS queries.
    pub retries: u32,
    /// Timeout for a single DNS query in seconds.
    pub timeout: u32,
    /// Timeout between DNS query retries in seconds.
    pub timeout_retry: u32,
    /// TTL in seconds for caching successful DNS responses.
    pub cache_valid: u32,
    /// TTL in seconds for caching failed DNS responses.
    pub cache_invalid: u32,
    /// ISO 8601 creation timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// ISO 8601 last update timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

// ─── Request types ────────────────────────────────────────────────────────────

/// Request to create a new load balancer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequest {
    pub name: String,
    /// Service plan name.
    pub plan: String,
    /// Zone to deploy in (e.g. `"fi-hel1"`).
    pub zone: String,
    /// Networks to attach to the load balancer.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub networks: Vec<NetworkInput>,
    /// Desired service status (`"started"` or `"stopped"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_status: Option<String>,
    /// Key-value labels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

/// Network configuration input for load balancer creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NetworkInput {
    pub name: String,
    /// Network type (e.g. `"public"`, `"private"`, `"utility"`).
    #[serde(rename = "type")]
    pub network_type: String,
    /// Address family (`"IPv4"` or `"IPv6"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// UUID of an existing network to attach.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

/// Request to modify an existing load balancer.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New service plan name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    /// Desired service status (`"started"` or `"stopped"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_status: Option<String>,
    /// Key-value labels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

/// Request to create a new backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBackendRequest {
    pub name: String,
    /// Name of the DNS resolver for backend member hostnames.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver: Option<String>,
    /// Backend mode (e.g. `"tcp"`, `"http"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Health check configuration for backend members.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<HealthCheck>,
    /// Additional backend properties as free-form JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

/// Request to create a new backend member.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBackendMemberRequest {
    pub name: String,
    /// IP address of the upstream server.
    pub ip: String,
    /// Port on the upstream server.
    pub port: u16,
    /// Load balancing weight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u32>,
    /// Maximum concurrent sessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_sessions: Option<u32>,
    /// Whether the member accepts traffic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Member type (e.g. `"static"`, `"dynamic"`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
}

/// Request to create a new frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFrontendRequest {
    pub name: String,
    /// Port the frontend listens on.
    pub port: u16,
    /// Frontend mode (e.g. `"tcp"`, `"http"`).
    pub mode: String,
    /// Name of the backend that receives traffic by default.
    pub default_backend: String,
    /// Networks to attach to the frontend.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub networks: Vec<FrontendNetwork>,
    /// Additional frontend properties as free-form JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

/// Request to create a new frontend rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFrontendRuleRequest {
    pub name: String,
    /// Evaluation priority (lower values are evaluated first).
    pub priority: u32,
    /// Conditions that must match for the rule to trigger.
    #[serde(default)]
    pub matchers: Vec<serde_json::Value>,
    /// Actions to execute when matchers are satisfied.
    #[serde(default)]
    pub actions: Vec<serde_json::Value>,
}

/// Request to create a new DNS resolver.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateResolverRequest {
    pub name: String,
    /// DNS nameserver IP addresses.
    pub nameservers: Vec<String>,
    /// Number of retry attempts for failed DNS queries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<u32>,
    /// Timeout for a single DNS query in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
    /// Timeout between DNS query retries in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_retry: Option<u32>,
    /// TTL in seconds for caching successful DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_valid: Option<u32>,
    /// TTL in seconds for caching failed DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_invalid: Option<u32>,
}

/// Request to create a new TLS configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTLSConfigRequest {
    pub name: String,
    /// PEM-encoded certificate chain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
    /// PEM-encoded private key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    /// Server name patterns to match for TLS SNI routing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_match: Option<Vec<String>>,
    /// UUID of a certificate bundle to use instead of inline cert/key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_bundle_uuid: Option<String>,
}

/// Request to modify an existing frontend.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyFrontendRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Name of the backend that receives traffic by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_backend: Option<String>,
    /// Frontend mode (e.g. `"tcp"`, `"http"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Networks to attach to the frontend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<FrontendNetwork>>,
    /// Additional frontend properties as free-form JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

/// Request to modify an existing backend member.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyBackendMemberRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Load balancing weight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u32>,
    /// Maximum concurrent sessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_sessions: Option<u32>,
    /// Whether the member accepts traffic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// IP address of the upstream server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Port on the upstream server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
}

/// Request to modify an existing DNS resolver.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyResolverRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// DNS nameserver IP addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nameservers: Option<Vec<String>>,
    /// Number of retry attempts for failed DNS queries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<u32>,
    /// Timeout for a single DNS query in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u32>,
    /// Timeout between DNS query retries in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_retry: Option<u32>,
    /// TTL in seconds for caching successful DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_valid: Option<u32>,
    /// TTL in seconds for caching failed DNS responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_invalid: Option<u32>,
}

/// Request to modify an existing frontend rule.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyFrontendRuleRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Evaluation priority (lower values are evaluated first).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u32>,
    /// Conditions that must match for the rule to trigger.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matchers: Option<Vec<serde_json::Value>>,
    /// Actions to execute when matchers are satisfied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<serde_json::Value>>,
    /// How multiple matchers combine (`"and"` or `"or"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matching_condition: Option<String>,
}

/// Request to fully replace a frontend rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplaceFrontendRuleRequest {
    pub name: String,
    /// Evaluation priority (lower values are evaluated first).
    pub priority: u32,
    /// Conditions that must match for the rule to trigger.
    #[serde(default)]
    pub matchers: Vec<serde_json::Value>,
    /// Actions to execute when matchers are satisfied.
    #[serde(default)]
    pub actions: Vec<serde_json::Value>,
    /// How multiple matchers combine (`"and"` or `"or"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matching_condition: Option<String>,
}

/// Request to create a new certificate bundle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCertificateBundleRequest {
    pub name: String,
    /// PEM-encoded server certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// PEM-encoded intermediate CA certificates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intermediates: Option<String>,
    /// PEM-encoded private key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    /// Bundle type (e.g. `"manual"`, `"dynamic"`, `"authority"`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub bundle_type: Option<String>,
}

/// Request to modify an existing certificate bundle.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyCertificateBundleRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// PEM-encoded server certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    /// PEM-encoded intermediate CA certificates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intermediates: Option<String>,
    /// PEM-encoded private key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    /// Bundle type (e.g. `"manual"`, `"dynamic"`, `"authority"`).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub bundle_type: Option<String>,
}

/// Request to modify an existing backend.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyBackendRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Name of the DNS resolver for backend members.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver: Option<String>,
    /// Additional backend properties as free-form JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

/// Request to add an IP address to a load balancer.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddIPAddressRequest {
    /// IP address to assign (leave empty for automatic assignment).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Name of the network to bind the address to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
}

/// DNS challenge domain returned by the load balancer API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsChallengeDomain {
    pub domain: String,
}

/// Request to modify a load balancer network attachment.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyLoadBalancerNetworkRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
