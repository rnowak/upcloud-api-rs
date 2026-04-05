//! Zones — `GET /1.3/zone`

use serde::{Deserialize, Serialize};

use crate::{client::Client, error::Error};

// ─── Types ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Zone {
    pub id: String,
    pub description: String,
    /// Whether this zone is publicly available (`"yes"` / `"no"` in the API).
    pub public: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_zone: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ZoneListResponse {
    zones: ZoneWrapper,
}

#[derive(Debug, Deserialize)]
struct ZoneWrapper {
    zone: Vec<Zone>,
}

// ─── Client methods ───────────────────────────────────────────────────────────

impl Client {
    /// Returns a list of all available zones.
    pub async fn list_zones(&self) -> Result<Vec<Zone>, Error> {
        let resp: ZoneListResponse = self.get("/1.3/zone").await?;
        Ok(resp.zones.zone)
    }
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
