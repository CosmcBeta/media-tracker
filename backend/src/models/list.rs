use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct List {
    pub id: Uuid,
    pub name: String,
    pub icon: Option<String>,
    pub created_at: DateTime<Utc>,
}
