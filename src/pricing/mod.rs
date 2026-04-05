//! Pricing — zone-based pricing information.

mod types;

pub use types::*;

use serde::Deserialize;

use crate::client::Client;
use crate::error::Error;

/// Handler for pricing operations.
pub struct PricingHandler<'a> {
    client: &'a Client,
}

impl<'a> PricingHandler<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Returns pricing information for all zones.
    pub async fn list_zones(&self) -> Result<Vec<PriceZone>, Error> {
        let resp: PriceListResponse = self.client.get("/1.3/price").await?;
        Ok(resp.prices.zone)
    }
}

#[derive(Debug, Deserialize)]
struct PriceListResponse {
    prices: PriceWrapper,
}

#[derive(Debug, Deserialize)]
struct PriceWrapper {
    zone: Vec<PriceZone>,
}

#[cfg(test)]
mod tests;
