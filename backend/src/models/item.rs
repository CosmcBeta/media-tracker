use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Item {
    pub id: Uuid,
    pub media_type: MediaType,
    pub title: String,
    pub external_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Movie,
    Show,
    Album,
    Artist,
    Book,
    Game,
    Podcast,
}

#[derive(Debug, Deserialize)]
pub struct CreateItem {
    pub media_type: MediaType,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateItem {
    pub media_type: Option<MediaType>,
    pub title: Option<String>,
    pub external_id: Option<String>,
    pub metadata: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub q: String,
    pub media_type: MediaType,
}
