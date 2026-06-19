pub mod api;
pub mod db;
pub mod error;
pub mod external;
pub mod models;
pub mod state;

use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use crate::{
    api::{item, list, progress},
    state::AppState,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/items", get(item::get_items).post(item::create_item))
        .route("/items/search", get(item::search_items))
        .route("/items/import", post(item::import_items))
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
