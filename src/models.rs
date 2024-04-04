mod user;

pub mod sessions_model;
pub mod users_model;

use bb8_redis::{
    bb8::{self, Pool},
    RedisConnectionManager,
};
use sqlx::{postgres::PgPoolOptions, PgPool};

pub use user::User;
pub type RedisPool = Pool<RedisConnectionManager>;

pub async fn init_postgres(url: &str, max_connections: u32) -> PgPool {
    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(url)
        .await
        .expect(&format!("Failed to connect to {url}"));
    // Load the database schema
    sqlx::query_file!("./schema.sql")
        .execute(&pool)
        .await
        .expect("Failed to load schema.sql");
    pool
}

pub async fn init_redis(url: &str) -> RedisPool {
    let manager = RedisConnectionManager::new(url).expect(&format!("Failed to connect to {url}"));
    bb8::Pool::builder()
        .build(manager)
        .await
        .expect("Failed to create Redis pool")
}
