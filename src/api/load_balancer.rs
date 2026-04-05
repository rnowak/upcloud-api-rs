//! Load Balancer — `GET/POST/PATCH/DELETE /1.3/load-balancer` + sub-resources.

use serde::{Deserialize, Serialize};

use crate::{client::Client, error::Error, types::common::PageParams};

// ─── Enums ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LoadBalancerOperationalState {
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
pub struct LoadBalancerPlan {
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
pub struct LoadBalancerIPAddress {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen_only: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerNetwork {
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
    pub state: LoadBalancerOperationalState,
    #[serde(default)]
    pub backends: Vec<LoadBalancerBackend>,
    #[serde(default)]
    pub frontends: Vec<LoadBalancerFrontend>,
    #[serde(default)]
    pub resolvers: Vec<LoadBalancerResolver>,
    #[serde(default)]
    pub networks: Vec<LoadBalancerNetwork>,
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
    pub nodes: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<serde_json::Value>,
}

// ─── Backend ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerHealthCheck {
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
pub struct LoadBalancerBackendMember {
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
pub struct LoadBalancerTLSConfig {
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
pub struct LoadBalancerBackend {
    pub name: String,
    pub resolver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_cookie_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_proxy_protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<LoadBalancerHealthCheck>,
    #[serde(default)]
    pub members: Vec<LoadBalancerBackendMember>,
    #[serde(default)]
    pub tls_configs: Vec<LoadBalancerTLSConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

// ─── Frontend ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerFrontendNetwork {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerFrontendRule {
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
pub struct LoadBalancerFrontend {
    pub name: String,
    pub port: u16,
    pub mode: String,
    pub default_backend: String,
    #[serde(default)]
    pub rules: Vec<LoadBalancerFrontendRule>,
    #[serde(default)]
    pub tls_configs: Vec<LoadBalancerTLSConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(default)]
    pub networks: Vec<LoadBalancerFrontendNetwork>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

// ─── Resolver ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerResolver {
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
pub struct CreateLoadBalancerRequest {
    pub name: String,
    pub plan: String,
    pub zone: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub networks: Vec<LoadBalancerNetworkInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<crate::types::common::Label>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerNetworkInput {
    pub name: String,
    #[serde(rename = "type")]
    pub network_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModifyLoadBalancerRequest {
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
pub struct CreateLoadBalancerBackendRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<LoadBalancerHealthCheck>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLoadBalancerBackendMemberRequest {
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
pub struct CreateLoadBalancerFrontendRequest {
    pub name: String,
    pub port: u16,
    pub mode: String,
    pub default_backend: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub networks: Vec<LoadBalancerFrontendNetwork>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLoadBalancerFrontendRuleRequest {
    pub name: String,
    pub priority: u32,
    #[serde(default)]
    pub matchers: Vec<serde_json::Value>,
    #[serde(default)]
    pub actions: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLoadBalancerResolverRequest {
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
pub struct CreateLoadBalancerTLSConfigRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_match: Option<Vec<String>>,
}

// ─── Client methods ───────────────────────────────────────────────────────────

impl Client {
    // ─── Plans ────────────────────────────────────────────────────────────────

    /// Returns a list of available load balancer plans.
    pub async fn list_load_balancer_plans(&self, params: PageParams) -> Result<Vec<LoadBalancerPlan>, Error> {
        self.get_with_query("/1.3/load-balancer/plans", &params)
            .await
    }

    /// Returns details for a specific load balancer plan.
    pub async fn get_load_balancer_plan(&self, name: &str) -> Result<LoadBalancerPlan, Error> {
        self.get(&format!("/1.3/load-balancer/plans/{name}")).await
    }

    // ─── Services ─────────────────────────────────────────────────────────────

    /// Returns a list of all load balancers.
    pub async fn list_load_balancers(&self, params: PageParams) -> Result<Vec<LoadBalancer>, Error> {
        self.get_with_query("/1.3/load-balancer", &params).await
    }

    /// Returns details for a specific load balancer.
    pub async fn get_load_balancer(&self, uuid: &str) -> Result<LoadBalancer, Error> {
        self.get(&format!("/1.3/load-balancer/{uuid}")).await
    }

    /// Creates a new load balancer.
    pub async fn create_load_balancer(
        &self,
        req: CreateLoadBalancerRequest,
    ) -> Result<LoadBalancer, Error> {
        self.post("/1.3/load-balancer", &req).await
    }

    /// Modifies an existing load balancer.
    pub async fn modify_load_balancer(
        &self,
        uuid: &str,
        req: ModifyLoadBalancerRequest,
    ) -> Result<LoadBalancer, Error> {
        self.patch(&format!("/1.3/load-balancer/{uuid}"), &req).await
    }

    /// Deletes a load balancer.
    pub async fn delete_load_balancer(&self, uuid: &str) -> Result<(), Error> {
        self.delete(&format!("/1.3/load-balancer/{uuid}")).await
    }

    // ─── Backends ─────────────────────────────────────────────────────────────

    /// Returns a list of backends for a load balancer.
    pub async fn list_load_balancer_backends(&self, lb_uuid: &str) -> Result<Vec<LoadBalancerBackend>, Error> {
        self.get(&format!("/1.3/load-balancer/{lb_uuid}/backends"))
            .await
    }

    /// Returns details for a specific backend.
    pub async fn get_load_balancer_backend(&self, lb_uuid: &str, name: &str) -> Result<LoadBalancerBackend, Error> {
        self.get(&format!("/1.3/load-balancer/{lb_uuid}/backends/{name}"))
            .await
    }

    /// Creates a backend for a load balancer.
    pub async fn create_load_balancer_backend(
        &self,
        lb_uuid: &str,
        req: CreateLoadBalancerBackendRequest,
    ) -> Result<LoadBalancerBackend, Error> {
        self.post(&format!("/1.3/load-balancer/{lb_uuid}/backends"), &req)
            .await
    }

    /// Modifies a backend.
    pub async fn modify_load_balancer_backend(
        &self,
        lb_uuid: &str,
        name: &str,
        req: serde_json::Value,
    ) -> Result<LoadBalancerBackend, Error> {
        self.patch(
            &format!("/1.3/load-balancer/{lb_uuid}/backends/{name}"),
            &req,
        )
        .await
    }

    /// Deletes a backend.
    pub async fn delete_load_balancer_backend(&self, lb_uuid: &str, name: &str) -> Result<(), Error> {
        self.delete(&format!("/1.3/load-balancer/{lb_uuid}/backends/{name}"))
            .await
    }

    // ─── Backend members ──────────────────────────────────────────────────────

    /// Returns a list of members for a backend.
    pub async fn list_load_balancer_backend_members(
        &self,
        lb_uuid: &str,
        backend_name: &str,
    ) -> Result<Vec<LoadBalancerBackendMember>, Error> {
        self.get(&format!(
            "/1.3/load-balancer/{lb_uuid}/backends/{backend_name}/members"
        ))
        .await
    }

    /// Creates a member in a backend.
    pub async fn create_load_balancer_backend_member(
        &self,
        lb_uuid: &str,
        backend_name: &str,
        req: CreateLoadBalancerBackendMemberRequest,
    ) -> Result<LoadBalancerBackendMember, Error> {
        self.post(
            &format!("/1.3/load-balancer/{lb_uuid}/backends/{backend_name}/members"),
            &req,
        )
        .await
    }

    /// Deletes a member from a backend.
    pub async fn delete_load_balancer_backend_member(
        &self,
        lb_uuid: &str,
        backend_name: &str,
        member_name: &str,
    ) -> Result<(), Error> {
        self.delete(&format!(
            "/1.3/load-balancer/{lb_uuid}/backends/{backend_name}/members/{member_name}"
        ))
        .await
    }

    // ─── Frontends ────────────────────────────────────────────────────────────

    /// Returns a list of frontends for a load balancer.
    pub async fn list_load_balancer_frontends(&self, lb_uuid: &str) -> Result<Vec<LoadBalancerFrontend>, Error> {
        self.get(&format!("/1.3/load-balancer/{lb_uuid}/frontends"))
            .await
    }

    /// Creates a frontend for a load balancer.
    pub async fn create_load_balancer_frontend(
        &self,
        lb_uuid: &str,
        req: CreateLoadBalancerFrontendRequest,
    ) -> Result<LoadBalancerFrontend, Error> {
        self.post(&format!("/1.3/load-balancer/{lb_uuid}/frontends"), &req)
            .await
    }

    /// Deletes a frontend.
    pub async fn delete_load_balancer_frontend(&self, lb_uuid: &str, name: &str) -> Result<(), Error> {
        self.delete(&format!("/1.3/load-balancer/{lb_uuid}/frontends/{name}"))
            .await
    }

    // ─── Frontend rules ───────────────────────────────────────────────────────

    /// Creates a rule for a frontend.
    pub async fn create_load_balancer_frontend_rule(
        &self,
        lb_uuid: &str,
        frontend_name: &str,
        req: CreateLoadBalancerFrontendRuleRequest,
    ) -> Result<LoadBalancerFrontendRule, Error> {
        self.post(
            &format!("/1.3/load-balancer/{lb_uuid}/frontends/{frontend_name}/rules"),
            &req,
        )
        .await
    }

    /// Deletes a frontend rule.
    pub async fn delete_load_balancer_frontend_rule(
        &self,
        lb_uuid: &str,
        frontend_name: &str,
        rule_name: &str,
    ) -> Result<(), Error> {
        self.delete(&format!(
            "/1.3/load-balancer/{lb_uuid}/frontends/{frontend_name}/rules/{rule_name}"
        ))
        .await
    }

    // ─── Resolvers ────────────────────────────────────────────────────────────

    /// Returns a list of resolvers for a load balancer.
    pub async fn list_load_balancer_resolvers(&self, lb_uuid: &str) -> Result<Vec<LoadBalancerResolver>, Error> {
        self.get(&format!("/1.3/load-balancer/{lb_uuid}/resolvers"))
            .await
    }

    /// Creates a resolver.
    pub async fn create_load_balancer_resolver(
        &self,
        lb_uuid: &str,
        req: CreateLoadBalancerResolverRequest,
    ) -> Result<LoadBalancerResolver, Error> {
        self.post(&format!("/1.3/load-balancer/{lb_uuid}/resolvers"), &req)
            .await
    }

    /// Deletes a resolver.
    pub async fn delete_load_balancer_resolver(&self, lb_uuid: &str, name: &str) -> Result<(), Error> {
        self.delete(&format!("/1.3/load-balancer/{lb_uuid}/resolvers/{name}"))
            .await
    }

    // ─── TLS configs ─────────────────────────────────────────────────────────

    /// Creates a TLS config for a backend.
    pub async fn create_load_balancer_backend_tls_config(
        &self,
        lb_uuid: &str,
        backend_name: &str,
        req: CreateLoadBalancerTLSConfigRequest,
    ) -> Result<LoadBalancerTLSConfig, Error> {
        self.post(
            &format!("/1.3/load-balancer/{lb_uuid}/backends/{backend_name}/tls-configs"),
            &req,
        )
        .await
    }

    /// Creates a TLS config for a frontend.
    pub async fn create_load_balancer_frontend_tls_config(
        &self,
        lb_uuid: &str,
        frontend_name: &str,
        req: CreateLoadBalancerTLSConfigRequest,
    ) -> Result<LoadBalancerTLSConfig, Error> {
        self.post(
            &format!("/1.3/load-balancer/{lb_uuid}/frontends/{frontend_name}/tls-configs"),
            &req,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LB_PLAN_LIST_JSON: &str = r#"[
  {
    "name": "development",
    "per_server_max_sessions": 10000,
    "server_number": 1
  },
  {
    "name": "production",
    "per_server_max_sessions": 50000,
    "server_number": 2
  }
]"#;

    const LB_LIST_JSON: &str = r#"[
  {
    "backends": [
      {
        "created_at": "2022-10-04T16:11:14.569638Z",
        "members": [
          {
            "backup": false,
            "created_at": "2022-10-04T16:11:14.569638Z",
            "enabled": true,
            "ip": "172.16.1.4",
            "max_sessions": 1000,
            "name": "example-member-1",
            "port": 8000,
            "type": "static",
            "weight": 100
          }
        ],
        "name": "example-backend-1",
        "properties": {
          "health_check_type": "http",
          "timeout_server": 10
        },
        "updated_at": "2022-10-04T16:11:14.569638Z"
      },
      {
        "created_at": "2022-10-04T16:11:14.569638Z",
        "members": [
          {
            "enabled": true,
            "ip": "",
            "max_sessions": 1000,
            "name": "member.example.com",
            "port": 8000,
            "type": "dynamic",
            "weight": 100
          }
        ],
        "name": "example-backend-2",
        "resolver": "example-resolver",
        "tls_configs": [],
        "updated_at": "2022-10-04T16:11:14.569638Z"
      }
    ],
    "configured_status": "started",
    "created_at": "2022-10-04T16:11:14.569638Z",
    "dns_name": "lb-0a498284629e4629a55d0415a6e89dda-1.upcloudlb.com",
    "frontends": [
      {
        "created_at": "2022-10-04T16:11:14.569638Z",
        "default_backend": "example-backend-1",
        "mode": "http",
        "name": "example-frontend",
        "networks": [
          {"name": "example-network-1"}
        ],
        "port": 443,
        "properties": {"timeout_client": 5},
        "rules": [
          {
            "actions": [
              {"type": "use_backend", "action_use_backend": {"backend": "example-backend-2"}}
            ],
            "matchers": [
              {"type": "path", "match_path": {"method": "exact", "value": "/app"}}
            ],
            "name": "example-rule-1",
            "priority": 100
          }
        ],
        "tls_configs": [
          {
            "certificate_bundle_uuid": "0a73c569-a2d1-481a-917f-b0fdafd78136",
            "name": "example-tls-config"
          }
        ],
        "updated_at": "2022-10-04T16:11:14.569638Z"
      }
    ],
    "labels": [
      {"key": "env", "value": "staging"},
      {"key": "foo", "value": "bar"}
    ],
    "name": "example-service",
    "networks": [
      {
        "created_at": "2022-10-04T16:11:14.569638Z",
        "dns_name": "lb-0a498284629e4629a55d0415a6e89dda-1.upcloudlb.com",
        "family": "IPv4",
        "name": "example-network-1",
        "type": "public"
      },
      {
        "created_at": "2022-10-04T16:11:14.569638Z",
        "dns_name": "lb-0a498284629e4629a55d0415a6e89dda-2.upcloudlb.com",
        "family": "IPv4",
        "name": "example-network-2",
        "type": "private",
        "uuid": "03631160-d57a-4926-ad48-a2f828229dcb"
      }
    ],
    "operational_state": "pending",
    "plan": "development",
    "resolvers": [
      {
        "cache_invalid": 10,
        "cache_valid": 180,
        "created_at": "2022-10-04T16:11:14.569638Z",
        "name": "example-resolver",
        "nameservers": ["172.16.1.4:53"],
        "retries": 5,
        "timeout": 30,
        "timeout_retry": 10
      }
    ],
    "updated_at": "2022-10-04T16:36:09.933249Z",
    "uuid": "0a498284-629e-4629-a55d-0415a6e89dda",
    "zone": "fi-hel1"
  }
]"#;

    #[test]
    fn deserialize_lb_plan_list() {
        let plans: Vec<LoadBalancerPlan> = serde_json::from_str(LB_PLAN_LIST_JSON).unwrap();
        assert_eq!(plans.len(), 2);
        assert_eq!(plans[0].name, "development");
        assert_eq!(plans[0].per_server_max_sessions, Some(10000));
        assert_eq!(plans[0].server_number, Some(1));
        assert_eq!(plans[0].per_month_price_eur, None);
        assert_eq!(plans[0].per_hour_price_eur, None);
        assert_eq!(plans[0].max_server_count, None);
        assert_eq!(plans[0].max_member_count, None);
        assert_eq!(plans[0].max_backend_count, None);
        assert_eq!(plans[0].max_frontend_count, None);
        assert_eq!(plans[1].name, "production");
        assert_eq!(plans[1].per_server_max_sessions, Some(50000));
        assert_eq!(plans[1].server_number, Some(2));
        assert_eq!(plans[1].per_month_price_eur, None);
        assert_eq!(plans[1].per_hour_price_eur, None);
        assert_eq!(plans[1].max_server_count, None);
        assert_eq!(plans[1].max_member_count, None);
        assert_eq!(plans[1].max_backend_count, None);
        assert_eq!(plans[1].max_frontend_count, None);
    }

    #[test]
    fn deserialize_lb_list() {
        let lbs: Vec<LoadBalancer> = serde_json::from_str(LB_LIST_JSON).unwrap();
        assert_eq!(lbs.len(), 1);
        let lb = &lbs[0];
        assert_eq!(lb.uuid, "0a498284-629e-4629-a55d-0415a6e89dda");
        assert_eq!(lb.name, "example-service");
        assert_eq!(lb.plan, "development");
        assert_eq!(lb.zone, "fi-hel1");
        assert_eq!(lb.state, LoadBalancerOperationalState::Pending);
        assert_eq!(lb.configured_status.as_deref(), Some("started"));
        assert_eq!(lb.created_at.as_deref(), Some("2022-10-04T16:11:14.569638Z"));
        assert_eq!(lb.updated_at.as_deref(), Some("2022-10-04T16:36:09.933249Z"));
        assert_eq!(lb.dns_name.as_deref(), Some("lb-0a498284629e4629a55d0415a6e89dda-1.upcloudlb.com"));
        assert!(lb.maintenance_dow.is_none());
        assert!(lb.maintenance_time.is_none());
        assert!(lb.nodes.is_none());
        assert!(lb.ip_addresses.is_none());

        // Backends
        assert_eq!(lb.backends.len(), 2);
        let be0 = &lb.backends[0];
        assert_eq!(be0.name, "example-backend-1");
        assert_eq!(be0.created_at.as_deref(), Some("2022-10-04T16:11:14.569638Z"));
        assert_eq!(be0.updated_at.as_deref(), Some("2022-10-04T16:11:14.569638Z"));
        assert!(be0.properties.is_some());
        assert_eq!(be0.properties.as_ref().unwrap()["health_check_type"], "http");
        assert_eq!(be0.properties.as_ref().unwrap()["timeout_server"], 10);
        assert!(be0.resolver.is_none());
        assert!(be0.mode.is_none());
        assert!(be0.http_cookie_name.is_none());
        assert!(be0.outbound_proxy_protocol.is_none());
        assert!(be0.health_check.is_none());
        assert_eq!(be0.tls_configs.len(), 0);
        assert_eq!(be0.members.len(), 1);
        let m0 = &be0.members[0];
        assert_eq!(m0.name, "example-member-1");
        assert!(m0.enabled);
        assert_eq!(m0.ip, "172.16.1.4");
        assert_eq!(m0.port, 8000);
        assert_eq!(m0.weight, 100);
        assert_eq!(m0.max_sessions, 1000);
        assert_eq!(m0.member_type, "static");
        assert_eq!(m0.backup, Some(false));
        assert_eq!(m0.created_at.as_deref(), Some("2022-10-04T16:11:14.569638Z"));
        assert!(m0.updated_at.is_none());

        let be1 = &lb.backends[1];
        assert_eq!(be1.name, "example-backend-2");
        assert_eq!(be1.resolver.as_deref(), Some("example-resolver"));
        assert_eq!(be1.created_at.as_deref(), Some("2022-10-04T16:11:14.569638Z"));
        assert_eq!(be1.updated_at.as_deref(), Some("2022-10-04T16:11:14.569638Z"));
        assert_eq!(be1.tls_configs.len(), 0);
        assert!(be1.mode.is_none());
        assert!(be1.http_cookie_name.is_none());
        assert!(be1.outbound_proxy_protocol.is_none());
        assert!(be1.health_check.is_none());
        assert!(be1.properties.is_none());
        assert_eq!(be1.members.len(), 1);
        let m1 = &be1.members[0];
        assert_eq!(m1.name, "member.example.com");
        assert!(m1.enabled);
        assert_eq!(m1.ip, "");
        assert_eq!(m1.port, 8000);
        assert_eq!(m1.weight, 100);
        assert_eq!(m1.max_sessions, 1000);
        assert_eq!(m1.member_type, "dynamic");
        assert!(m1.backup.is_none());
        assert!(m1.created_at.is_none());
        assert!(m1.updated_at.is_none());

        // Frontends
        assert_eq!(lb.frontends.len(), 1);
        let fe = &lb.frontends[0];
        assert_eq!(fe.name, "example-frontend");
        assert_eq!(fe.port, 443);
        assert_eq!(fe.mode, "http");
        assert_eq!(fe.default_backend, "example-backend-1");
        assert_eq!(fe.created_at.as_deref(), Some("2022-10-04T16:11:14.569638Z"));
        assert_eq!(fe.updated_at.as_deref(), Some("2022-10-04T16:11:14.569638Z"));
        assert!(fe.properties.is_some());
        assert_eq!(fe.properties.as_ref().unwrap()["timeout_client"], 5);
        assert_eq!(fe.networks.len(), 1);
        assert_eq!(fe.networks[0].name, "example-network-1");
        assert_eq!(fe.rules.len(), 1);
        assert_eq!(fe.rules[0].name, "example-rule-1");
        assert_eq!(fe.rules[0].priority, 100);
        assert_eq!(fe.rules[0].matchers.len(), 1);
        assert_eq!(fe.rules[0].matchers[0]["type"], "path");
        assert_eq!(fe.rules[0].matchers[0]["match_path"]["method"], "exact");
        assert_eq!(fe.rules[0].matchers[0]["match_path"]["value"], "/app");
        assert_eq!(fe.rules[0].actions.len(), 1);
        assert_eq!(fe.rules[0].actions[0]["type"], "use_backend");
        assert_eq!(fe.rules[0].actions[0]["action_use_backend"]["backend"], "example-backend-2");
        assert!(fe.rules[0].matching_condition.is_none());
        assert!(fe.rules[0].created_at.is_none());
        assert!(fe.rules[0].updated_at.is_none());
        assert_eq!(fe.tls_configs.len(), 1);
        assert_eq!(fe.tls_configs[0].name, "example-tls-config");
        assert_eq!(fe.tls_configs[0].certificate_bundle_uuid.as_deref(), Some("0a73c569-a2d1-481a-917f-b0fdafd78136"));
        assert!(fe.tls_configs[0].certificate_chain.is_none());
        assert!(fe.tls_configs[0].name_match.is_none());
        assert!(fe.tls_configs[0].created_at.is_none());
        assert!(fe.tls_configs[0].updated_at.is_none());

        // Networks
        assert_eq!(lb.networks.len(), 2);
        assert_eq!(lb.networks[0].name, "example-network-1");
        assert_eq!(lb.networks[0].network_type, "public");
        assert_eq!(lb.networks[0].family.as_deref(), Some("IPv4"));
        assert_eq!(lb.networks[0].dns_name.as_deref(), Some("lb-0a498284629e4629a55d0415a6e89dda-1.upcloudlb.com"));
        assert!(lb.networks[0].uuid.is_none());
        assert_eq!(lb.networks[1].name, "example-network-2");
        assert_eq!(lb.networks[1].network_type, "private");
        assert_eq!(lb.networks[1].family.as_deref(), Some("IPv4"));
        assert_eq!(lb.networks[1].dns_name.as_deref(), Some("lb-0a498284629e4629a55d0415a6e89dda-2.upcloudlb.com"));
        assert_eq!(lb.networks[1].uuid.as_deref(), Some("03631160-d57a-4926-ad48-a2f828229dcb"));

        // Resolvers
        assert_eq!(lb.resolvers.len(), 1);
        assert_eq!(lb.resolvers[0].name, "example-resolver");
        assert_eq!(lb.resolvers[0].nameservers, vec!["172.16.1.4:53"]);
        assert_eq!(lb.resolvers[0].retries, 5);
        assert_eq!(lb.resolvers[0].timeout, 30);
        assert_eq!(lb.resolvers[0].timeout_retry, 10);
        assert_eq!(lb.resolvers[0].cache_valid, 180);
        assert_eq!(lb.resolvers[0].cache_invalid, 10);
        assert_eq!(lb.resolvers[0].created_at.as_deref(), Some("2022-10-04T16:11:14.569638Z"));
        assert!(lb.resolvers[0].updated_at.is_none());

        // Labels
        let labels = lb.labels.as_ref().unwrap();
        assert_eq!(labels.len(), 2);
        assert_eq!(labels[0].key, "env");
        assert_eq!(labels[0].value, "staging");
        assert_eq!(labels[1].key, "foo");
        assert_eq!(labels[1].value, "bar");
    }

    // ─── Test 1: Plan detail ──────────────────────────────────────────────────

    #[test]
    fn deserialize_lb_plan_detail() {
        let json = r#"{"name": "development", "per_server_max_sessions": 10000, "server_number": 1}"#;
        let plan: LoadBalancerPlan = serde_json::from_str(json).unwrap();
        assert_eq!(plan.name, "development");
        assert_eq!(plan.per_server_max_sessions, Some(10000));
        assert_eq!(plan.server_number, Some(1));
        assert_eq!(plan.per_month_price_eur, None);
        assert_eq!(plan.per_hour_price_eur, None);
        assert_eq!(plan.max_server_count, None);
        assert_eq!(plan.max_member_count, None);
        assert_eq!(plan.max_backend_count, None);
        assert_eq!(plan.max_frontend_count, None);
    }

    // ─── Test 2: Create LB request ────────────────────────────────────────────

    #[test]
    fn deserialize_create_lb_request() {
        let json = r#"{
            "name": "example-service",
            "plan": "development",
            "zone": "fi-hel1",
            "configured_status": "started",
            "networks": [
                {"name": "example-network-1", "type": "public", "family": "IPv4"},
                {"name": "example-network-2", "type": "private", "family": "IPv4", "uuid": "03631160-d57a-4926-ad48-a2f828229dcb"}
            ]
        }"#;
        let val: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(val["name"], "example-service");
        assert_eq!(val["plan"], "development");
        assert_eq!(val["zone"], "fi-hel1");
        assert_eq!(val["configured_status"], "started");
        assert_eq!(val["networks"].as_array().unwrap().len(), 2);
        assert_eq!(val["networks"][0]["name"], "example-network-1");
        assert_eq!(val["networks"][0]["type"], "public");
        assert_eq!(val["networks"][0]["family"], "IPv4");
        assert!(val["networks"][0].get("uuid").is_none());
        assert_eq!(val["networks"][1]["name"], "example-network-2");
        assert_eq!(val["networks"][1]["type"], "private");
        assert_eq!(val["networks"][1]["family"], "IPv4");
        assert_eq!(val["networks"][1]["uuid"], "03631160-d57a-4926-ad48-a2f828229dcb");

        // Also parse as CreateLoadBalancerRequest
        let req: CreateLoadBalancerRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.name, "example-service");
        assert_eq!(req.plan, "development");
        assert_eq!(req.zone, "fi-hel1");
        assert_eq!(req.configured_status.as_deref(), Some("started"));
        assert!(req.labels.is_none());
        assert_eq!(req.networks.len(), 2);
        assert_eq!(req.networks[0].name, "example-network-1");
        assert_eq!(req.networks[0].network_type, "public");
        assert_eq!(req.networks[0].family.as_deref(), Some("IPv4"));
        assert!(req.networks[0].uuid.is_none());
        assert_eq!(req.networks[1].name, "example-network-2");
        assert_eq!(req.networks[1].network_type, "private");
        assert_eq!(req.networks[1].family.as_deref(), Some("IPv4"));
        assert_eq!(req.networks[1].uuid.as_deref(), Some("03631160-d57a-4926-ad48-a2f828229dcb"));
    }

    // ─── Test 3: LB service details ───────────────────────────────────────────

    #[test]
    fn deserialize_lb_service_details() {
        let json = r#"{
            "backends": [
                {
                    "created_at": "2022-10-04T16:11:14.569638Z",
                    "members": [
                        {"backup": false, "created_at": "2022-10-04T16:11:14.569638Z", "enabled": true, "ip": "172.16.1.4", "max_sessions": 1000, "name": "example-member-1", "port": 8000, "type": "static", "updated_at": "2022-10-04T16:11:14.569638Z", "weight": 100}
                    ],
                    "name": "example-backend-1",
                    "properties": {"health_check_type": "http", "timeout_server": 10},
                    "updated_at": "2022-10-04T16:11:14.569638Z"
                }
            ],
            "configured_status": "started",
            "created_at": "2022-10-04T16:11:14.569638Z",
            "dns_name": "lb-0a498284629e4629a55d0415a6e89dda-1.upcloudlb.com",
            "frontends": [
                {
                    "created_at": "2022-10-04T16:11:14.569638Z",
                    "default_backend": "example-backend-1",
                    "mode": "http",
                    "name": "example-frontend",
                    "networks": [{"name": "example-network-1"}],
                    "port": 443,
                    "properties": {"timeout_client": 5},
                    "rules": [
                        {
                            "actions": [{"type": "use_backend", "action_use_backend": {"backend": "example-backend-2"}}],
                            "created_at": "2022-10-04T16:11:14.569638Z",
                            "matchers": [{"type": "path", "inverse": false, "match_path": {"method": "exact", "value": "/app"}}],
                            "name": "example-rule-1",
                            "priority": 100,
                            "updated_at": "2022-10-04T16:11:14.569638Z"
                        }
                    ],
                    "tls_configs": [
                        {"certificate_bundle_uuid": "0a73c569-a2d1-481a-917f-b0fdafd78136", "created_at": "2022-10-04T16:11:14.569638Z", "name": "example-tls-config", "updated_at": "2022-10-04T16:11:14.569638Z"}
                    ],
                    "updated_at": "2022-10-04T16:11:14.569638Z"
                }
            ],
            "labels": [{"key": "env", "value": "staging"}, {"key": "foo", "value": "bar"}],
            "name": "example-service",
            "networks": [
                {"created_at": "2022-10-04T16:11:14.569638Z", "dns_name": "lb-1.upcloudlb.com", "family": "IPv4", "name": "example-network-1", "type": "public"},
                {"created_at": "2022-10-04T16:11:14.569638Z", "dns_name": "lb-2.upcloudlb.com", "family": "IPv4", "name": "example-network-2", "type": "private", "uuid": "03631160-d57a-4926-ad48-a2f828229dcb"}
            ],
            "operational_state": "pending",
            "plan": "development",
            "resolvers": [
                {"cache_invalid": 10, "cache_valid": 180, "created_at": "2022-10-04T16:11:14.569638Z", "name": "example-resolver", "nameservers": ["172.16.1.4:53"], "retries": 5, "timeout": 30, "timeout_retry": 10, "updated_at": "2022-10-04T16:11:14.569638Z"}
            ],
            "updated_at": "2022-10-04T16:36:09.933249Z",
            "uuid": "0a498284-629e-4629-a55d-0415a6e89dda",
            "zone": "fi-hel1"
        }"#;
        let lb: LoadBalancer = serde_json::from_str(json).unwrap();
        assert_eq!(lb.uuid, "0a498284-629e-4629-a55d-0415a6e89dda");
        assert_eq!(lb.name, "example-service");
        assert_eq!(lb.plan, "development");
        assert_eq!(lb.zone, "fi-hel1");
        assert_eq!(lb.state, LoadBalancerOperationalState::Pending);
        assert_eq!(lb.configured_status.as_deref(), Some("started"));
        assert_eq!(lb.dns_name.as_deref(), Some("lb-0a498284629e4629a55d0415a6e89dda-1.upcloudlb.com"));
        assert_eq!(lb.created_at.as_deref(), Some("2022-10-04T16:11:14.569638Z"));
        assert_eq!(lb.updated_at.as_deref(), Some("2022-10-04T16:36:09.933249Z"));
        assert!(lb.maintenance_dow.is_none());
        assert!(lb.maintenance_time.is_none());
        assert!(lb.nodes.is_none());
        assert!(lb.ip_addresses.is_none());

        // Backends
        assert_eq!(lb.backends.len(), 1);
        let be = &lb.backends[0];
        assert_eq!(be.name, "example-backend-1");
        assert_eq!(be.created_at.as_deref(), Some("2022-10-04T16:11:14.569638Z"));
        assert_eq!(be.updated_at.as_deref(), Some("2022-10-04T16:11:14.569638Z"));
        assert!(be.properties.is_some());
        assert_eq!(be.properties.as_ref().unwrap()["health_check_type"], "http");
        assert_eq!(be.properties.as_ref().unwrap()["timeout_server"], 10);
        assert!(be.resolver.is_none());
        assert!(be.mode.is_none());
        assert!(be.http_cookie_name.is_none());
        assert!(be.outbound_proxy_protocol.is_none());
        assert!(be.health_check.is_none());
        assert_eq!(be.tls_configs.len(), 0);
        assert_eq!(be.members.len(), 1);
        let m = &be.members[0];
        assert_eq!(m.name, "example-member-1");
        assert!(m.enabled);
        assert_eq!(m.ip, "172.16.1.4");
        assert_eq!(m.port, 8000);
        assert_eq!(m.weight, 100);
        assert_eq!(m.max_sessions, 1000);
        assert_eq!(m.member_type, "static");
        assert_eq!(m.backup, Some(false));
        assert_eq!(m.created_at.as_deref(), Some("2022-10-04T16:11:14.569638Z"));
        assert_eq!(m.updated_at.as_deref(), Some("2022-10-04T16:11:14.569638Z"));

        // Frontends
        assert_eq!(lb.frontends.len(), 1);
        let fe = &lb.frontends[0];
        assert_eq!(fe.name, "example-frontend");
        assert_eq!(fe.port, 443);
        assert_eq!(fe.mode, "http");
        assert_eq!(fe.default_backend, "example-backend-1");
        assert_eq!(fe.created_at.as_deref(), Some("2022-10-04T16:11:14.569638Z"));
        assert_eq!(fe.updated_at.as_deref(), Some("2022-10-04T16:11:14.569638Z"));
        assert!(fe.properties.is_some());
        assert_eq!(fe.properties.as_ref().unwrap()["timeout_client"], 5);
        assert_eq!(fe.networks.len(), 1);
        assert_eq!(fe.networks[0].name, "example-network-1");

        // Frontend rules
        assert_eq!(fe.rules.len(), 1);
        assert_eq!(fe.rules[0].name, "example-rule-1");
        assert_eq!(fe.rules[0].priority, 100);
        assert_eq!(fe.rules[0].created_at.as_deref(), Some("2022-10-04T16:11:14.569638Z"));
        assert_eq!(fe.rules[0].updated_at.as_deref(), Some("2022-10-04T16:11:14.569638Z"));
        assert!(fe.rules[0].matching_condition.is_none());
        assert_eq!(fe.rules[0].matchers.len(), 1);
        assert_eq!(fe.rules[0].matchers[0]["type"], "path");
        assert_eq!(fe.rules[0].matchers[0]["inverse"], false);
        assert_eq!(fe.rules[0].matchers[0]["match_path"]["method"], "exact");
        assert_eq!(fe.rules[0].matchers[0]["match_path"]["value"], "/app");
        assert_eq!(fe.rules[0].actions.len(), 1);
        assert_eq!(fe.rules[0].actions[0]["type"], "use_backend");
        assert_eq!(fe.rules[0].actions[0]["action_use_backend"]["backend"], "example-backend-2");

        // Frontend TLS configs
        assert_eq!(fe.tls_configs.len(), 1);
        assert_eq!(fe.tls_configs[0].name, "example-tls-config");
        assert_eq!(fe.tls_configs[0].certificate_bundle_uuid.as_deref(), Some("0a73c569-a2d1-481a-917f-b0fdafd78136"));
        assert_eq!(fe.tls_configs[0].created_at.as_deref(), Some("2022-10-04T16:11:14.569638Z"));
        assert_eq!(fe.tls_configs[0].updated_at.as_deref(), Some("2022-10-04T16:11:14.569638Z"));
        assert!(fe.tls_configs[0].certificate_chain.is_none());
        assert!(fe.tls_configs[0].name_match.is_none());

        // Networks
        assert_eq!(lb.networks.len(), 2);
        assert_eq!(lb.networks[0].name, "example-network-1");
        assert_eq!(lb.networks[0].network_type, "public");
        assert_eq!(lb.networks[0].family.as_deref(), Some("IPv4"));
        assert_eq!(lb.networks[0].dns_name.as_deref(), Some("lb-1.upcloudlb.com"));
        assert!(lb.networks[0].uuid.is_none());
        assert_eq!(lb.networks[1].name, "example-network-2");
        assert_eq!(lb.networks[1].network_type, "private");
        assert_eq!(lb.networks[1].family.as_deref(), Some("IPv4"));
        assert_eq!(lb.networks[1].dns_name.as_deref(), Some("lb-2.upcloudlb.com"));
        assert_eq!(lb.networks[1].uuid.as_deref(), Some("03631160-d57a-4926-ad48-a2f828229dcb"));

        // Resolvers
        assert_eq!(lb.resolvers.len(), 1);
        let r = &lb.resolvers[0];
        assert_eq!(r.name, "example-resolver");
        assert_eq!(r.retries, 5);
        assert_eq!(r.timeout, 30);
        assert_eq!(r.timeout_retry, 10);
        assert_eq!(r.cache_valid, 180);
        assert_eq!(r.cache_invalid, 10);
        assert_eq!(r.nameservers, vec!["172.16.1.4:53"]);
        assert_eq!(r.created_at.as_deref(), Some("2022-10-04T16:11:14.569638Z"));
        assert_eq!(r.updated_at.as_deref(), Some("2022-10-04T16:11:14.569638Z"));

        // Labels
        let labels = lb.labels.as_ref().unwrap();
        assert_eq!(labels.len(), 2);
        assert_eq!(labels[0].key, "env");
        assert_eq!(labels[0].value, "staging");
        assert_eq!(labels[1].key, "foo");
        assert_eq!(labels[1].value, "bar");
    }

    // ─── Test 4: Modify LB request ────────────────────────────────────────────

    #[test]
    fn deserialize_modify_lb_request() {
        let json = r#"{"name": "example-service", "plan": "development", "configured_status": "started"}"#;
        let val: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(val["name"], "example-service");
        assert_eq!(val["plan"], "development");
        assert_eq!(val["configured_status"], "started");

        let req: ModifyLoadBalancerRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.name.as_deref(), Some("example-service"));
        assert_eq!(req.plan.as_deref(), Some("development"));
        assert_eq!(req.configured_status.as_deref(), Some("started"));
        assert!(req.labels.is_none());
    }

    // ─── Test 5: List backends ────────────────────────────────────────────────

    #[test]
    fn deserialize_lb_backend_list() {
        let json = r#"[
            {
                "created_at": "2021-12-07T13:58:30.817272Z",
                "members": [
                    {"backup": false, "created_at": "2022-02-11T16:39:55.321306Z", "enabled": true, "ip": "", "max_sessions": 1000, "name": "member.example.com", "port": 8000, "type": "dynamic", "updated_at": "2022-02-11T17:33:08.490581Z", "weight": 100}
                ],
                "name": "example-backend-2",
                "properties": {"health_check_type": "http", "timeout_server": 10},
                "resolver": "dns",
                "tls_configs": [],
                "updated_at": "2022-02-11T17:33:08.490581Z"
            }
        ]"#;
        let backends: Vec<LoadBalancerBackend> = serde_json::from_str(json).unwrap();
        assert_eq!(backends.len(), 1);
        let be = &backends[0];
        assert_eq!(be.name, "example-backend-2");
        assert_eq!(be.resolver.as_deref(), Some("dns"));
        assert_eq!(be.created_at.as_deref(), Some("2021-12-07T13:58:30.817272Z"));
        assert_eq!(be.updated_at.as_deref(), Some("2022-02-11T17:33:08.490581Z"));
        assert!(be.properties.is_some());
        assert_eq!(be.properties.as_ref().unwrap()["health_check_type"], "http");
        assert_eq!(be.properties.as_ref().unwrap()["timeout_server"], 10);
        assert!(be.mode.is_none());
        assert!(be.http_cookie_name.is_none());
        assert!(be.outbound_proxy_protocol.is_none());
        assert!(be.health_check.is_none());
        assert_eq!(be.tls_configs.len(), 0);
        assert_eq!(be.members.len(), 1);
        assert_eq!(be.members[0].name, "member.example.com");
        assert_eq!(be.members[0].member_type, "dynamic");
        assert_eq!(be.members[0].ip, "");
        assert!(be.members[0].enabled);
        assert_eq!(be.members[0].port, 8000);
        assert_eq!(be.members[0].weight, 100);
        assert_eq!(be.members[0].max_sessions, 1000);
        assert_eq!(be.members[0].backup, Some(false));
        assert_eq!(be.members[0].created_at.as_deref(), Some("2022-02-11T16:39:55.321306Z"));
        assert_eq!(be.members[0].updated_at.as_deref(), Some("2022-02-11T17:33:08.490581Z"));
    }

    // ─── Test 6: Backend detail ───────────────────────────────────────────────

    #[test]
    fn deserialize_lb_backend_detail() {
        let json = r#"{
            "created_at": "2021-12-07T13:58:30.817272Z",
            "members": [
                {"backup": false, "created_at": "2021-12-07T13:58:30.817272Z", "enabled": true, "ip": "172.16.1.4", "max_sessions": 1000, "name": "example-member-1", "port": 8000, "type": "static", "updated_at": "2022-02-11T17:33:08.490581Z", "weight": 100}
            ],
            "name": "example-backend-1",
            "properties": {"health_check_type": "http", "health_check_url": "/health", "timeout_server": 10},
            "resolver": "dns",
            "tls_configs": [
                {"certificate_bundle_uuid": "0a98ddbf-29e3-408c-b081-278a67101f76", "created_at": "2023-07-28T08:01:11.729382Z", "name": "ca", "updated_at": "2023-07-28T08:26:30.436412Z"}
            ],
            "updated_at": "2022-02-11T17:33:08.490581Z"
        }"#;
        let be: LoadBalancerBackend = serde_json::from_str(json).unwrap();
        assert_eq!(be.name, "example-backend-1");
        assert_eq!(be.resolver.as_deref(), Some("dns"));
        assert_eq!(be.created_at.as_deref(), Some("2021-12-07T13:58:30.817272Z"));
        assert_eq!(be.updated_at.as_deref(), Some("2022-02-11T17:33:08.490581Z"));
        assert!(be.mode.is_none());
        assert!(be.http_cookie_name.is_none());
        assert!(be.outbound_proxy_protocol.is_none());
        assert!(be.health_check.is_none());

        // Members
        assert_eq!(be.members.len(), 1);
        let m = &be.members[0];
        assert_eq!(m.name, "example-member-1");
        assert_eq!(m.ip, "172.16.1.4");
        assert_eq!(m.member_type, "static");
        assert!(m.enabled);
        assert_eq!(m.port, 8000);
        assert_eq!(m.weight, 100);
        assert_eq!(m.max_sessions, 1000);
        assert_eq!(m.backup, Some(false));
        assert_eq!(m.created_at.as_deref(), Some("2021-12-07T13:58:30.817272Z"));
        assert_eq!(m.updated_at.as_deref(), Some("2022-02-11T17:33:08.490581Z"));

        // TLS configs
        assert_eq!(be.tls_configs.len(), 1);
        assert_eq!(be.tls_configs[0].name, "ca");
        assert_eq!(be.tls_configs[0].certificate_bundle_uuid.as_deref(), Some("0a98ddbf-29e3-408c-b081-278a67101f76"));
        assert_eq!(be.tls_configs[0].created_at.as_deref(), Some("2023-07-28T08:01:11.729382Z"));
        assert_eq!(be.tls_configs[0].updated_at.as_deref(), Some("2023-07-28T08:26:30.436412Z"));
        assert!(be.tls_configs[0].certificate_chain.is_none());
        assert!(be.tls_configs[0].name_match.is_none());

        // Properties
        let props = be.properties.as_ref().unwrap();
        assert_eq!(props["health_check_type"], "http");
        assert_eq!(props["health_check_url"], "/health");
        assert_eq!(props["timeout_server"], 10);
    }

    // ─── Test 7: List backend members ─────────────────────────────────────────

    #[test]
    fn deserialize_lb_member_list() {
        let json = r#"[
            {"backup": false, "created_at": "2022-02-11T16:39:55.321306Z", "enabled": true, "ip": "", "max_sessions": 1000, "name": "member.example.com", "port": 8000, "type": "dynamic", "updated_at": "2022-02-11T17:33:08.490581Z", "weight": 100},
            {"backup": false, "created_at": "2021-12-07T13:58:30.817272Z", "enabled": true, "ip": "172.16.1.4", "max_sessions": 1000, "name": "example-member-1", "port": 8000, "type": "static", "updated_at": "2022-02-11T17:33:08.490581Z", "weight": 100}
        ]"#;
        let members: Vec<LoadBalancerBackendMember> = serde_json::from_str(json).unwrap();
        assert_eq!(members.len(), 2);
        assert_eq!(members[0].name, "member.example.com");
        assert_eq!(members[0].member_type, "dynamic");
        assert_eq!(members[0].ip, "");
        assert!(members[0].enabled);
        assert_eq!(members[0].port, 8000);
        assert_eq!(members[0].weight, 100);
        assert_eq!(members[0].max_sessions, 1000);
        assert_eq!(members[0].backup, Some(false));
        assert_eq!(members[0].created_at.as_deref(), Some("2022-02-11T16:39:55.321306Z"));
        assert_eq!(members[0].updated_at.as_deref(), Some("2022-02-11T17:33:08.490581Z"));
        assert_eq!(members[1].name, "example-member-1");
        assert_eq!(members[1].member_type, "static");
        assert_eq!(members[1].ip, "172.16.1.4");
        assert!(members[1].enabled);
        assert_eq!(members[1].port, 8000);
        assert_eq!(members[1].weight, 100);
        assert_eq!(members[1].max_sessions, 1000);
        assert_eq!(members[1].backup, Some(false));
        assert_eq!(members[1].created_at.as_deref(), Some("2021-12-07T13:58:30.817272Z"));
        assert_eq!(members[1].updated_at.as_deref(), Some("2022-02-11T17:33:08.490581Z"));
    }

    // ─── Test 8: Member detail ────────────────────────────────────────────────

    #[test]
    fn deserialize_lb_member_detail() {
        let json = r#"{"backup": false, "created_at": "2021-12-07T13:58:30.817272Z", "enabled": true, "ip": "172.16.1.4", "max_sessions": 1000, "name": "example-member-1", "port": 8000, "type": "static", "updated_at": "2022-02-11T17:33:08.490581Z", "weight": 100}"#;
        let m: LoadBalancerBackendMember = serde_json::from_str(json).unwrap();
        assert_eq!(m.name, "example-member-1");
        assert!(m.enabled);
        assert_eq!(m.ip, "172.16.1.4");
        assert_eq!(m.port, 8000);
        assert_eq!(m.weight, 100);
        assert_eq!(m.max_sessions, 1000);
        assert_eq!(m.member_type, "static");
        assert_eq!(m.backup, Some(false));
        assert_eq!(m.created_at.as_deref(), Some("2021-12-07T13:58:30.817272Z"));
        assert_eq!(m.updated_at.as_deref(), Some("2022-02-11T17:33:08.490581Z"));
    }

    // ─── Test 9: List frontends ───────────────────────────────────────────────

    #[test]
    fn deserialize_lb_frontend_list() {
        let json = r#"[
            {
                "created_at": "2021-12-07T13:58:30.817272Z",
                "default_backend": "example-backend-1",
                "mode": "http",
                "name": "example-frontend",
                "networks": [{"name": "example-network-1"}],
                "port": 443,
                "properties": {"timeout_client": 10},
                "rules": [
                    {
                        "actions": [{"type": "use_backend", "action_use_backend": {"backend": "example-backend-2"}}],
                        "created_at": "2021-12-07T13:58:30.817272Z",
                        "matching_condition": "and",
                        "matchers": [{"type": "path", "match_path": {"method": "exact", "value": "/app"}}],
                        "name": "example-rule-1",
                        "priority": 100,
                        "updated_at": "2022-02-11T17:33:08.490581Z"
                    }
                ],
                "tls_configs": [
                    {"certificate_bundle_uuid": "0aded5c1-c7a3-498a-b9c8-a871611c47a2", "created_at": "2022-02-11T17:33:08.490581Z", "name": "example-tls-config", "updated_at": "2022-02-11T17:33:08.490581Z"}
                ],
                "updated_at": "2022-02-11T17:33:08.490581Z"
            }
        ]"#;
        let frontends: Vec<LoadBalancerFrontend> = serde_json::from_str(json).unwrap();
        assert_eq!(frontends.len(), 1);
        let fe = &frontends[0];
        assert_eq!(fe.name, "example-frontend");
        assert_eq!(fe.port, 443);
        assert_eq!(fe.mode, "http");
        assert_eq!(fe.default_backend, "example-backend-1");
        assert_eq!(fe.created_at.as_deref(), Some("2021-12-07T13:58:30.817272Z"));
        assert_eq!(fe.updated_at.as_deref(), Some("2022-02-11T17:33:08.490581Z"));
        assert_eq!(fe.networks.len(), 1);
        assert_eq!(fe.networks[0].name, "example-network-1");
        assert!(fe.properties.is_some());
        assert_eq!(fe.properties.as_ref().unwrap()["timeout_client"], 10);

        // Rules
        assert_eq!(fe.rules.len(), 1);
        assert_eq!(fe.rules[0].name, "example-rule-1");
        assert_eq!(fe.rules[0].priority, 100);
        assert_eq!(fe.rules[0].matching_condition.as_deref(), Some("and"));
        assert_eq!(fe.rules[0].created_at.as_deref(), Some("2021-12-07T13:58:30.817272Z"));
        assert_eq!(fe.rules[0].updated_at.as_deref(), Some("2022-02-11T17:33:08.490581Z"));
        assert_eq!(fe.rules[0].matchers.len(), 1);
        assert_eq!(fe.rules[0].matchers[0]["type"], "path");
        assert_eq!(fe.rules[0].matchers[0]["match_path"]["method"], "exact");
        assert_eq!(fe.rules[0].matchers[0]["match_path"]["value"], "/app");
        assert_eq!(fe.rules[0].actions.len(), 1);
        assert_eq!(fe.rules[0].actions[0]["type"], "use_backend");
        assert_eq!(fe.rules[0].actions[0]["action_use_backend"]["backend"], "example-backend-2");

        // TLS configs
        assert_eq!(fe.tls_configs.len(), 1);
        assert_eq!(fe.tls_configs[0].name, "example-tls-config");
        assert_eq!(fe.tls_configs[0].certificate_bundle_uuid.as_deref(), Some("0aded5c1-c7a3-498a-b9c8-a871611c47a2"));
        assert_eq!(fe.tls_configs[0].created_at.as_deref(), Some("2022-02-11T17:33:08.490581Z"));
        assert_eq!(fe.tls_configs[0].updated_at.as_deref(), Some("2022-02-11T17:33:08.490581Z"));
        assert!(fe.tls_configs[0].certificate_chain.is_none());
        assert!(fe.tls_configs[0].name_match.is_none());
    }

    // ─── Test 10: Frontend detail ─────────────────────────────────────────────

    #[test]
    fn deserialize_lb_frontend_detail() {
        let json = r#"{
            "created_at": "2021-12-07T13:58:30.817272Z",
            "default_backend": "example-backend-1",
            "mode": "http",
            "name": "example-frontend",
            "networks": [{"name": "example-network-1"}],
            "port": 443,
            "properties": {"timeout_client": 10},
            "rules": [
                {
                    "actions": [{"type": "use_backend", "action_use_backend": {"backend": "example-backend-2"}}],
                    "created_at": "2021-12-07T13:58:30.817272Z",
                    "matching_condition": "and",
                    "matchers": [{"type": "path", "match_path": {"method": "exact", "value": "/app"}}],
                    "name": "example-rule-1",
                    "priority": 100,
                    "updated_at": "2022-02-11T17:33:08.490581Z"
                }
            ],
            "tls_configs": [
                {"certificate_bundle_uuid": "0aded5c1-c7a3-498a-b9c8-a871611c47a2", "created_at": "2022-02-11T17:33:08.490581Z", "name": "example-tls-config", "updated_at": "2022-02-11T17:33:08.490581Z"}
            ],
            "updated_at": "2022-02-11T17:33:08.490581Z"
        }"#;
        let fe: LoadBalancerFrontend = serde_json::from_str(json).unwrap();
        assert_eq!(fe.name, "example-frontend");
        assert_eq!(fe.port, 443);
        assert_eq!(fe.mode, "http");
        assert_eq!(fe.default_backend, "example-backend-1");
        assert_eq!(fe.created_at.as_deref(), Some("2021-12-07T13:58:30.817272Z"));
        assert_eq!(fe.updated_at.as_deref(), Some("2022-02-11T17:33:08.490581Z"));
        assert_eq!(fe.networks.len(), 1);
        assert_eq!(fe.networks[0].name, "example-network-1");
        assert!(fe.properties.is_some());
        assert_eq!(fe.properties.as_ref().unwrap()["timeout_client"], 10);
        assert_eq!(fe.rules.len(), 1);
        assert_eq!(fe.rules[0].name, "example-rule-1");
        assert_eq!(fe.rules[0].priority, 100);
        assert_eq!(fe.rules[0].matching_condition.as_deref(), Some("and"));
        assert_eq!(fe.rules[0].created_at.as_deref(), Some("2021-12-07T13:58:30.817272Z"));
        assert_eq!(fe.rules[0].updated_at.as_deref(), Some("2022-02-11T17:33:08.490581Z"));
        assert_eq!(fe.rules[0].matchers.len(), 1);
        assert_eq!(fe.rules[0].matchers[0]["type"], "path");
        assert_eq!(fe.rules[0].matchers[0]["match_path"]["method"], "exact");
        assert_eq!(fe.rules[0].matchers[0]["match_path"]["value"], "/app");
        assert_eq!(fe.rules[0].actions.len(), 1);
        assert_eq!(fe.rules[0].actions[0]["type"], "use_backend");
        assert_eq!(fe.rules[0].actions[0]["action_use_backend"]["backend"], "example-backend-2");
        assert_eq!(fe.tls_configs.len(), 1);
        assert_eq!(fe.tls_configs[0].name, "example-tls-config");
        assert_eq!(fe.tls_configs[0].certificate_bundle_uuid.as_deref(), Some("0aded5c1-c7a3-498a-b9c8-a871611c47a2"));
        assert_eq!(fe.tls_configs[0].created_at.as_deref(), Some("2022-02-11T17:33:08.490581Z"));
        assert_eq!(fe.tls_configs[0].updated_at.as_deref(), Some("2022-02-11T17:33:08.490581Z"));
        assert!(fe.tls_configs[0].certificate_chain.is_none());
        assert!(fe.tls_configs[0].name_match.is_none());
    }

    // ─── Test 11: Frontend TLS config list ────────────────────────────────────

    #[test]
    fn deserialize_lb_frontend_tls_config_list() {
        let json = r#"[
            {"certificate_bundle_uuid": "0aded5c1-c7a3-498a-b9c8-a871611c47a2", "created_at": "2022-02-11T17:33:08.490581Z", "name": "example-tls-config-1", "updated_at": "2022-02-11T17:33:08.490581Z"},
            {"certificate_bundle_uuid": "c136c429-d18f-4192-b55e-e7b5079dee40", "created_at": "2021-11-11T11:21:51.121789Z", "name": "example-tls-config-2", "updated_at": "2021-11-11T11:21:51.121789Z"}
        ]"#;
        let configs: Vec<LoadBalancerTLSConfig> = serde_json::from_str(json).unwrap();
        assert_eq!(configs.len(), 2);
        assert_eq!(configs[0].name, "example-tls-config-1");
        assert_eq!(configs[0].certificate_bundle_uuid.as_deref(), Some("0aded5c1-c7a3-498a-b9c8-a871611c47a2"));
        assert_eq!(configs[0].created_at.as_deref(), Some("2022-02-11T17:33:08.490581Z"));
        assert_eq!(configs[0].updated_at.as_deref(), Some("2022-02-11T17:33:08.490581Z"));
        assert!(configs[0].certificate_chain.is_none());
        assert!(configs[0].name_match.is_none());
        assert_eq!(configs[1].name, "example-tls-config-2");
        assert_eq!(configs[1].certificate_bundle_uuid.as_deref(), Some("c136c429-d18f-4192-b55e-e7b5079dee40"));
        assert_eq!(configs[1].created_at.as_deref(), Some("2021-11-11T11:21:51.121789Z"));
        assert_eq!(configs[1].updated_at.as_deref(), Some("2021-11-11T11:21:51.121789Z"));
        assert!(configs[1].certificate_chain.is_none());
        assert!(configs[1].name_match.is_none());
    }

    // ─── Test 12: Frontend TLS config detail ──────────────────────────────────

    #[test]
    fn deserialize_lb_frontend_tls_config_detail() {
        let json = r#"{"certificate_bundle_uuid": "0aded5c1-c7a3-498a-b9c8-a871611c47a2", "created_at": "2022-02-11T17:33:08.490581Z", "name": "example-tls-config", "updated_at": "2022-02-11T17:33:08.490581Z"}"#;
        let config: LoadBalancerTLSConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.name, "example-tls-config");
        assert_eq!(config.certificate_bundle_uuid.as_deref(), Some("0aded5c1-c7a3-498a-b9c8-a871611c47a2"));
        assert_eq!(config.created_at.as_deref(), Some("2022-02-11T17:33:08.490581Z"));
        assert_eq!(config.updated_at.as_deref(), Some("2022-02-11T17:33:08.490581Z"));
        assert_eq!(config.certificate_chain, None);
        assert_eq!(config.name_match, None);
    }

    // ─── Test 13: Frontend rule list ──────────────────────────────────────────

    #[test]
    fn deserialize_lb_rule_list() {
        let json = r#"[
            {
                "actions": [{"type": "use_backend", "action_use_backend": {"backend": "example-backend-2"}}],
                "created_at": "2021-12-07T13:58:30.817272Z",
                "matching_condition": "and",
                "matchers": [{"type": "path", "match_path": {"method": "exact", "value": "/app"}}],
                "name": "example-rule-1",
                "priority": 100,
                "updated_at": "2022-02-11T17:33:08.490581Z"
            }
        ]"#;
        let rules: Vec<LoadBalancerFrontendRule> = serde_json::from_str(json).unwrap();
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].name, "example-rule-1");
        assert_eq!(rules[0].priority, 100);
        assert_eq!(rules[0].matching_condition.as_deref(), Some("and"));
        assert_eq!(rules[0].created_at.as_deref(), Some("2021-12-07T13:58:30.817272Z"));
        assert_eq!(rules[0].updated_at.as_deref(), Some("2022-02-11T17:33:08.490581Z"));
        assert_eq!(rules[0].matchers.len(), 1);
        assert_eq!(rules[0].actions.len(), 1);
        assert_eq!(rules[0].actions[0]["type"], "use_backend");
        assert_eq!(rules[0].actions[0]["action_use_backend"]["backend"], "example-backend-2");
        assert_eq!(rules[0].matchers[0]["type"], "path");
        assert_eq!(rules[0].matchers[0]["match_path"]["method"], "exact");
        assert_eq!(rules[0].matchers[0]["match_path"]["value"], "/app");
    }

    // ─── Test 14: Rule detail ─────────────────────────────────────────────────

    #[test]
    fn deserialize_lb_rule_detail() {
        let json = r#"{
            "actions": [{"type": "use_backend", "action_use_backend": {"backend": "example-backend-2"}}],
            "created_at": "2021-12-07T13:58:30.817272Z",
            "matching_condition": "and",
            "matchers": [{"type": "path", "match_path": {"method": "exact", "value": "/app"}}],
            "name": "example-rule-1",
            "priority": 100,
            "updated_at": "2022-02-11T17:33:08.490581Z"
        }"#;
        let rule: LoadBalancerFrontendRule = serde_json::from_str(json).unwrap();
        assert_eq!(rule.name, "example-rule-1");
        assert_eq!(rule.priority, 100);
        assert_eq!(rule.matching_condition.as_deref(), Some("and"));
        assert_eq!(rule.created_at.as_deref(), Some("2021-12-07T13:58:30.817272Z"));
        assert_eq!(rule.updated_at.as_deref(), Some("2022-02-11T17:33:08.490581Z"));
        assert_eq!(rule.matchers.len(), 1);
        assert_eq!(rule.matchers[0]["type"], "path");
        assert_eq!(rule.matchers[0]["match_path"]["method"], "exact");
        assert_eq!(rule.matchers[0]["match_path"]["value"], "/app");
        assert_eq!(rule.actions.len(), 1);
        assert_eq!(rule.actions[0]["type"], "use_backend");
        assert_eq!(rule.actions[0]["action_use_backend"]["backend"], "example-backend-2");
    }

    // ─── Test 15: List resolvers ──────────────────────────────────────────────

    #[test]
    fn deserialize_lb_resolver_list() {
        let json = r#"[
            {
                "cache_invalid": 30,
                "cache_valid": 180,
                "created_at": "2022-02-11T16:39:55.321306Z",
                "name": "example-resolver",
                "nameservers": ["172.16.1.250"],
                "retries": 5,
                "timeout": 30,
                "timeout_retry": 10,
                "updated_at": "2022-02-11T17:33:08.490581Z"
            }
        ]"#;
        let resolvers: Vec<LoadBalancerResolver> = serde_json::from_str(json).unwrap();
        assert_eq!(resolvers.len(), 1);
        let r = &resolvers[0];
        assert_eq!(r.name, "example-resolver");
        assert_eq!(r.nameservers, vec!["172.16.1.250"]);
        assert_eq!(r.retries, 5);
        assert_eq!(r.timeout, 30);
        assert_eq!(r.timeout_retry, 10);
        assert_eq!(r.cache_valid, 180);
        assert_eq!(r.cache_invalid, 30);
        assert_eq!(r.created_at.as_deref(), Some("2022-02-11T16:39:55.321306Z"));
        assert_eq!(r.updated_at.as_deref(), Some("2022-02-11T17:33:08.490581Z"));
    }

    // ─── Test 16: Resolver detail ─────────────────────────────────────────────

    #[test]
    fn deserialize_lb_resolver_detail() {
        let json = r#"{
            "cache_invalid": 30,
            "cache_valid": 180,
            "created_at": "2022-02-11T16:39:55.321306Z",
            "name": "example-resolver",
            "nameservers": ["172.16.1.250"],
            "retries": 5,
            "timeout": 30,
            "timeout_retry": 10,
            "updated_at": "2022-02-11T17:33:08.490581Z"
        }"#;
        let r: LoadBalancerResolver = serde_json::from_str(json).unwrap();
        assert_eq!(r.name, "example-resolver");
        assert_eq!(r.nameservers, vec!["172.16.1.250"]);
        assert_eq!(r.retries, 5);
        assert_eq!(r.timeout, 30);
        assert_eq!(r.timeout_retry, 10);
        assert_eq!(r.cache_valid, 180);
        assert_eq!(r.cache_invalid, 30);
        assert_eq!(r.created_at.as_deref(), Some("2022-02-11T16:39:55.321306Z"));
        assert_eq!(r.updated_at.as_deref(), Some("2022-02-11T17:33:08.490581Z"));
    }

    // ─── Test 17: Create resolver request ─────────────────────────────────────

    #[test]
    fn deserialize_create_resolver_request() {
        let json = r#"{"name": "example-resolver", "nameservers": ["172.16.1.4:53"], "retries": 5, "timeout": 30, "timeout_retry": 10, "cache_valid": 180, "cache_invalid": 10}"#;
        let req: CreateLoadBalancerResolverRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.name, "example-resolver");
        assert_eq!(req.nameservers, vec!["172.16.1.4:53"]);
        assert_eq!(req.retries, Some(5));
        assert_eq!(req.timeout, Some(30));
        assert_eq!(req.timeout_retry, Some(10));
        assert_eq!(req.cache_valid, Some(180));
        assert_eq!(req.cache_invalid, Some(10));
    }

    // ─── Test 18: Create backend request ──────────────────────────────────────

    #[test]
    fn deserialize_create_backend_request() {
        let json = r#"{
            "name": "example-backend-2",
            "resolver": "example-resolver",
            "members": [
                {"name": "member1.example.com", "port": 8000, "type": "dynamic", "max_sessions": 1000, "weight": 100, "enabled": true}
            ],
            "properties": {"timeout_server": 30, "health_check_type": "tcp"}
        }"#;
        let val: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(val["name"], "example-backend-2");
        assert_eq!(val["resolver"], "example-resolver");
        assert_eq!(val["members"].as_array().unwrap().len(), 1);
        assert_eq!(val["members"][0]["name"], "member1.example.com");
        assert_eq!(val["members"][0]["type"], "dynamic");
        assert_eq!(val["members"][0]["port"], 8000);
        assert_eq!(val["members"][0]["max_sessions"], 1000);
        assert_eq!(val["members"][0]["weight"], 100);
        assert_eq!(val["members"][0]["enabled"], true);
        assert_eq!(val["properties"]["health_check_type"], "tcp");
        assert_eq!(val["properties"]["timeout_server"], 30);
    }

    // ─── Test 19: Create frontend request ─────────────────────────────────────

    #[test]
    fn deserialize_create_frontend_request() {
        let json = r#"{
            "name": "example-frontend",
            "mode": "http",
            "port": 443,
            "default_backend": "example-backend",
            "networks": [{"name": "example-network-1"}],
            "properties": {"timeout_client": 5}
        }"#;
        let req: CreateLoadBalancerFrontendRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.name, "example-frontend");
        assert_eq!(req.mode, "http");
        assert_eq!(req.port, 443);
        assert_eq!(req.default_backend, "example-backend");
        assert_eq!(req.networks.len(), 1);
        assert_eq!(req.networks[0].name, "example-network-1");
        assert!(req.properties.is_some());
        assert_eq!(req.properties.as_ref().unwrap()["timeout_client"], 5);
    }

    // ─── Test 20: Create frontend rule request ────────────────────────────────

    #[test]
    fn deserialize_create_frontend_rule_request() {
        let json = r#"{
            "name": "example-rule-1",
            "priority": 100,
            "matchers": [{"type": "path", "match_path": {"method": "exact", "value": "/app"}}],
            "actions": [{"type": "use_backend", "action_use_backend": {"backend": "example-backend-2"}}]
        }"#;
        let req: CreateLoadBalancerFrontendRuleRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.name, "example-rule-1");
        assert_eq!(req.priority, 100);
        assert_eq!(req.matchers.len(), 1);
        assert_eq!(req.matchers[0]["type"], "path");
        assert_eq!(req.matchers[0]["match_path"]["method"], "exact");
        assert_eq!(req.matchers[0]["match_path"]["value"], "/app");
        assert_eq!(req.actions.len(), 1);
        assert_eq!(req.actions[0]["type"], "use_backend");
        assert_eq!(req.actions[0]["action_use_backend"]["backend"], "example-backend-2");
    }

    // ─── Test 21: Backend TLS config list ─────────────────────────────────────

    #[test]
    fn deserialize_lb_backend_tls_config_list() {
        let json = r#"[
            {"certificate_bundle_uuid": "0aded5c1-c7a3-498a-b9c8-a871611c47a2", "created_at": "2022-02-11T17:33:08.490581Z", "name": "example-backend-tls-1", "updated_at": "2022-02-11T17:33:08.490581Z"},
            {"certificate_bundle_uuid": "c136c429-d18f-4192-b55e-e7b5079dee40", "created_at": "2021-11-11T11:21:51.121789Z", "name": "example-backend-tls-2", "updated_at": "2021-11-11T11:21:51.121789Z"}
        ]"#;
        let configs: Vec<LoadBalancerTLSConfig> = serde_json::from_str(json).unwrap();
        assert_eq!(configs.len(), 2);
        assert_eq!(configs[0].name, "example-backend-tls-1");
        assert_eq!(configs[0].certificate_bundle_uuid.as_deref(), Some("0aded5c1-c7a3-498a-b9c8-a871611c47a2"));
        assert_eq!(configs[0].created_at.as_deref(), Some("2022-02-11T17:33:08.490581Z"));
        assert_eq!(configs[0].updated_at.as_deref(), Some("2022-02-11T17:33:08.490581Z"));
        assert!(configs[0].certificate_chain.is_none());
        assert!(configs[0].name_match.is_none());
        assert_eq!(configs[1].name, "example-backend-tls-2");
        assert_eq!(configs[1].certificate_bundle_uuid.as_deref(), Some("c136c429-d18f-4192-b55e-e7b5079dee40"));
        assert_eq!(configs[1].created_at.as_deref(), Some("2021-11-11T11:21:51.121789Z"));
        assert_eq!(configs[1].updated_at.as_deref(), Some("2021-11-11T11:21:51.121789Z"));
        assert!(configs[1].certificate_chain.is_none());
        assert!(configs[1].name_match.is_none());
    }

    // ─── Test 22: Create TLS config request ───────────────────────────────────

    #[test]
    fn deserialize_create_tls_config_request() {
        let json = r#"{"name": "example-tls-config", "certificate_bundle_uuid": "0aded5c1-c7a3-498a-b9c8-a871611c47a2"}"#;
        let val: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(val["name"], "example-tls-config");
        assert_eq!(val["certificate_bundle_uuid"], "0aded5c1-c7a3-498a-b9c8-a871611c47a2");

        let req: CreateLoadBalancerTLSConfigRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.name, "example-tls-config");
        assert!(req.certificate_chain.is_none());
        assert!(req.private_key.is_none());
        assert!(req.name_match.is_none());
    }
}
