use sqlx::PgPool;

use crate::external::client::ApiClient;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: PgPool,
    pub client: ApiClient,
}
