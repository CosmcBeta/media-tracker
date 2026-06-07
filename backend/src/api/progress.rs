use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::{DateTime, Utc};
use sqlx::query;
use uuid::Uuid;

use crate::{
    models::progress::{CreateProgress, Progress, ProgressKind},
    state::AppState,
};

pub async fn get_item_progress(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let progresses = sqlx::query_as!(
        Progress,
        r#"SELECT
        id AS "id!: Uuid",
        item_id AS "item_id: Uuid",
        kind AS "kind: ProgressKind",
        value,
        note,
        logged_at as "logged_at: DateTime<Utc>"
        FROM progress WHERE item_id = ?"#,
        id
    )
    .fetch_all(&state.db)
    .await
    .unwrap();

    Json(progresses)
}

pub async fn create_item_progress(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(input): Json<CreateProgress>,
) -> impl IntoResponse {
    let logged_at = match input.logged_at {
        Some(s) => DateTime::parse_from_rfc3339(&s)
            .unwrap()
            .with_timezone(&Utc),
        None => Utc::now(),
    };

    let progress = Progress {
        id: Uuid::new_v4(),
        item_id: id,
        kind: input.kind,
        value: input.value,
        note: input.note,
        logged_at: logged_at,
    };

    sqlx::query!(
        r#"INSERT INTO progress (id, item_id, kind, value, note, logged_at)
        VALUES (?, ?, ?, ?, ?, ?)"#,
        progress.id,
        progress.item_id,
        progress.kind,
        progress.value,
        progress.note,
        progress.logged_at.to_rfc3339()
    )
    .execute(&state.db)
    .await
    .unwrap();

    (StatusCode::CREATED, Json(progress))
}

pub async fn delete_item_progress(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    query!("DELETE FROM progress WHERE id = ?", id)
        .execute(&state.db)
        .await
        .unwrap();

    StatusCode::NO_CONTENT
}
