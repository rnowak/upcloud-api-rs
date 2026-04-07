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
    ///
    /// `GET /1.3/load-balancer/plans`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#create-service)
    pub async fn list_plans(&self, params: PageParams) -> Result<Vec<Plan>, Error> {
        self.client
            .get_with_query("/1.3/load-balancer/plans", &params)
            .await
    }

    /// Returns details for a specific load balancer plan.
    ///
    /// `GET /1.3/load-balancer/plans/{name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#create-service)
    pub async fn get_plan(&self, name: &str) -> Result<Plan, Error> {
        self.client
            .get(&format!("/1.3/load-balancer/plans/{name}"))
            .await
    }

    // ─── Services ─────────────────────────────────────────────────────────────

    /// Returns a list of all load balancers.
    ///
    /// `GET /1.3/load-balancer`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#list-services)
    pub async fn list(&self, params: PageParams) -> Result<Vec<LoadBalancer>, Error> {
        self.client
            .get_with_query("/1.3/load-balancer", &params)
            .await
    }

    /// Returns details for a specific load balancer.
    ///
    /// `GET /1.3/load-balancer/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#get-service-details)
    pub async fn get(&self, uuid: &str) -> Result<LoadBalancer, Error> {
        self.client.get(&format!("/1.3/load-balancer/{uuid}")).await
    }

    /// Creates a new load balancer.
    ///
    /// `POST /1.3/load-balancer`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#create-service)
    pub async fn create(&self, req: CreateRequest) -> Result<LoadBalancer, Error> {
        self.client.post("/1.3/load-balancer", &req).await
    }

    /// Modifies an existing load balancer.
    ///
    /// `PATCH /1.3/load-balancer/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#modify-service)
    pub async fn modify(&self, uuid: &str, req: ModifyRequest) -> Result<LoadBalancer, Error> {
        self.client
            .patch(&format!("/1.3/load-balancer/{uuid}"), &req)
            .await
    }

    /// Deletes a load balancer.
    ///
    /// `DELETE /1.3/load-balancer/{uuid}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#delete-service)
    pub async fn delete(&self, uuid: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/load-balancer/{uuid}"))
            .await
    }

    // ─── Backends ─────────────────────────────────────────────────────────────

    /// Returns a list of backends for a load balancer.
    ///
    /// `GET /1.3/load-balancer/{uuid}/backends`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#list-backends)
    pub async fn list_backends(&self, lb_uuid: &str) -> Result<Vec<Backend>, Error> {
        self.client
            .get(&format!("/1.3/load-balancer/{lb_uuid}/backends"))
            .await
    }

    /// Returns details for a specific backend.
    ///
    /// `GET /1.3/load-balancer/{uuid}/backends/{name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#get-backend-details)
    pub async fn get_backend(&self, lb_uuid: &str, name: &str) -> Result<Backend, Error> {
        self.client
            .get(&format!("/1.3/load-balancer/{lb_uuid}/backends/{name}"))
            .await
    }

    /// Creates a backend for a load balancer.
    ///
    /// `POST /1.3/load-balancer/{uuid}/backends`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#create-backend)
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
    ///
    /// `PATCH /1.3/load-balancer/{uuid}/backends/{name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#modify-backend)
    pub async fn modify_backend(
        &self,
        lb_uuid: &str,
        name: &str,
        req: ModifyBackendRequest,
    ) -> Result<Backend, Error> {
        self.client
            .patch(
                &format!("/1.3/load-balancer/{lb_uuid}/backends/{name}"),
                &req,
            )
            .await
    }

    /// Deletes a backend.
    ///
    /// `DELETE /1.3/load-balancer/{uuid}/backends/{name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#delete-backend)
    pub async fn delete_backend(&self, lb_uuid: &str, name: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/load-balancer/{lb_uuid}/backends/{name}"))
            .await
    }

    // ─── Backend members ──────────────────────────────────────────────────────

    /// Returns a list of members for a backend.
    ///
    /// `GET /1.3/load-balancer/{uuid}/backends/{name}/members`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#list-members)
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

    /// Returns details for a specific backend member.
    ///
    /// `GET /1.3/load-balancer/{uuid}/backends/{backend_name}/members/{member_name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#get-member-details)
    pub async fn get_backend_member(
        &self,
        lb_uuid: &str,
        backend_name: &str,
        member_name: &str,
    ) -> Result<BackendMember, Error> {
        self.client
            .get(&format!(
                "/1.3/load-balancer/{lb_uuid}/backends/{backend_name}/members/{member_name}"
            ))
            .await
    }

    /// Creates a member in a backend.
    ///
    /// `POST /1.3/load-balancer/{uuid}/backends/{name}/members`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#create-static-member)
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

    /// Modifies a backend member.
    ///
    /// `PATCH /1.3/load-balancer/{uuid}/backends/{backend_name}/members/{member_name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#modify-member)
    pub async fn modify_backend_member(
        &self,
        lb_uuid: &str,
        backend_name: &str,
        member_name: &str,
        req: ModifyBackendMemberRequest,
    ) -> Result<BackendMember, Error> {
        self.client
            .patch(
                &format!(
                    "/1.3/load-balancer/{lb_uuid}/backends/{backend_name}/members/{member_name}"
                ),
                &req,
            )
            .await
    }

    /// Deletes a member from a backend.
    ///
    /// `DELETE /1.3/load-balancer/{uuid}/backends/{name}/members/{member_name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#delete-member)
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
    ///
    /// `GET /1.3/load-balancer/{uuid}/frontends`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#list-frontends)
    pub async fn list_frontends(&self, lb_uuid: &str) -> Result<Vec<Frontend>, Error> {
        self.client
            .get(&format!("/1.3/load-balancer/{lb_uuid}/frontends"))
            .await
    }

    /// Returns details for a specific frontend.
    ///
    /// `GET /1.3/load-balancer/{uuid}/frontends/{name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#get-frontend-details)
    pub async fn get_frontend(&self, lb_uuid: &str, name: &str) -> Result<Frontend, Error> {
        self.client
            .get(&format!("/1.3/load-balancer/{lb_uuid}/frontends/{name}"))
            .await
    }

    /// Creates a frontend for a load balancer.
    ///
    /// `POST /1.3/load-balancer/{uuid}/frontends`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#create-frontend)
    pub async fn create_frontend(
        &self,
        lb_uuid: &str,
        req: CreateFrontendRequest,
    ) -> Result<Frontend, Error> {
        self.client
            .post(&format!("/1.3/load-balancer/{lb_uuid}/frontends"), &req)
            .await
    }

    /// Modifies a frontend.
    ///
    /// `PATCH /1.3/load-balancer/{uuid}/frontends/{name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#modify-frontend)
    pub async fn modify_frontend(
        &self,
        lb_uuid: &str,
        name: &str,
        req: ModifyFrontendRequest,
    ) -> Result<Frontend, Error> {
        self.client
            .patch(
                &format!("/1.3/load-balancer/{lb_uuid}/frontends/{name}"),
                &req,
            )
            .await
    }

    /// Deletes a frontend.
    ///
    /// `DELETE /1.3/load-balancer/{uuid}/frontends/{name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#delete-frontend)
    pub async fn delete_frontend(&self, lb_uuid: &str, name: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/load-balancer/{lb_uuid}/frontends/{name}"))
            .await
    }

    // ─── Frontend rules ───────────────────────────────────────────────────────

    /// Returns a list of rules for a frontend.
    ///
    /// `GET /1.3/load-balancer/{uuid}/frontends/{frontend_name}/rules`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#list-rules)
    pub async fn list_frontend_rules(
        &self,
        lb_uuid: &str,
        frontend_name: &str,
    ) -> Result<Vec<FrontendRule>, Error> {
        self.client
            .get(&format!(
                "/1.3/load-balancer/{lb_uuid}/frontends/{frontend_name}/rules"
            ))
            .await
    }

    /// Returns details for a specific frontend rule.
    ///
    /// `GET /1.3/load-balancer/{uuid}/frontends/{frontend_name}/rules/{rule_name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#get-rule-details)
    pub async fn get_frontend_rule(
        &self,
        lb_uuid: &str,
        frontend_name: &str,
        rule_name: &str,
    ) -> Result<FrontendRule, Error> {
        self.client
            .get(&format!(
                "/1.3/load-balancer/{lb_uuid}/frontends/{frontend_name}/rules/{rule_name}"
            ))
            .await
    }

    /// Creates a rule for a frontend.
    ///
    /// `POST /1.3/load-balancer/{uuid}/frontends/{name}/rules`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#create-rule)
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

    /// Modifies a frontend rule.
    ///
    /// `PATCH /1.3/load-balancer/{uuid}/frontends/{frontend_name}/rules/{rule_name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#modify-rule)
    pub async fn modify_frontend_rule(
        &self,
        lb_uuid: &str,
        frontend_name: &str,
        rule_name: &str,
        req: ModifyFrontendRuleRequest,
    ) -> Result<FrontendRule, Error> {
        self.client
            .patch(
                &format!(
                    "/1.3/load-balancer/{lb_uuid}/frontends/{frontend_name}/rules/{rule_name}"
                ),
                &req,
            )
            .await
    }

    /// Replaces a frontend rule.
    ///
    /// `PUT /1.3/load-balancer/{uuid}/frontends/{frontend_name}/rules/{rule_name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#replace-rule)
    pub async fn replace_frontend_rule(
        &self,
        lb_uuid: &str,
        frontend_name: &str,
        rule_name: &str,
        req: ReplaceFrontendRuleRequest,
    ) -> Result<FrontendRule, Error> {
        self.client
            .put(
                &format!(
                    "/1.3/load-balancer/{lb_uuid}/frontends/{frontend_name}/rules/{rule_name}"
                ),
                &req,
            )
            .await
    }

    /// Deletes a frontend rule.
    ///
    /// `DELETE /1.3/load-balancer/{uuid}/frontends/{name}/rules/{rule_name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#delete-rule)
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
    ///
    /// `GET /1.3/load-balancer/{uuid}/resolvers`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#list-resolvers)
    pub async fn list_resolvers(&self, lb_uuid: &str) -> Result<Vec<Resolver>, Error> {
        self.client
            .get(&format!("/1.3/load-balancer/{lb_uuid}/resolvers"))
            .await
    }

    /// Returns details for a specific resolver.
    ///
    /// `GET /1.3/load-balancer/{uuid}/resolvers/{name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#get-resolver-details)
    pub async fn get_resolver(&self, lb_uuid: &str, name: &str) -> Result<Resolver, Error> {
        self.client
            .get(&format!("/1.3/load-balancer/{lb_uuid}/resolvers/{name}"))
            .await
    }

    /// Creates a resolver.
    ///
    /// `POST /1.3/load-balancer/{uuid}/resolvers`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#create-resolver)
    pub async fn create_resolver(
        &self,
        lb_uuid: &str,
        req: CreateResolverRequest,
    ) -> Result<Resolver, Error> {
        self.client
            .post(&format!("/1.3/load-balancer/{lb_uuid}/resolvers"), &req)
            .await
    }

    /// Modifies a resolver.
    ///
    /// `PATCH /1.3/load-balancer/{uuid}/resolvers/{name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#modify-resolver)
    pub async fn modify_resolver(
        &self,
        lb_uuid: &str,
        name: &str,
        req: ModifyResolverRequest,
    ) -> Result<Resolver, Error> {
        self.client
            .patch(
                &format!("/1.3/load-balancer/{lb_uuid}/resolvers/{name}"),
                &req,
            )
            .await
    }

    /// Deletes a resolver.
    ///
    /// `DELETE /1.3/load-balancer/{uuid}/resolvers/{name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#delete-resolver)
    pub async fn delete_resolver(&self, lb_uuid: &str, name: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/load-balancer/{lb_uuid}/resolvers/{name}"))
            .await
    }

    // ─── Backend TLS configs ─────────────────────────────────────────────────

    /// Returns a list of TLS configs for a backend.
    ///
    /// `GET /1.3/load-balancer/{uuid}/backends/{backend_name}/tls-configs`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#list-backend-tls-configs)
    pub async fn list_backend_tls_configs(
        &self,
        lb_uuid: &str,
        backend_name: &str,
    ) -> Result<Vec<TLSConfig>, Error> {
        self.client
            .get(&format!(
                "/1.3/load-balancer/{lb_uuid}/backends/{backend_name}/tls-configs"
            ))
            .await
    }

    /// Returns details for a specific backend TLS config.
    ///
    /// `GET /1.3/load-balancer/{uuid}/backends/{backend_name}/tls-configs/{config_name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#get-backend-tls-config-details)
    pub async fn get_backend_tls_config(
        &self,
        lb_uuid: &str,
        backend_name: &str,
        config_name: &str,
    ) -> Result<TLSConfig, Error> {
        self.client
            .get(&format!(
                "/1.3/load-balancer/{lb_uuid}/backends/{backend_name}/tls-configs/{config_name}"
            ))
            .await
    }

    /// Creates a TLS config for a backend.
    ///
    /// `POST /1.3/load-balancer/{uuid}/backends/{name}/tls-configs`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#create-backend-tls-config)
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

    /// Deletes a backend TLS config.
    ///
    /// `DELETE /1.3/load-balancer/{uuid}/backends/{backend_name}/tls-configs/{config_name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#delete-backend-tls-config)
    pub async fn delete_backend_tls_config(
        &self,
        lb_uuid: &str,
        backend_name: &str,
        config_name: &str,
    ) -> Result<(), Error> {
        self.client
            .delete(&format!(
                "/1.3/load-balancer/{lb_uuid}/backends/{backend_name}/tls-configs/{config_name}"
            ))
            .await
    }

    /// Modifies a backend TLS config.
    ///
    /// `PATCH /1.3/load-balancer/{uuid}/backends/{backend_name}/tls-configs/{config_name}`
    pub async fn modify_backend_tls_config(
        &self,
        lb_uuid: &str,
        backend_name: &str,
        config_name: &str,
        req: CreateTLSConfigRequest,
    ) -> Result<TLSConfig, Error> {
        self.client
            .patch(
                &format!(
                    "/1.3/load-balancer/{lb_uuid}/backends/{backend_name}/tls-configs/{config_name}"
                ),
                &req,
            )
            .await
    }

    // ─── Frontend TLS configs ─────────────────────────────────────────────────

    /// Returns a list of TLS configs for a frontend.
    ///
    /// `GET /1.3/load-balancer/{uuid}/frontends/{frontend_name}/tls-configs`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#list-frontend-tls-configs)
    pub async fn list_frontend_tls_configs(
        &self,
        lb_uuid: &str,
        frontend_name: &str,
    ) -> Result<Vec<TLSConfig>, Error> {
        self.client
            .get(&format!(
                "/1.3/load-balancer/{lb_uuid}/frontends/{frontend_name}/tls-configs"
            ))
            .await
    }

    /// Returns details for a specific frontend TLS config.
    ///
    /// `GET /1.3/load-balancer/{uuid}/frontends/{frontend_name}/tls-configs/{config_name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#get-frontend-tls-config-details)
    pub async fn get_frontend_tls_config(
        &self,
        lb_uuid: &str,
        frontend_name: &str,
        config_name: &str,
    ) -> Result<TLSConfig, Error> {
        self.client
            .get(&format!(
                "/1.3/load-balancer/{lb_uuid}/frontends/{frontend_name}/tls-configs/{config_name}"
            ))
            .await
    }

    /// Creates a TLS config for a frontend.
    ///
    /// `POST /1.3/load-balancer/{uuid}/frontends/{name}/tls-configs`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#create-frontend-tls-config)
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

    /// Deletes a frontend TLS config.
    ///
    /// `DELETE /1.3/load-balancer/{uuid}/frontends/{frontend_name}/tls-configs/{config_name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#delete-frontend-tls-config)
    pub async fn delete_frontend_tls_config(
        &self,
        lb_uuid: &str,
        frontend_name: &str,
        config_name: &str,
    ) -> Result<(), Error> {
        self.client
            .delete(&format!(
                "/1.3/load-balancer/{lb_uuid}/frontends/{frontend_name}/tls-configs/{config_name}"
            ))
            .await
    }

    /// Modifies a frontend TLS config.
    ///
    /// `PATCH /1.3/load-balancer/{uuid}/frontends/{frontend_name}/tls-configs/{config_name}`
    pub async fn modify_frontend_tls_config(
        &self,
        lb_uuid: &str,
        frontend_name: &str,
        config_name: &str,
        req: CreateTLSConfigRequest,
    ) -> Result<TLSConfig, Error> {
        self.client
            .patch(
                &format!(
                    "/1.3/load-balancer/{lb_uuid}/frontends/{frontend_name}/tls-configs/{config_name}"
                ),
                &req,
            )
            .await
    }

    // ─── Certificate bundles ──────────────────────────────────────────────────

    /// Returns a list of certificate bundles for a load balancer.
    ///
    /// `GET /1.3/load-balancer/{uuid}/certificate-bundles`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#list-certificate-bundles)
    pub async fn list_certificate_bundles(
        &self,
        lb_uuid: &str,
    ) -> Result<Vec<CertificateBundle>, Error> {
        self.client
            .get(&format!("/1.3/load-balancer/{lb_uuid}/certificate-bundles"))
            .await
    }

    /// Returns details for a specific certificate bundle.
    ///
    /// `GET /1.3/load-balancer/{uuid}/certificate-bundles/{id}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#get-certificate-bundle-details)
    pub async fn get_certificate_bundle(
        &self,
        lb_uuid: &str,
        id: &str,
    ) -> Result<CertificateBundle, Error> {
        self.client
            .get(&format!(
                "/1.3/load-balancer/{lb_uuid}/certificate-bundles/{id}"
            ))
            .await
    }

    /// Creates a certificate bundle for a load balancer.
    ///
    /// `POST /1.3/load-balancer/{uuid}/certificate-bundles`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#create-certificate-bundle)
    pub async fn create_certificate_bundle(
        &self,
        lb_uuid: &str,
        req: CreateCertificateBundleRequest,
    ) -> Result<CertificateBundle, Error> {
        self.client
            .post(
                &format!("/1.3/load-balancer/{lb_uuid}/certificate-bundles"),
                &req,
            )
            .await
    }

    /// Modifies a certificate bundle.
    ///
    /// `PATCH /1.3/load-balancer/{uuid}/certificate-bundles/{id}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#modify-certificate-bundle)
    pub async fn modify_certificate_bundle(
        &self,
        lb_uuid: &str,
        id: &str,
        req: ModifyCertificateBundleRequest,
    ) -> Result<CertificateBundle, Error> {
        self.client
            .patch(
                &format!("/1.3/load-balancer/{lb_uuid}/certificate-bundles/{id}"),
                &req,
            )
            .await
    }

    /// Deletes a certificate bundle.
    ///
    /// `DELETE /1.3/load-balancer/{uuid}/certificate-bundles/{id}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#delete-certificate-bundle)
    pub async fn delete_certificate_bundle(&self, lb_uuid: &str, id: &str) -> Result<(), Error> {
        self.client
            .delete(&format!(
                "/1.3/load-balancer/{lb_uuid}/certificate-bundles/{id}"
            ))
            .await
    }

    // ─── Networks ─────────────────────────────────────────────────────────────

    /// Modifies a load balancer network.
    ///
    /// `PATCH /1.3/load-balancer/{uuid}/networks/{network_name}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/17-managed-loadbalancer/#modify-network)
    pub async fn modify_network(
        &self,
        lb_uuid: &str,
        network_name: &str,
        req: ModifyLoadBalancerNetworkRequest,
    ) -> Result<Network, Error> {
        self.client
            .patch(
                &format!("/1.3/load-balancer/{lb_uuid}/networks/{network_name}"),
                &req,
            )
            .await
    }

    // ─── IP Addresses ─────────────────────────────────────────────────────────

    /// Returns a list of IP addresses assigned to a load balancer.
    ///
    /// `GET /1.3/load-balancer/{uuid}/ip-addresses`
    pub async fn list_ip_addresses(&self, lb_uuid: &str) -> Result<Vec<IPAddress>, Error> {
        self.client
            .get(&format!("/1.3/load-balancer/{lb_uuid}/ip-addresses"))
            .await
    }

    /// Adds an IP address to a load balancer.
    ///
    /// `POST /1.3/load-balancer/{uuid}/ip-addresses`
    pub async fn add_ip_address(
        &self,
        lb_uuid: &str,
        req: AddIPAddressRequest,
    ) -> Result<IPAddress, Error> {
        self.client
            .post(
                &format!("/1.3/load-balancer/{lb_uuid}/ip-addresses"),
                &req,
            )
            .await
    }

    /// Removes an IP address from a load balancer.
    ///
    /// `DELETE /1.3/load-balancer/{uuid}/ip-addresses/{address}`
    pub async fn remove_ip_address(
        &self,
        lb_uuid: &str,
        address: &str,
    ) -> Result<(), Error> {
        self.client
            .delete(&format!(
                "/1.3/load-balancer/{lb_uuid}/ip-addresses/{address}"
            ))
            .await
    }

    // ─── DNS Challenge ────────────────────────────────────────────────────────

    /// Returns the DNS challenge domain for load balancer certificate validation.
    ///
    /// `GET /1.3/load-balancer/dns-challenge`
    pub async fn get_dns_challenge_domain(&self) -> Result<DnsChallengeDomain, Error> {
        self.client
            .get("/1.3/load-balancer/dns-challenge")
            .await
    }
}

#[cfg(test)]
mod tests;
