use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use utoipa::{OpenApi};
use utoipa_swagger_ui::SwaggerUi;
use dotenvy::dotenv;
use std::env;
use database::{AppState, init_db};
use crate::database::delete_old_records;

mod entity;
mod handlers;
mod database;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    // Read port from environment or default to 3000
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    // Bind to the configured port
    let addr = format!("127.0.0.1:{}", port);

    let db_pool = init_db()
        .await
        .expect("Failed to initialize the database");

    // Spawn background task for deleting old records
    tokio::spawn(delete_old_records(db_pool.clone()));

    let state = AppState {
        db: Arc::new(db_pool),
    };

    let mut app = Router::new()
        .route("/cache/{key}", get(handlers::get_cache))
        .route("/cache", post(handlers::set_cache))
        .layer(tower_http::cors::CorsLayer::permissive())
        .with_state(state);

    let swagger_enabled = env::var("ENABLE_SWAGGER").unwrap_or_else(|_| "true".to_string()) == "true";
    if swagger_enabled {
        app = app.merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", handlers::ApiDoc::openapi()));
        println!("✅ Swagger UI enabled at http://{}/swagger", addr);
    } else {
        println!("🚀 Running in production mode (Swagger disabled)");
    }

    let listener = TcpListener::bind(&addr).await?;
    println!("🚀 Listening on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
