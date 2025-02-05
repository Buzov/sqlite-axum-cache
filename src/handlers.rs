use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use utoipa::OpenApi;
use crate::AppState;

use crate::entity::CacheEntry;

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
pub async fn get_cache(State(state): State<AppState>, Path(key): Path<String>) -> Result<Json<CacheEntry>, StatusCode> {
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
pub async fn set_cache(State(state): State<AppState>, Json(entry): Json<CacheEntry>) -> Result<StatusCode, StatusCode> {
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
pub struct ApiDoc;