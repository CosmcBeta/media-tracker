use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Item {
    pub id: Uuid,
    pub mediaType: MediaType,
    pub title: String,
    pub externalId: Option<String>,
    pub metaData: Option<serde_json::Value>,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum MediaType {
    Movie,
    Show,
    Album,
    Artist,
    Book,
    Game,
    Podcast,
}
