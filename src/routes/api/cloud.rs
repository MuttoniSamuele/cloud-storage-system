use super::{auth::AuthState, AppState, ErrorResponse};
use crate::{models::files_model, MAX_UPLOAD_MB};
use axum::{
    body::Bytes,
    extract::{Multipart, State},
    http::StatusCode,
    Extension, Json,
};

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
    while let Some(field) = multipart.next_field().await.map_err(|_| {
        ErrorResponse::response(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong.")
    })? {
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
        Err(_) => Err(ErrorResponse::response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong.",
        )),
    }
}
