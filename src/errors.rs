use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub error: String,
    pub message: String,
    #[serde(skip)]          // don't include status in JSON body
    pub status: StatusCode,
}

impl ApiError {
    pub fn new(
        status: StatusCode, 
        error: &str, 
        message: &str
    ) -> Self {
        Self { 
            error: error.to_string(),
            message: message.to_string(),
            status
        }
    }

    pub fn bad_request(error: &str, message: &str) -> Self {
        Self::new(StatusCode::BAD_REQUEST, error, message)
    }

    pub fn not_found(error: &str, message: &str) -> Self {
        Self::new(StatusCode::NOT_FOUND, error, message)
    }

    pub fn conflict(error: &str, message: &str) -> Self {
        Self::new(StatusCode::CONFLICT, error, message)
    }

    pub fn internal(error: &str, message: &str) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, error, message)
    }

    pub fn gone(error: &str, message: &str) -> Self {
        Self::new(StatusCode::GONE, error, message)
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.error, self.message)
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        self.status
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status).json(self)
    }
}