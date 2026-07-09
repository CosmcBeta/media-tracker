use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::{DateTime, Utc};
use sqlx::{Arguments, AssertSqlSafe, PgPool, postgres::PgArguments, query};
use uuid::Uuid;

use crate::{
    error::AppError,
    external::{igdb, musicbrainz, tmdb},
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
        id: Uuid::now_v7(),
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
    let mut args = PgArguments::default();
    args.add(&id)?;
    let mut sets = Vec::new();
    let mut idx = 2;

    if let Some(title) = input.title {
        sets.push(format!("title = ${idx}"));
        args.add(title)?;
        idx += 1;
    }

    if let Some(media_type) = input.media_type {
        sets.push(format!("media_type = ${idx}"));
        args.add(media_type)?;
        idx += 1;
    }

    if let Some(external_id) = input.external_id {
        sets.push(format!("external_id = ${idx}"));
        args.add(external_id)?;
        idx += 1;
    }

    if let Some(metadata) = input.metadata {
        sets.push(format!("metadata = ${idx}"));
        args.add(metadata)?;
        idx += 1;
    }

    if sets.is_empty() {
        return Err(AppError::BadRequest("no arguments given".to_string()));
    }

    sets.push(format!("updated_at = ${idx}"));
    args.add(Utc::now())?;

    let query = format!("UPDATE items SET {} WHERE id = $1", sets.join(", "));
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
    let result = query!("DELETE FROM items WHERE id = $1", id)
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
        MediaType::Album => {
            musicbrainz::search_release_groups(&state.client.client, &params.q).await?
        }
        MediaType::Artist => musicbrainz::search_artists(&state.client.client, &params.q).await?,
        MediaType::Game => {
            igdb::search_games(
                &state.client.client,
                &state.client.igdb_access_token,
                &state.client.igdb_client_id,
                &params.q,
            )
            .await?
        }
        MediaType::Book => {
            return Err(AppError::BadRequest(
                "book search is not yet implemented".to_string(),
            ));
        }
        MediaType::Podcast => {
            return Err(AppError::BadRequest(
                "podcast search is not yet implemented".to_string(),
            ));
        }
    };

    Ok(Json(results))
}

pub async fn import_item(
    State(state): State<AppState>,
    Json(candidate): Json<SearchCandidate>,
) -> Result<impl IntoResponse, AppError> {
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
        FROM items WHERE external_id = $1 AND media_type = $2"#,
        candidate.external_id,
        candidate.media_type as MediaType
    )
    .fetch_optional(&state.db)
    .await?;

    if let Some(item) = item {
        return Ok((StatusCode::OK, Json(item)));
    }

    let mut metadata = candidate.metadata;
    if let MediaType::Game = candidate.media_type {
        let time_to_complete = igdb::fetch_game_completion_time(
            &state.client.client,
            &state.client.igdb_access_token,
            &state.client.igdb_client_id,
            &candidate.external_id,
        )
        .await?;

        if let Some(ttc) = time_to_complete {
            metadata["time_to_beat"] = serde_json::to_value(&ttc)?;
        }
    }

    let item = Item {
        id: Uuid::now_v7(),
        media_type: candidate.media_type,
        title: candidate.title,
        external_id: Some(candidate.external_id),
        metadata: Some(metadata),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    insert_item(&state.db, &item).await?;

    Ok((StatusCode::CREATED, Json(item)))
}

pub async fn get_item_by_id(pool: &PgPool, id: &Uuid) -> Result<Item, AppError> {
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
        FROM items WHERE id = $1"#,
        id
    )
    .fetch_optional(pool)
    .await?;

    match item {
        Some(item) => Ok(item),
        None => Err(AppError::NotFound),
    }
}

async fn insert_item(pool: &PgPool, item: &Item) -> Result<(), AppError> {
    let result = sqlx::query!(
        r#"INSERT INTO items (id, media_type, title, external_id, metadata, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
        item.id,
        item.media_type as MediaType,
        item.title,
        item.external_id,
        item.metadata,
        item.created_at,
        item.updated_at
    )
    .execute(pool)
    .await;

    match result {
        Ok(_) => Ok(()),
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            Err(AppError::Conflict)
        }
        Err(e) => Err(AppError::Database(e)),
    }
}
