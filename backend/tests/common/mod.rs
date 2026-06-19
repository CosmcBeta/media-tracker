use axum_test::TestServer;
use sqlx::SqlitePool;

use backend::{create_router, external::client::ApiClient, state::AppState};

#[allow(dead_code)]
pub async fn setup() -> TestServer {
    dotenvy::dotenv().ok();

    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let tmdb_access_token =
        std::env::var("TMDB_ACCESS_TOKEN").expect("TMDB_ACCESS_TOKEN must be set");
    let client = ApiClient::new(tmdb_access_token);

    let state = AppState { db: pool, client };
    let app = create_router(state);
    TestServer::new(app)
}
