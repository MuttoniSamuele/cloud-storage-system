use super::User;
use crate::errors::SignupError;
use bcrypt;
use email_address::EmailAddress;
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
        if !Self::validate_username(username) {
            return Err(SignupError::InvalidUsername);
        }
        if !EmailAddress::is_valid(email) {
            return Err(SignupError::InvalidEmail);
        }
        if !Self::validate_password(password) {
            return Err(SignupError::ShortPassword);
        }
        let hashed_psw = bcrypt::hash(&password, bcrypt::DEFAULT_COST)
            .map_err(|_| SignupError::InternalError)?;
        let res = sqlx::query_as!(
            User,
            "INSERT INTO users (username, email, password)
            VALUES ($1, $2, $3)
            RETURNING *;",
            username,
            email,
            hashed_psw
        )
        .fetch_one(self.pool)
        .await;
        match res {
            Ok(user) => Ok(user.id),
            Err(e) => match e {
                sqlx::Error::Database(db) => {
                    if db.constraint() == Some("users_username_key") {
                        return Err(SignupError::UsernameExists);
                    }
                    if db.constraint() == Some("users_email_key") {
                        return Err(SignupError::EmailExists);
                    }
                    Err(SignupError::InternalError)
                }
                _ => Err(SignupError::InternalError),
            },
        }
    }

    fn validate_username(username: &str) -> bool {
        username.len() >= 3
            && username.len() <= 20
            && username
                .chars()
                .all(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_'))
    }

    fn validate_password(password: &str) -> bool {
        password.len() >= 8
    }
}
