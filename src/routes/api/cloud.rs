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
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse::json("Something went wrong."),
        )
    })? {
        let field_name = field.name().ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse::json("Internal error."),
        ))?;
        match field_name {
            "file" => {
                file_name = field.file_name().map(|s| s.to_string());
                content = field.bytes().await.ok();
            }
            "parent" => parent_folder = field.text().await.ok().and_then(|n| n.parse().ok()),
            _ => continue,
        };
    }
    let (Some(file_name), Some(content), Some(parent_folder_id)) =
        (file_name, content, parent_folder)
    else {
        return Err((
            StatusCode::BAD_REQUEST,
            ErrorResponse::json("Invalid form data."),
        ));
    };
    println!("{} {} {}", file_name, content.len(), parent_folder_id);
    if content.len() > (*MAX_UPLOAD_MB * 1_000_000) {
        return Err((
            StatusCode::PAYLOAD_TOO_LARGE,
            ErrorResponse::json(&format!(
                "The file can't be larger than {} MB.",
                *MAX_UPLOAD_MB
            )),
        ));
    }
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
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse::json("Something went wrong."),
        )),
    }
}
