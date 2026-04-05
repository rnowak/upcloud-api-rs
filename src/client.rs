use std::time::Duration;

use base64::Engine;
use reqwest::{Method, RequestBuilder, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    config::{AuthMode, ClientConfig},
    error::{ApiError, Error, ErrorEnvelope},
};

/// The main UpCloud API client.
///
/// Construct with [`Client::builder()`] or [`Client::with_bearer_token()`].
#[derive(Clone, Debug)]
pub struct Client {
    pub(crate) http: reqwest::Client,
    pub(crate) config: ClientConfig,
}

impl Client {
    /// Create a builder for configuring the client.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// Shorthand: create a client authenticated with a bearer token.
    pub fn with_bearer_token(token: impl Into<String>) -> Result<Self, Error> {
        ClientBuilder::default()
            .bearer_token(token)
            .build()
    }

    /// Shorthand: create a client authenticated with username + password (legacy Basic auth).
    pub fn with_basic_auth(
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<Self, Error> {
        ClientBuilder::default()
            .basic_auth(username, password)
            .build()
    }

    // ─── Domain handlers ───────────────────────────────────────────────────

    pub fn account(&self) -> crate::account::AccountHandler<'_> {
        crate::account::AccountHandler::new(self)
    }

    pub fn audit_log(&self) -> crate::audit_log::AuditLogHandler<'_> {
        crate::audit_log::AuditLogHandler::new(self)
    }

    pub fn file_storage(&self) -> crate::file_storage::FileStorageHandler<'_> {
        crate::file_storage::FileStorageHandler::new(self)
    }

    pub fn gateway(&self) -> crate::gateway::GatewayHandler<'_> {
        crate::gateway::GatewayHandler::new(self)
    }

    pub fn hosts(&self) -> crate::host::HostHandler<'_> {
        crate::host::HostHandler::new(self)
    }

    pub fn ip_addresses(&self) -> crate::ip_address::IpAddressHandler<'_> {
        crate::ip_address::IpAddressHandler::new(self)
    }

    pub fn kubernetes(&self) -> crate::kubernetes::KubernetesHandler<'_> {
        crate::kubernetes::KubernetesHandler::new(self)
    }

    pub fn load_balancer(&self) -> crate::load_balancer::LoadBalancerHandler<'_> {
        crate::load_balancer::LoadBalancerHandler::new(self)
    }

    pub fn managed_database(&self) -> crate::managed_database::DatabaseHandler<'_> {
        crate::managed_database::DatabaseHandler::new(self)
    }

    pub fn networks(&self) -> crate::network::NetworkHandler<'_> {
        crate::network::NetworkHandler::new(self)
    }

    pub fn object_storage(&self) -> crate::object_storage::ObjectStorageHandler<'_> {
        crate::object_storage::ObjectStorageHandler::new(self)
    }

    pub fn partners(&self) -> crate::partner::PartnerHandler<'_> {
        crate::partner::PartnerHandler::new(self)
    }

    pub fn permissions(&self) -> crate::permission::PermissionHandler<'_> {
        crate::permission::PermissionHandler::new(self)
    }

    pub fn plans(&self) -> crate::plan::PlanHandler<'_> {
        crate::plan::PlanHandler::new(self)
    }

    pub fn pricing(&self) -> crate::pricing::PricingHandler<'_> {
        crate::pricing::PricingHandler::new(self)
    }

    pub fn servers(&self) -> crate::server::ServerHandler<'_> {
        crate::server::ServerHandler::new(self)
    }

    pub fn storages(&self) -> crate::storage::StorageHandler<'_> {
        crate::storage::StorageHandler::new(self)
    }

    pub fn tags(&self) -> crate::tag::TagHandler<'_> {
        crate::tag::TagHandler::new(self)
    }

    pub fn timezones(&self) -> crate::timezone::TimezoneHandler<'_> {
        crate::timezone::TimezoneHandler::new(self)
    }

    pub fn zones(&self) -> crate::zone::ZoneHandler<'_> {
        crate::zone::ZoneHandler::new(self)
    }

    // ─── Low-level HTTP helpers ──────────────────────────────────────────────

    pub(crate) fn url(&self, path: &str) -> String {
        format!("{}{}", self.config.base_url, path)
    }

    pub(crate) fn request(&self, method: Method, path: &str) -> RequestBuilder {
        let mut req = self.http.request(method, self.url(path));
        req = match &self.config.auth {
            AuthMode::Bearer(token) => req.bearer_auth(token),
            AuthMode::Basic { username, password } => {
                let creds = base64::engine::general_purpose::STANDARD
                    .encode(format!("{username}:{password}"));
                req.header("Authorization", format!("Basic {creds}"))
            }
        };
        req.header("Accept", "application/json")
            .header("Content-Type", "application/json")
    }

    /// Execute a request and deserialize the JSON response body.
    pub(crate) async fn send<T: DeserializeOwned>(&self, req: RequestBuilder) -> Result<T, Error> {
        let resp = req.send().await?;
        parse_response(resp).await
    }

    /// Execute a request and return nothing on success (204 No Content or similar).
    pub(crate) async fn send_empty(&self, req: RequestBuilder) -> Result<(), Error> {
        let resp = req.send().await?;
        check_response_status(resp).await
    }

    // ─── Convenience methods ─────────────────────────────────────────────────

    pub(crate) async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let req = self.request(Method::GET, path);
        self.send(req).await
    }

    pub(crate) async fn get_with_query<T: DeserializeOwned, Q: Serialize>(
        &self,
        path: &str,
        query: &Q,
    ) -> Result<T, Error> {
        let req = self.request(Method::GET, path).query(query);
        self.send(req).await
    }

    pub(crate) async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, Error> {
        let req = self.request(Method::POST, path).json(body);
        self.send(req).await
    }

    pub(crate) async fn post_empty<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let req = self.request(Method::POST, path);
        self.send(req).await
    }

    pub(crate) async fn post_no_response<B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<(), Error> {
        let req = self.request(Method::POST, path).json(body);
        self.send_empty(req).await
    }

    pub(crate) async fn post_no_response_empty(&self, path: &str) -> Result<(), Error> {
        let req = self.request(Method::POST, path);
        self.send_empty(req).await
    }

    pub(crate) async fn put<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, Error> {
        let req = self.request(Method::PUT, path).json(body);
        self.send(req).await
    }

    pub(crate) async fn patch<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, Error> {
        let req = self.request(Method::PATCH, path).json(body);
        self.send(req).await
    }

    pub(crate) async fn delete(&self, path: &str) -> Result<(), Error> {
        let req = self.request(Method::DELETE, path);
        self.send_empty(req).await
    }

    pub(crate) async fn delete_with_body<B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<(), Error> {
        let req = self.request(Method::DELETE, path).json(body);
        self.send_empty(req).await
    }
}

// ─── Response parsing ─────────────────────────────────────────────────────────

async fn parse_response<T: DeserializeOwned>(resp: Response) -> Result<T, Error> {
    let status = resp.status();
    if status.is_success() {
        let body = resp.bytes().await?;
        serde_json::from_slice(&body).map_err(Error::Json)
    } else {
        Err(map_error_response(status, resp).await)
    }
}

async fn check_response_status(resp: Response) -> Result<(), Error> {
    let status = resp.status();
    if status.is_success() {
        Ok(())
    } else {
        Err(map_error_response(status, resp).await)
    }
}

async fn map_error_response(status: StatusCode, resp: Response) -> Error {
    let code = status.as_u16();
    let body = resp.bytes().await.unwrap_or_default();
    if let Ok(env) = serde_json::from_slice::<ErrorEnvelope>(&body) {
        Error::Api(ApiError {
            error_code: env.error.error_code,
            error_message: env.error.error_message,
            status: code,
        })
    } else {
        let message = String::from_utf8_lossy(&body).into_owned();
        Error::Api(ApiError {
            error_code: "UNKNOWN".to_string(),
            error_message: if message.is_empty() {
                format!("HTTP {code}")
            } else {
                message
            },
            status: code,
        })
    }
}

// ─── Builder ─────────────────────────────────────────────────────────────────

/// Builder for [`Client`].
#[derive(Debug, Default)]
pub struct ClientBuilder {
    auth: Option<AuthMode>,
    base_url: Option<String>,
    timeout: Option<Duration>,
}

impl ClientBuilder {
    /// Authenticate with a bearer token (API token, `ucat_...`).
    pub fn bearer_token(mut self, token: impl Into<String>) -> Self {
        self.auth = Some(AuthMode::Bearer(token.into()));
        self
    }

    /// Authenticate with username and password (HTTP Basic, legacy).
    pub fn basic_auth(
        mut self,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        self.auth = Some(AuthMode::Basic {
            username: username.into(),
            password: password.into(),
        });
        self
    }

    /// Override the base URL (default: `https://api.upcloud.com`).
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Override the request timeout (default: 30 seconds).
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Build the [`Client`].
    pub fn build(self) -> Result<Client, Error> {
        let auth = self
            .auth
            .ok_or_else(|| Error::InvalidConfig("authentication must be configured".to_string()))?;

        let mut config = ClientConfig::new(auth);
        if let Some(url) = self.base_url {
            config = config.with_base_url(url);
        }
        if let Some(t) = self.timeout {
            config = config.with_timeout(t);
        }

        let http = reqwest::Client::builder()
            .timeout(config.timeout)
            .use_rustls_tls()
            .build()
            .map_err(Error::Http)?;

        Ok(Client { http, config })
    }
}
