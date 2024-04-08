mod auth;

use crate::models::RedisPool;
use auth::{auth_middleware, login, logout, me, signup};
use axum::{
    routing::{get, post},
    Router,
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
        .layer(axum::middleware::from_fn(move |req, next| {
            auth_middleware(req, next, redis_pool.clone())
        }));
    // Combine the rest of the routes with the protected ones
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .nest("/", protected_routes)
        .with_state(state)
}
