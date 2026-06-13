mod common;

use axum::http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use crate::common::setup;

#[tokio::test]
async fn get_items_returns_empty_array_when_none_exist() {
    let server = setup().await;
    let response = server.get("/items").await;

    response.assert_json(&json!([]));
}

#[tokio::test]
async fn get_items_returns_all_items() {
    let server = setup().await;

    server
        .post("/items")
        .json(&json!({"media_type": "Show", "title": "One Piece"}))
        .await;
    server
        .post("/items")
        .json(&json!({"media_type": "Movie", "title": "Scary Movie"}))
        .await;

    let response = server.get("/items").await;

    response.assert_status_ok();

    let body: serde_json::Value = response.json();
    assert_eq!(body.as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn create_item_returns_201_with_item() {
    let server = setup().await;

    let response = server
        .post("/items")
        .json(&json!({"media_type": "Show", "title": "One Piece"}))
        .await;
    let body: serde_json::Value = response.json();

    response.assert_status(StatusCode::CREATED);

    assert_eq!(body["media_type"], "Show");
    assert_eq!(body["title"], "One Piece");
}

#[tokio::test]
async fn create_item_returns_422_with_missing_required_fields() {
    let server = setup().await;

    let response = server
        .post("/items")
        .json(&json!({"media_type": "Show"}))
        .await;

    response.assert_status_unprocessable_entity();
}

#[tokio::test]
async fn get_item_returns_404_when_not_found() {
    let server = setup().await;

    let uuid = Uuid::new_v4();

    let response = server.get(&format!("/items/{uuid}")).await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn get_item_returns_item_when_found() {
    let server = setup().await;

    let response = server
        .post("/items")
        .json(&json!({"media_type": "Show", "title": "One Piece"}))
        .await;

    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    let response = server.get(&format!("/items/{uuid}")).await;
    let body: serde_json::Value = response.json();

    response.assert_status_ok();

    assert_eq!(body["media_type"], "Show");
    assert_eq!(body["title"], "One Piece");
}

#[tokio::test]
async fn update_item_returns_400_with_no_fields() {
    let server = setup().await;

    let response = server
        .post("/items")
        .json(&json!({"media_type": "Show", "title": "One Piece"}))
        .await;

    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    let response = server
        .patch(&format!("/items/{uuid}"))
        .json(&json!({}))
        .await;

    response.assert_status_bad_request();
}

#[tokio::test]
async fn update_item_returns_404_when_not_found() {
    let server = setup().await;

    let uuid = Uuid::new_v4();

    let response = server
        .patch(&format!("/items/{uuid}"))
        .json(&json!({"media_type": "Movie"}))
        .await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn update_item_returns_updated_item() {
    let server = setup().await;

    let response = server
        .post("/items")
        .json(&json!({"media_type": "Show", "title": "One Piece"}))
        .await;

    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    assert_eq!(body["media_type"], "Show");
    assert_eq!(body["title"], "One Piece");

    let response = server
        .patch(&format!("/items/{uuid}"))
        .json(&json!({"title": "Attack On Titan"}))
        .await;
    let body: serde_json::Value = response.json();

    response.assert_status_ok();

    assert_eq!(body["media_type"], "Show");
    assert_eq!(body["title"], "Attack On Titan");
}

#[tokio::test]
async fn update_item_changes_are_persisted() {
    let server = setup().await;

    let response = server
        .post("/items")
        .json(&json!({"media_type": "Show", "title": "One Piece"}))
        .await;

    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    server
        .patch(&format!("/items/{uuid}"))
        .json(&json!({"title": "Attack On Titan"}))
        .await;

    let response = server.get(&format!("/items/{uuid}")).await;
    let body: serde_json::Value = response.json();

    assert_eq!(body["media_type"], "Show");
    assert_eq!(body["title"], "Attack On Titan");
}

#[tokio::test]
async fn delete_item_returns_404_when_not_found() {
    let server = setup().await;

    let uuid = Uuid::new_v4();

    let response = server.delete(&format!("/items/{uuid}")).await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn delete_item_returns_204() {
    let server = setup().await;

    let response = server
        .post("/items")
        .json(&json!({"media_type": "Show", "title": "One Piece"}))
        .await;

    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    let response = server.delete(&format!("/items/{uuid}")).await;

    response.assert_status_no_content();
}

#[tokio::test]
async fn delete_item_is_no_longer_retrievable() {
    let server = setup().await;

    let response = server
        .post("/items")
        .json(&json!({"media_type": "Show", "title": "One Piece"}))
        .await;

    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    server.delete(&format!("/items/{uuid}")).await;

    let response = server.get(&format!("/items/{uuid}")).await;

    response.assert_status_not_found();
}
