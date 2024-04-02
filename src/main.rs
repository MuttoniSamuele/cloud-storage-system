mod errors;
mod models;
mod routes;

use lazy_static::lazy_static;
use models::{init_postgres, init_redis};
use rand_chacha::ChaCha8Rng;
use rand_core::{OsRng, RngCore, SeedableRng};
use routes::create_routes;
use std::env;
use std::net::SocketAddr;

lazy_static! {
    // Load environment variables from .env file
    pub static ref POSTGRES_URL: String =
        env::var("POSTGRES_URL").expect("POSTGRES_URL missing in .env");
    pub static ref PG_MAX_CONNECTIONS: u32 = env::var("PG_MAX_CONNECTIONS")
        .expect("PG_MAX_CONNECTIONS missing in .env")
        .parse()
        .expect("PG_MAX_CONNECTIONS must be a u32");
    pub static ref REDIS_URL: String = env::var("REDIS_URL").expect("REDIS_URL missing in .env");
    pub static ref SESSION_TTL: u64 = env::var("SESSION_TTL")
        .expect("SESSION_TTL missing in .env")
        .parse()
        .expect("SESSION_TTL must be a u64");
}

#[tokio::main]
async fn main() {
    // Load .env file
    dotenvy::dotenv().expect("Failed to load .env");
    // Initialize postgres
    let pg_pool = init_postgres(&POSTGRES_URL, *PG_MAX_CONNECTIONS).await;
    // Initialize redis
    let redis_pool = init_redis(&REDIS_URL).await;
    // Initialize ChaCha algorithm
    let rng = ChaCha8Rng::seed_from_u64(OsRng.next_u64());
    // Initalize the controller
    let app = create_routes(pg_pool, redis_pool, rng);
    // IP address and port of the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    // Start the server
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
