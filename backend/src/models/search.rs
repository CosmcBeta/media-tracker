use serde::{Deserialize, Serialize};

use crate::models::item::MediaType;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchCandidate {
    pub external_id: i64,
    pub title: String,
    pub media_type: MediaType,
    pub year: Option<String>,
    pub description: Option<String>,
    pub poster_url: Option<String>,
    pub metadata: String,
}
