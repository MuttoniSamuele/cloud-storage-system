use crate::{
    errors::SignupError,
    models::{RedisPool, UsersModel},
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

/// Input data required for the signup.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SignupJsonData {
    username: String,
    email: String,
    password: String,
}

/// Data returned by the signup.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SignupResponse {
    session_id: Option<u128>,
    message: Option<String>,
}

impl SignupResponse {
    fn from_session_id(session_id: u128) -> Self {
        Self {
            session_id: Some(session_id),
            message: None,
        }
    }

    fn from_message(message: &str) -> Self {
        Self {
            session_id: None,
            message: Some(message.to_string()),
        }
    }
}

pub fn api(pg_pool: PgPool, redis_pool: RedisPool) -> Router {
    Router::new()
        .route("/signup", post(post_signup))
        .with_state(pg_pool)
}

async fn post_signup(
    State(pool): State<PgPool>,
    Json(user): Json<SignupJsonData>,
) -> impl IntoResponse {
    let users_model = UsersModel::new(&pool);
    let res = users_model
        .signup(&user.username, &user.email, &user.password)
        .await;
    // TODO: Handle session stuff
    match res {
        Ok(_) => (
            StatusCode::CREATED,
            Json(SignupResponse::from_session_id(0)),
        ),
        Err(err) => match err {
            SignupError::UsernameExists => (
                StatusCode::CONFLICT,
                Json(SignupResponse::from_message("Username already taken.")),
            ),
            SignupError::EmailExists => (
                StatusCode::CONFLICT,
                Json(SignupResponse::from_message(
                    "An account with this email already exists.",
                )),
            ),
            SignupError::InvalidUsername => (
                StatusCode::BAD_REQUEST,
                Json(SignupResponse::from_message(
                    "The username must be from 8 to 20 characters long and
                    should only contain letters, numbers, '-' and '_'.",
                )),
            ),
            SignupError::InvalidEmail => (
                StatusCode::BAD_REQUEST,
                Json(SignupResponse::from_message("Invalid email.")),
            ),
            SignupError::ShortPassword => (
                StatusCode::BAD_REQUEST,
                Json(SignupResponse::from_message(
                    "The password must be at least 8 characters long.",
                )),
            ),
            SignupError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SignupResponse::from_message(
                    "Something went wrong, please try again later.",
                )),
            ),
        },
    }
}
