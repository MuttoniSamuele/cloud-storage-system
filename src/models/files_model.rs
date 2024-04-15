use super::file::File;
use super::FILES_FOLDER;
use crate::errors::InternalError;
use axum::body::Bytes;
use sqlx::PgPool;
use std::path::Path;
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

async fn save_file_content(file_id: i32, content: &Bytes) -> Result<(), InternalError> {
    let raw_path = format!("{}/{}", FILES_FOLDER, file_id);
    let path = Path::new(&raw_path);
    let data = content.to_vec();
    let mut file = fs::File::create(path)
        .await
        .map_err(|_| InternalError(format!("Failed to create file '{}'", file_id)))?;
    file.write_all(&data)
        .await
        .map_err(|_| InternalError(format!("Failed to write data to file '{}'", file_id)))?;
    Ok(())
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
