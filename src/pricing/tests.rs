use super::*;
// Source: https://developers.upcloud.com/1.3/4-pricing/#list-prices
const PRICE_LIST_JSON: &str = r#"
    {
        "prices": {
            "zone": [
                {
                    "name": "fi-hel1",
                    "firewall": {
                        "amount": 1,
                        "price": 0.5
                    },
                    "io_request_backup": {
                        "amount": 1000000,
                        "price": 10.0
                    },
                    "io_request_hdd": {
                        "amount": 1000000,
                        "price": 0.0
                    },
                    "io_request_maxiops": {
                        "amount": 1000000,
                        "price": 0.0
                    },
                    "ipv4_address": {
                        "amount": 1,
                        "price": 0.3
                    },
                    "ipv6_address": {
                        "amount": 1,
                        "price": 0.0
                    },
                    "public_ipv4_bandwidth_in": {
                        "amount": 1,
                        "price": 0.0
                    },
                    "public_ipv4_bandwidth_out": {
                        "amount": 1,
                        "price": 5.0
                    },
                    "public_ipv6_bandwidth_in": {
                        "amount": 1,
                        "price": 0.0
                    },
                    "public_ipv6_bandwidth_out": {
                        "amount": 1,
                        "price": 5.0
                    },
                    "server_core": {
                        "amount": 1,
                        "price": 1.3
                    },
                    "server_memory": {
                        "amount": 256,
                        "price": 0.45
                    },
                    "storage_backup": {
                        "amount": 1,
                        "price": 0.007
                    },
                    "storage_hdd": {
                        "amount": 1,
                        "price": 0.013
                    },
                    "storage_maxiops": {
                        "amount": 1,
                        "price": 0.028
                    },
                    "server_plan_1xCPU-2GB": {
                        "amount": 1,
                        "price": 2.2321
                    }
                }
            ]
        }
    }
"#;

#[test]
fn deserialize_price_list() {
    let resp: PriceListResponse = serde_json::from_str(PRICE_LIST_JSON).unwrap();
    let zones = resp.prices.zone;
    assert_eq!(zones.len(), 1);
    assert_eq!(zones[0].name, "fi-hel1");

    let fw = zones[0].firewall.as_ref().unwrap();
    assert_eq!(fw.amount, 1);
    assert!((fw.price - 0.5).abs() < f64::EPSILON);

    let io_backup = zones[0].io_request_backup.as_ref().unwrap();
    assert_eq!(io_backup.amount, 1_000_000);
    assert!((io_backup.price - 10.0).abs() < f64::EPSILON);

    let io_hdd = zones[0].io_request_hdd.as_ref().unwrap();
    assert_eq!(io_hdd.amount, 1_000_000);
    assert!((io_hdd.price - 0.0).abs() < f64::EPSILON);

    let io_maxiops = zones[0].io_request_maxiops.as_ref().unwrap();
    assert_eq!(io_maxiops.amount, 1_000_000);
    assert!((io_maxiops.price - 0.0).abs() < f64::EPSILON);

    let ipv4 = zones[0].ipv4_address.as_ref().unwrap();
    assert_eq!(ipv4.amount, 1);
    assert!((ipv4.price - 0.3).abs() < f64::EPSILON);

    let ipv6 = zones[0].ipv6_address.as_ref().unwrap();
    assert_eq!(ipv6.amount, 1);
    assert!((ipv6.price - 0.0).abs() < f64::EPSILON);

    let bw_in4 = zones[0].public_ipv4_bandwidth_in.as_ref().unwrap();
    assert_eq!(bw_in4.amount, 1);
    assert!((bw_in4.price - 0.0).abs() < f64::EPSILON);

    let bw_out4 = zones[0].public_ipv4_bandwidth_out.as_ref().unwrap();
    assert_eq!(bw_out4.amount, 1);
    assert!((bw_out4.price - 5.0).abs() < f64::EPSILON);

    let bw_in6 = zones[0].public_ipv6_bandwidth_in.as_ref().unwrap();
    assert_eq!(bw_in6.amount, 1);
    assert!((bw_in6.price - 0.0).abs() < f64::EPSILON);

    let bw_out6 = zones[0].public_ipv6_bandwidth_out.as_ref().unwrap();
    assert_eq!(bw_out6.amount, 1);
    assert!((bw_out6.price - 5.0).abs() < f64::EPSILON);

    let core = zones[0].server_core.as_ref().unwrap();
    assert_eq!(core.amount, 1);
    assert!((core.price - 1.3).abs() < f64::EPSILON);

    let mem = zones[0].server_memory.as_ref().unwrap();
    assert_eq!(mem.amount, 256);
    assert!((mem.price - 0.45).abs() < f64::EPSILON);

    let st_backup = zones[0].storage_backup.as_ref().unwrap();
    assert_eq!(st_backup.amount, 1);
    assert!((st_backup.price - 0.007).abs() < f64::EPSILON);

    let st_hdd = zones[0].storage_hdd.as_ref().unwrap();
    assert_eq!(st_hdd.amount, 1);
    assert!((st_hdd.price - 0.013).abs() < f64::EPSILON);

    let maxiops = zones[0].storage_maxiops.as_ref().unwrap();
    assert_eq!(maxiops.amount, 1);
    assert!((maxiops.price - 0.028).abs() < f64::EPSILON);

    assert!(zones[0].storage_encrypted.is_none());
    assert!(zones[0].server_plan_1xcpu_1gb.is_none());
}

#[test]
fn deserialize_price_ignores_unknown_plan_fields() {
    let resp: PriceListResponse = serde_json::from_str(PRICE_LIST_JSON).unwrap();
    let zone = &resp.prices.zone[0];
    assert_eq!(zone.name, "fi-hel1");
    assert!(zone.server_plan_1xcpu_1gb.is_none());
    assert!(zone.storage_encrypted.is_none());
}
