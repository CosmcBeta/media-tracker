use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
struct Item {
    id: Uuid,
    mediaType: MediaType,
    title: String,
    externalId: String,
    metaData: Option<serde_json::Value>,
    createdAt: DateTime<Utc>,
    updatedAt: DateTime<Utc>,
}

#[derive(Debug, Serialize, Clone)]
enum MediaType {
    Movie,
    Show,
    Album,
    Artist,
    Book,
    Game,
    Podcast,
}
