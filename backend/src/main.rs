use dotenvy::dotenv;
use reqwest::Client;
use std::env;

use backend::{
    create_router, db,
    external::{client::ApiClient, igdb},
    state::AppState,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let tmdb_access_token = env::var("TMDB_ACCESS_TOKEN").expect("TMDB_ACCESS_TOKEN must be set");
    let igdb_client_id = env::var("IGDB_CLIENT_ID").expect("IGDB_CLIENT_ID must be set");
    let igdb_client_secret =
        env::var("IGDB_CLIENT_SECRET").expect("IGDB_CLIENT_SECRET must be set");

    let client = Client::new();
    let pool = db::connect(&database_url)
        .await
        .expect("failed to connect to database");

    let igdb_access_token = igdb::fetch_igdb_token(&client, &igdb_client_id, &igdb_client_secret)
        .await
        .expect("failed to get igdb access token");

    let api_client = ApiClient::new(client, tmdb_access_token, igdb_client_id, igdb_access_token);
    let state = AppState {
        db: pool,
        client: api_client,
    };
    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .expect("failed to bind to port");

    axum::serve(listener, app).await.expect("server error");
}
