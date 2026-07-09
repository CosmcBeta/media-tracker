use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Progress {
    pub id: Uuid,
    pub item_id: Uuid,
    pub kind: ProgressKind,
    pub value: Option<String>,
    pub note: Option<String>,
    pub logged_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ProgressKind {
    Episode,
    Page,
    Percentage,
    Complete,
}

#[derive(Debug, Deserialize)]
pub struct CreateProgress {
    pub kind: ProgressKind,
    pub value: Option<String>,
    pub note: Option<String>,
    pub logged_at: Option<String>,
}
