use super::{files_model, folder::Folder};
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

pub async fn folder_size(
    pg_pool: &PgPool,
    folder_id: i32,
    owner_id: i32,
) -> Result<i64, InternalError> {
    // Recursive query to get the storage of the folder and all its children
    let size = sqlx::query!(
        "WITH RECURSIVE folder_tree AS (
            SELECT id, fk_parent
            FROM folders
            WHERE id = $1 AND fk_owner = $2
            UNION
            SELECT f.id, f.fk_parent
            FROM folders f
            JOIN folder_tree ft ON f.fk_parent = ft.id
        )
        SELECT SUM(size) as size
        FROM files
        WHERE fk_parent IN (SELECT id FROM folder_tree);",
        folder_id,
        owner_id
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
        WHERE id = $1 AND fk_owner = $2 AND fk_owner = (SELECT fk_owner FROM folders WHERE id = $3);",
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
        "WITH RECURSIVE folder_tree AS (
            SELECT id, fk_parent
            FROM folders
            WHERE id = $1 AND fk_owner = $2
            UNION
            SELECT f.id, f.fk_parent
            FROM folders f
            JOIN folder_tree ft ON f.fk_parent = ft.id
        )
        DELETE FROM files
        WHERE fk_parent IN (SELECT id FROM folder_tree)
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
        "WITH RECURSIVE folder_tree AS (
            SELECT id, fk_parent
            FROM folders
            WHERE id = $1 AND fk_owner = $2
            UNION
            SELECT f.id, f.fk_parent
            FROM folders f
            JOIN folder_tree ft ON f.fk_parent = ft.id
        )
        DELETE FROM folders
        WHERE id IN (SELECT id FROM folder_tree) AND id <> $3;",
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
