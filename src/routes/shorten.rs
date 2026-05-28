use actix_web::{HttpResponse, Responder, post, web};

use crate::{models::CreateUrlRequest, startup::AppState};

#[post("/shorten")]
pub async fn shorten(
    state: web::Data<AppState>,
    body: web::Json<CreateUrlRequest>
) -> impl Responder {
   
   
   HttpResponse::Ok().finish()
}