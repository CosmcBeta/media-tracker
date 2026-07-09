use axum_test::TestServer;
use reqwest::Client;
use sqlx::PgPool;
use std::env;

use backend::{
    create_router,
    external::{client::ApiClient, igdb},
    state::AppState,
};

#[allow(dead_code)]
pub const API: &str = "/api/v1";

#[allow(dead_code)]
pub async fn setup(pool: PgPool) -> TestServer {
    dotenvy::dotenv().ok();

    let client = Client::new();

    let tmdb_access_token = env::var("TMDB_ACCESS_TOKEN").expect("TMDB_ACCESS_TOKEN must be set");
    let igdb_client_id = env::var("IGDB_CLIENT_ID").expect("IGDB_CLIENT_ID must be set");
    let igdb_client_secret =
        env::var("IGDB_CLIENT_SECRET").expect("IGDB_CLIENT_SECRET must be set");
    let igdb_access_token = igdb::fetch_igdb_token(&client, &igdb_client_id, &igdb_client_secret)
        .await
        .expect("failed to get igdb access token");

    let api_client = ApiClient::new(client, tmdb_access_token, igdb_client_id, igdb_access_token);

    let state = AppState {
        db: pool,
        client: api_client,
    };
    let app = create_router(state);
    TestServer::new(app)
}
