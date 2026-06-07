pub mod api;
pub mod db;
pub mod models;
pub mod state;

use axum::{
    Router,
    routing::{delete, get, patch},
};
use dotenvy::dotenv;

use crate::{
    api::{item, list, progress},
    state::AppState,
};

fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/items", get(item::get_items).post(item::create_item))
        .route(
            "/items/{id}",
            get(item::get_item)
                .patch(item::update_item)
                .delete(item::delete_item),
        )
        .route(
            "/items/{id}/progress",
            get(progress::get_item_progress).post(progress::create_item_progress),
        )
        .route("/lists", get(list::get_lists).post(list::create_list))
        .route(
            "/lists/{id}",
            patch(list::update_list).delete(list::delete_list),
        )
        .route(
            "/lists/{id}/items",
            get(list::get_list_items).post(list::add_item_to_list),
        )
        .route(
            "/lists/{id}/items/{item_id}",
            delete(list::delete_item_from_list),
        )
        .route("/progress/{id}", delete(progress::delete_item_progress))
        .with_state(state)
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let pool = db::connect(&database_url).await;
    let state = AppState { db: pool };

    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
