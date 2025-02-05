use std::sync::Arc;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

pub type DbPool = Pool<Sqlite>;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DbPool>,
}

pub async fn init_db() -> Result<DbPool, sqlx::Error> {
    let db_url = "sqlite::memory:"; // Change to a file path for on-disk storage
    // let db_pool = Pool::<Sqlite>::connect(db_url)
    //     .await?;

    let db_pool = SqlitePoolOptions::new()
        .connect(db_url)
        .await?;

    // Initialize schema
    sqlx::query(r#"
         CREATE TABLE cache (
             key TEXT PRIMARY KEY,
             value TEXT NOT NULL
         );"#
    )
        .execute(&db_pool)
        .await?;

    Ok(db_pool)
}