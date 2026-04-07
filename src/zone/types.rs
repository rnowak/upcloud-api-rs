use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::types::common::YesNo;

/// An UpCloud data center zone.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Zone {
    /// Unique zone identifier (e.g. `de-fra1`). Used when creating servers and storages.
    pub id: String,
    /// Human-readable description of the zone (e.g. `Helsinki #1`).
    pub description: String,
    /// Whether the zone is publicly available. Private cloud zones have this set to `false`.
    #[serde_as(as = "YesNo")]
    pub public: bool,
    /// The ID of the parent zone. Only present for private cloud zones; identifies
    /// the public zone that contains this private zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_zone: Option<String>,
}
