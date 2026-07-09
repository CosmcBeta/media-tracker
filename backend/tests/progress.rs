mod common;

use axum::http::StatusCode;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use common::setup;

#[sqlx::test]
async fn get_item_progress_returns_empty_array_when_none_exist(pool: PgPool) {
    let server = setup(pool).await;

    let response = server
        .post(&format!("{}/items", common::API))
        .json(&json!({"media_type": "show", "title": "One Piece"}))
        .await;
    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    let response = server
        .get(&format!("{}/items/{uuid}/progress", common::API))
        .await;

    response.assert_json(&json!([]));
}

#[sqlx::test]
async fn get_item_progress_returns_all_progresses(pool: PgPool) {
    let server = setup(pool).await;

    let response = server
        .post(&format!("{}/items", common::API))
        .json(&json!({"media_type": "show", "title": "One Piece"}))
        .await;
    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    server
        .post(&format!("{}/items/{uuid}/progress", common::API))
        .json(&json!({"kind": "episode"}))
        .await;
    server
        .post(&format!("{}/items/{uuid}/progress", common::API))
        .json(&json!({"kind": "page"}))
        .await;

    let response = server
        .get(&format!("{}/items/{uuid}/progress", common::API))
        .await;

    response.assert_status_ok();

    let body: serde_json::Value = response.json();
    assert_eq!(body.as_array().unwrap().len(), 2);
}

#[sqlx::test]
async fn get_item_progress_returns_404_when_item_not_found(pool: PgPool) {
    let server = setup(pool).await;

    let uuid = Uuid::now_v7();

    let response = server
        .get(&format!("{}/items/{uuid}/progress", common::API))
        .await;

    response.assert_status_not_found();
}

#[sqlx::test]
async fn create_item_progress_returns_404_when_item_not_found(pool: PgPool) {
    let server = setup(pool).await;

    let uuid = Uuid::now_v7();

    let response = server
        .post(&format!("{}/items/{uuid}/progress", common::API))
        .json(&json!({"kind": "episode"}))
        .await;

    response.assert_status_not_found();
}

#[sqlx::test]
async fn create_item_progress_returns_400_when_invalid_date_provided(pool: PgPool) {
    let server = setup(pool).await;

    let response = server
        .post(&format!("{}/items", common::API))
        .json(&json!({"media_type": "show", "title": "One Piece"}))
        .await;
    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    let response = server
        .post(&format!("{}/items/{uuid}/progress", common::API))
        .json(&json!({"kind": "episode", "logged_at": "today"}))
        .await;

    response.assert_status_bad_request();
}

#[sqlx::test]
async fn create_item_progress_returns_201_with_progress(pool: PgPool) {
    let server = setup(pool).await;

    let response = server
        .post(&format!("{}/items", common::API))
        .json(&json!({"media_type": "show", "title": "One Piece"}))
        .await;
    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    let response = server
        .post(&format!("{}/items/{uuid}/progress", common::API))
        .json(&json!({"kind": "episode"}))
        .await;
    let body: serde_json::Value = response.json();

    response.assert_status(StatusCode::CREATED);
    assert_eq!(body["kind"], "episode");
}

#[sqlx::test]
async fn create_item_progress_returns_422_when_missing_required_fields(pool: PgPool) {
    let server = setup(pool).await;

    let response = server
        .post(&format!("{}/items", common::API))
        .json(&json!({"media_type": "show", "title": "One Piece"}))
        .await;
    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    let response = server
        .post(&format!("{}/items/{uuid}/progress", common::API))
        .json(&json!({}))
        .await;

    response.assert_status_unprocessable_entity();
}

#[sqlx::test]
async fn delete_item_progress_returns_404_when_not_found(pool: PgPool) {
    let server = setup(pool).await;

    let uuid_progress = Uuid::now_v7();

    let response = server
        .delete(&format!("{}/progress/{uuid_progress}", common::API))
        .await;

    response.assert_status_not_found();
}

#[sqlx::test]
async fn delete_item_progress_returns_204(pool: PgPool) {
    let server = setup(pool).await;

    let response = server
        .post(&format!("{}/items", common::API))
        .json(&json!({"media_type": "show", "title": "One Piece"}))
        .await;
    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    let response = server
        .post(&format!("{}/items/{uuid}/progress", common::API))
        .json(&json!({"kind": "episode"}))
        .await;
    let body: serde_json::Value = response.json();
    let uuid_progress = body["id"].as_str().unwrap();

    let response = server
        .delete(&format!("{}/progress/{uuid_progress}", common::API))
        .await;

    response.assert_status_no_content();
}

#[sqlx::test]
async fn delete_item_progress_is_no_longer_retrievable(pool: PgPool) {
    let server = setup(pool).await;

    let response = server
        .post(&format!("{}/items", common::API))
        .json(&json!({"media_type": "show", "title": "One Piece"}))
        .await;
    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    let response = server
        .post(&format!("{}/items/{uuid}/progress", common::API))
        .json(&json!({"kind": "episode"}))
        .await;
    let body: serde_json::Value = response.json();
    let uuid_progress = body["id"].as_str().unwrap();

    server
        .delete(&format!("{}/progress/{uuid_progress}", common::API))
        .await;

    let response = server
        .get(&format!("{}/items/{uuid}/progress", common::API))
        .await;
    let body: serde_json::Value = response.json();
    assert_eq!(body.as_array().unwrap().len(), 0);
}
