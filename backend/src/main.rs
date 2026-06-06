use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use serde::Serialize;
use std::{collections::HashMap, sync::{Arc, RwLock}};
use uuid::Uuid;

mod db;
mod models;

#[tokio::main]
async fn main() {
    let db = Db::default();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        // .route("/lists", get(lists_index).post(lists_create))
        .route("/items", get(items_index).post(items_create))
        .with_state(db);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn items_index(State(db): State<Db>) -> impl IntoResponse {
    let items = db.read().unwrap();

    let items = items.values().cloned().collect::<Vec<_>>();

    Json(items)
}

async fn items_create(State(db): State<Db>) -> impl IntoResponse
{
    let item = Item {
        id: Uuid::new_v4(),
        text: "Hey man".to_string(),
    };

    db.write().unwrap().insert(item.id, item.clone());

    (StatusCode::CREATED, Json(item))
}

// async fn lists_index(State(db): State<Db>) -> impl IntoResponse {
//     let lists = db.read().unwrap();

//     let lists = lists.values().cloned().collect::<Vec<_>>();

//     Json(lists)
// }

// async fn lists_create(State(db): State<Db>) -> impl IntoResponse
// {
//     let list = List {
//         id: Uuid::new_v4(),
//         items: "Hey man".to_string(),
//     };

//     db.write().unwrap().insert(item.id, item.clone());

//     (StatusCode::CREATED, Json(itme))
// }

type Db = Arc<RwLock<HashMap<Uuid, Item>>>;

#[derive(Debug, Serialize, Clone)]
struct Item {
    id: Uuid,
    text: String,
}

// #[derive(Debug, Serialize, Clone)]
// struct List {
//     id: Uuid,
//     items: Vec<Item>,
// }
