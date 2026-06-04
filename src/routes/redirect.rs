use actix_web::{HttpResponse, get, web};
use chrono::Utc;

use crate::{cache::{CacheResult, CachedUrl, get_cached_url, set_cached_url, set_not_found}, db::urls::get_url_by_code, errors::ApiError, startup::AppState};

#[get("/{short_code}")]
pub async fn redirect(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let code =  path.into_inner();
    
    // validate
    if !code.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return Err(ApiError::bad_request(
            "invalid_short_code",
            "short code may only contain letters, numbers, and hyphens"
        ));
    }

    // check cache
    match get_cached_url(&state.redis_pool, &code).await {
        CacheResult::Hit(cached) => {
            if let Some(expires_at) = cached.expires_at {
                if expires_at < Utc::now() {
                    return Err(ApiError::gone("expired", "this short URL has expired"));
                }
            }

            return Ok(HttpResponse::Found()
                .insert_header(("Location", cached.long_url))
                .finish());
        },
        CacheResult::NotFound => {
            return Err(ApiError::not_found("not_found", "short code not found"));
        },
        CacheResult::Miss => {},
    }

    // check db in case of cache miss
    match get_url_by_code(&state.db_pool, &code).await {
        // 3. validate if short_code
        Ok(Some(url)) => {
            // populate cache
            let cached = CachedUrl {
                long_url: url.long_url.clone(),
                expires_at: url.expires_at,
            };

            set_cached_url(&state.redis_pool, &code, &cached).await;

            // - if not expired
            if let Some(expires_at) = url.expires_at {
                if expires_at < Utc::now() {
                    return Err(ApiError::gone(
                        "expired",
                        "this short URL has expired"
                    ));
                }
            } 
            
            Ok(HttpResponse::Found().insert_header(("Location", url.long_url)).finish())
        },
        // if no short code found
        Ok(None) => {
            set_not_found(&state.redis_pool, &code).await;
            Err(ApiError::not_found(
                "not_found",
                "short code not found"
            ))
        },
        // db error
        Err(_) => {
            Err(ApiError::internal(
                "db_error",
                "failed to look up short code"
            ))
        },
    }
}