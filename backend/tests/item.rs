mod common;

use axum::http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use common::setup;

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
        .json(&json!({"media_type": "show", "title": "One Piece"}))
        .await;
    server
        .post("/items")
        .json(&json!({"media_type": "movie", "title": "Scary Movie"}))
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
        .json(&json!({"media_type": "show", "title": "One Piece"}))
        .await;
    let body: serde_json::Value = response.json();

    response.assert_status(StatusCode::CREATED);

    assert_eq!(body["media_type"], "show");
    assert_eq!(body["title"], "One Piece");
}

#[tokio::test]
async fn create_item_returns_422_with_missing_required_fields() {
    let server = setup().await;

    let response = server
        .post("/items")
        .json(&json!({"media_type": "show"}))
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
        .json(&json!({"media_type": "show", "title": "One Piece"}))
        .await;

    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    let response = server.get(&format!("/items/{uuid}")).await;
    let body: serde_json::Value = response.json();

    response.assert_status_ok();

    assert_eq!(body["media_type"], "show");
    assert_eq!(body["title"], "One Piece");
}

#[tokio::test]
async fn update_item_returns_400_with_no_fields() {
    let server = setup().await;

    let response = server
        .post("/items")
        .json(&json!({"media_type": "show", "title": "One Piece"}))
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
        .json(&json!({"media_type": "movie"}))
        .await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn update_item_returns_updated_item() {
    let server = setup().await;

    let response = server
        .post("/items")
        .json(&json!({"media_type": "show", "title": "One Piece"}))
        .await;

    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    assert_eq!(body["media_type"], "show");
    assert_eq!(body["title"], "One Piece");

    let response = server
        .patch(&format!("/items/{uuid}"))
        .json(&json!({"title": "Attack On Titan"}))
        .await;
    let body: serde_json::Value = response.json();

    response.assert_status_ok();

    assert_eq!(body["media_type"], "show");
    assert_eq!(body["title"], "Attack On Titan");
}

#[tokio::test]
async fn update_item_changes_are_persisted() {
    let server = setup().await;

    let response = server
        .post("/items")
        .json(&json!({"media_type": "show", "title": "One Piece"}))
        .await;

    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    server
        .patch(&format!("/items/{uuid}"))
        .json(&json!({"title": "Attack On Titan"}))
        .await;

    let response = server.get(&format!("/items/{uuid}")).await;
    let body: serde_json::Value = response.json();

    assert_eq!(body["media_type"], "show");
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
        .json(&json!({"media_type": "show", "title": "One Piece"}))
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
        .json(&json!({"media_type": "show", "title": "One Piece"}))
        .await;

    let body: serde_json::Value = response.json();
    let uuid = body["id"].as_str().unwrap();

    server.delete(&format!("/items/{uuid}")).await;

    let response = server.get(&format!("/items/{uuid}")).await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn search_items_returns_results() {
    let server = setup().await;

    let response = server
        .get("/items/search")
        .add_query_param("q", "One Piece")
        .add_query_param("media_type", "show")
        .await;
    let body: serde_json::Value = response.json();

    response.assert_status_ok();
    assert!(body.as_array().unwrap().len() > 0);
}

#[tokio::test]
async fn search_items_returns_400_for_unsupported_media_type() {
    let server = setup().await;

    let response = server
        .get("/items/search")
        .add_query_param("q", "One Piece")
        .add_query_param("media_type", "not_a_media_type")
        .await;

    response.assert_status_bad_request();
}

#[tokio::test]
async fn import_item_returns_201_with_item() {
    let server = setup().await;

    let response = server
        .post("/items/import")
        .json(&json!({
            "external_id": 37854,
            "title": "One Piece",
            "year": "1999-10-20",
            "media_type": "show",
            "description": "Years ago...",
            "poster_url": "/dB4EDhre2dsC2kxYDavyKWqLQwi.jpg",
            "metadata": r#"{"id":37854,"name":"One Piece","overview":"Years ago...",
                "first_air_date":"1999-10-20","poster_path":"/dB4EDhre2dsC2kxYDavyKWqLQwi.jpg",
                "backdrop_path":"/2rmK7mnchw9Xr3XdiTFSxTTLXqv.jpg","genre_ids":[10759,35,16],
                "origin_country":["JP"],"original_language":"ja","vote_average":8.738,"vote_count":5355,
                "popularity":43.6558}"#
        }))
        .await;
    let body: serde_json::Value = response.json();

    response.assert_status(StatusCode::CREATED);
    assert_eq!(body["media_type"], "show");
    assert_eq!(body["title"], "One Piece");
}

#[tokio::test]
async fn import_item_returns_409_with_existing_external_id() {
    let server = setup().await;

    server
        .post("/items/import")
        .json(&json!({
            "external_id": 37854,
            "title": "One Piece",
            "year": "1999-10-20",
            "media_type": "show",
            "description": "Years ago...",
            "poster_url": "/dB4EDhre2dsC2kxYDavyKWqLQwi.jpg",
            "metadata": r#"{"id":37854,"name":"One Piece","overview":"Years ago...",
                "first_air_date":"1999-10-20","poster_path":"/dB4EDhre2dsC2kxYDavyKWqLQwi.jpg",
                "backdrop_path":"/2rmK7mnchw9Xr3XdiTFSxTTLXqv.jpg","genre_ids":[10759,35,16],
                "origin_country":["JP"],"original_language":"ja","vote_average":8.738,"vote_count":5355,
                "popularity":43.6558}"#
        }))
        .await;

    let response = server
        .post("/items/import")
        .json(&json!({
            "external_id": 37854,
            "title": "ONE PIECE",
            "year": "2023-08-31",
            "media_type": "show",
            "description": "With his straw...",
            "poster_url": "/blWCPEqDGLBuLB9u89CxP9ORQP4.jpg",
            "metadata": r#"{"id":37854,"name":"ONE PIECE","overview":"With his straw...",
                "first_air_date":"2023-08-31","poster_path":"/blWCPEqDGLBuLB9u89CxP9ORQP4.jpg",
                "backdrop_path":"/qD211Hb5XwFxrszzBBe5EUYJerh.jpg","genre_ids":[10759,10765],
                "origin_country":["US"],"original_language":"en","vote_average":8.125,"vote_count":1794,
                "popularity":36.544}"#
        }))
        .await;

    response.assert_status_conflict();
}

#[tokio::test]
async fn import_item_returns_422_with_missing_required_fields() {
    let server = setup().await;

    let response = server
        .post("/items/import")
        .json(&json!({
            "title": "One Piece",
            "year": "1999-10-20",
            "media_type": "show",
            "description": "Years ago...",
            "poster_url": "/dB4EDhre2dsC2kxYDavyKWqLQwi.jpg",
            "metadata": r#"{"id":37854,"name":"One Piece","overview":"Years ago...",
                "first_air_date":"1999-10-20","poster_path":"/dB4EDhre2dsC2kxYDavyKWqLQwi.jpg",
                "backdrop_path":"/2rmK7mnchw9Xr3XdiTFSxTTLXqv.jpg","genre_ids":[10759,35,16],
                "origin_country":["JP"],"original_language":"ja","vote_average":8.738,"vote_count":5355,
                "popularity":43.6558}"#
        }))
        .await;

    response.assert_status_unprocessable_entity();
}
