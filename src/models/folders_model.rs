use super::folder::Folder;
use crate::errors::InternalError;
use sqlx::PgPool;

pub async fn init_root_folders(pg_pool: &PgPool, owner_id: i32) -> Result<(), InternalError> {
    for f in ["My Cloud", "Trash"] {
        new_raw_folder(pg_pool, f, None, owner_id).await?;
    }
    Ok(())
}

pub async fn new_folder(
    pg_pool: &PgPool,
    folder_name: &str,
    parent_folder_id: i32,
    owner_id: i32,
) -> Result<Folder, InternalError> {
    new_raw_folder(pg_pool, folder_name, Some(parent_folder_id), owner_id).await
}

pub async fn get_root_folders(
    pg_pool: &PgPool,
    user_id: i32,
) -> Result<Vec<Folder>, InternalError> {
    sqlx::query_as!(
        Folder,
        "SELECT * FROM folders
        WHERE fk_owner = $1 AND fk_parent IS null;",
        user_id
    )
    .fetch_all(pg_pool)
    .await
    .map_err(|_| InternalError("Failed to add folder to database".to_string()))
}

pub async fn get_folders(
    pg_pool: &PgPool,
    parent_folder_id: i32,
    owner_id: i32,
) -> Result<Vec<Folder>, InternalError> {
    sqlx::query_as!(
        Folder,
        "SELECT *
        FROM folders
        WHERE fk_owner = $1 AND fk_parent = $2;",
        owner_id,
        parent_folder_id
    )
    .fetch_all(pg_pool)
    .await
    .map_err(|_| InternalError("Failed get the folders from the database".to_string()))
}

pub async fn rename_folder(
    pg_pool: &PgPool,
    file_id: i32,
    owner_id: i32,
    new_name: &str,
) -> Result<(), InternalError> {
    // TODO: Validate name
    sqlx::query!(
        "UPDATE folders
        SET name = $3
        WHERE id = $1 AND fk_owner = $2;",
        file_id,
        owner_id,
        new_name
    )
    .fetch_all(pg_pool)
    .await
    .map_err(|_| InternalError("Failed to rename the folder".to_string()))?;
    Ok(())
}

async fn new_raw_folder(
    pg_pool: &PgPool,
    folder_name: &str,
    parent_folder_id: Option<i32>,
    owner_id: i32,
) -> Result<Folder, InternalError> {
    // TODO: Validate name
    let folder = sqlx::query_as!(
        Folder,
        "INSERT INTO folders (name, last_modified, starred, fk_owner, fk_parent)
        VALUES ($1, CURRENT_TIMESTAMP, $2, $3, $4)
        RETURNING *;",
        folder_name,
        false,
        owner_id,
        parent_folder_id
    )
    .fetch_one(pg_pool)
    .await
    .map_err(|_| InternalError("Failed to add folder to database".to_string()))?;
    Ok(folder)
}
