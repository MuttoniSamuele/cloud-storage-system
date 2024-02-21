use axum::Router;
use dotenv::dotenv;
use routes::dummy_api;
use sqlx::{postgres::PgPoolOptions, Executor};
use std::env;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&format!(
            "postgresql://{}:{}@{}:{}/{}",
            env::var("POSTGRES_USER").expect("POSTGRES_USER missing in .env"),
            env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD missing in .env"),
            env::var("POSTGRES_HOST").expect("POSTGRES_HOST missing in .env"),
            env::var("POSTGRES_PORT").expect("POSTGRES_PORT missing in .env"),
            env::var("POSTGRES_DB").expect("POSTGRES_DB missing in .env"),
        ))
        .await
        .expect("Failed to connect to db");
    pool.execute(include_str!("../schema.sql"))
        .await
        .expect("Failed to load schema");
    // Combine the routes
    let app = Router::new()
        .merge(dummy_api())
        .nest_service("/", ServeDir::new("public/dist"));
    // IP address and port of the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on http://{}", addr);
    // Start the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
