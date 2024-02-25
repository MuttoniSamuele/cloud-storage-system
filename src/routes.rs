mod api;
mod dummy_api;

use api::api;
use axum::Router;
use dummy_api::dummy_api;
use sqlx::PgPool;
use tower_http::services::ServeDir;

pub fn create_routes(pool: PgPool) -> Router {
    // Combine the routes
    Router::new()
        .nest("/dummy", dummy_api())
        .nest("/api", api(pool))
        .nest_service("/", ServeDir::new("public/dist"))
}
