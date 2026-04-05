//! Load Balancer — `GET/POST/PATCH/DELETE /1.3/load-balancer` + sub-resources.

mod types;

pub use types::*;

use crate::client::Client;
use crate::error::Error;
use crate::types::common::PageParams;

/// Handler for load balancer operations.
pub struct LoadBalancerHandler<'a> {
    client: &'a Client,
}

impl<'a> LoadBalancerHandler<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    // ─── Plans ────────────────────────────────────────────────────────────────

    /// Returns a list of available load balancer plans.
    pub async fn list_plans(&self, params: PageParams) -> Result<Vec<Plan>, Error> {
        self.client
            .get_with_query("/1.3/load-balancer/plans", &params)
            .await
    }

    /// Returns details for a specific load balancer plan.
    pub async fn get_plan(&self, name: &str) -> Result<Plan, Error> {
        self.client
            .get(&format!("/1.3/load-balancer/plans/{name}"))
            .await
    }

    // ─── Services ─────────────────────────────────────────────────────────────

    /// Returns a list of all load balancers.
    pub async fn list(&self, params: PageParams) -> Result<Vec<LoadBalancer>, Error> {
        self.client
            .get_with_query("/1.3/load-balancer", &params)
            .await
    }

    /// Returns details for a specific load balancer.
    pub async fn get(&self, uuid: &str) -> Result<LoadBalancer, Error> {
        self.client
            .get(&format!("/1.3/load-balancer/{uuid}"))
            .await
    }

    /// Creates a new load balancer.
    pub async fn create(&self, req: CreateRequest) -> Result<LoadBalancer, Error> {
        self.client.post("/1.3/load-balancer", &req).await
    }

    /// Modifies an existing load balancer.
    pub async fn modify(
        &self,
        uuid: &str,
        req: ModifyRequest,
    ) -> Result<LoadBalancer, Error> {
        self.client
            .patch(&format!("/1.3/load-balancer/{uuid}"), &req)
            .await
    }

    /// Deletes a load balancer.
    pub async fn delete(&self, uuid: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/load-balancer/{uuid}"))
            .await
    }

    // ─── Backends ─────────────────────────────────────────────────────────────

    /// Returns a list of backends for a load balancer.
    pub async fn list_backends(&self, lb_uuid: &str) -> Result<Vec<Backend>, Error> {
        self.client
            .get(&format!("/1.3/load-balancer/{lb_uuid}/backends"))
            .await
    }

    /// Returns details for a specific backend.
    pub async fn get_backend(&self, lb_uuid: &str, name: &str) -> Result<Backend, Error> {
        self.client
            .get(&format!("/1.3/load-balancer/{lb_uuid}/backends/{name}"))
            .await
    }

    /// Creates a backend for a load balancer.
    pub async fn create_backend(
        &self,
        lb_uuid: &str,
        req: CreateBackendRequest,
    ) -> Result<Backend, Error> {
        self.client
            .post(&format!("/1.3/load-balancer/{lb_uuid}/backends"), &req)
            .await
    }

    /// Modifies a backend.
    pub async fn modify_backend(
        &self,
        lb_uuid: &str,
        name: &str,
        req: serde_json::Value,
    ) -> Result<Backend, Error> {
        self.client
            .patch(
                &format!("/1.3/load-balancer/{lb_uuid}/backends/{name}"),
                &req,
            )
            .await
    }

    /// Deletes a backend.
    pub async fn delete_backend(&self, lb_uuid: &str, name: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/load-balancer/{lb_uuid}/backends/{name}"))
            .await
    }

    // ─── Backend members ──────────────────────────────────────────────────────

    /// Returns a list of members for a backend.
    pub async fn list_backend_members(
        &self,
        lb_uuid: &str,
        backend_name: &str,
    ) -> Result<Vec<BackendMember>, Error> {
        self.client
            .get(&format!(
                "/1.3/load-balancer/{lb_uuid}/backends/{backend_name}/members"
            ))
            .await
    }

    /// Creates a member in a backend.
    pub async fn create_backend_member(
        &self,
        lb_uuid: &str,
        backend_name: &str,
        req: CreateBackendMemberRequest,
    ) -> Result<BackendMember, Error> {
        self.client
            .post(
                &format!("/1.3/load-balancer/{lb_uuid}/backends/{backend_name}/members"),
                &req,
            )
            .await
    }

    /// Deletes a member from a backend.
    pub async fn delete_backend_member(
        &self,
        lb_uuid: &str,
        backend_name: &str,
        member_name: &str,
    ) -> Result<(), Error> {
        self.client
            .delete(&format!(
                "/1.3/load-balancer/{lb_uuid}/backends/{backend_name}/members/{member_name}"
            ))
            .await
    }

    // ─── Frontends ────────────────────────────────────────────────────────────

    /// Returns a list of frontends for a load balancer.
    pub async fn list_frontends(&self, lb_uuid: &str) -> Result<Vec<Frontend>, Error> {
        self.client
            .get(&format!("/1.3/load-balancer/{lb_uuid}/frontends"))
            .await
    }

    /// Creates a frontend for a load balancer.
    pub async fn create_frontend(
        &self,
        lb_uuid: &str,
        req: CreateFrontendRequest,
    ) -> Result<Frontend, Error> {
        self.client
            .post(&format!("/1.3/load-balancer/{lb_uuid}/frontends"), &req)
            .await
    }

    /// Deletes a frontend.
    pub async fn delete_frontend(&self, lb_uuid: &str, name: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/load-balancer/{lb_uuid}/frontends/{name}"))
            .await
    }

    // ─── Frontend rules ───────────────────────────────────────────────────────

    /// Creates a rule for a frontend.
    pub async fn create_frontend_rule(
        &self,
        lb_uuid: &str,
        frontend_name: &str,
        req: CreateFrontendRuleRequest,
    ) -> Result<FrontendRule, Error> {
        self.client
            .post(
                &format!("/1.3/load-balancer/{lb_uuid}/frontends/{frontend_name}/rules"),
                &req,
            )
            .await
    }

    /// Deletes a frontend rule.
    pub async fn delete_frontend_rule(
        &self,
        lb_uuid: &str,
        frontend_name: &str,
        rule_name: &str,
    ) -> Result<(), Error> {
        self.client
            .delete(&format!(
                "/1.3/load-balancer/{lb_uuid}/frontends/{frontend_name}/rules/{rule_name}"
            ))
            .await
    }

    // ─── Resolvers ────────────────────────────────────────────────────────────

    /// Returns a list of resolvers for a load balancer.
    pub async fn list_resolvers(&self, lb_uuid: &str) -> Result<Vec<Resolver>, Error> {
        self.client
            .get(&format!("/1.3/load-balancer/{lb_uuid}/resolvers"))
            .await
    }

    /// Creates a resolver.
    pub async fn create_resolver(
        &self,
        lb_uuid: &str,
        req: CreateResolverRequest,
    ) -> Result<Resolver, Error> {
        self.client
            .post(&format!("/1.3/load-balancer/{lb_uuid}/resolvers"), &req)
            .await
    }

    /// Deletes a resolver.
    pub async fn delete_resolver(&self, lb_uuid: &str, name: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/load-balancer/{lb_uuid}/resolvers/{name}"))
            .await
    }

    // ─── TLS configs ─────────────────────────────────────────────────────────

    /// Creates a TLS config for a backend.
    pub async fn create_backend_tls_config(
        &self,
        lb_uuid: &str,
        backend_name: &str,
        req: CreateTLSConfigRequest,
    ) -> Result<TLSConfig, Error> {
        self.client
            .post(
                &format!("/1.3/load-balancer/{lb_uuid}/backends/{backend_name}/tls-configs"),
                &req,
            )
            .await
    }

    /// Creates a TLS config for a frontend.
    pub async fn create_frontend_tls_config(
        &self,
        lb_uuid: &str,
        frontend_name: &str,
        req: CreateTLSConfigRequest,
    ) -> Result<TLSConfig, Error> {
        self.client
            .post(
                &format!("/1.3/load-balancer/{lb_uuid}/frontends/{frontend_name}/tls-configs"),
                &req,
            )
            .await
    }
}

#[cfg(test)]
mod tests;
