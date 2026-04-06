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
    assert!(zones[0].public);
    assert!(zones[0].parent_zone.is_none());

    assert_eq!(zones[1].id, "fi-hel1");
    assert_eq!(zones[1].description, "Helsinki #1");
    assert!(zones[1].public);
    assert!(zones[1].parent_zone.is_none());

    assert_eq!(zones[2].id, "fi-fake-priv-cloud");
    assert_eq!(zones[2].description, "Fake private cloud");
    assert!(!zones[2].public);
    assert_eq!(zones[2].parent_zone.as_deref(), Some("fi-hel1"));
}
