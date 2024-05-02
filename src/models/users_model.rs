use super::{files_model, folders_model, User};
use crate::errors::{InternalError, LoginError, SignupError};
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
    let email = email.to_lowercase();
    if !EmailAddress::is_valid(&email) {
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

pub async fn get_user_by_id(pg_pool: &PgPool, user_id: i32) -> Result<Option<User>, InternalError> {
    sqlx::query_as!(
        User,
        "SELECT *
        FROM users
        WHERE id = $1;",
        user_id
    )
    .fetch_optional(pg_pool)
    .await
    .map_err(|_| InternalError("Error while fetching user".to_string()))
}

pub async fn get_user_by_email(
    pg_pool: &PgPool,
    email: &str,
) -> Result<Option<User>, InternalError> {
    let email = email.to_lowercase();
    sqlx::query_as!(
        User,
        "SELECT *
        FROM users
        WHERE email = $1;",
        email
    )
    .fetch_optional(pg_pool)
    .await
    .map_err(|_| InternalError("Error while fetching user".to_string()))
}

pub async fn verify_user(pg_pool: &PgPool, email: &str, password: &str) -> Result<i32, LoginError> {
    let user = get_user_by_email(pg_pool, email)
        .await
        .map_err(|_| LoginError::InternalError)?
        .ok_or(LoginError::EmailDoesNotExists)?;
    let is_pwd_correct =
        bcrypt::verify(password, &user.password).map_err(|_| LoginError::InternalError)?;
    if is_pwd_correct {
        Ok(user.id)
    } else {
        Err(LoginError::WrongPassword)
    }
}

pub async fn delete_user(pg_pool: &PgPool, user_id: i32) -> Result<(), InternalError> {
    files_model::delete_user_files(pg_pool, user_id).await?;
    folders_model::delete_user_folders(pg_pool, user_id).await?;
    sqlx::query!(
        "DELETE FROM users
        WHERE id = $1;",
        user_id,
    )
    .execute(pg_pool)
    .await
    .map_err(|_| InternalError("Error while deleting user".to_string()))?;
    Ok(())
}

fn validate_username(username: &str) -> bool {
    username.len() >= 4
        && username.len() <= 20
        && username
            .chars()
            .all(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_'))
}

fn validate_password(password: &str) -> bool {
    password.len() >= 8
}
