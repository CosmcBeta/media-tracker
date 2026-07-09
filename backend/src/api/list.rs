use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::{DateTime, Utc};
use sqlx::{Arguments, AssertSqlSafe, PgPool, postgres::PgArguments, query};
use uuid::Uuid;

use crate::{
    api::item::get_item_by_id,
    error::AppError,
    models::{
        item::{Item, MediaType},
        list::{AddItemToList, CreateList, List, UpdateList},
    },
    state::AppState,
};

pub async fn get_lists(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let lists = sqlx::query_as!(
        List,
        r#"SELECT
        id AS "id!: Uuid",
        name,
        icon,
        created_at as "created_at: DateTime<Utc>",
        updated_at as "updated_at: DateTime<Utc>"
        FROM lists"#
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(lists))
}

pub async fn create_list(
    State(state): State<AppState>,
    Json(input): Json<CreateList>,
) -> Result<impl IntoResponse, AppError> {
    let list = List {
        id: Uuid::now_v7(),
        name: input.name,
        icon: input.icon,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    sqlx::query!(
        r#"INSERT INTO lists (id, name, icon, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)"#,
        list.id,
        list.name,
        list.icon,
        list.created_at,
        list.updated_at
    )
    .execute(&state.db)
    .await?;

    Ok((StatusCode::CREATED, Json(list)))
}

pub async fn update_list(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(input): Json<UpdateList>,
) -> Result<impl IntoResponse, AppError> {
    let mut args = PgArguments::default();
    args.add(&id)?;
    let mut sets = Vec::new();
    let mut idx = 2;

    if let Some(name) = input.name {
        sets.push(format!("name = ${idx}"));
        args.add(name)?;
        idx += 1;
    }

    if let Some(icon) = input.icon {
        sets.push(format!("icon = ${idx}"));
        args.add(icon)?;
        idx += 1;
    }

    if sets.is_empty() {
        return Err(AppError::BadRequest("no arguments given".to_string()));
    }

    sets.push(format!("updated_at = ${idx}"));
    args.add(Utc::now())?;

    let query = format!("UPDATE lists SET {} WHERE id = $1", sets.join(", "));
    sqlx::query_with(AssertSqlSafe(query), args)
        .execute(&state.db)
        .await?;

    let list = get_list_by_id(&state.db, &id).await?;

    Ok(Json(list))
}

pub async fn delete_list(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let result = query!("DELETE FROM lists WHERE id = $1", id)
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        Err(AppError::NotFound)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

pub async fn get_list_items(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    get_list_by_id(&state.db, &id).await?;

    let items = sqlx::query_as!(
        Item,
        r#"SELECT
        i.id AS "id!: Uuid",
        i.media_type AS "media_type: MediaType",
        i.title,
        i.external_id,
        i.metadata,
        i.created_at as "created_at: DateTime<Utc>",
        i.updated_at as "updated_at: DateTime<Utc>"
        FROM items i
        INNER JOIN list_items li ON i.id = li.item_id
        WHERE li.list_id = $1"#,
        id
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(items))
}

pub async fn add_item_to_list(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(input): Json<AddItemToList>,
) -> Result<impl IntoResponse, AppError> {
    get_list_by_id(&state.db, &id).await?;
    get_item_by_id(&state.db, &input.item_id).await?;

    sqlx::query!(
        r#"INSERT INTO list_items (list_id, item_id, added_at, sort_order)
        VALUES ($1, $2, $3, $4)"#,
        id,
        input.item_id,
        Utc::now(),
        0
    )
    .execute(&state.db)
    .await?;

    Ok(StatusCode::CREATED)
}

pub async fn delete_item_from_list(
    Path((id, item_id)): Path<(Uuid, Uuid)>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let result = query!(
        "DELETE FROM list_items WHERE list_id = $1 AND item_id = $2",
        id,
        item_id
    )
    .execute(&state.db)
    .await?;

    if result.rows_affected() == 0 {
        Err(AppError::NotFound)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

async fn get_list_by_id(pool: &PgPool, id: &Uuid) -> Result<List, AppError> {
    let list = sqlx::query_as!(
        List,
        r#"SELECT
        id AS "id!: Uuid",
        name,
        icon,
        created_at as "created_at: DateTime<Utc>",
        updated_at as "updated_at: DateTime<Utc>"
        FROM lists WHERE id = $1"#,
        id
    )
    .fetch_optional(pool)
    .await?;

    match list {
        Some(list) => Ok(list),
        None => Err(AppError::NotFound),
    }
}
