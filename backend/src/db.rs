use sqlx::{Sqlite, SqlitePool, migrate::MigrateDatabase};

pub async fn connect(database_url: &str) -> SqlitePool {
    if !Sqlite::database_exists(database_url).await.unwrap_or(false) {
        match Sqlite::create_database(database_url).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error), // change this into an error we can catch.
        }
    }

    let db = SqlitePool::connect(database_url).await.unwrap();

    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations = std::path::Path::new(&crate_dir).join("./migrations");

    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(&db)
        .await;

    match migration_results {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }

    db
}
