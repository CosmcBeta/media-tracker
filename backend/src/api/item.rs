use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::{DateTime, Utc};
use sqlx::{Arguments, AssertSqlSafe, SqlitePool, query, sqlite::SqliteArguments};
use uuid::Uuid;

use crate::{
    error::AppError,
    external::tmdb,
    models::{
        item::{CreateItem, Item, MediaType, SearchParams, UpdateItem},
        search::SearchCandidate,
    },
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
        external_id: None,
        metadata: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    insert_item(&state.db, &item).await?;

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

pub async fn search_items(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<impl IntoResponse, AppError> {
    let results = match params.media_type {
        MediaType::Show => {
            tmdb::search_shows(
                &state.client.client,
                &state.client.tmdb_access_token,
                &params.q,
            )
            .await?
        }
        MediaType::Movie => {
            tmdb::search_movies(
                &state.client.client,
                &state.client.tmdb_access_token,
                &params.q,
            )
            .await?
        }
        _ => return Err(AppError::BadRequest("unsupported media type".to_string())),
    };

    Ok(Json(results))
}

pub async fn import_item(
    State(state): State<AppState>,
    Json(candidate): Json<SearchCandidate>,
) -> Result<impl IntoResponse, AppError> {
    let item = Item {
        id: Uuid::new_v4(),
        media_type: candidate.media_type,
        title: candidate.title,
        external_id: Some(candidate.external_id.to_string()),
        metadata: Some(candidate.metadata),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    insert_item(&state.db, &item).await?;

    Ok((StatusCode::CREATED, Json(item)))
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

async fn insert_item(pool: &SqlitePool, item: &Item) -> Result<(), AppError> {
    let result = sqlx::query!(
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
    .execute(pool)
    .await;

    match result {
        Ok(_) => Ok(()),
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            Err(AppError::Conflict)
        }
        Err(e) => Err(AppError::Database(e))
    }
}
