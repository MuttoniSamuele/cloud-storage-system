use super::{AppState, ErrorResponse};
use crate::{
    errors::{LoginError, SignupError},
    models::{sessions_model, users_model, RedisPool},
};
use axum::{
    body::Body,
    extract::State,
    http::{self, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use serde::{Deserialize, Serialize};

const SESSION_COOKIE_NAME: &str = "session_id";
const SESSION_COOKIE_AGE: u32 = 9999999;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignupJsonData {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginJsonData {
    email: String,
    password: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MeResponse {
    username: String,
    email: String,
}

pub type AuthState = (u128, i32);

pub async fn auth_middleware<B>(
    req: http::Request<B>,
    next: axum::middleware::Next<B>,
    redis_pool: RedisPool,
) -> Result<axum::response::Response, StatusCode> {
    // Weird way to get the session id from the cookie
    // If it isn't present, returns UNAUTHORIZED
    let session_id = req
        .headers()
        .get_all("Cookie")
        .iter()
        .filter_map(|cookie| {
            cookie
                .to_str()
                .ok()
                .and_then(|cookie| cookie.parse::<cookie::Cookie>().ok())
        })
        .find_map(|cookie| {
            (cookie.name() == SESSION_COOKIE_NAME).then(move || cookie.value().to_owned())
        })
        .and_then(|cookie_value| cookie_value.parse::<u128>().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    // Given the session id, get the user id from Redis
    // If no session can be found, returns UNAUTHORIZED
    let user_id = sessions_model::get_session_user_id(&redis_pool, session_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;
    // Insert the auth data in the request state
    let (mut parts, body) = req.into_parts();
    let auth_state: AuthState = (session_id, user_id);
    parts.extensions.insert(auth_state);
    Ok(next.run(axum::http::Request::from_parts(parts, body)).await)
}

// TODO: Make this less repetitive
pub async fn signup(
    State(state): State<AppState>,
    Json(user): Json<SignupJsonData>,
) -> impl IntoResponse {
    // Try to create the user
    let res =
        users_model::new_user(&state.pg_pool, &user.username, &user.email, &user.password).await;
    match res {
        Ok(user_id) => {
            // Try to create a session for the new user
            let mut rng = state.rng.lock().await;
            let res = sessions_model::new_session(&state.redis_pool, &mut rng, user_id).await;
            if let Ok(session_id) = res {
                signup_response(session_id).into_response()
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

pub async fn login(
    State(state): State<AppState>,
    Json(user): Json<LoginJsonData>,
) -> impl IntoResponse {
    let res = users_model::verify_user(&state.pg_pool, &user.email, &user.password).await;
    match res {
        Ok(user_id) => {
            // Try to create a new session
            let mut rng = state.rng.lock().await;
            let res = sessions_model::new_session(&state.redis_pool, &mut rng, user_id).await;
            if let Ok(session_id) = res {
                login_response(session_id).into_response()
            } else {
                // The session can't be created
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
        Err(err) => match err {
            LoginError::EmailDoesNotExists => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    message: "Email does not exist.".to_string(),
                }),
            )
                .into_response(),
            LoginError::WrongPassword => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    message: "Wrong password.".to_string(),
                }),
            )
                .into_response(),
            LoginError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: "Something went wrong, please try again later.".to_string(),
                }),
            )
                .into_response(),
        },
    }
}

pub async fn logout(
    Extension((session_id, _)): Extension<AuthState>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let res = sessions_model::delete_session(&state.redis_pool, session_id).await;
    if res.is_ok() {
        logout_response().into_response()
    } else {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

pub async fn me(
    Extension((_, user_id)): Extension<AuthState>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let res = users_model::get_user_by_id(&state.pg_pool, user_id).await;
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

fn signup_response(session_id: u128) -> impl IntoResponse {
    session_cookie_response(
        StatusCode::CREATED,
        &session_id.to_string(),
        SESSION_COOKIE_AGE,
    )
}

fn login_response(session_id: u128) -> impl IntoResponse {
    session_cookie_response(
        StatusCode::NO_CONTENT,
        &session_id.to_string(),
        SESSION_COOKIE_AGE,
    )
}

fn logout_response() -> impl IntoResponse {
    session_cookie_response(StatusCode::NO_CONTENT, "_", 0)
}

/// Returns a response while setting the session_id cookie with the given value and age.
fn session_cookie_response(status: StatusCode, value: &str, age: u32) -> impl IntoResponse {
    http::Response::builder()
        .status(status)
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