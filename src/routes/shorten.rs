use actix_web::{HttpResponse, Responder, post, web};
use crate::{db::urls::{insert_url, short_code_exists}, models::{CreateUrlRequest, UrlResponse}, startup::AppState, utils::{generate_short_code, validate_url}};

#[post("/shorten")]
pub async fn shorten(
    state: web::Data<AppState>,
    body: web::Json<CreateUrlRequest>
) -> impl Responder {
    // 1. validation
    if !validate_url(&body.long_url) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "Error": "Url is invalid",
            "message": "long_url must be a valid URL",
        }));
    }

    // 2. determine short_code
    let short_code: String = match &body.alias {
        Some(alias) => {
            if !alias.chars().all(|c| c.is_alphanumeric() || c == '-') {
                return HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "invalid_alias",
                    "message": "alias may only contain letters, numbers, and hyphens"
                }));
            }

            match short_code_exists(&state.db_pool, alias).await {
                Ok(true) => return HttpResponse::Conflict().json(serde_json::json!({
                    "error": "alias_conflict",
                    "message": "that alias is already in use"
                })),
                Ok(false) => alias.clone(),
                Err(_) => return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "db_error",
                    "message": "database error"
                })),
            }

        },
        None => {
            let mut code = generate_short_code();
            let mut found = false;

            for _ in 0..3 {
                match short_code_exists(&state.db_pool, &code).await {
                    Ok(false) => { found = true; break; }
                    Ok(true)  => code = generate_short_code(),
                    Err(_)    => return HttpResponse::InternalServerError().finish(),
                }
            }
            
            if !found {
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "generation_failed",
                    "message": "failed to generate unique short code, try again"
                }));
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
            HttpResponse::Created().json(response)
        },
        Err(sqlx::Error::Database(db_err)) if db_err.constraint() == Some("urls_short_code_key") =>
        {
            HttpResponse::Conflict().json(serde_json::json!({
                "error": "alias_conflict",
                "message": "that alias is already in use"
            }))
        },
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "db_error",
            "message": "failed to persist URL"
        })),
    }
}
