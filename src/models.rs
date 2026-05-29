use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// db entity
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Url {
    pub id: Uuid,
    pub short_code: String,
    pub long_url: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUrlRequest {
    pub long_url: String,
    pub alias: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UrlResponse {
    pub short_code: String,
    pub short_url: String,
    pub long_url: String,
    pub expires_at: Option<String>,
}