use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Zone {
    pub id: String,
    pub description: String,
    /// Whether this zone is publicly available (`"yes"` / `"no"` in the API).
    pub public: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_zone: Option<String>,
}
