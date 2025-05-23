use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use utoipa::OpenApi;
use crate::AppState;

use crate::entity::{SaveCacheEntryRequest, CacheEntry};
use chrono::{Utc};
use tracing::{info, debug};
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
    let key_for_log = key.clone(); // clone before consuming
    info!(%key_for_log, "Received request for cache entry");
    let row = sqlx::query_as::<_, CacheEntry>(
        "SELECT key, value, created_at FROM cache WHERE key = ?"
    )
        .bind(key)
        .fetch_optional(&*state.db)
        .await
        .map_err(|err| {
            info!(error = %err, "Database query failed");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    match row {
        Some(cache_entry) => {
            info!(%key_for_log, "Cache hit");
            Ok(Json(cache_entry))
        },
        None => {
            info!(%key_for_log, "Cache miss");
            Err(StatusCode::NOT_FOUND)
        },
    }
}

/// Insert a key-value pair into the cache
#[utoipa::path(
    post,
    path = "/cache",
    request_body = SaveCacheEntryRequest,
    responses(
        (status = 200, description = "Cache stored successfully"),
        (status = 500, description = "Error storing cache")
    )
)]
pub async fn set_cache(State(state): State<AppState>, Json(entry): Json<SaveCacheEntryRequest>) -> Result<StatusCode, StatusCode> {
    let key_for_log = entry.key.clone(); // clone before consuming
    info!(key_for_log, "Received request to set cache");
    let created_at: String = Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string();

    sqlx::query(
        r#"
         INSERT INTO cache (key, value, created_at)
         VALUES (?, ?, ?)
         ON CONFLICT(key)
         DO UPDATE SET value = excluded.value;
         "#
    )
        .bind(&entry.key)
        .bind(&entry.value)
        .bind(created_at)
        .execute(&*state.db)
        .await
        .map_err(|err| {
            info!(key_for_log, error = %err, "Failed to insert or update cache entry");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    debug!(key_for_log, "Cache entry successfully saved or updated");

    Ok(StatusCode::OK)
}

#[derive(OpenApi)]
#[openapi(paths(get_cache, set_cache), components(schemas(CacheEntry)))]
pub struct ApiDoc;