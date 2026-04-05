//! Zones — available UpCloud data center zones.

mod types;

pub use types::*;

use crate::client::Client;
use crate::error::Error;

/// Handler for zone operations.
pub struct ZoneHandler<'a> {
    client: &'a Client,
}

impl<'a> ZoneHandler<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Returns a list of all available zones.
    pub async fn list(&self) -> Result<Vec<Zone>, Error> {
        let resp: ZoneListResponse = self.client.get("/1.3/zone").await?;
        Ok(resp.zones.zone)
    }
}

// ─── Response wrappers (crate-private) ────────────────────────────────────────

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ZoneListResponse {
    zones: ZoneWrapper,
}

#[derive(Debug, Deserialize)]
struct ZoneWrapper {
    zone: Vec<Zone>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const ZONE_LIST_JSON: &str = r#"{
  "zones": {
    "zone": [
      {
        "description": "Frankfurt #1",
        "id": "de-fra1",
        "public": "yes"
      },
      {
        "description": "Helsinki #1",
        "id": "fi-hel1",
        "public": "yes"
      },
      {
        "description": "Fake private cloud",
        "id": "fi-fake-priv-cloud",
        "parent_zone": "fi-hel1",
        "public": "no"
      }
    ]
  }
}"#;

    #[test]
    fn deserialize_zone_list() {
        let resp: ZoneListResponse = serde_json::from_str(ZONE_LIST_JSON).unwrap();
        let zones = resp.zones.zone;
        assert_eq!(zones.len(), 3);

        assert_eq!(zones[0].id, "de-fra1");
        assert_eq!(zones[0].description, "Frankfurt #1");
        assert_eq!(zones[0].public, "yes");
        assert!(zones[0].parent_zone.is_none());

        assert_eq!(zones[1].id, "fi-hel1");
        assert_eq!(zones[1].description, "Helsinki #1");
        assert_eq!(zones[1].public, "yes");
        assert!(zones[1].parent_zone.is_none());

        assert_eq!(zones[2].id, "fi-fake-priv-cloud");
        assert_eq!(zones[2].description, "Fake private cloud");
        assert_eq!(zones[2].public, "no");
        assert_eq!(zones[2].parent_zone.as_deref(), Some("fi-hel1"));
    }
}
