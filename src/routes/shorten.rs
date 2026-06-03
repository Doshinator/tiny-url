use actix_web::{HttpResponse, post, web};
use crate::{db::urls::{insert_url, short_code_exists}, errors::ApiError, models::{CreateUrlRequest, UrlResponse}, startup::AppState, utils::{generate_short_code, validate_url}};

#[post("")]
pub async fn shorten(
    state: web::Data<AppState>,
    body: web::Json<CreateUrlRequest>
) -> Result<HttpResponse, ApiError> {
    // 1. validation
    if !validate_url(&body.long_url) {
        return Err(ApiError::bad_request(
            "invalid_url",
            "long_url must be a valid URL"
        ));
    }

    // 2. determine short_code
    let short_code: String = match &body.alias {
        Some(alias) => {
            if !alias.chars().all(|c| c.is_alphanumeric() || c == '-') {
                return Err(ApiError::bad_request(
                    "invalid_alias",
                    "alias may only contain letters, numbers, and hyphens"
                ));
            }

            match short_code_exists(&state.db_pool, alias).await {
                Ok(true) => return Err(ApiError::conflict(
                    "alias conflict",
                   "alias already in use" 
                )),
                Ok(false) => alias.clone(),
                Err(_) => return Err(ApiError::internal(
                    "db_error", 
                    "database error"    
                )),
            }

        },
        None => {
            let mut code = generate_short_code();
            let mut found = false;

            for _ in 0..3 {
                match short_code_exists(&state.db_pool, &code).await {
                    Ok(false) => { found = true; break; }
                    Ok(true)  => code = generate_short_code(),
                    Err(_)    => return Err(ApiError::internal("db_err", "database error")),
                }
            }
            
            if !found {
                return Err(ApiError::internal(
                    "generation_failed",
                    "failed to generate unique short code, try again"
                ));
            }
            code
        },
    };

    // 3. persist
    match insert_url(&state.db_pool, &short_code, &body.long_url).await {
        Ok(url) => {
            let response = UrlResponse {
                short_code: url.short_code.clone(),
                short_url: format!("http://127.0.0.1:8080/{}", url.short_code),
                long_url: url.long_url,
                expires_at: url.expires_at.map(|t| t.to_rfc3339()),
            };
            Ok(HttpResponse::Created().json(response))
        },
        Err(sqlx::Error::Database(db_err)) if db_err.constraint() == Some("urls_short_code_key") =>
        {
            Err(ApiError::conflict(
                "alias_conflict",
                "that alias is already in use"
            ))
        },
        Err(_) => Err(ApiError::internal(
            "db_error",
            "failed to persist URL"
        )),
    }
}
