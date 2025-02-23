use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use tokio::time::interval;
use chrono::{Utc};
use time::TimeUnit;
use crate::time;

pub type DbPool = Pool<Sqlite>;

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

pub async fn delete_old_records(db_pool: DbPool, interval_value: u64, unit: TimeUnit) {
    let mut interval = interval(unit.to_tokio_duration(interval_value));

    loop {
        interval.tick().await;

        let cutoff_time = Utc::now() - unit.to_duration(interval_value);
        let cutoff_time_str = cutoff_time.format("%Y-%m-%d %H:%M:%S").to_string();

        match sqlx::query("DELETE FROM cache WHERE created_at < ?")
            .bind(cutoff_time_str)
            .execute(&db_pool)
            .await
        {
            Ok(result) => println!("Deleted {} old records", result.rows_affected()),
            Err(e) => println!("Error deleting old records: {}", e),
        }
    }
}
