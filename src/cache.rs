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

pub async fn set_cached_url(
    pool: &RedisPool,
    short_code: String,
    url: &CachedUrl
) {
    let mut conn  = match pool.get().await {
        Ok(c) => c,
        Err(_) => return ,
    };

    let key = format!("url:{}", short_code);
    let value = match serde_json::to_string(url) {
        Ok(v) => v,
        Err(_) => return,
    };

    let ttl = match url.expires_at {
        None => 3600i64,
        Some(exp) => {
            let secs = (exp - Utc::now()).num_seconds();
            if secs <= 0 { return; }
            secs.min(3600)
        },
    };

    let _: Result<(), _> = conn.set_ex(&key, value, ttl as u64).await;
}

pub async fn set_not_found(
    pool: &RedisPool,
    short_code: &str
) {
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return,
    };

    let key = format!("url:{}", short_code);
    let _: Result<(), _> = conn.set_ex(&key, "NOT FOUND", 60).await;
}