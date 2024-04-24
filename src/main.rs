mod errors;
mod models;
mod routes;

use lazy_static::lazy_static;
use models::{init_files_folder, init_postgres, init_redis};
use rand_chacha::ChaCha8Rng;
use rand_core::{OsRng, RngCore, SeedableRng};
use routes::create_routes;
use std::env;
use std::net::SocketAddr;

lazy_static! {
    // Load environment variables from .env file
    pub static ref DATABASE_URL: String = env::var("DATABASE_URL")
        .expect("DATABASE_URL missing in .env");
    pub static ref DB_MAX_CONNECTIONS: u32 = env::var("DB_MAX_CONNECTIONS")
        .expect("DB_MAX_CONNECTIONS missing in .env")
        .parse()
        .expect("DB_MAX_CONNECTIONS must be a u32");
    pub static ref REDIS_URL: String = env::var("REDIS_URL")
        .expect("REDIS_URL missing in .env");
    pub static ref SESSION_TTL: u64 = env::var("SESSION_TTL")
        .expect("SESSION_TTL missing in .env")
        .parse()
        .expect("SESSION_TTL must be a u64");
    pub static ref MAX_UPLOAD_MB: usize = env::var("MAX_UPLOAD_MB")
        .expect("MAX_UPLOAD_MB missing in .env")
        .parse()
        .expect("MAX_UPLOAD_MB must be a usize");
    pub static ref MAX_STORAGE_MB: i64 = env::var("MAX_STORAGE_MB")
        .expect("MAX_STORAGE_MB missing in .env")
        .parse()
        .expect("MAX_STORAGE_MB must be a i64");
}

#[tokio::main]
async fn main() {
    // Load .env file
    dotenvy::dotenv().expect("Failed to load .env");
    // Initialize postgres
    let pg_pool = init_postgres(&DATABASE_URL, *DB_MAX_CONNECTIONS).await;
    // Initialize redis
    let redis_pool = init_redis(&REDIS_URL).await;
    // Intialize the folder with the actual files
    init_files_folder().await;
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
