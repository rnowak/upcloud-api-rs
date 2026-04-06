//! Partner API — `GET/POST /1.3/partner/accounts`

mod types;
pub use types::*;

use crate::client::Client;
use crate::error::Error;

// ─── Handler ──────────────────────────────────────────────────────────────────

/// Handler for partner API operations.
pub struct PartnerHandler<'a> {
    client: &'a Client,
}

impl<'a> PartnerHandler<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Lists all partner-managed customer accounts.
    ///
    /// `GET /1.3/partner/accounts`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/23-partner-api/#get-account-list)
    pub async fn list(&self) -> Result<Vec<PartnerAccount>, Error> {
        let resp: Vec<PartnerAccount> = self.client.get("/1.3/partner/accounts").await?;
        Ok(resp)
    }

    /// Returns details for a specific partner-managed account.
    ///
    /// `GET /1.3/partner/accounts/{username}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/23-partner-api/#get-account-list)
    pub async fn get(&self, username: &str) -> Result<PartnerAccount, Error> {
        self.client
            .get(&format!("/1.3/partner/accounts/{username}"))
            .await
    }

    /// Modifies a partner-managed customer account.
    ///
    /// `PATCH /1.3/partner/accounts/{username}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/23-partner-api/#create-new-account)
    pub async fn modify(
        &self,
        username: &str,
        req: ModifyPartnerAccountRequest,
    ) -> Result<PartnerAccount, Error> {
        self.client
            .patch(&format!("/1.3/partner/accounts/{username}"), &req)
            .await
    }

    /// Creates a new partner-managed customer account.
    ///
    /// `POST /1.3/partner/accounts`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/23-partner-api/#create-new-account)
    pub async fn create(&self, req: CreatePartnerAccountRequest) -> Result<PartnerAccount, Error> {
        self.client.post("/1.3/partner/accounts", &req).await
    }
}

#[cfg(test)]
mod tests;
