use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::types::common::YesNo;

/// An UpCloud data center zone.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Zone {
    pub id: String,
    pub description: String,
    /// Whether this zone is publicly available (`"yes"` / `"no"` in the API).
    #[serde_as(as = "YesNo")]
    pub public: bool,
    /// Parent zone ID (for private zones).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_zone: Option<String>,
}
