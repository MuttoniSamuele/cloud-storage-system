mod user;
mod users_model;

use sqlx::{postgres::PgPoolOptions, PgPool};
pub use user::User;
pub use users_model::UsersModel;

pub async fn init_db(db_url: &str, max_connections: u32) -> PgPool {
    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(db_url)
        .await
        .expect("Failed to connect to db");
    // Load the database schema
    sqlx::query_file!("./schema.sql")
        .execute(&pool)
        .await
        .expect("Failed to load schema");
    pool
}
