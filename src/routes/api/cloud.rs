use super::{auth::AuthState, AppState, ErrorResponse};
use crate::{
    models::{files_model, folders_model},
    MAX_UPLOAD_MB,
};
use axum::{
    body::Bytes,
    extract::{Multipart, Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ViewQuery {
    parent_folder_id: i32,
    folders_only: Option<bool>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewResponse {
    files: Vec<File>,
    folders: Vec<Folder>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub id: i32,
    pub name: String,
    pub file_type: Option<String>,
    pub size: i32,
    pub last_modified: String,
    pub starred: bool,
    pub owner_id: i32,
    pub parent_id: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    pub id: i32,
    pub name: String,
    pub last_modified: String,
    pub starred: bool,
    pub owner_id: i32,
    pub parent_id: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct IdQuery {
    id: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewFolderData {
    parent_id: i32,
    name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameData {
    id: i32,
    new_name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderSizeResponse {
    size: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveData {
    id: i32,
    folder_id: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DeleteQuery {
    id: i32,
    preserve_parent: Option<bool>,
}

pub async fn upload(
    Extension((_, user_id)): Extension<AuthState>,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    // Data to be extracted from the multipart
    let mut file_name: Option<String> = None;
    let mut content: Option<Bytes> = None;
    let mut parent_folder: Option<i32> = None;
    // Parse the multipart data
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| ErrorResponse::internal_err())?
    {
        // Get the name of the field
        let field_name = field.name().ok_or(ErrorResponse::response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal error.",
        ))?;
        // For each field get their data (if present, None otherwise)
        match field_name {
            "file" => {
                file_name = field.file_name().map(|s| s.to_string());
                content = field.bytes().await.ok();
            }
            "parent" => parent_folder = field.text().await.ok().and_then(|n| n.parse().ok()),
            _ => continue,
        };
    }
    // If some of the necessary data from the fields is missing return an error
    let (Some(file_name), Some(content), Some(parent_folder_id)) =
        (file_name, content, parent_folder)
    else {
        return Err(ErrorResponse::response(
            StatusCode::BAD_REQUEST,
            "Invalid form data.",
        ));
    };
    // Check the size of the file (Axum should handle this already)
    if content.len() > (*MAX_UPLOAD_MB * 1_000_000) {
        return Err(ErrorResponse::response(
            StatusCode::PAYLOAD_TOO_LARGE,
            &format!("The file can't be larger than {} MB.", *MAX_UPLOAD_MB),
        ));
    }
    // Add the file to the databases
    let res = files_model::new_file(
        &state.pg_pool,
        &file_name,
        &content,
        parent_folder_id,
        user_id,
    )
    .await;
    match res {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Err(ErrorResponse::internal_err()),
    }
}

pub async fn view(
    Extension((_, user_id)): Extension<AuthState>,
    State(state): State<AppState>,
    Query(params): Query<ViewQuery>,
) -> Result<(StatusCode, Json<ViewResponse>), (StatusCode, Json<ErrorResponse>)> {
    // TODO: Check max storage for user
    // Fetch the folders from the database first
    let raw_folders = folders_model::get_folders(&state.pg_pool, params.parent_folder_id, user_id)
        .await
        .map_err(|_| ErrorResponse::internal_err())?;
    // Map the folder models to objects that can be sent to the user
    let folders = raw_folders
        .iter()
        .map(|f| Folder {
            id: f.get_id(),
            name: f.get_name().clone(),
            last_modified: f.get_last_modified().to_string(),
            starred: f.get_starred(),
            owner_id: f.get_fk_owner(),
            // All of these folders have a parent because that's how they were selected
            parent_id: f.get_fk_parent().unwrap(),
        })
        .collect();
    // If folders_only is specified and it's true
    if let Some(folders_only) = params.folders_only {
        if folders_only {
            // Return the vector of folders and an empty vector of files
            return Ok((
                StatusCode::OK,
                Json(ViewResponse {
                    files: Vec::new(),
                    folders,
                }),
            ));
        }
    }
    // Otherwise, fetch the files too
    let raw_files = files_model::get_files(&state.pg_pool, params.parent_folder_id, user_id)
        .await
        .map_err(|_| ErrorResponse::internal_err())?;
    // Map the file models to objects that can be sent to the user
    let files = raw_files
        .iter()
        .map(|f| File {
            id: f.get_id(),
            name: f.get_name().clone(),
            file_type: f.get_file_type().clone(),
            size: f.get_size(),
            last_modified: f.get_last_modified().to_string(),
            starred: f.get_starred(),
            owner_id: f.get_fk_owner(),
            parent_id: f.get_fk_parent(),
        })
        .collect();
    // Send the files and folders
    Ok((StatusCode::OK, Json(ViewResponse { files, folders })))
}

pub async fn folder_new(
    Extension((_, user_id)): Extension<AuthState>,
    State(state): State<AppState>,
    Json(data): Json<NewFolderData>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    folders_model::new_folder(&state.pg_pool, &data.name, data.parent_id, user_id)
        .await
        .map_err(|_| ErrorResponse::internal_err())?;
    Ok(StatusCode::CREATED)
}

pub async fn folder_rename(
    Extension((_, user_id)): Extension<AuthState>,
    State(state): State<AppState>,
    Json(data): Json<RenameData>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    folders_model::rename_folder(&state.pg_pool, data.id, user_id, &data.new_name)
        .await
        .map_err(|_| ErrorResponse::internal_err())?;
    Ok(StatusCode::OK)
}

pub async fn file_rename(
    Extension((_, user_id)): Extension<AuthState>,
    State(state): State<AppState>,
    Json(data): Json<RenameData>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    files_model::rename_file(&state.pg_pool, data.id, user_id, &data.new_name)
        .await
        .map_err(|_| ErrorResponse::internal_err())?;
    Ok(StatusCode::OK)
}

pub async fn file_download(
    Extension((_, user_id)): Extension<AuthState>,
    State(state): State<AppState>,
    Query(IdQuery { id: file_id }): Query<IdQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let (name, content) = files_model::get_file(&state.pg_pool, file_id, user_id)
        .await
        .map_err(|_| ErrorResponse::internal_err())?;
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Disposition",
        format!("attachment; filename={}", name).parse().unwrap(),
    );
    Ok((StatusCode::OK, headers, content))
}

pub async fn folder_size(
    Extension((_, user_id)): Extension<AuthState>,
    State(state): State<AppState>,
    Query(IdQuery { id: folder_id }): Query<IdQuery>,
) -> Result<(StatusCode, Json<FolderSizeResponse>), (StatusCode, Json<ErrorResponse>)> {
    let size = folders_model::folder_size(&state.pg_pool, folder_id, user_id)
        .await
        .map_err(|_| ErrorResponse::internal_err())?;
    Ok((StatusCode::OK, Json(FolderSizeResponse { size })))
}

pub async fn folder_move(
    Extension((_, user_id)): Extension<AuthState>,
    State(state): State<AppState>,
    Json(data): Json<MoveData>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    folders_model::move_folder(&state.pg_pool, data.id, data.folder_id, user_id)
        .await
        .map_err(|_| ErrorResponse::internal_err())?;
    Ok(StatusCode::OK)
}

pub async fn file_move(
    Extension((_, user_id)): Extension<AuthState>,
    State(state): State<AppState>,
    Json(data): Json<MoveData>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    files_model::move_file(&state.pg_pool, data.id, data.folder_id, user_id)
        .await
        .map_err(|_| ErrorResponse::internal_err())?;
    Ok(StatusCode::OK)
}

pub async fn file_delete(
    Extension((_, user_id)): Extension<AuthState>,
    State(state): State<AppState>,
    Query(IdQuery { id: file_id }): Query<IdQuery>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    files_model::delete_file(&state.pg_pool, file_id, user_id)
        .await
        .map_err(|_| ErrorResponse::internal_err())?;
    Ok(StatusCode::OK)
}

pub async fn folder_delete(
    Extension((_, user_id)): Extension<AuthState>,
    State(state): State<AppState>,
    Query(query): Query<DeleteQuery>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    folders_model::delete_folder(
        &state.pg_pool,
        query.id,
        user_id,
        query.preserve_parent.unwrap_or(false),
    )
    .await
    .map_err(|_| ErrorResponse::internal_err())?;
    Ok(StatusCode::OK)
}
