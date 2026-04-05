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

pub struct IpAddressHandler<'a> {
    client: &'a Client,
}

impl<'a> IpAddressHandler<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Returns a list of all IP addresses assigned to the current account.
    pub async fn list(&self) -> Result<Vec<IpAddress>, Error> {
        let resp: IpAddressListResponse = self.client.get("/1.3/ip_address").await?;
        Ok(resp.ip_addresses.ip_address)
    }

    /// Returns details for a specific IP address.
    pub async fn get(&self, address: &str) -> Result<IpAddress, Error> {
        let resp: IpAddressResponse =
            self.client.get(&format!("/1.3/ip_address/{address}")).await?;
        Ok(resp.ip_address)
    }

    /// Assigns a new IP address (or allocates a floating IP).
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
    pub async fn release(&self, address: &str) -> Result<(), Error> {
        self.client
            .delete(&format!("/1.3/ip_address/{address}"))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIST_IP_ADDRESSES_JSON: &str = r#"{
  "ip_addresses": {
    "ip_address": [
      {
        "access": "utility",
        "address": "10.0.0.0",
        "family": "IPv4",
        "ptr_record": "",
        "server": "0053cd80-5945-4105-9081-11192806a8f7",
        "mac": "mm:mm:mm:mm:mm:m1",
        "floating": "no",
        "release_policy": "release",
        "zone": "fi-hel2"
      },
      {
        "access": "public",
        "address": "x.x.x.x",
        "family": "IPv4",
        "part_of_plan": "yes",
        "ptr_record": "x-x-x-x.zone.upcloud.host",
        "server": "0053cd80-5945-4105-9081-11192806a8f7",
        "mac": "mm:mm:mm:mm:mm:m1",
        "floating": "yes",
        "release_policy": "release",
        "zone": "de-fra1"
      },
      {
        "access": "public",
        "address": "xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx",
        "family": "IPv6",
        "ptr_record": "xxxx-xxxx-xxxx-xxxx.v6.zone.upcloud.host",
        "server": "006b6701-55d2-4374-ac40-56cc1501037f",
        "mac": "mm:mm:mm:mm:mm:m3",
        "floating": "no",
        "release_policy": "release",
        "zone": "fi-hel1"
      },
      {
        "access": "public",
        "address": "y.y.y.y",
        "family": "IPv4",
        "ptr_record": "y.y.y.y.zone.host.upcloud.com",
        "floating": "yes",
        "release_policy": "keep",
        "zone": "nl-ams1"
      },
      {
        "access": "public",
        "address": "z.z.z.z",
        "family": "IPv4",
        "ptr_record": "z.z.z.z.zone.host.upcloud.com",
        "floating": "yes",
        "release_policy": "keep",
        "is_delegated": "yes",
        "zone": "nl-ams1"
      }
    ]
  }
}"#;

    const GET_IP_ADDRESS_JSON: &str = r#"{
  "ip_address": {
    "access": "public",
    "address": "0.0.0.0",
    "family": "IPv4",
    "part_of_plan": "yes",
    "ptr_record": "test.example.com",
    "server": "009d64ef-31d1-4684-a26b-c86c955cbf46",
    "mac": "mm:mm:mm:mm:mm:m5",
    "floating": "no",
    "release_policy": "keep",
    "is_delegated": "yes",
    "zone": "sg-sin1"
  }
}"#;

    #[test]
    fn deserialize_ip_address_list() {
        let resp: IpAddressListResponse = serde_json::from_str(LIST_IP_ADDRESSES_JSON).unwrap();
        let ips = resp.ip_addresses.ip_address;
        assert_eq!(ips.len(), 5);

        // ips[0]: Utility IP
        assert_eq!(ips[0].access, Access::Utility);
        assert_eq!(ips[0].address, "10.0.0.0");
        assert_eq!(ips[0].family, Family::IPv4);
        assert_eq!(ips[0].ptr_record.as_deref(), Some(""));
        assert_eq!(ips[0].server.as_deref(), Some("0053cd80-5945-4105-9081-11192806a8f7"));
        assert_eq!(ips[0].mac.as_deref(), Some("mm:mm:mm:mm:mm:m1"));
        assert_eq!(ips[0].floating, Some(false));
        assert_eq!(ips[0].release_policy, Some(ReleasePolicy::Release));
        assert_eq!(ips[0].zone.as_deref(), Some("fi-hel2"));
        assert!(ips[0].is_delegated.is_none());
        assert!(ips[0].part_of_plan.is_none());

        // ips[1]: Floating public IP
        assert_eq!(ips[1].access, Access::Public);
        assert_eq!(ips[1].address, "x.x.x.x");
        assert_eq!(ips[1].family, Family::IPv4);
        assert_eq!(ips[1].part_of_plan, Some(true));
        assert_eq!(ips[1].ptr_record.as_deref(), Some("x-x-x-x.zone.upcloud.host"));
        assert_eq!(ips[1].server.as_deref(), Some("0053cd80-5945-4105-9081-11192806a8f7"));
        assert_eq!(ips[1].mac.as_deref(), Some("mm:mm:mm:mm:mm:m1"));
        assert_eq!(ips[1].floating, Some(true));
        assert_eq!(ips[1].release_policy, Some(ReleasePolicy::Release));
        assert_eq!(ips[1].zone.as_deref(), Some("de-fra1"));
        assert!(ips[1].is_delegated.is_none());

        // ips[2]: IPv6
        assert_eq!(ips[2].access, Access::Public);
        assert_eq!(ips[2].address, "xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx");
        assert_eq!(ips[2].family, Family::IPv6);
        assert_eq!(ips[2].ptr_record.as_deref(), Some("xxxx-xxxx-xxxx-xxxx.v6.zone.upcloud.host"));
        assert_eq!(ips[2].server.as_deref(), Some("006b6701-55d2-4374-ac40-56cc1501037f"));
        assert_eq!(ips[2].mac.as_deref(), Some("mm:mm:mm:mm:mm:m3"));
        assert_eq!(ips[2].floating, Some(false));
        assert_eq!(ips[2].release_policy, Some(ReleasePolicy::Release));
        assert_eq!(ips[2].zone.as_deref(), Some("fi-hel1"));
        assert!(ips[2].is_delegated.is_none());
        assert!(ips[2].part_of_plan.is_none());

        // ips[3]: Detached floating — no server or mac
        assert_eq!(ips[3].access, Access::Public);
        assert_eq!(ips[3].address, "y.y.y.y");
        assert_eq!(ips[3].family, Family::IPv4);
        assert_eq!(ips[3].ptr_record.as_deref(), Some("y.y.y.y.zone.host.upcloud.com"));
        assert!(ips[3].server.is_none());
        assert!(ips[3].mac.is_none());
        assert_eq!(ips[3].floating, Some(true));
        assert_eq!(ips[3].release_policy, Some(ReleasePolicy::Keep));
        assert_eq!(ips[3].zone.as_deref(), Some("nl-ams1"));
        assert!(ips[3].is_delegated.is_none());
        assert!(ips[3].part_of_plan.is_none());

        // ips[4]: Delegated IP
        assert_eq!(ips[4].access, Access::Public);
        assert_eq!(ips[4].address, "z.z.z.z");
        assert_eq!(ips[4].family, Family::IPv4);
        assert_eq!(ips[4].ptr_record.as_deref(), Some("z.z.z.z.zone.host.upcloud.com"));
        assert!(ips[4].server.is_none());
        assert!(ips[4].mac.is_none());
        assert_eq!(ips[4].floating, Some(true));
        assert_eq!(ips[4].release_policy, Some(ReleasePolicy::Keep));
        assert_eq!(ips[4].is_delegated, Some(true));
        assert_eq!(ips[4].zone.as_deref(), Some("nl-ams1"));
        assert!(ips[4].part_of_plan.is_none());
    }

    #[test]
    fn deserialize_ip_address_detail() {
        let resp: IpAddressResponse = serde_json::from_str(GET_IP_ADDRESS_JSON).unwrap();
        let ip = resp.ip_address;
        assert_eq!(ip.access, Access::Public);
        assert_eq!(ip.address, "0.0.0.0");
        assert_eq!(ip.family, Family::IPv4);
        assert_eq!(ip.ptr_record.as_deref(), Some("test.example.com"));
        assert_eq!(ip.server.as_deref(), Some("009d64ef-31d1-4684-a26b-c86c955cbf46"));
        assert_eq!(ip.mac.as_deref(), Some("mm:mm:mm:mm:mm:m5"));
        assert_eq!(ip.floating, Some(false));
        assert_eq!(ip.release_policy, Some(ReleasePolicy::Keep));
        assert_eq!(ip.part_of_plan, Some(true));
        assert_eq!(ip.is_delegated, Some(true));
        assert_eq!(ip.zone.as_deref(), Some("sg-sin1"));
    }

    // ── Assign IP address (docs: POST /1.3/ip_address) ──

    const ASSIGN_IP_ADDRESS_REQUEST_JSON: &str = r#"{
  "ip_address": {
    "family": "IPv4",
    "server": "009d64ef-31d1-4684-a26b-c86c955cbf46"
  }
}"#;

    #[test]
    fn deserialize_assign_ip_address_request() {
        let v: serde_json::Value = serde_json::from_str(ASSIGN_IP_ADDRESS_REQUEST_JSON).unwrap();
        let ip = &v["ip_address"];
        assert_eq!(ip["family"], "IPv4");
        assert_eq!(ip["server"], "009d64ef-31d1-4684-a26b-c86c955cbf46");
    }

    // ── Assign attached floating IP (docs: POST /1.3/ip_address) ──

    const ASSIGN_ATTACHED_FLOATING_IP_REQUEST_JSON: &str = r#"{
  "ip_address": {
    "family": "IPv4",
    "mac": "mm:mm:mm:mm:mm:m1",
    "release_policy": "release",
    "floating": "yes"
  }
}"#;

    const ASSIGN_ATTACHED_FLOATING_IP_RESPONSE_JSON: &str = r#"{
  "ip_address": {
    "access": "public",
    "address": "0.0.0.0",
    "family": "IPv4",
    "ptr_record": "xxxx-xxxx-xxxx-xxxx.v6.zone.upcloud.host",
    "server": "009d64ef-31d1-4684-a26b-c86c955cbf46",
    "mac": "mm:mm:mm:mm:mm:m1",
    "floating": "yes",
    "release_policy": "release",
    "zone": "fi-hel2"
  }
}"#;

    #[test]
    fn deserialize_assign_attached_floating_ip_request() {
        let v: serde_json::Value =
            serde_json::from_str(ASSIGN_ATTACHED_FLOATING_IP_REQUEST_JSON).unwrap();
        let ip = &v["ip_address"];
        assert_eq!(ip["family"], "IPv4");
        assert_eq!(ip["floating"], "yes");
        assert_eq!(ip["mac"], "mm:mm:mm:mm:mm:m1");
        assert_eq!(ip["release_policy"], "release");
    }

    #[test]
    fn deserialize_assign_attached_floating_ip_response() {
        let resp: IpAddressResponse =
            serde_json::from_str(ASSIGN_ATTACHED_FLOATING_IP_RESPONSE_JSON).unwrap();
        let ip = resp.ip_address;
        assert_eq!(ip.access, Access::Public);
        assert_eq!(ip.address, "0.0.0.0");
        assert_eq!(ip.family, Family::IPv4);
        assert_eq!(ip.ptr_record.as_deref(), Some("xxxx-xxxx-xxxx-xxxx.v6.zone.upcloud.host"));
        assert_eq!(ip.server.as_deref(), Some("009d64ef-31d1-4684-a26b-c86c955cbf46"));
        assert_eq!(ip.mac.as_deref(), Some("mm:mm:mm:mm:mm:m1"));
        assert_eq!(ip.floating, Some(true));
        assert_eq!(ip.release_policy, Some(ReleasePolicy::Release));
        assert!(ip.is_delegated.is_none());
        assert!(ip.part_of_plan.is_none());
        assert_eq!(ip.zone.as_deref(), Some("fi-hel2"));
    }

    // ── Assign detached floating IP (docs: POST /1.3/ip_address) ──

    const ASSIGN_DETACHED_FLOATING_IP_REQUEST_JSON: &str = r#"{
  "ip_address": {
    "family": "IPv4",
    "zone": "fi-hel2",
    "floating": "yes"
  }
}"#;

    const ASSIGN_DETACHED_FLOATING_IP_RESPONSE_JSON: &str = r#"{
  "ip_address": {
    "access": "public",
    "address": "xx.xx.xx.xx",
    "family": "IPv4",
    "floating": "yes",
    "ptr_record": "xx-xx-xx-xx.de-fra1.upcloud.host",
    "release_policy": "keep",
    "zone": "de-fra1"
  }
}"#;

    #[test]
    fn deserialize_assign_detached_floating_ip_request() {
        let v: serde_json::Value =
            serde_json::from_str(ASSIGN_DETACHED_FLOATING_IP_REQUEST_JSON).unwrap();
        let ip = &v["ip_address"];
        assert_eq!(ip["family"], "IPv4");
        assert_eq!(ip["floating"], "yes");
        assert_eq!(ip["zone"], "fi-hel2");
        assert!(ip.get("mac").is_none());
    }

    #[test]
    fn deserialize_assign_detached_floating_ip_response() {
        let resp: IpAddressResponse =
            serde_json::from_str(ASSIGN_DETACHED_FLOATING_IP_RESPONSE_JSON).unwrap();
        let ip = resp.ip_address;
        assert_eq!(ip.access, Access::Public);
        assert_eq!(ip.address, "xx.xx.xx.xx");
        assert_eq!(ip.family, Family::IPv4);
        assert_eq!(ip.ptr_record.as_deref(), Some("xx-xx-xx-xx.de-fra1.upcloud.host"));
        assert_eq!(ip.floating, Some(true));
        assert_eq!(ip.release_policy, Some(ReleasePolicy::Keep));
        // Detached: no server or mac
        assert!(ip.server.is_none());
        assert!(ip.mac.is_none());
        assert!(ip.is_delegated.is_none());
        assert!(ip.part_of_plan.is_none());
        assert_eq!(ip.zone.as_deref(), Some("de-fra1"));
    }

    // ── Modify IP address (docs: PATCH /1.3/ip_address/{address}) ──

    const MODIFY_IP_ADDRESS_REQUEST_JSON: &str = r#"{
  "ip_address": {
    "mac": "mm:mm:mm:mm:mm:m1",
    "ptr_record": "hostname.example.com",
    "release_policy": "release"
  }
}"#;

    const MODIFY_IP_ADDRESS_RESPONSE_JSON: &str = r#"{
  "ip_address": {
    "access": "public",
    "address": "0.0.0.0",
    "family": "IPv4",
    "ptr_record": "hostname.example.com",
    "server": "009d64ef-31d1-4684-a26b-c86c95c5bf46",
    "mac": "mm:mm:mm:mm:mm:m1",
    "floating": "yes",
    "zone": "sg-sin1"
  }
}"#;

    #[test]
    fn deserialize_modify_ip_address_request() {
        #[derive(Debug, Deserialize)]
        struct Wrapper {
            ip_address: ModifyRequest,
        }
        let w: Wrapper = serde_json::from_str(MODIFY_IP_ADDRESS_REQUEST_JSON).unwrap();
        let req = w.ip_address;
        assert_eq!(req.mac.as_deref(), Some("mm:mm:mm:mm:mm:m1"));
        assert_eq!(req.ptr_record.as_deref(), Some("hostname.example.com"));
        assert_eq!(req.release_policy, Some(ReleasePolicy::Release));
    }

    #[test]
    fn deserialize_modify_ip_address_response() {
        let resp: IpAddressResponse =
            serde_json::from_str(MODIFY_IP_ADDRESS_RESPONSE_JSON).unwrap();
        let ip = resp.ip_address;
        assert_eq!(ip.access, Access::Public);
        assert_eq!(ip.address, "0.0.0.0");
        assert_eq!(ip.family, Family::IPv4);
        assert_eq!(ip.ptr_record.as_deref(), Some("hostname.example.com"));
        assert_eq!(ip.server.as_deref(), Some("009d64ef-31d1-4684-a26b-c86c95c5bf46"));
        assert_eq!(ip.mac.as_deref(), Some("mm:mm:mm:mm:mm:m1"));
        assert_eq!(ip.floating, Some(true));
        assert_eq!(ip.zone.as_deref(), Some("sg-sin1"));
        // No release_policy in this response
        assert!(ip.release_policy.is_none());
        assert!(ip.is_delegated.is_none());
        assert!(ip.part_of_plan.is_none());
    }
}
