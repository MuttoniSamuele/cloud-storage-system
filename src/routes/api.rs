mod auth;
mod cloud;

use crate::{models::RedisPool, MAX_UPLOAD_MB};
use auth::{auth_middleware, login, logout, me, signup};
use axum::{
    extract::DefaultBodyLimit,
    http::StatusCode,
    routing::{delete, get, patch, post},
    Json, Router,
};
use cloud::{
    file_delete, file_download, file_move, file_rename, folder_delete, folder_move, folder_new,
    folder_rename, folder_size, upload, view,
};
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub pg_pool: PgPool,
    pub redis_pool: RedisPool,
    pub rng: Arc<Mutex<ChaCha8Rng>>,
}

/// Data returned when something goes wrong.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub message: String,
}

impl ErrorResponse {
    pub fn json(message: &str) -> Json<Self> {
        Json(ErrorResponse {
            message: message.to_string(),
        })
    }

    pub fn response(code: StatusCode, message: &str) -> (StatusCode, Json<Self>) {
        (code, Self::json(message))
    }

    pub fn internal_err() -> (StatusCode, Json<Self>) {
        Self::response(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong.")
    }
}

// TODO: Consider this idea
// pub type ApiResponse<T> = Result<(StatusCode, T), (StatusCode, Json<ErrorResponse>)>;

pub fn api(pg_pool: PgPool, redis_pool: RedisPool, rng: ChaCha8Rng) -> Router {
    let state = AppState {
        pg_pool: pg_pool.clone(),
        redis_pool: redis_pool.clone(),
        rng: Arc::new(Mutex::new(rng)),
    };
    // Routes protected by the auth middleware (require authentication)
    let protected_routes = Router::new()
        .route("/logout", post(logout))
        .route("/me", get(me))
        .route("/upload", post(upload))
        .route("/view", get(view))
        .route("/folder/new", post(folder_new))
        .route("/folder/rename", patch(folder_rename))
        .route("/folder/size", get(folder_size))
        .route("/folder/move", patch(folder_move))
        .route("/folder/delete", delete(folder_delete))
        .route("/file/download", get(file_download))
        .route("/file/rename", patch(file_rename))
        .route("/file/move", patch(file_move))
        .route("/file/delete", delete(file_delete))
        .layer(axum::middleware::from_fn(move |req, next| {
            auth_middleware(req, next, redis_pool.clone())
        }))
        .layer(DefaultBodyLimit::max(*MAX_UPLOAD_MB * 1_000_000));
    // Combine the rest of the routes with the protected ones
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .nest("/", protected_routes)
        .with_state(state)
}
