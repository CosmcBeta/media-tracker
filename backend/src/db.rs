use sqlx::{Sqlite, SqlitePool, migrate::MigrateDatabase};

pub async fn connect(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    if !Sqlite::database_exists(database_url).await.unwrap_or(false) {
        Sqlite::create_database(database_url).await?;
    }

    let pool = SqlitePool::connect(database_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
