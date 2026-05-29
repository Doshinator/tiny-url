use actix_web::{HttpResponse, get, web};
use chrono::Utc;

use crate::{db::urls::get_url_by_code, errors::ApiError, startup::AppState};

#[get("/{short_code}")]
pub async fn redirect(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let code =  path.into_inner();
    // 1. validate
    if !code.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return Err(ApiError::bad_request(
            "invalid_short_code",
            "short code may only contain letters, numbers, and hyphens"
        ));
    }

    // 2. check db
    match get_url_by_code(&state.db_pool, &code).await {
        // 3. validate if short_code
        Ok(Some(url)) => {
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