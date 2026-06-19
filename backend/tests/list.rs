mod common;

use axum::http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use common::setup;

#[tokio::test]
async fn get_lists_returns_empty_array_when_none_exist() {
    let server = setup().await;
    let response = server.get("/lists").await;

    response.assert_json(&json!([]));
}

#[tokio::test]
async fn get_lists_returns_all_lists() {
    let server = setup().await;

    server
        .post("/lists")
        .json(&json!({"name": "Movies to Watch"}))
        .await;
    server
        .post("/lists")
        .json(&json!({"name": "Books to Read"}))
        .await;

    let response = server.get("/lists").await;

    response.assert_status_ok();

    let body: serde_json::Value = response.json();
    assert_eq!(body.as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn create_list_returns_201_with_list() {
    let server = setup().await;

    let response = server
        .post("/lists")
        .json(&json!({"name": "Movies to Watch"}))
        .await;
    let body: serde_json::Value = response.json();

    response.assert_status(StatusCode::CREATED);

    assert_eq!(body["name"], "Movies to Watch");
}

#[tokio::test]
async fn create_list_returns_422_with_missing_required_fields() {
    let server = setup().await;

    let response = server
        .post("/lists")
        .json(&json!({"icon": "icon.png"}))
        .await;

    response.assert_status_unprocessable_entity();
}

#[tokio::test]
async fn update_list_returns_400_with_no_fields() {
    let server = setup().await;

    let response = server
        .post("/lists")
        .json(&json!({"name": "Movies to Watch"}))
        .await;

    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    let response = server
        .patch(&format!("/lists/{uuid}"))
        .json(&json!({}))
        .await;

    response.assert_status_bad_request();
}

#[tokio::test]
async fn update_list_returns_404_when_not_found() {
    let server = setup().await;

    let uuid = Uuid::new_v4();

    let response = server
        .patch(&format!("/lists/{uuid}"))
        .json(&json!({"name": "Movies to Watch"}))
        .await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn update_list_returns_updated_list() {
    let server = setup().await;

    let response = server
        .post("/lists")
        .json(&json!({"name": "Movies to Watch"}))
        .await;

    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    assert_eq!(body["name"], "Movies to Watch");

    let response = server
        .patch(&format!("/lists/{uuid}"))
        .json(&json!({"name": "Shows to Watch"}))
        .await;
    let body: serde_json::Value = response.json();

    response.assert_status_ok();

    assert_eq!(body["name"], "Shows to Watch");
}

#[tokio::test]
async fn update_list_changes_are_persisted() {
    let server = setup().await;

    let response = server
        .post("/lists")
        .json(&json!({"name": "Movies to Watch"}))
        .await;

    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    server
        .patch(&format!("/lists/{uuid}"))
        .json(&json!({"name": "Shows to Watch"}))
        .await;

    let response = server.get("/lists").await;
    let body: serde_json::Value = response.json();

    let name = body.as_array().unwrap().get(0).unwrap()["name"]
        .as_str()
        .unwrap();

    assert_eq!(name, "Shows to Watch");
}

#[tokio::test]
async fn delete_list_returns_404_when_not_found() {
    let server = setup().await;

    let uuid = Uuid::new_v4();

    let response = server.delete(&format!("/lists/{uuid}")).await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn delete_list_returns_204() {
    let server = setup().await;

    let response = server
        .post("/lists")
        .json(&json!({"name": "Movies to Watch"}))
        .await;

    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    let response = server.delete(&format!("/lists/{uuid}")).await;

    response.assert_status_no_content();
}

#[tokio::test]
async fn delete_list_is_no_longer_retrievable() {
    let server = setup().await;

    let response = server
        .post("/lists")
        .json(&json!({"name": "Movies to Watch"}))
        .await;

    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    server.delete(&format!("/lists/{uuid}")).await;

    let response = server.get("/lists").await;

    response.assert_json(&json!([]));
}

#[tokio::test]
async fn get_list_items_returns_empty_array_when_none_exist() {
    let server = setup().await;

    let response = server
        .post("/lists")
        .json(&json!({"name": "Movies to Watch"}))
        .await;

    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    let response = server.get(&format!("/lists/{uuid}/items")).await;

    response.assert_json(&json!([]));
}

#[tokio::test]
async fn get_list_items_returns_404_when_not_found() {
    let server = setup().await;

    let uuid = Uuid::new_v4();

    let response = server.get(&format!("/lists/{uuid}/items")).await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn get_list_items_returns_all_items() {
    let server = setup().await;

    let response = server
        .post("/items")
        .json(&json!({"media_type": "show", "title": "One Piece"}))
        .await;
    let body: serde_json::Value = response.json();
    let uuid_item_1 = body["id"].as_str().unwrap();

    let response = server
        .post("/items")
        .json(&json!({"media_type": "movie", "title": "Scary Movie"}))
        .await;
    let body: serde_json::Value = response.json();
    let uuid_item_2 = body["id"].as_str().unwrap();

    let response = server
        .post("/lists")
        .json(&json!({"name": "Movies to Watch"}))
        .await;

    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    server
        .post(&format!("/lists/{uuid}/items"))
        .json(&json!({"item_id": uuid_item_1}))
        .await;

    server
        .post(&format!("/lists/{uuid}/items"))
        .json(&json!({"item_id": uuid_item_2}))
        .await;

    let response = server.get(&format!("/lists/{uuid}/items")).await;
    let body: serde_json::Value = response.json();

    response.assert_status_ok();
    assert_eq!(body.as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn add_item_to_list_returns_404_when_list_not_found() {
    let server = setup().await;

    let response = server
        .post("/items")
        .json(&json!({"media_type": "show", "title": "One Piece"}))
        .await;
    let body: serde_json::Value = response.json();
    let uuid_item = body["id"].as_str().unwrap();

    let uuid = Uuid::new_v4();

    let response = server
        .post(&format!("/lists/{uuid}/items"))
        .json(&json!({"item_id": uuid_item}))
        .await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn add_item_to_list_returns_404_when_item_not_found() {
    let server = setup().await;

    let response = server
        .post("/lists")
        .json(&json!({"name": "Movies to Watch"}))
        .await;

    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();
    let uuid_item = Uuid::new_v4();

    let response = server
        .post(&format!("/lists/{uuid}/items"))
        .json(&json!({"item_id": format!("{uuid_item}")}))
        .await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn add_item_to_list_returns_422_with_missing_item_id() {
    let server = setup().await;

    server
        .post("/items")
        .json(&json!({"media_type": "show", "title": "One Piece"}))
        .await;

    let response = server
        .post("/lists")
        .json(&json!({"name": "Movies to Watch"}))
        .await;

    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    let response = server
        .post(&format!("/lists/{uuid}/items"))
        .json(&json!({}))
        .await;

    response.assert_status_unprocessable_entity();
}

#[tokio::test]
async fn add_item_to_list_returns_201_when_added() {
    let server = setup().await;

    let response = server
        .post("/items")
        .json(&json!({"media_type": "show", "title": "One Piece"}))
        .await;

    let body: serde_json::Value = response.json();
    let uuid_item = body["id"].as_str().unwrap();

    let response = server
        .post("/lists")
        .json(&json!({"name": "Movies to Watch"}))
        .await;

    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    let response = server
        .post(&format!("/lists/{uuid}/items"))
        .json(&json!({"item_id": format!("{uuid_item}")}))
        .await;

    response.assert_status(StatusCode::CREATED);
}

#[tokio::test]
async fn delete_item_from_list_returns_404_when_not_found() {
    let server = setup().await;

    let response = server
        .post("/lists")
        .json(&json!({"name": "Movies to Watch"}))
        .await;

    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();
    let uuid_item = Uuid::new_v4();

    let response = server
        .delete(&format!("/lists/{uuid}/items/{uuid_item}"))
        .await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn delete_item_from_list_returns_204() {
    let server = setup().await;

    let response = server
        .post("/items")
        .json(&json!({"media_type": "show", "title": "One Piece"}))
        .await;

    let body: serde_json::Value = response.json();
    let uuid_item = body["id"].as_str().unwrap();

    let response = server
        .post("/lists")
        .json(&json!({"name": "Movies to Watch"}))
        .await;

    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    server
        .post(&format!("/lists/{uuid}/items"))
        .json(&json!({"item_id": format!("{uuid_item}")}))
        .await;

    let response = server
        .delete(&format!("/lists/{uuid}/items/{uuid_item}"))
        .await;

    response.assert_status_no_content();
}

#[tokio::test]
async fn delete_item_from_list_is_no_longer_on_list() {
    let server = setup().await;

    let response = server
        .post("/items")
        .json(&json!({"media_type": "show", "title": "One Piece"}))
        .await;

    let body: serde_json::Value = response.json();
    let uuid_item = body["id"].as_str().unwrap();

    let response = server
        .post("/lists")
        .json(&json!({"name": "Movies to Watch"}))
        .await;

    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    server
        .post(&format!("/lists/{uuid}/items"))
        .json(&json!({"item_id": format!("{uuid_item}")}))
        .await;

    server
        .delete(&format!("/lists/{uuid}/items/{uuid_item}"))
        .await;

    let response = server.get(&format!("/lists/{uuid}/items")).await;

    response.assert_json(&json!([]));
}
