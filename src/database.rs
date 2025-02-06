use std::sync::Arc;
use std::time::Duration;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use tokio::time::interval;
use chrono::{Utc, Duration as ChronoDuration};
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
    sqlx::query(
        r#"
         CREATE TABLE cache (
             key TEXT PRIMARY KEY,
             value TEXT NOT NULL,
             created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
         );
         "#
    )
        .execute(&db_pool)
        .await?;

    Ok(db_pool)
}

pub async fn delete_old_records(db_pool: DbPool) {
    let mut interval = interval(Duration::from_secs(10));

    loop {
        interval.tick().await;

        let cutoff_time = Utc::now() - ChronoDuration::minutes(1);
        let cutoff_time_str = cutoff_time.format("%Y-%m-%d %H:%M:%S").to_string();

        match sqlx::query("DELETE FROM cache WHERE created_at < ?")
            .bind(cutoff_time_str)
            .execute(&db_pool)
            .await
        {
            Ok(result) => println!("Deleted {} old transactions", result.rows_affected()),
            Err(e) => println!("Error deleting old transactions: {}", e),
        }
    }
}
