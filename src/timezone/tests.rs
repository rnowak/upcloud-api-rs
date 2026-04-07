use super::*;

// Source: https://developers.upcloud.com/1.3/6-timezones/#list-timezones
const TIMEZONE_LIST_JSON: &str = r#"
    {
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
    }
"#;

#[test]
fn deserialize_timezone_list() {
    let resp: TimezoneListResponse = serde_json::from_str(TIMEZONE_LIST_JSON).unwrap();
    let tzs = resp.timezones.timezone;
    assert_eq!(tzs.len(), 6);
    assert_eq!(tzs[0], "Africa/Abidjan");
    assert_eq!(tzs[1], "Africa/Accra");
    assert_eq!(tzs[2], "Africa/Addis_Ababa");
    assert_eq!(tzs[3], "Pacific/Wake");
    assert_eq!(tzs[4], "Pacific/Wallis");
    assert_eq!(tzs[5], "UTC");
}
