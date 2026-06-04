use deadpool_redis::Pool as RedisPool;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Deserialize, Serialize)]
pub struct CachedUrl {
    long_url: String,
    expires_at: Option<DateTime<Utc>>
}

pub enum CacheResult {
    Hit(CachedUrl),
    NotFound,
    Miss,
}

pub async fn get_cached_url(
    pool: &RedisPool,
    short_code: &str,
) -> CacheResult {
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return CacheResult::Miss,
    };

    let key = format!("url:{}", short_code);
    let result: Option<String> = match conn.get(&key).await {
        Ok(c) => c,
        Err(_) => return CacheResult::Miss,
    };

    match result {
        None => CacheResult::Miss,
        Some(v) if v == "NOT FOUND" => CacheResult::NotFound,
        Some(v) => match serde_json::from_str(&v) {
            Ok(cached) => CacheResult::Hit(cached),
            Err(_) => CacheResult::Miss,
        }
    }
}