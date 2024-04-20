use super::file::File;
use super::FILES_FOLDER;
use crate::errors::InternalError;
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
) -> Result<File, InternalError> {
    // TODO: Check file name
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
    .map_err(|_| InternalError("Failed to add file to database".to_string()))?;
    save_file_content(file.id, content).await?;
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
) -> Result<(), InternalError> {
    // TODO: Validate name
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
    .map_err(|_| InternalError("Failed to rename the file".to_string()))?;
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

async fn save_file_content(file_id: i32, content: &Bytes) -> Result<(), InternalError> {
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

async fn read_file_content(file_id: i32) -> Result<Vec<u8>, InternalError> {
    let path = build_file_path(file_id);
    fs::read(path)
        .await
        .map_err(|_| InternalError(format!("Failed to read content from '{}'", file_id)))
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
        | "xml" | "yaml" | "toml" | "rs" | "svelte" | "jsx" | "nim" | "go" => {
            Some("Text".to_string())
        }
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "svg" | "ico" | "webp" | "tiff" | "tif"
        | "heif" | "heic" => Some("Image".to_string()),
        _ => None,
    }
}
