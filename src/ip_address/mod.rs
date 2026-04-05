//! IP addresses — `GET/POST/PATCH/DELETE /1.3/ip_address`

mod types;
pub use types::*;

use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::Error;

// ─── Response wrappers ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct IpAddressListResponse {
    ip_addresses: IpAddressWrapper,
}

#[derive(Debug, Deserialize)]
struct IpAddressWrapper {
    ip_address: Vec<IpAddress>,
}

#[derive(Debug, Deserialize)]
struct IpAddressResponse {
    ip_address: IpAddress,
}

// ─── Handler ──────────────────────────────────────────────────────────────────

/// Handler for IP address operations.
pub struct IpAddressHandler<'a> {
    client: &'a Client,
}

impl<'a> IpAddressHandler<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Returns a list of all IP addresses assigned to the current account.
    ///
    /// `GET /1.3/ip_address`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/10-ip-addresses/#list-ip-addresses)
    pub async fn list(&self) -> Result<Vec<IpAddress>, Error> {
        let resp: IpAddressListResponse = self.client.get("/1.3/ip_address").await?;
        Ok(resp.ip_addresses.ip_address)
    }

    /// Returns details for a specific IP address.
    ///
    /// `GET /1.3/ip_address/{address}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/10-ip-addresses/#get-ip-address-details)
    pub async fn get(&self, address: &str) -> Result<IpAddress, Error> {
        let resp: IpAddressResponse =
            self.client.get(&format!("/1.3/ip_address/{address}")).await?;
        Ok(resp.ip_address)
    }

    /// Assigns a new IP address (or allocates a floating IP).
    ///
    /// `POST /1.3/ip_address`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/10-ip-addresses/#assign-ip-address)
    pub async fn assign(&self, req: AssignRequest) -> Result<IpAddress, Error> {
        #[derive(Serialize)]
        struct Body {
            ip_address: AssignRequest,
        }
        let resp: IpAddressResponse = self
            .client
            .post("/1.3/ip_address", &Body { ip_address: req })
            .await?;
        Ok(resp.ip_address)
    }

    /// Modifies an IP address (e.g. set reverse PTR record or reassign floating IP).
    ///
    /// `PATCH /1.3/ip_address/{address}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/10-ip-addresses/#modify-ip-address)
    pub async fn modify(
        &self,
        address: &str,
        req: ModifyRequest,
    ) -> Result<IpAddress, Error> {
        #[derive(Serialize)]
        struct Body {
            ip_address: ModifyRequest,
        }
        let resp: IpAddressResponse = self
            .client
            .patch(
                &format!("/1.3/ip_address/{address}"),
                &Body { ip_address: req },
            )
            .await?;
        Ok(resp.ip_address)
    }

    /// Releases (deletes) an IP address.
    ///
    /// `DELETE /1.3/ip_address/{address}`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/10-ip-addresses/#release-ip-address)
    pub async fn release(&self, address: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/ip_address/{address}"))
            .await
    }
}

#[cfg(test)]
mod tests;
