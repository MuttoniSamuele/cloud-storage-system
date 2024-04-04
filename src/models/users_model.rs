use super::User;
use crate::errors::{InternalError, SignupError};
use bcrypt;
use email_address::EmailAddress;
use sqlx::PgPool;

pub async fn new_user(
    pg_pool: &PgPool,
    username: &str,
    email: &str,
    password: &str,
) -> Result<i32, SignupError> {
    if !validate_username(username) {
        return Err(SignupError::InvalidUsername);
    }
    if !EmailAddress::is_valid(email) {
        return Err(SignupError::InvalidEmail);
    }
    if !validate_password(password) {
        return Err(SignupError::ShortPassword);
    }
    let hashed_psw =
        bcrypt::hash(&password, bcrypt::DEFAULT_COST).map_err(|_| SignupError::InternalError)?;
    let res = sqlx::query_as!(
        User,
        "INSERT INTO users (username, email, password)
        VALUES ($1, $2, $3)
        RETURNING *;",
        username,
        email,
        hashed_psw
    )
    .fetch_one(pg_pool)
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

pub async fn delete_user(pg_pool: &PgPool, user_id: i32) -> Result<(), InternalError> {
    let res: Result<_, sqlx::Error> = sqlx::query!(
        "DELETE FROM users
        WHERE id = $1;",
        user_id,
    )
    .fetch_one(pg_pool)
    .await;
    if res.is_err() {
        Err(InternalError("Failed to delete user".to_string()))
    } else {
        Ok(())
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
