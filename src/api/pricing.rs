//! Pricing — `GET /1.3/price`

use serde::{Deserialize, Serialize};

use crate::{client::Client, error::Error};

// ─── Types ────────────────────────────────────────────────────────────────────

/// A single price item (amount + price per unit).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceItem {
    pub amount: u64,
    pub price: f64,
}

/// Pricing for a single zone.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceZone {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_request_backup: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_request_hdd: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_request_maxiops: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_address: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ipv4_bandwidth_in: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ipv4_bandwidth_out: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ipv6_bandwidth_in: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ipv6_bandwidth_out: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_core: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_memory: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_backup: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_hdd: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_maxiops: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<PriceItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_plan_1xcpu_1gb: Option<PriceItem>,
}

#[derive(Debug, Deserialize)]
struct PriceListResponse {
    prices: PriceWrapper,
}

#[derive(Debug, Deserialize)]
struct PriceWrapper {
    zone: Vec<PriceZone>,
}

// ─── Client methods ───────────────────────────────────────────────────────────

impl Client {
    /// Returns pricing information for all zones.
    pub async fn list_price_zones(&self) -> Result<Vec<PriceZone>, Error> {
        let resp: PriceListResponse = self.get("/1.3/price").await?;
        Ok(resp.prices.zone)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRICE_LIST_JSON: &str = r#"{
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
}"#;

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
        // The API includes dynamic server_plan_* keys not modeled in PriceZone;
        // serde silently ignores them.
        let resp: PriceListResponse = serde_json::from_str(PRICE_LIST_JSON).unwrap();
        let zone = &resp.prices.zone[0];
        assert_eq!(zone.name, "fi-hel1");
        // server_plan_1xcpu_1gb is None because the JSON key is "server_plan_1xCPU-2GB"
        assert!(zone.server_plan_1xcpu_1gb.is_none());
        assert!(zone.storage_encrypted.is_none());
    }
}
