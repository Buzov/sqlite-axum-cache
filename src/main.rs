use tokio::net::TcpListener;
use dotenvy::dotenv;
use std::env;
use application::AppState;
use database::init_db;
use crate::database::{delete_old_records, DbPool};
use time::TimeUnit;
use crate::application::create_app;

mod entity;
mod handlers;
mod database;
mod time;
mod application;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let db_pool = init_db()
        .await
        .expect("Failed to initialize the database");

    let addr = get_addr();

    init_schedule_task(db_pool.clone());

    let app = create_app(db_pool, &addr);

    let listener = TcpListener::bind(&addr).await?;
    println!("🚀 Listening on http://{}", &addr);

    axum::serve(listener, app).await?;

    Ok(())
}

fn init_schedule_task(db_pool: DbPool) {
    // Destructure tuple into separate variables
    let (interval_value, time_unit) = get_schedule_setting();
    // Spawn background task for deleting old records
    tokio::spawn(delete_old_records(db_pool, interval_value, time_unit));
}

fn get_addr() -> String {
    // Read port from environment or default to 3000
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    // Bind to the configured port
    let addr = format!("0.0.0.0:{}", port);
    addr
}

fn get_schedule_setting() -> (u64, TimeUnit) {
    let interval_value: u64 = env::var("INTERVAL_VALUE")
        .unwrap_or_else(|_| "5".to_string()) // Default: 5
        .parse()
        .expect("INTERVAL_VALUE must be a number");
    println!("interval_value: {}", interval_value);

    let time_unit: TimeUnit = env::var("TIME_UNIT")
        .unwrap_or_else(|_| "Minutes".to_string()) // Default: minutes
        .parse()
        .expect("Invalid TIME_UNIT. Use 'Seconds', 'Minutes', or 'Hours'.");
    println!("time_unit: {}", time_unit);
    (interval_value, time_unit)
}
