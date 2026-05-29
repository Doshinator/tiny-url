use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::Url;

pub async fn insert_url(
    pool: &PgPool,
    short_code: &str,
    long_url: &str,
) -> Result<Url, sqlx::Error> {
    let rec = sqlx::query_as!(
        Url,
        r#"
        INSERT INTO urls (id, short_code, long_url, created_at)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
        Uuid::new_v4(),
        short_code,
        long_url,
        Utc::now()
    )
    .fetch_one(pool)
    .await?;

    Ok(rec)
}

pub async fn short_code_exists(
    pool: &PgPool,
    short_code: &str
) -> Result<bool, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
        SELECT EXISTS(SELECT 1 FROM urls WHERE short_code = $1) as "exists!"
        "#,
        short_code
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.exists)
}

pub async fn get_url_by_code(
    pool: &PgPool,
    short_code: &str
) -> Result<Option<Url>, sqlx::Error> {
    let rec = sqlx::query_as!(
        Url,
        r#"
        SELECT * FROM urls WHERE short_code = $1
        "#,
        short_code
    )
    .fetch_optional(pool)
    .await?;

    Ok(rec)
}