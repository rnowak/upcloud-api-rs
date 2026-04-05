//! Accounts, subaccounts, and API tokens.
//!
//! - Accounts: `GET /1.3/account`, `/1.3/account/list`, `/1.3/account/details/{username}`, etc.
//! - API Tokens: `POST/GET/DELETE /1.3/account/tokens` (NOT `/1.3/token`).

mod types;
pub use types::*;

use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::Error;

// ─── Response wrappers ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct AccountResponse {
    account: Account,
}

#[derive(Debug, Deserialize)]
struct AccountListResponse {
    accounts: AccountListWrapper,
}

#[derive(Debug, Deserialize)]
struct AccountListWrapper {
    account: Vec<AccountEntry>,
}

// ─── Handler ──────────────────────────────────────────────────────────────────

pub struct AccountHandler<'a> {
    client: &'a Client,
}

impl<'a> AccountHandler<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    // ─── Account ──────────────────────────────────────────────────────────────

    /// Returns account information for the current account.
    pub async fn get(&self) -> Result<Account, Error> {
        let resp: AccountResponse = self.client.get("/1.3/account").await?;
        Ok(resp.account)
    }

    /// Returns a list of all accounts (including subaccounts).
    pub async fn list(&self) -> Result<Vec<AccountEntry>, Error> {
        let resp: AccountListResponse = self.client.get("/1.3/account/list").await?;
        Ok(resp.accounts.account)
    }

    /// Returns details for a specific account or subaccount.
    pub async fn get_details(&self, username: &str) -> Result<Account, Error> {
        let resp: AccountResponse = self
            .client
            .get(&format!("/1.3/account/details/{username}"))
            .await?;
        Ok(resp.account)
    }

    /// Updates account details.
    pub async fn update_details(
        &self,
        username: &str,
        req: UpdateAccountDetailsRequest,
    ) -> Result<Account, Error> {
        #[derive(Serialize)]
        struct Body {
            account: UpdateAccountDetailsRequest,
        }
        let resp: AccountResponse = self
            .client
            .put(
                &format!("/1.3/account/details/{username}"),
                &Body { account: req },
            )
            .await?;
        Ok(resp.account)
    }

    /// Creates a new subaccount.
    pub async fn create_subaccount(&self, req: CreateSubaccountRequest) -> Result<Account, Error> {
        #[derive(Serialize)]
        struct Body {
            account: CreateSubaccountRequest,
        }
        let resp: AccountResponse = self
            .client
            .post("/1.3/account/sub", &Body { account: req })
            .await?;
        Ok(resp.account)
    }

    /// Updates an existing subaccount.
    pub async fn update_subaccount(
        &self,
        username: &str,
        req: UpdateAccountDetailsRequest,
    ) -> Result<Account, Error> {
        #[derive(Serialize)]
        struct Body {
            account: UpdateAccountDetailsRequest,
        }
        let resp: AccountResponse = self
            .client
            .put(
                &format!("/1.3/account/sub/{username}"),
                &Body { account: req },
            )
            .await?;
        Ok(resp.account)
    }

    /// Deletes a subaccount.
    pub async fn delete_subaccount(&self, username: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/account/sub/{username}"))
            .await
    }

    // ─── API Tokens ───────────────────────────────────────────────────────────

    /// Creates a new API token.
    ///
    /// The `token` value in the response is only returned once.
    pub async fn create_token(
        &self,
        req: CreateTokenRequest,
    ) -> Result<CreateTokenResponse, Error> {
        self.client.post("/1.3/account/tokens", &req).await
    }

    /// Returns a list of all API tokens for the current account.
    pub async fn list_tokens(&self) -> Result<Vec<ApiToken>, Error> {
        self.client.get("/1.3/account/tokens").await
    }

    /// Returns details for a specific API token.
    pub async fn get_token(&self, id: &str) -> Result<ApiToken, Error> {
        self.client.get(&format!("/1.3/account/tokens/{id}")).await
    }

    /// Deletes an API token.
    pub async fn delete_token(&self, id: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/account/tokens/{id}"))
            .await
    }
}

#[cfg(test)]
mod tests;
