mod api;

use crate::models::RedisPool;
use api::api;
use axum::Router;
use rand_chacha::ChaCha8Rng;
use sqlx::PgPool;
use tower_http::services::ServeDir;

pub fn create_routes(pg_pool: PgPool, redis_pool: RedisPool, rng: ChaCha8Rng) -> Router {
    // Combine the routes
    Router::new()
        .nest("/api", api(pg_pool, redis_pool, rng))
        .nest_service("/", ServeDir::new("public/dist"))
}
