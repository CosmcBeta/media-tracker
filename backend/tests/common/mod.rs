use axum_test::TestServer;
use sqlx::SqlitePool;

use backend::{create_router, state::AppState};

#[allow(dead_code)]
pub async fn setup() -> TestServer {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    let state = AppState { db: pool };
    let app = create_router(state);
    TestServer::new(app)
}
