use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::NaiveDateTime;
use chrono::{Utc};

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

// Helper function to convert SQLite timestamp (TEXT) to `NaiveDateTime`
impl CacheEntry {
    pub fn created_at_as_datetime(&self) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(&self.created_at, "%Y-%m-%d %H:%M:%S")
            .unwrap_or_else(|_| Utc::now().naive_utc()) // Fallback if parsing fails
    }
}

