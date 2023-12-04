use std::vec;

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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
enum FileType {
    Unknown,
    Text,
    Image,
    Empty,
    NotEmpty,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct FilesParams {
    path: String,
    folders_only: Option<bool>,
    filter: Option<FileType>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct File {
    name: String,
    is_folder: bool,
    file_type: FileType,
    owner: String,
    last_modified: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Files {
    files: Vec<File>,
}

pub fn dummy_api() -> Router {
    Router::new()
        .route("/dummy/files", get(files))
        .route("/dummy/login", post(login))
}

async fn login(Json(data): Json<LoginData>) -> impl IntoResponse {
    if data.username == "User" && data.password == "password" {
        StatusCode::OK
    } else {
        StatusCode::UNAUTHORIZED
    }
}

async fn files(Query(params): Query<FilesParams>) -> impl IntoResponse {
    println!("Filter: {:?}", params.filter);
    println!("FoldersOnly: {:?}", params.folders_only);
    let all_files = vec![
        File {
            name: "Homework".to_string(),
            is_folder: true,
            file_type: FileType::NotEmpty,
            owner: "User".to_string(),
            last_modified: "2003/1/2".to_string(),
        },
        File {
            name: "games".to_string(),
            is_folder: true,
            file_type: FileType::NotEmpty,
            owner: "Another user".to_string(),
            last_modified: "2003/1/2".to_string(),
        },
        File {
            name: "app.exe".to_string(),
            is_folder: false,
            file_type: FileType::Unknown,
            owner: "User".to_string(),
            last_modified: "2003/5/2".to_string(),
        },
        File {
            name: "my essay.txt".to_string(),
            is_folder: false,
            file_type: FileType::Text,
            owner: "User".to_string(),
            last_modified: "2003/1/8".to_string(),
        },
        File {
            name: "Book.pdf".to_string(),
            is_folder: false,
            file_type: FileType::Unknown,
            owner: "Another user".to_string(),
            last_modified: "2005/3/3".to_string(),
        },
        File {
            name: "New folder".to_string(),
            is_folder: true,
            file_type: FileType::Empty,
            owner: "User".to_string(),
            last_modified: "12005/2/1".to_string(),
        },
        File {
            name: "mountains.png".to_string(),
            is_folder: false,
            file_type: FileType::Image,
            owner: "Another user".to_string(),
            last_modified: "2005/5/5".to_string(),
        },
        File {
            name: "cat.jpg".to_string(),
            is_folder: false,
            file_type: FileType::Image,
            owner: "User".to_string(),
            last_modified: "2005/9/8".to_string(),
        },
        File {
            name: "todo.txt".to_string(),
            is_folder: false,
            file_type: FileType::Text,
            owner: "User".to_string(),
            last_modified: "2005/9/8".to_string(),
        },
        File {
            name: "3d print.stl".to_string(),
            is_folder: false,
            file_type: FileType::Unknown,
            owner: "User".to_string(),
            last_modified: "2004/12/12".to_string(),
        },
    ];
    let files: Vec<File> = all_files
        .into_iter()
        .filter(|f| {
            let folders_only = params.folders_only.unwrap_or(false);
            let folders_condition = !folders_only || f.is_folder;
            let filter_condition = params
                .filter
                .map_or(true, |file_type| f.file_type == file_type);
            folders_condition && filter_condition
        })
        .collect();
    (StatusCode::OK, Json(Files { files }))
}
