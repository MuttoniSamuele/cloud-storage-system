mod errors;
mod models;
mod routes;

use models::{init_postgres, init_redis};
use routes::create_routes;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenvy::dotenv().expect("Failed to load .env");
    // Initialize postgres
    let pg_pool = init_postgres(
        &env::var("POSTGRES_URL").expect("POSTGRES_URL missing in .env"),
        env::var("PG_MAX_CONNECTIONS")
            .expect("PG_MAX_CONNECTIONS missing in .env")
            .parse()
            .expect("PG_MAX_CONNECTIONS must be a u32"),
    )
    .await;
    // Initialize redis
    let redis_pool = init_redis(&env::var("REDIS_URL").expect("REDIS_URL missing in .env")).await;
    // Initalize the controller
    let app = create_routes(pg_pool, redis_pool);
    // IP address and port of the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on http://{}", addr);
    // Start the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
