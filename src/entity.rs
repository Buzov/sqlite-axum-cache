use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SaveCacheEntryRequest {
    pub key: String,
    pub value: String,
}
#[derive(Debug, Serialize, Deserialize, ToSchema, FromRow)]
pub struct CacheEntry {
    pub key: String,
    pub value: String,
    pub created_at: String,
}
