mod models;
mod routes;

use dotenv::dotenv; // TODO: use dotenvy instead
use models::init_db;
use routes::create_routes;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();
    // Initialize the model
    let pool = init_db(
        &env::var("DATABASE_URL").expect("DATABASE_URL missing in .env"),
        env::var("DB_MAX_CONNECTIONS")
            .expect("DB_MAX_CONNECTIONS missing in .env")
            .parse()
            .expect("DB_MAX_CONNECTIONS must be a u32"),
    )
    .await;
    // Initalize the controller
    let app = create_routes(pool);
    // IP address and port of the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on http://{}", addr);
    // Start the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
