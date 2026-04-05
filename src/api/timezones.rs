//! Timezones — `GET /1.3/timezone`

use serde::Deserialize;

use crate::{client::Client, error::Error};

#[derive(Debug, Deserialize)]
struct TimezoneListResponse {
    timezones: TimezoneWrapper,
}

#[derive(Debug, Deserialize)]
struct TimezoneWrapper {
    timezone: Vec<String>,
}

impl Client {
    /// Returns a list of all available timezone identifiers.
    pub async fn list_timezones(&self) -> Result<Vec<String>, Error> {
        let resp: TimezoneListResponse = self.get("/1.3/timezone").await?;
        Ok(resp.timezones.timezone)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TIMEZONE_LIST_JSON: &str = r#"{
  "timezones": {
    "timezone": [
      "Africa/Abidjan",
      "Africa/Accra",
      "Africa/Addis_Ababa",
      "Pacific/Wake",
      "Pacific/Wallis",
      "UTC"
    ]
  }
}"#;

    #[test]
    fn deserialize_timezone_list() {
        let resp: TimezoneListResponse =
            serde_json::from_str(TIMEZONE_LIST_JSON).unwrap();
        let tzs = resp.timezones.timezone;
        assert_eq!(tzs.len(), 6);
        assert_eq!(tzs[0], "Africa/Abidjan");
        assert_eq!(tzs[1], "Africa/Accra");
        assert_eq!(tzs[2], "Africa/Addis_Ababa");
        assert_eq!(tzs[3], "Pacific/Wake");
        assert_eq!(tzs[4], "Pacific/Wallis");
        assert_eq!(tzs[5], "UTC");
    }
}
