use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::{DateTime, Utc};
use sqlx::{Arguments, AssertSqlSafe, SqlitePool, query, sqlite::SqliteArguments};
use uuid::Uuid;

use crate::{
    error::AppError,
    models::item::{CreateItem, Item, MediaType, UpdateItem},
    state::AppState,
};

pub async fn get_items(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let items = sqlx::query_as!(
        Item,
        r#"SELECT
        id AS "id!: Uuid",
        media_type AS "media_type: MediaType",
        title,
        external_id,
        metadata,
        created_at as "created_at: DateTime<Utc>",
        updated_at as "updated_at: DateTime<Utc>"
        FROM items"#
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(items))
}

pub async fn create_item(
    State(state): State<AppState>,
    Json(input): Json<CreateItem>,
) -> Result<impl IntoResponse, AppError> {
    let item = Item {
        id: Uuid::new_v4(),
        media_type: input.media_type,
        title: input.title,
        external_id: None, // none for now until we get external api
        metadata: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    sqlx::query!(
        r#"INSERT INTO items (id, media_type, title, external_id, metadata, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?)"#,
        item.id,
        item.media_type,
        item.title,
        item.external_id,
        item.metadata,
        item.created_at.to_rfc3339(),
        item.updated_at.to_rfc3339()
    )
    .execute(&state.db)
    .await?;

    Ok((StatusCode::CREATED, Json(item)))
}

pub async fn get_item(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let item = get_item_by_id(&state.db, &id).await?;

    Ok(Json(item))
}

pub async fn update_item(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(input): Json<UpdateItem>,
) -> Result<impl IntoResponse, AppError> {
    let mut sets = Vec::new();
    let mut args = SqliteArguments::default();

    if let Some(title) = input.title {
        sets.push("title = ?");
        args.add(title)?;
    }

    if let Some(media_type) = input.media_type {
        sets.push("media_type = ?");
        args.add(media_type)?;
    }

    if let Some(external_id) = input.external_id {
        sets.push("external_id = ?");
        args.add(external_id)?;
    }

    if let Some(metadata) = input.metadata {
        sets.push("metadata = ?");
        args.add(metadata)?;
    }

    if sets.is_empty() {
        return Err(AppError::BadRequest("no arguments given".to_string()));
    }

    sets.push("updated_at = ?");
    args.add(Utc::now().to_rfc3339())?;

    let query = format!("UPDATE items SET {} WHERE id = ?", sets.join(", "));
    args.add(&id)?;

    sqlx::query_with(AssertSqlSafe(query), args)
        .execute(&state.db)
        .await?;

    let item = get_item_by_id(&state.db, &id).await?;

    Ok(Json(item))
}

pub async fn delete_item(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let result = query!("DELETE FROM items WHERE id = ?", id)
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        Err(AppError::NotFound)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

pub async fn get_item_by_id(pool: &SqlitePool, id: &Uuid) -> Result<Item, AppError> {
    let item = sqlx::query_as!(
        Item,
        r#"SELECT
        id AS "id!: Uuid",
        media_type AS "media_type: MediaType",
        title,
        external_id,
        metadata,
        created_at as "created_at: DateTime<Utc>",
        updated_at as "updated_at: DateTime<Utc>"
        FROM items WHERE id = ?"#,
        id
    )
    .fetch_optional(pool)
    .await?;

    match item {
        Some(item) => Ok(item),
        None => Err(AppError::NotFound),
    }
}
