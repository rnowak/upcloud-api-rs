//! Timezones — available timezone identifiers.

use serde::Deserialize;

use crate::client::Client;
use crate::error::Error;

/// Handler for timezone operations.
pub struct TimezoneHandler<'a> {
    client: &'a Client,
}

impl<'a> TimezoneHandler<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Returns a list of all available timezone identifiers.
    ///
    /// `GET /1.3/timezone`
    ///
    /// [UpCloud API docs](https://developers.upcloud.com/1.3/6-timezones/#list-timezones)
    pub async fn list(&self) -> Result<Vec<String>, Error> {
        let resp: TimezoneListResponse = self.client.get("/1.3/timezone").await?;
        Ok(resp.timezones.timezone)
    }
}

#[derive(Debug, Deserialize)]
struct TimezoneListResponse {
    timezones: TimezoneWrapper,
}

#[derive(Debug, Deserialize)]
struct TimezoneWrapper {
    timezone: Vec<String>,
}

#[cfg(test)]
mod tests;
