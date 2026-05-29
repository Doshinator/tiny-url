use actix_web::{HttpResponse, Responder, get, web};
use chrono::Utc;

use crate::{db::urls::get_url_by_code, startup::AppState};

#[get("/{short_code}")]
pub async fn redirect(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let code =  path.into_inner();
    // 1. validate
    if !code.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "invalid short code",
            "message": "shortcode may only contain letters, numbers, and hyphens"
        }));
    }

    // 2. check db
    match get_url_by_code(&state.db_pool, &code).await {
        // 3. validate if short_code
        Ok(Some(url)) => {
            // - if not expired
            if let Some(expires_at) = url.expires_at {
                if expires_at < Utc::now() {
                    return HttpResponse::Gone().json(serde_json::json!({
                        "error": "expired",
                "message": "this short URL has expired"
                    }));
                }
            } 
            
            HttpResponse::Found()
                .insert_header(("Location", url.long_url))
                .finish()
        },
        // if no short code found
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "record not found",
                "message": "short code not found"
            }))
        },
        // db error
        Err(_) => {
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "db_error",
                "message": "failed to look up short code"
            }))
        },
    }
}