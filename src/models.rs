mod file;
mod folder;
mod user;

pub mod files_model;
pub mod folders_model;
pub mod sessions_model;
pub mod users_model;

use bb8_redis::{
    bb8::{self, Pool},
    RedisConnectionManager,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::fs;

pub use user::User;
pub type RedisPool = Pool<RedisConnectionManager>;

pub const FILES_FOLDER: &str = "files_data";

pub async fn init_postgres(url: &str, max_connections: u32) -> PgPool {
    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(url)
        .await
        .expect(&format!("Failed to connect to {url}"));
    // Load the database schema
    load_schema(&pool)
        .await
        .expect("Failed to load database schema");
    pool
}

pub async fn init_redis(url: &str) -> RedisPool {
    let manager = RedisConnectionManager::new(url).expect(&format!("Failed to connect to {url}"));
    bb8::Pool::builder()
        .build(manager)
        .await
        .expect("Failed to create Redis pool")
}

pub async fn init_files_folder() {
    fs::create_dir_all(FILES_FOLDER)
        .await
        .expect(&format!("Failed to create '{}' folder", FILES_FOLDER));
}

async fn load_schema(pg_pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query_file!("./schema/users.sql")
        .execute(pg_pool)
        .await?;
    sqlx::query_file!("./schema/folders.sql")
        .execute(pg_pool)
        .await?;
    sqlx::query_file!("./schema/files.sql")
        .execute(pg_pool)
        .await?;
    Ok(())
}
