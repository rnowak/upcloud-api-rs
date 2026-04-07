//! Firewall rule types (used by the servers module for per-server rules).

use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

use crate::types::deserialize_empty_string_as_none;

// ─── Enums ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    In,
    Out,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Accept,
    #[deprecated(note = "no longer supported by UpCloud API")]
    Reject,
    Drop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Tcp,
    Udp,
    Icmp,
    #[serde(rename = "udplite")]
    UdpLite,
    Esp,
    Ah,
    Sctp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Family {
    IPv4,
    IPv6,
}

// ─── Rule ─────────────────────────────────────────────────────────────────────

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub action: Action,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_address_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_address_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_port_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_port_start: Option<String>,
    pub direction: Direction,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub family: Option<Family>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icmp_type: Option<String>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_empty_string_as_none"
    )]
    pub protocol: Option<Protocol>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_address_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_address_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port_start: Option<String>,
}

/// Request body for creating a firewall rule.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRuleRequest {
    pub action: Action,
    pub direction: Direction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_address_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_address_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_port_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_port_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<Family>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icmp_type: Option<String>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Protocol>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_address_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_address_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port_start: Option<String>,
}

// ─── Response wrappers ────────────────────────────────────────────────────────

#[cfg(test)]
#[derive(Debug, Deserialize)]
struct RuleListResponse {
    firewall_rules: RuleWrapper,
}

#[cfg(test)]
#[derive(Debug, Deserialize)]
struct RuleWrapper {
    firewall_rule: Vec<Rule>,
}

#[cfg(test)]
#[derive(Debug, Deserialize)]
struct RuleResponse {
    firewall_rule: Rule,
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIST_FIREWALL_RULES_JSON: &str = r#"{
  "firewall_rules": {
    "firewall_rule": [
      {
        "action": "accept",
        "comment": "Allow HTTP from anywhere",
        "destination_address_end": "",
        "destination_address_start": "",
        "destination_port_end": "80",
        "destination_port_start": "80",
        "direction": "in",
        "family": "IPv4",
        "icmp_type": "",
        "position": "1",
        "protocol": "",
        "source_address_end": "",
        "source_address_start": "",
        "source_port_end": "",
        "source_port_start": ""
      },
      {
        "action": "accept",
        "comment": "Allow SSH from a specific network only",
        "destination_address_end": "",
        "destination_address_start": "",
        "destination_port_end": "22",
        "destination_port_start": "22",
        "direction": "in",
        "family": "IPv4",
        "icmp_type": "",
        "position": "2",
        "protocol": "tcp",
        "source_address_end": "192.168.1.255",
        "source_address_start": "192.168.1.1",
        "source_port_end": "",
        "source_port_start": ""
      },
      {
        "action": "accept",
        "comment": "Allow ICMP echo request (ping)",
        "destination_address_end": "",
        "destination_address_start": "",
        "destination_port_end": "",
        "destination_port_start": "",
        "direction": "in",
        "family": "IPv4",
        "icmp_type": "8",
        "position": "4",
        "protocol": "icmp",
        "source_address_end": "",
        "source_address_start": "",
        "source_port_end": "",
        "source_port_start": ""
      },
      {
        "action": "drop",
        "comment": "",
        "destination_address_end": "",
        "destination_address_start": "",
        "destination_port_end": "",
        "destination_port_start": "",
        "direction": "in",
        "family": "",
        "icmp_type": "",
        "position": "5",
        "protocol": "",
        "source_address_end": "",
        "source_address_start": "",
        "source_port_end": "",
        "source_port_start": ""
      }
    ]
  }
}"#;

    const GET_FIREWALL_RULE_JSON: &str = r#"{
  "firewall_rule": {
    "action": "accept",
    "comment": "Allow HTTP from anywhere",
    "destination_address_end": "",
    "destination_address_start": "",
    "destination_port_end": "80",
    "destination_port_start": "80",
    "direction": "in",
    "family": "IPv4",
    "icmp_type": "",
    "position": "1",
    "protocol": "",
    "source_address_end": "",
    "source_address_start": "",
    "source_port_end": "",
    "source_port_start": ""
  }
}"#;

    #[test]
    fn deserialize_firewall_rule_list() {
        let resp: RuleListResponse = serde_json::from_str(LIST_FIREWALL_RULES_JSON).unwrap();
        let rules = resp.firewall_rules.firewall_rule;
        assert_eq!(rules.len(), 4);

        // Rule 0: family but empty protocol
        assert_eq!(rules[0].action, Action::Accept);
        assert_eq!(
            rules[0].comment.as_deref(),
            Some("Allow HTTP from anywhere")
        );
        assert_eq!(rules[0].destination_address_end.as_deref(), Some(""));
        assert_eq!(rules[0].destination_address_start.as_deref(), Some(""));
        assert_eq!(rules[0].destination_port_end.as_deref(), Some("80"));
        assert_eq!(rules[0].destination_port_start.as_deref(), Some("80"));
        assert_eq!(rules[0].direction, Direction::In);
        assert_eq!(rules[0].family, Some(Family::IPv4));
        assert_eq!(rules[0].icmp_type.as_deref(), Some(""));
        assert_eq!(rules[0].position, Some(1));
        assert!(rules[0].protocol.is_none()); // "" → None
        assert_eq!(rules[0].source_address_end.as_deref(), Some(""));
        assert_eq!(rules[0].source_address_start.as_deref(), Some(""));
        assert_eq!(rules[0].source_port_end.as_deref(), Some(""));
        assert_eq!(rules[0].source_port_start.as_deref(), Some(""));

        // Rule 1: both family and protocol
        assert_eq!(rules[1].action, Action::Accept);
        assert_eq!(
            rules[1].comment.as_deref(),
            Some("Allow SSH from a specific network only")
        );
        assert_eq!(rules[1].destination_address_end.as_deref(), Some(""));
        assert_eq!(rules[1].destination_address_start.as_deref(), Some(""));
        assert_eq!(rules[1].destination_port_end.as_deref(), Some("22"));
        assert_eq!(rules[1].destination_port_start.as_deref(), Some("22"));
        assert_eq!(rules[1].direction, Direction::In);
        assert_eq!(rules[1].family, Some(Family::IPv4));
        assert_eq!(rules[1].icmp_type.as_deref(), Some(""));
        assert_eq!(rules[1].position, Some(2));
        assert_eq!(rules[1].protocol, Some(Protocol::Tcp));
        assert_eq!(
            rules[1].source_address_end.as_deref(),
            Some("192.168.1.255")
        );
        assert_eq!(
            rules[1].source_address_start.as_deref(),
            Some("192.168.1.1")
        );
        assert_eq!(rules[1].source_port_end.as_deref(), Some(""));
        assert_eq!(rules[1].source_port_start.as_deref(), Some(""));

        // Rule 2: ICMP rule
        assert_eq!(rules[2].action, Action::Accept);
        assert_eq!(
            rules[2].comment.as_deref(),
            Some("Allow ICMP echo request (ping)")
        );
        assert_eq!(rules[2].destination_address_end.as_deref(), Some(""));
        assert_eq!(rules[2].destination_address_start.as_deref(), Some(""));
        assert_eq!(rules[2].destination_port_end.as_deref(), Some(""));
        assert_eq!(rules[2].destination_port_start.as_deref(), Some(""));
        assert_eq!(rules[2].direction, Direction::In);
        assert_eq!(rules[2].family, Some(Family::IPv4));
        assert_eq!(rules[2].icmp_type.as_deref(), Some("8"));
        assert_eq!(rules[2].position, Some(4));
        assert_eq!(rules[2].protocol, Some(Protocol::Icmp));
        assert_eq!(rules[2].source_address_end.as_deref(), Some(""));
        assert_eq!(rules[2].source_address_start.as_deref(), Some(""));
        assert_eq!(rules[2].source_port_end.as_deref(), Some(""));
        assert_eq!(rules[2].source_port_start.as_deref(), Some(""));

        // Rule 3: Drop-all rule — empty family and protocol
        assert_eq!(rules[3].action, Action::Drop);
        assert_eq!(rules[3].comment.as_deref(), Some(""));
        assert_eq!(rules[3].destination_address_end.as_deref(), Some(""));
        assert_eq!(rules[3].destination_address_start.as_deref(), Some(""));
        assert_eq!(rules[3].destination_port_end.as_deref(), Some(""));
        assert_eq!(rules[3].destination_port_start.as_deref(), Some(""));
        assert_eq!(rules[3].direction, Direction::In);
        assert!(rules[3].family.is_none()); // "" → None
        assert_eq!(rules[3].icmp_type.as_deref(), Some(""));
        assert_eq!(rules[3].position, Some(5));
        assert!(rules[3].protocol.is_none()); // "" → None
        assert_eq!(rules[3].source_address_end.as_deref(), Some(""));
        assert_eq!(rules[3].source_address_start.as_deref(), Some(""));
        assert_eq!(rules[3].source_port_end.as_deref(), Some(""));
        assert_eq!(rules[3].source_port_start.as_deref(), Some(""));
    }

    #[test]
    fn deserialize_firewall_rule_detail() {
        let resp: RuleResponse = serde_json::from_str(GET_FIREWALL_RULE_JSON).unwrap();
        let rule = resp.firewall_rule;
        assert_eq!(rule.action, Action::Accept);
        assert_eq!(rule.comment.as_deref(), Some("Allow HTTP from anywhere"));
        assert_eq!(rule.destination_address_end.as_deref(), Some(""));
        assert_eq!(rule.destination_address_start.as_deref(), Some(""));
        assert_eq!(rule.destination_port_end.as_deref(), Some("80"));
        assert_eq!(rule.destination_port_start.as_deref(), Some("80"));
        assert_eq!(rule.direction, Direction::In);
        assert_eq!(rule.family, Some(Family::IPv4));
        assert_eq!(rule.icmp_type.as_deref(), Some(""));
        assert_eq!(rule.position, Some(1));
        assert!(rule.protocol.is_none()); // "" → None
        assert_eq!(rule.source_address_end.as_deref(), Some(""));
        assert_eq!(rule.source_address_start.as_deref(), Some(""));
        assert_eq!(rule.source_port_end.as_deref(), Some(""));
        assert_eq!(rule.source_port_start.as_deref(), Some(""));
    }

    // ── Create firewall rule (docs: POST /1.3/server/{uuid}/firewall_rule) ──

    const CREATE_FIREWALL_RULE_REQUEST_JSON: &str = r#"{
  "firewall_rule": {
    "position": "1",
    "direction": "in",
    "family": "IPv4",
    "protocol": "tcp",
    "source_address_start": "192.168.1.1",
    "source_address_end": "192.168.1.255",
    "source_port_end": "",
    "source_port_start": "",
    "destination_address_start": "",
    "destination_address_end": "",
    "destination_port_start": "22",
    "destination_port_end": "22",
    "icmp_type": "",
    "action": "accept",
    "comment": "Allow SSH from this network"
  }
}"#;

    const CREATE_FIREWALL_RULE_RESPONSE_JSON: &str = r#"{
  "firewall_rule": {
    "action": "accept",
    "comment": "Allow SSH from this network",
    "destination_address_end": "",
    "destination_address_start": "",
    "destination_port_end": "80",
    "destination_port_start": "80",
    "direction": "in",
    "family": "IPv4",
    "icmp_type": "",
    "position": "1",
    "protocol": "",
    "source_address_end": "",
    "source_address_start": "",
    "source_port_end": "",
    "source_port_start": ""
  }
}"#;

    #[test]
    fn deserialize_create_firewall_rule_request() {
        // Test inner object with CreateRuleRequest
        let inner_json = r#"{
            "position": "1",
            "direction": "in",
            "family": "IPv4",
            "protocol": "tcp",
            "source_address_start": "192.168.1.1",
            "source_address_end": "192.168.1.255",
            "source_port_end": "",
            "source_port_start": "",
            "destination_address_start": "",
            "destination_address_end": "",
            "destination_port_start": "22",
            "destination_port_end": "22",
            "icmp_type": "",
            "action": "accept",
            "comment": "Allow SSH from this network"
        }"#;
        let req: CreateRuleRequest = serde_json::from_str(inner_json).unwrap();
        assert_eq!(req.direction, Direction::In);
        assert_eq!(req.action, Action::Accept);
        assert_eq!(req.family, Some(Family::IPv4));
        assert_eq!(req.protocol, Some(Protocol::Tcp));
        assert_eq!(req.source_address_start.as_deref(), Some("192.168.1.1"));
        assert_eq!(req.source_address_end.as_deref(), Some("192.168.1.255"));
        assert_eq!(req.destination_port_start.as_deref(), Some("22"));
        assert_eq!(req.destination_port_end.as_deref(), Some("22"));
        assert_eq!(req.destination_address_start.as_deref(), Some(""));
        assert_eq!(req.destination_address_end.as_deref(), Some(""));
        assert_eq!(req.icmp_type.as_deref(), Some(""));
        assert_eq!(req.position, Some(1));
        assert_eq!(req.source_port_start.as_deref(), Some(""));
        assert_eq!(req.source_port_end.as_deref(), Some(""));
        assert_eq!(req.comment.as_deref(), Some("Allow SSH from this network"));

        // Also verify the full wrapped JSON
        let v: serde_json::Value = serde_json::from_str(CREATE_FIREWALL_RULE_REQUEST_JSON).unwrap();
        let fr = &v["firewall_rule"];
        assert_eq!(fr["position"], "1");
        assert_eq!(fr["direction"], "in");
        assert_eq!(fr["family"], "IPv4");
        assert_eq!(fr["protocol"], "tcp");
        assert_eq!(fr["source_address_start"], "192.168.1.1");
        assert_eq!(fr["source_address_end"], "192.168.1.255");
        assert_eq!(fr["source_port_start"], "");
        assert_eq!(fr["source_port_end"], "");
        assert_eq!(fr["destination_address_start"], "");
        assert_eq!(fr["destination_address_end"], "");
        assert_eq!(fr["destination_port_start"], "22");
        assert_eq!(fr["destination_port_end"], "22");
        assert_eq!(fr["icmp_type"], "");
        assert_eq!(fr["action"], "accept");
        assert_eq!(fr["comment"], "Allow SSH from this network");
    }

    #[test]
    fn deserialize_create_firewall_rule_response() {
        let resp: RuleResponse = serde_json::from_str(CREATE_FIREWALL_RULE_RESPONSE_JSON).unwrap();
        let rule = resp.firewall_rule;
        assert_eq!(rule.action, Action::Accept);
        assert_eq!(rule.comment.as_deref(), Some("Allow SSH from this network"));
        assert_eq!(rule.destination_port_start.as_deref(), Some("80"));
        assert_eq!(rule.position, Some(1));
        assert_eq!(rule.direction, Direction::In);
        assert_eq!(rule.family, Some(Family::IPv4));
        assert_eq!(rule.destination_address_end.as_deref(), Some(""));
        assert_eq!(rule.destination_address_start.as_deref(), Some(""));
        assert_eq!(rule.destination_port_end.as_deref(), Some("80"));
        assert_eq!(rule.icmp_type.as_deref(), Some(""));
        // Empty protocol string → None
        assert!(rule.protocol.is_none());
        assert_eq!(rule.source_address_end.as_deref(), Some(""));
        assert_eq!(rule.source_address_start.as_deref(), Some(""));
        assert_eq!(rule.source_port_end.as_deref(), Some(""));
        assert_eq!(rule.source_port_start.as_deref(), Some(""));
    }

    // ── Create multiple firewall rules (docs: PUT /1.3/server/{uuid}/firewall_rule) ──

    const CREATE_MULTIPLE_FIREWALL_RULES_REQUEST_JSON: &str = r#"{
  "firewall_rules": {
    "firewall_rule": [
      {
        "direction": "in",
        "family": "IPv4",
        "protocol": "tcp",
        "destination_port_start": "22",
        "destination_port_end": "22",
        "action": "accept",
        "comment": "Allow SSH to this network"
      },
      {
        "direction": "in",
        "family": "IPv4",
        "protocol": "tcp",
        "destination_port_start": "80",
        "destination_port_end": "80",
        "action": "accept",
        "comment": "Allow HTTP to this network"
      }
    ]
  }
}"#;

    #[test]
    fn deserialize_create_multiple_firewall_rules_request() {
        // Verify the full JSON structure
        let v: serde_json::Value =
            serde_json::from_str(CREATE_MULTIPLE_FIREWALL_RULES_REQUEST_JSON).unwrap();
        let rules = v["firewall_rules"]["firewall_rule"].as_array().unwrap();
        assert_eq!(rules.len(), 2);
        assert_eq!(rules[0]["comment"], "Allow SSH to this network");
        assert_eq!(rules[1]["comment"], "Allow HTTP to this network");

        // Verify each rule deserializes as CreateRuleRequest
        for rule_value in rules {
            let rule: CreateRuleRequest = serde_json::from_value(rule_value.clone()).unwrap();
            assert_eq!(rule.direction, Direction::In);
            assert_eq!(rule.family, Some(Family::IPv4));
            assert_eq!(rule.protocol, Some(Protocol::Tcp));
        }

        // Assert all raw JSON fields for rule 0
        assert_eq!(rules[0]["direction"], "in");
        assert_eq!(rules[0]["family"], "IPv4");
        assert_eq!(rules[0]["protocol"], "tcp");
        assert_eq!(rules[0]["destination_port_start"], "22");
        assert_eq!(rules[0]["destination_port_end"], "22");
        assert_eq!(rules[0]["action"], "accept");
        assert_eq!(rules[0]["comment"], "Allow SSH to this network");

        // Assert all raw JSON fields for rule 1
        assert_eq!(rules[1]["direction"], "in");
        assert_eq!(rules[1]["family"], "IPv4");
        assert_eq!(rules[1]["protocol"], "tcp");
        assert_eq!(rules[1]["destination_port_start"], "80");
        assert_eq!(rules[1]["destination_port_end"], "80");
        assert_eq!(rules[1]["action"], "accept");
        assert_eq!(rules[1]["comment"], "Allow HTTP to this network");

        let rule0: CreateRuleRequest = serde_json::from_value(rules[0].clone()).unwrap();
        assert_eq!(rule0.action, Action::Accept);
        assert_eq!(rule0.comment.as_deref(), Some("Allow SSH to this network"));
        assert_eq!(rule0.destination_port_start.as_deref(), Some("22"));
        assert_eq!(rule0.destination_port_end.as_deref(), Some("22"));
        assert!(rule0.destination_address_start.is_none());
        assert!(rule0.destination_address_end.is_none());
        assert!(rule0.icmp_type.is_none());
        assert!(rule0.position.is_none());
        assert!(rule0.source_address_start.is_none());
        assert!(rule0.source_address_end.is_none());
        assert!(rule0.source_port_start.is_none());
        assert!(rule0.source_port_end.is_none());

        let rule1: CreateRuleRequest = serde_json::from_value(rules[1].clone()).unwrap();
        assert_eq!(rule1.action, Action::Accept);
        assert_eq!(rule1.comment.as_deref(), Some("Allow HTTP to this network"));
        assert_eq!(rule1.destination_port_start.as_deref(), Some("80"));
        assert_eq!(rule1.destination_port_end.as_deref(), Some("80"));
        assert!(rule1.destination_address_start.is_none());
        assert!(rule1.destination_address_end.is_none());
        assert!(rule1.icmp_type.is_none());
        assert!(rule1.position.is_none());
        assert!(rule1.source_address_start.is_none());
        assert!(rule1.source_address_end.is_none());
        assert!(rule1.source_port_start.is_none());
        assert!(rule1.source_port_end.is_none());
    }
}
