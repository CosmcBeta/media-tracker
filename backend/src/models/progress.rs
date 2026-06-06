use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Progress {
    pub id: Uuid,
    pub itemId: Uuid,
    pub kind: Kind,
    pub value: Option<String>,
    pub note: Option<String>,
    pub loggedAt: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum Kind {
    Episode,
    Page,
    Percentage,
    Complete,
}
