use chrono::{Duration as ChronoDuration};
use tokio::time::{Duration};
use strum_macros::{EnumString, Display};
#[derive(Debug, Clone, Copy, EnumString, Display)]
pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
}

impl TimeUnit {
    pub fn to_duration(&self, value: u64) -> ChronoDuration {
        match self {
            TimeUnit::Seconds => ChronoDuration::seconds(value as i64),
            TimeUnit::Minutes => ChronoDuration::minutes(value as i64),
            TimeUnit::Hours => ChronoDuration::hours(value as i64),
        }
    }

    pub fn to_tokio_duration(&self, value: u64) -> Duration {
        match self {
            TimeUnit::Seconds => Duration::from_secs(value),
            TimeUnit::Minutes => Duration::from_secs(value * 60),
            TimeUnit::Hours => Duration::from_secs(value * 3600),
        }
    }
}