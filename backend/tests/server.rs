mod common;

use sqlx::PgPool;

use crate::common::setup;

#[sqlx::test]
async fn health_check_returns_ok(pool: PgPool) {
    let server = setup(pool).await;
    let response = server.get(&format!("{}/health", common::API)).await;
    response.assert_status_ok();
}
