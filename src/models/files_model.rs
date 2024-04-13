use crate::errors::InternalError;
use axum::body::Bytes;
use sqlx::PgPool;

pub async fn new_file(
    pg_pool: &PgPool,
    file_name: &str,
    content: &Bytes,
    parent_folder_id: i32,
    owner_id: i32,
) -> Result<(), InternalError> {
    Ok(())
}
