use sqlx::PgPool;

use crate::models::Url;

pub async fn insert_url(
    db_pool: &PgPool
) -> Result<Url, DbError> {
    // sqlx query::(Url, etc, etc)
    // .execute(state.db_pool)
    // .await 
}