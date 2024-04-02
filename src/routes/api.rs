use crate::{
    errors::SignupError,
    models::{RedisPool, SessionsModel, UsersModel},
};
use axum::{
    body::Body,
    extract::State,
    http::{self, StatusCode},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;

const SESSION_COOKIE_NAME: &str = "session_id";
const SESSION_COOKIE_AGE: u32 = 9999999;

#[derive(Clone)]
struct AppState {
    pg_pool: PgPool,
    redis_pool: RedisPool,
    rng: Arc<Mutex<ChaCha8Rng>>,
}

/// Input data required for the signup.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SignupJsonData {
    username: String,
    email: String,
    password: String,
}

/// Data returned when something goes wrong.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ErrorResponse {
    message: String,
}

pub fn api(pg_pool: PgPool, redis_pool: RedisPool, rng: ChaCha8Rng) -> Router {
    let state = AppState {
        pg_pool,
        redis_pool,
        rng: Arc::new(Mutex::new(rng)),
    };
    Router::new()
        .route("/signup", post(post_signup))
        .with_state(state)
}

// TODO: Make this less repetitive
async fn post_signup(
    State(state): State<AppState>,
    Json(user): Json<SignupJsonData>,
) -> impl IntoResponse {
    let users_model = UsersModel::new(&state.pg_pool);
    // Try to create the user
    let res = users_model
        .signup(&user.username, &user.email, &user.password)
        .await;
    match res {
        Ok(user_id) => {
            // Try to create a session for the new user
            let mut rng = state.rng.lock().await;
            let mut sessions_model = SessionsModel::new(state.redis_pool.clone(), &mut rng);
            let res = sessions_model.new_session(user_id).await;
            if let Ok(session_id) = res {
                // Everything went well
                login_response(session_id).into_response()
            } else {
                // If the session can't be created, the user will have to manually login
                StatusCode::CREATED.into_response()
            }
        }
        Err(err) => match err {
            SignupError::UsernameExists => (
                StatusCode::CONFLICT,
                Json(ErrorResponse {
                    message: "Username already taken.".to_string(),
                }),
            )
                .into_response(),
            SignupError::EmailExists => (
                StatusCode::CONFLICT,
                Json(ErrorResponse {
                    message: "An account with this email already exists.".to_string(),
                }),
            )
                .into_response(),
            SignupError::InvalidUsername => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    message: "The username must be from 8 to 20 characters long and
                    should only contain letters, numbers, '-' and '_'."
                        .to_string(),
                }),
            )
                .into_response(),
            SignupError::InvalidEmail => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    message: "Invalid email.".to_string(),
                }),
            )
                .into_response(),
            SignupError::ShortPassword => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    message: "The password must be at least 8 characters long.".to_string(),
                }),
            )
                .into_response(),
            SignupError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: "Something went wrong, please try again later.".to_string(),
                }),
            )
                .into_response(),
        },
    }
}

/// Wrapper that returns a new session.
fn login_response(session_id: u128) -> impl IntoResponse {
    session_cookie_response(&session_id.to_string(), SESSION_COOKIE_AGE)
}

/// Wrapper that removes the saved session.
fn logout_response() -> impl IntoResponse {
    session_cookie_response("_", 0)
}

/// Returns a response while setting the session_id cookie with the given value and age.
fn session_cookie_response(value: &str, age: u32) -> impl IntoResponse {
    http::Response::builder()
        .status(StatusCode::CREATED)
        .header(
            "Set-Cookie",
            format!(
                "{}={}; Max-Age={}; SameSite=Strict",
                SESSION_COOKIE_NAME, value, age
            ),
        )
        .body(Body::empty())
        .unwrap()
}
