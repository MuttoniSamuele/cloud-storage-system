use crate::models::{sessions_model::get_session_user_id, RedisPool};
use axum::{
    body::Body,
    http::{self, StatusCode},
    response::IntoResponse,
};

const SESSION_COOKIE_NAME: &str = "session_id";
const SESSION_COOKIE_AGE: u32 = 9999999;

pub type AuthState = (u128, i32);

pub async fn auth<B>(
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
    let user_id = get_session_user_id(&redis_pool, session_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;
    // Insert the auth data in the request state
    let (mut parts, body) = req.into_parts();
    let auth_state: AuthState = (session_id, user_id);
    parts.extensions.insert(auth_state);
    Ok(next.run(axum::http::Request::from_parts(parts, body)).await)
}

pub fn signup_response(session_id: u128) -> impl IntoResponse {
    session_cookie_response(
        StatusCode::CREATED,
        &session_id.to_string(),
        SESSION_COOKIE_AGE,
    )
}

pub fn login_response(session_id: u128) -> impl IntoResponse {
    session_cookie_response(
        StatusCode::NO_CONTENT,
        &session_id.to_string(),
        SESSION_COOKIE_AGE,
    )
}

pub fn logout_response() -> impl IntoResponse {
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
