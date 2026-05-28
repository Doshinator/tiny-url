use actix_web::{Responder, post, web};

use crate::{models::CreateUrlRequest, startup::AppState};

#[post("/shorten")]
pub async fn shorten(
    state: web::Data<AppState>,
    body: web::Json<CreateUrlRequest>
) -> impl Responder {
    // validate_url() 

    // i want to check if alias exists before we generate short_url
    
    // generate short_url ; perhaps it's a struct? Short { short_url, short_code} 
    
    // insert data into db
    
    // return json 
    todo!()
}