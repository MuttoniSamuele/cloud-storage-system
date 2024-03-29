use std::{
    time::{SystemTime, UNIX_EPOCH},
    vec,
};

use axum::{
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct LoginData {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
enum FileType {
    Unsupported,
    Text,
    Image,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct FilesParams {
    path: String,
    folders_only: Option<bool>,
    filter: Option<FileType>,
}

// TODO: Implement "new" method for child structs of FileBase
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FileBase {
    name: String,
    owner: String,
    last_modified: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct File {
    #[serde(flatten)]
    base: FileBase,
    file_type: FileType,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Folder {
    #[serde(flatten)]
    base: FileBase,
    is_empty: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Files {
    files: Vec<File>,
    folders: Vec<Folder>,
}

pub fn dummy_api() -> Router {
    Router::new()
        .route("/files", get(files))
        .route("/login", post(login))
}

async fn login(Json(data): Json<LoginData>) -> impl IntoResponse {
    if data.username == "User" && data.password == "password" {
        StatusCode::OK
    } else {
        StatusCode::UNAUTHORIZED
    }
}

async fn files(Query(params): Query<FilesParams>) -> impl IntoResponse {
    let all_files = vec![
        File {
            base: FileBase {
                name: "app.exe".to_string(),
                owner: "User".to_string(),
                last_modified: get_time_now(),
            },
            file_type: FileType::Unsupported,
        },
        File {
            base: FileBase {
                name: "my essay.txt".to_string(),
                owner: "User".to_string(),
                last_modified: get_time_now(),
            },
            file_type: FileType::Text,
        },
        File {
            base: FileBase {
                name: "Book.pdf".to_string(),
                owner: "Another user".to_string(),
                last_modified: get_time_now(),
            },
            file_type: FileType::Unsupported,
        },
        File {
            base: FileBase {
                name: "mountains.png".to_string(),
                owner: "Another user".to_string(),
                last_modified: get_time_now(),
            },
            file_type: FileType::Image,
        },
        File {
            base: FileBase {
                name: "cat.jpg".to_string(),
                owner: "User".to_string(),
                last_modified: get_time_now(),
            },
            file_type: FileType::Image,
        },
        File {
            base: FileBase {
                name: "todo.txt".to_string(),
                owner: "User".to_string(),
                last_modified: get_time_now(),
            },
            file_type: FileType::Text,
        },
        File {
            base: FileBase {
                name: "3d print.stl".to_string(),
                owner: "User".to_string(),
                last_modified: get_time_now(),
            },
            file_type: FileType::Unsupported,
        },
    ];
    let all_folders = vec![
        Folder {
            base: FileBase {
                name: "Homework".to_string(),
                owner: "User".to_string(),
                last_modified: get_time_now(),
            },
            is_empty: false,
        },
        Folder {
            base: FileBase {
                name: "games".to_string(),
                owner: "Another user".to_string(),
                last_modified: get_time_now(),
            },
            is_empty: false,
        },
        Folder {
            base: FileBase {
                name: "New folder".to_string(),
                owner: "User".to_string(),
                last_modified: get_time_now(),
            },
            is_empty: true,
        },
    ];
    let files: Vec<File> = if params.folders_only.unwrap_or(false) {
        Vec::new()
    } else {
        all_files
            .into_iter()
            .filter(|f| {
                params
                    .filter
                    .map_or(true, |file_type| f.file_type == file_type)
            })
            .collect()
    };
    let folders: Vec<Folder> = if params.filter.is_none() {
        all_folders
    } else {
        Vec::new()
    };
    (StatusCode::OK, Json(Files { files, folders }))
}

fn get_time_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
