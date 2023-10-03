use axum::Router;
use routes::dummy_api;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

mod routes;

#[tokio::main]
async fn main() {
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
