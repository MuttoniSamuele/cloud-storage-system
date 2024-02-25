use super::User;
use sqlx::PgPool;

pub struct UsersModel<'p> {
    pool: &'p PgPool,
}

impl<'p> UsersModel<'p> {
    pub fn new(pool: &'p PgPool) -> Self {
        UsersModel { pool }
    }

    pub async fn get_by_email(&self, email: String) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, username, email, password FROM users WHERE email = $1",
            email
        )
        .fetch_one(self.pool)
        .await?;
        Ok(user)
    }
}
