use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
struct List {
    id: Uuid,
    name: String,
    icon: String,
    created_at: DateTime<Utc>,
}
