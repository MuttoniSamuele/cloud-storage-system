use super::User;
use crate::errors::SignupError;
use sqlx::PgPool;

pub struct UsersModel<'p> {
    pool: &'p PgPool,
}

impl<'p> UsersModel<'p> {
    pub fn new(pool: &'p PgPool) -> Self {
        UsersModel { pool }
    }

    pub async fn signup(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<i32, SignupError> {
        let res = sqlx::query_as!(
            User,
            "INSERT INTO users (username, email, password)
            VALUES ($1, $2, $3)
            RETURNING *;",
            username,
            email,
            password
        )
        .fetch_one(self.pool)
        .await;
        // TODO: Handle session stuff
        match res {
            Ok(user) => Ok(user.id),
            Err(sqlx::Error::Database(db)) if db.constraint() == Some("users_username_key") => {
                Err(SignupError::UsernameExists)
            }
            Err(_) => Err(SignupError::InternalError),
        }
    }
}
