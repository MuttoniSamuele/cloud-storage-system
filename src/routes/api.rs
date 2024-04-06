use super::auth::{auth, login_response, AuthState};
// TODO: include sessions_model::new_session, users_model::new_user with "namespace"
use crate::{
    errors::SignupError,
    models::{
        sessions_model::new_session,
        users_model::{get_user_by_id, new_user},
        RedisPool,
    },
};
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;

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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MeResponse {
    username: String,
    email: String,
}

/// Data returned when something goes wrong.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ErrorResponse {
    message: String,
}

pub fn api(pg_pool: PgPool, redis_pool: RedisPool, rng: ChaCha8Rng) -> Router {
    let state = AppState {
        pg_pool: pg_pool.clone(),
        redis_pool: redis_pool.clone(),
        rng: Arc::new(Mutex::new(rng)),
    };
    // Routes protected by the auth middleware (require authentication)
    let protected_routes = Router::new()
        .route("/me", get(me))
        .layer(axum::middleware::from_fn(move |req, next| {
            auth(req, next, redis_pool.clone())
        }));
    // Combine the rest of the routes with the protected ones
    Router::new()
        .route("/signup", post(signup))
        .nest("/", protected_routes)
        .with_state(state)
}

// TODO: Make this less repetitive
async fn signup(
    State(state): State<AppState>,
    Json(user): Json<SignupJsonData>,
) -> impl IntoResponse {
    // Try to create the user
    let res = new_user(&state.pg_pool, &user.username, &user.email, &user.password).await;
    match res {
        Ok(user_id) => {
            // Try to create a session for the new user
            let mut rng = state.rng.lock().await;
            let res = new_session(&state.redis_pool, &mut rng, user_id).await;
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

async fn me(
    Extension((_, user_id)): Extension<AuthState>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let res = get_user_by_id(&state.pg_pool, user_id).await;
    match res {
        Ok(user) => {
            if let Some(u) = user {
                (
                    StatusCode::OK,
                    Json(MeResponse {
                        username: u.get_username(),
                        email: u.get_email(),
                    }),
                )
                    .into_response()
            } else {
                // This should never happen
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
