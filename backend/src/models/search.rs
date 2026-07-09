use serde::{Deserialize, Serialize};

use crate::models::item::MediaType;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchCandidate {
    pub external_id: String,
    pub title: String,
    pub media_type: MediaType,
    pub year: Option<String>,
    pub description: Option<String>,
    pub poster_url: Option<String>,
    pub metadata: serde_json::Value,
}
