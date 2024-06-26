use super::file::File;
use super::FILES_FOLDER;
use crate::errors::{FileError, InternalError};
use axum::body::Bytes;
use sqlx::PgPool;
use std::path::{Path, PathBuf};
use tokio::{fs, io::AsyncWriteExt};

pub async fn new_file(
    pg_pool: &PgPool,
    file_name: &str,
    content: &Bytes,
    parent_folder_id: i32,
    owner_id: i32,
) -> Result<File, FileError> {
    if !validate_name(file_name) {
        return Err(FileError::NameError);
    }
    let file_type = get_file_type(file_name);
    let file_size = content.len() as i32;
    let file = sqlx::query_as!(
        File,
        "INSERT INTO files (name, file_type, size, last_modified, starred, fk_owner, fk_parent)
        VALUES ($1, $2, $3, CURRENT_TIMESTAMP, $4, $5, $6)
        RETURNING *;",
        file_name,
        file_type,
        file_size,
        false,
        owner_id,
        parent_folder_id
    )
    .fetch_one(pg_pool)
    .await
    .map_err(|_| FileError::InternalError)?;
    save_file_content(file.id, content)
        .await
        .map_err(|_| FileError::InternalError)?;
    Ok(file)
}

pub async fn get_files(
    pg_pool: &PgPool,
    parent_folder_id: i32,
    owner_id: i32,
) -> Result<Vec<File>, InternalError> {
    sqlx::query_as!(
        File,
        "SELECT *
        FROM files
        WHERE fk_owner = $1 AND fk_parent = $2;",
        owner_id,
        parent_folder_id
    )
    .fetch_all(pg_pool)
    .await
    .map_err(|_| InternalError("Failed get the files from the database".to_string()))
}

pub async fn rename_file(
    pg_pool: &PgPool,
    file_id: i32,
    owner_id: i32,
    new_name: &str,
) -> Result<(), FileError> {
    if !validate_name(new_name) {
        return Err(FileError::NameError);
    }
    sqlx::query!(
        "UPDATE files
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

pub async fn get_file(
    pg_pool: &PgPool,
    file_id: i32,
    owner_id: i32,
) -> Result<(String, Vec<u8>), InternalError> {
    let name = sqlx::query!(
        "SELECT name
        FROM files
        WHERE id = $1 AND fk_owner = $2;",
        file_id,
        owner_id
    )
    .fetch_one(pg_pool)
    .await
    .map_err(|_| InternalError("Failed to get the file".to_string()))?
    .name;
    read_file_content(file_id)
        .await
        .map(|content| (name, content))
}

pub async fn move_file(
    pg_pool: &PgPool,
    file_id: i32,
    to_folder_id: i32,
    owner_id: i32,
) -> Result<(), InternalError> {
    sqlx::query!(
        "UPDATE files
        SET fk_parent = $3
        WHERE id = $1 AND fk_owner = $2 AND fk_owner = (SELECT fk_owner FROM folders WHERE id = $3);",
        file_id,
        owner_id,
        to_folder_id
    )
    .execute(pg_pool)
    .await
    .map_err(|_| InternalError("Failed to move the file".to_string()))?;
    Ok(())
}

pub async fn delete_file(
    pg_pool: &PgPool,
    file_id: i32,
    owner_id: i32,
) -> Result<(), InternalError> {
    sqlx::query!(
        "DELETE FROM files
        WHERE id = $1 AND fk_owner = $2;",
        file_id,
        owner_id
    )
    .execute(pg_pool)
    .await
    .map_err(|_| InternalError("Failed to delete the file".to_string()))?;
    delete_file_content(file_id).await?;
    Ok(())
}

pub async fn duplicate_file(
    pg_pool: &PgPool,
    file_id: i32,
    owner_id: i32,
) -> Result<File, InternalError> {
    let file = sqlx::query_as!(
        File,
        "INSERT INTO files (name, file_type, size, last_modified, starred, fk_owner, fk_parent)
        SELECT name, file_type, size, CURRENT_TIMESTAMP, starred, fk_owner, fk_parent
        FROM files
        WHERE id = $1 AND fk_owner = $2
        RETURNING *;",
        file_id,
        owner_id
    )
    .fetch_one(pg_pool)
    .await
    .map_err(|_| InternalError("Failed to duplicate the file".to_string()))?;
    let content = read_file_content(file_id).await?;
    save_file_content(file.id, &Bytes::from(content)).await?;
    Ok(file)
}

pub async fn get_file_by_id(
    pg_pool: &PgPool,
    file_id: i32,
    owner_id: i32,
) -> Result<File, InternalError> {
    sqlx::query_as!(
        File,
        "SELECT *
        FROM files
        WHERE id = $1 AND fk_owner = $2;",
        file_id,
        owner_id
    )
    .fetch_one(pg_pool)
    .await
    .map_err(|_| InternalError("Failed to get the file".to_string()))
}

pub(super) async fn delete_user_files(
    pg_pool: &PgPool,
    owner_id: i32,
) -> Result<(), InternalError> {
    let files_ids = sqlx::query!(
        "DELETE FROM files
        WHERE fk_owner = $1
        RETURNING id;",
        owner_id
    )
    .fetch_all(pg_pool)
    .await
    .map_err(|_| InternalError("Failed to delete user files".to_string()))?;
    for file_id in files_ids {
        delete_file_content(file_id.id).await?;
    }
    Ok(())
}

pub(super) async fn save_file_content(file_id: i32, content: &Bytes) -> Result<(), InternalError> {
    let path = build_file_path(file_id);
    let data = content.to_vec();
    let mut file = fs::File::create(path)
        .await
        .map_err(|_| InternalError(format!("Failed to create file '{}'", file_id)))?;
    file.write_all(&data)
        .await
        .map_err(|_| InternalError(format!("Failed to write data to file '{}'", file_id)))?;
    Ok(())
}

pub(super) async fn read_file_content(file_id: i32) -> Result<Vec<u8>, InternalError> {
    let path = build_file_path(file_id);
    fs::read(path)
        .await
        .map_err(|_| InternalError(format!("Failed to read content from '{}'", file_id)))
}

pub(super) async fn delete_file_content(file_id: i32) -> Result<(), InternalError> {
    let path = build_file_path(file_id);
    fs::remove_file(path)
        .await
        .map_err(|_| InternalError(format!("Failed to delete content for file '{}'", file_id)))
}

pub(super) fn validate_name(mut name: &str) -> bool {
    name = name.trim();
    name.len() >= 1 && name.len() <= 255
}

fn build_file_path(file_id: i32) -> PathBuf {
    let mut path = PathBuf::from(FILES_FOLDER);
    path.push(file_id.to_string());
    path
}

fn get_file_type(file_name: &str) -> Option<String> {
    let path = Path::new(file_name);
    let extension = path.extension()?;
    let extension_str = extension.to_str()?.to_lowercase();
    match extension_str.as_str() {
        "txt" | "c" | "cpp" | "h" | "hpp" | "py" | "js" | "html" | "css" | "md" | "json"
        | "xml" | "yaml" | "toml" | "rs" | "svelte" | "jsx" | "nim" | "go" | "java" | "php"
        | "rb" | "lua" | "swift" | "kt" | "scala" | "pl" | "sh" | "bat" | "ps1" | "ts" | "asm"
        | "config" | "yml" | "env" | "sql" | "gitignore" | "tsx" | "cs" => Some("Text".to_string()),
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "svg" | "ico" | "webp" | "tiff" | "tif"
        | "heif" | "heic" => Some("Image".to_string()),
        _ => None,
    }
}
