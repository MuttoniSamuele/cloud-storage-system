use super::{
    files_model::{self, validate_name},
    folder::Folder,
};
use crate::errors::{FileError, InternalError};
use sqlx::PgPool;

pub async fn init_root_folders(pg_pool: &PgPool, owner_id: i32) -> Result<(), InternalError> {
    for f in ["My Cloud", "Trash"] {
        new_raw_folder(pg_pool, f, None, owner_id)
            .await
            .map_err(|_| InternalError("Something went wrong.".to_string()))?;
    }
    Ok(())
}

pub async fn new_folder(
    pg_pool: &PgPool,
    folder_name: &str,
    parent_folder_id: i32,
    owner_id: i32,
) -> Result<Folder, FileError> {
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
) -> Result<(), FileError> {
    if !validate_name(new_name) {
        return Err(FileError::NameError);
    }
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
    .map_err(|_| FileError::InternalError)?;
    Ok(())
}

pub async fn folder_size(
    pg_pool: &PgPool,
    folder_id: i32,
    owner_id: i32,
    filter: Option<&str>,
) -> Result<i64, InternalError> {
    match filter {
        Some(f) => folder_size_filter(pg_pool, folder_id, owner_id, f).await,
        None => folder_size_no_filter(pg_pool, folder_id, owner_id).await,
    }
}

async fn folder_size_no_filter(
    pg_pool: &PgPool,
    folder_id: i32,
    owner_id: i32,
) -> Result<i64, InternalError> {
    // Recursive query to get the storage of the folder and all its children
    let size = sqlx::query!(
        "SELECT SUM(size) as size
        FROM files
        WHERE fk_parent IN (SELECT folder_id FROM get_folder_tree($1, $2));",
        folder_id,
        owner_id
    )
    .fetch_one(pg_pool)
    .await
    .map_err(|_| InternalError("Failed to get the storage of the folder".to_string()))?
    .size;
    Ok(size.unwrap_or(0))
}

async fn folder_size_filter(
    pg_pool: &PgPool,
    folder_id: i32,
    owner_id: i32,
    filter: &str,
) -> Result<i64, InternalError> {
    // Recursive query to get the storage of the folder and all its children
    let size = sqlx::query!(
        "SELECT SUM(size) as size
        FROM files
        WHERE fk_parent IN (SELECT folder_id FROM get_folder_tree($1, $2))
        AND file_type = $3;",
        folder_id,
        owner_id,
        filter
    )
    .fetch_one(pg_pool)
    .await
    .map_err(|_| InternalError("Failed to get the storage of the folder".to_string()))?
    .size;
    Ok(size.unwrap_or(0))
}

pub async fn move_folder(
    pg_pool: &PgPool,
    folder_id: i32,
    to_folder_id: i32,
    owner_id: i32,
) -> Result<(), InternalError> {
    sqlx::query!(
        "UPDATE folders
        SET fk_parent = $3
        WHERE id = $1 AND fk_owner = $2 AND fk_owner = (SELECT fk_owner FROM folders WHERE id = $3)
        AND $3 NOT IN (SELECT folder_id FROM get_folder_tree($1, $2));",
        folder_id,
        owner_id,
        to_folder_id
    )
    .execute(pg_pool)
    .await
    .map_err(|_| InternalError("Failed to move the folder".to_string()))?;
    Ok(())
}

pub async fn delete_folder(
    pg_pool: &PgPool,
    folder_id: i32,
    owner_id: i32,
    preserve_parent: bool,
) -> Result<(), InternalError> {
    // Delete the files from the database
    let files_ids = sqlx::query!(
        "DELETE FROM files
        WHERE fk_parent IN (SELECT folder_id FROM get_folder_tree($1, $2))
        RETURNING id;",
        folder_id,
        owner_id
    )
    .fetch_all(pg_pool)
    .await
    .map_err(|_| InternalError(format!("Failed to delete files in folder {}", folder_id)))?;
    // Delete the files from the storage
    for file_id in files_ids {
        files_model::delete_file_content(file_id.id).await?;
    }
    // Delete the folders from the database
    // TODO: Check that the folder is not the root folder
    let preserved_folder_id = if preserve_parent { folder_id } else { -1 };
    sqlx::query!(
        "DELETE FROM folders
        WHERE id IN (SELECT folder_id FROM get_folder_tree($1, $2)) AND id <> $3;",
        folder_id,
        owner_id,
        preserved_folder_id
    )
    .fetch_all(pg_pool)
    .await
    .map_err(|_| InternalError(format!("Failed to delete folders in folder {}", folder_id)))?;
    Ok(())
}

pub(super) async fn delete_user_folders(
    pg_pool: &PgPool,
    owner_id: i32,
) -> Result<(), InternalError> {
    sqlx::query!(
        "DELETE FROM folders
        WHERE fk_owner = $1;",
        owner_id
    )
    .execute(pg_pool)
    .await
    .map_err(|_| InternalError("Failed to delete user folders".to_string()))?;
    Ok(())
}

async fn new_raw_folder(
    pg_pool: &PgPool,
    folder_name: &str,
    parent_folder_id: Option<i32>,
    owner_id: i32,
) -> Result<Folder, FileError> {
    if !validate_name(folder_name) {
        return Err(FileError::NameError);
    }
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
    .map_err(|_| FileError::InternalError)?;
    Ok(folder)
}
