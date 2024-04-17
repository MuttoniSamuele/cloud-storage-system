mod auth;
mod cloud;

use crate::{models::RedisPool, MAX_UPLOAD_MB};
use auth::{auth_middleware, login, logout, me, signup};
use axum::{
    extract::DefaultBodyLimit,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use cloud::{new_folder, upload, view};
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
        .route("/new-folder", post(new_folder))
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
