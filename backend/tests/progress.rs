mod common;

use axum::http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use crate::common::setup;

#[tokio::test]
async fn get_item_progress_returns_empty_array_when_none_exist() {
    let server = setup().await;

    let response = server
        .post("/items")
        .json(&json!({"media_type": "Show", "title": "One Piece"}))
        .await;
    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    let response = server.get(&format!("/items/{uuid}/progress")).await;

    response.assert_json(&json!([]));
}

#[tokio::test]
async fn get_item_progress_returns_all_progresses() {
    let server = setup().await;

    let response = server
        .post("/items")
        .json(&json!({"media_type": "Show", "title": "One Piece"}))
        .await;
    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    server
        .post(&format!("/items/{uuid}/progress"))
        .json(&json!({"kind": "Episode"}))
        .await;
    server
        .post(&format!("/items/{uuid}/progress"))
        .json(&json!({"kind": "Page"}))
        .await;

    let response = server.get(&format!("/items/{uuid}/progress")).await;

    response.assert_status_ok();

    let body: serde_json::Value = response.json();
    assert_eq!(body.as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn get_item_progress_returns_404_when_item_not_found() {
    let server = setup().await;

    let uuid = Uuid::new_v4();

    let response = server.get(&format!("/items/{uuid}/progress")).await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn create_item_progress_returns_404_when_item_not_found() {
    let server = setup().await;

    let uuid = Uuid::new_v4();

    let response = server
        .post(&format!("/items/{uuid}/progress"))
        .json(&json!({"kind": "Episode"}))
        .await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn create_item_progress_returns_400_when_invalid_date_provided() {
    let server = setup().await;

    let response = server
        .post("/items")
        .json(&json!({"media_type": "Show", "title": "One Piece"}))
        .await;
    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    let response = server
        .post(&format!("/items/{uuid}/progress"))
        .json(&json!({"kind": "Episode", "logged_at": "today"}))
        .await;

    response.assert_status_bad_request();
}

#[tokio::test]
async fn create_item_progress_returns_201_with_progress() {
    let server = setup().await;

    let response = server
        .post("/items")
        .json(&json!({"media_type": "Show", "title": "One Piece"}))
        .await;
    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    let response = server
        .post(&format!("/items/{uuid}/progress"))
        .json(&json!({"kind": "Episode"}))
        .await;
    let body: serde_json::Value = response.json();

    response.assert_status(StatusCode::CREATED);
    assert_eq!(body["kind"], "Episode");
}

#[tokio::test]
async fn create_item_progress_returns_422_when_missing_required_fields() {
    let server = setup().await;

    let response = server
        .post("/items")
        .json(&json!({"media_type": "Show", "title": "One Piece"}))
        .await;
    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    let response = server
        .post(&format!("/items/{uuid}/progress"))
        .json(&json!({}))
        .await;

    response.assert_status_unprocessable_entity();
}

#[tokio::test]
async fn delete_item_progress_returns_404_when_not_found() {
    let server = setup().await;

    let uuid_progress = Uuid::new_v4();

    let response = server.delete(&format!("/progress/{uuid_progress}")).await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn delete_item_progress_returns_204() {
    let server = setup().await;

    let response = server
        .post("/items")
        .json(&json!({"media_type": "Show", "title": "One Piece"}))
        .await;
    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    let response = server
        .post(&format!("/items/{uuid}/progress"))
        .json(&json!({"kind": "Episode"}))
        .await;
    let body: serde_json::Value = response.json();
    let uuid_progress = body["id"].as_str().unwrap();

    let response = server.delete(&format!("/progress/{uuid_progress}")).await;

    response.assert_status_no_content();
}

#[tokio::test]
async fn delete_item_progress_is_no_longer_retrievable() {
    let server = setup().await;

    let response = server
        .post("/items")
        .json(&json!({"media_type": "Show", "title": "One Piece"}))
        .await;
    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    let response = server
        .post(&format!("/items/{uuid}/progress"))
        .json(&json!({"kind": "Episode"}))
        .await;
    let body: serde_json::Value = response.json();
    let uuid_progress = body["id"].as_str().unwrap();

    server.delete(&format!("/progress/{uuid_progress}")).await;

    let response = server.get(&format!("/items/{uuid}/progress")).await;
    let body: serde_json::Value = response.json();
    assert_eq!(body.as_array().unwrap().len(), 0);
}
