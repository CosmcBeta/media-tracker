use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
struct Progress {
    id: Uuid,
    itemId: Uuid,
    kind: Kind,
    value: String,
    note: String,
    loggedAt: DateTime<Utc>,
}

#[derive(Debug, Serialize, Clone)]
enum Kind {
    Episode,
    Page,
    Percentage,
    Complete,
}
