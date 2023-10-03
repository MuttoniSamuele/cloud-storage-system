use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct FoldersParams {
    path: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Folder {
    name: String,
    empty: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Folders {
    folders: Vec<Folder>,
}

pub fn dummy_api() -> Router {
    Router::new().route("/dummy/folders", get(folders))
}

async fn folders(Query(params): Query<FoldersParams>) -> impl IntoResponse {
    let path = if let Some(path) = params.path {
        path
    } else {
        "/MyCloud".to_string()
    };
    match path.as_str() {
        "/MyCloud" => (
            StatusCode::OK,
            Json(Folders {
                folders: vec![
                    Folder {
                        name: "Sub folder".to_string(),
                        empty: false,
                    },
                    Folder {
                        name: "Test".to_string(),
                        empty: false,
                    },
                    Folder {
                        name: "Empty".to_string(),
                        empty: true,
                    },
                ],
            }),
        )
            .into_response(),
        "/MyCloud/Sub folder" => (
            StatusCode::OK,
            Json(Folders {
                folders: vec![
                    Folder {
                        name: "Hi".to_string(),
                        empty: true,
                    },
                    Folder {
                        name: "How are you".to_string(),
                        empty: false,
                    },
                    Folder {
                        name: "It's me".to_string(),
                        empty: true,
                    },
                ],
            }),
        )
            .into_response(),
        "/MyCloud/Test" => (
            StatusCode::OK,
            Json(Folders {
                folders: vec![Folder {
                    name: "no".to_string(),
                    empty: true,
                }],
            }),
        )
            .into_response(),
        "/MyCloud/Sub folder/How are you" => (
            StatusCode::OK,
            Json(Folders {
                folders: vec![
                    Folder {
                        name: "fine".to_string(),
                        empty: true,
                    },
                    Folder {
                        name: "great".to_string(),
                        empty: true,
                    },
                ],
            }),
        )
            .into_response(),
        _ => StatusCode::NOT_FOUND.into_response(),
    }
}
