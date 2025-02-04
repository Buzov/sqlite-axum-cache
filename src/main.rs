use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::sync::Arc;
use tokio::net::TcpListener;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
struct AppState {
    db: Arc<Pool<Sqlite>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // Default port 3000
    let addr = "127.0.0.1:3000";

    let db_pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await?;

    // Initialize schema
    sqlx::query("CREATE TABLE cache (key TEXT PRIMARY KEY, value TEXT);")
        .execute(&db_pool)
        .await?;

    let state = AppState {
        db: Arc::new(db_pool),
    };

    let app = Router::new()
        .route("/cache/{key}", get(get_cache))
        .route("/cache", post(set_cache))
        .layer(tower_http::cors::CorsLayer::permissive())
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(state);

    let listener = TcpListener::bind(&addr).await?;
    println!("🚀 Listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Serialize, Deserialize, ToSchema)]
struct CacheEntry {
    key: String,
    value: String,
}

/// Fetch a cached value by key
#[utoipa::path(
    get,
    path = "/cache/{key}",
    params(("key" = String, Path, description = "Cache key")),
    responses(
        (status = 200, description = "Cache hit", body = CacheEntry),
        (status = 404, description = "Cache miss")
    )
)]
async fn get_cache(State(state): State<AppState>, Path(key): Path<String>) -> Result<Json<CacheEntry>, StatusCode> {
    let row: Option<(String,)> = sqlx::query_as("SELECT value FROM cache WHERE key = ?")
        .bind(&key)
        .fetch_optional(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match row {
        Some((value,)) => Ok(Json(CacheEntry { key, value })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Insert a key-value pair into the cache
#[utoipa::path(
    post,
    path = "/cache",
    request_body = CacheEntry,
    responses(
        (status = 200, description = "Cache stored successfully"),
        (status = 500, description = "Error storing cache")
    )
)]
async fn set_cache(State(state): State<AppState>, Json(entry): Json<CacheEntry>) -> Result<StatusCode, StatusCode> {
    sqlx::query("INSERT INTO cache (key, value) VALUES (?, ?) ON CONFLICT(key) DO UPDATE SET value = excluded.value;")
        .bind(&entry.key)
        .bind(&entry.value)
        .execute(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

#[derive(OpenApi)]
#[openapi(paths(get_cache, set_cache), components(schemas(CacheEntry)))]
struct ApiDoc;
