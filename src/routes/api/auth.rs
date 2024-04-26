use super::{AppState, ErrorResponse};
use crate::{
    errors::{LoginError, SignupError},
    models::{folders_model, sessions_model, users_model, RedisPool},
    MAX_STORAGE_MB, MAX_UPLOAD_MB,
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
pub struct SignupData {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginData {
    email: String,
    password: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MeResponse {
    username: String,
    email: String,
    personal_folder_id: i32,
    trash_folder_id: i32,
    max_upload_mb: usize,
    max_storage_mb: i64,
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

pub async fn signup(
    State(state): State<AppState>,
    Json(user): Json<SignupData>,
) -> impl IntoResponse {
    // Try to create the user
    let res =
        users_model::new_user(&state.pg_pool, &user.username, &user.email, &user.password).await;
    match res {
        Ok(user_id) => {
            // Try to create the root folders for the user
            if folders_model::init_root_folders(&state.pg_pool, user_id)
                .await
                .is_err()
            {
                // If the root folders can't be created, it's a really big problem.
                // It should never happen, but if it does... oh well
                return ErrorResponse::response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to initialize root folders. Please, create a new account.",
                )
                .into_response();
            }
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
            SignupError::UsernameExists => {
                ErrorResponse::response(StatusCode::CONFLICT, "Username already taken.")
                    .into_response()
            }
            SignupError::EmailExists => ErrorResponse::response(
                StatusCode::CONFLICT,
                "An account with this email already exists.",
            )
            .into_response(),
            SignupError::InvalidUsername => ErrorResponse::response(
                StatusCode::BAD_REQUEST,
                "The username must be from 8 to 20 characters long and
                should only contain letters, numbers, '-' and '_'.",
            )
            .into_response(),
            SignupError::InvalidEmail => {
                ErrorResponse::response(StatusCode::BAD_REQUEST, "Invalid email.").into_response()
            }
            SignupError::ShortPassword => ErrorResponse::response(
                StatusCode::BAD_REQUEST,
                "The password must be at least 8 characters long.",
            )
            .into_response(),
            SignupError::InternalError => ErrorResponse::response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again later.",
            )
            .into_response(),
        },
    }
}

pub async fn login(
    State(state): State<AppState>,
    Json(user): Json<LoginData>,
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
            LoginError::EmailDoesNotExists => {
                ErrorResponse::response(StatusCode::UNAUTHORIZED, "Email does not exist.")
                    .into_response()
            }
            LoginError::WrongPassword => {
                ErrorResponse::response(StatusCode::UNAUTHORIZED, "Wrong password.").into_response()
            }
            LoginError::InternalError => ErrorResponse::response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again later.",
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
    let Ok(folders) = folders_model::get_root_folders(&state.pg_pool, user_id).await else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };
    match res {
        Ok(user) => {
            if let Some(u) = user {
                (
                    StatusCode::OK,
                    Json(MeResponse {
                        username: u.get_username().clone(),
                        email: u.get_email().clone(),
                        // This works as long as there are only 2 root folders
                        personal_folder_id: folders[0].get_id(),
                        trash_folder_id: folders[1].get_id(),
                        max_upload_mb: *MAX_UPLOAD_MB,
                        max_storage_mb: *MAX_STORAGE_MB,
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
