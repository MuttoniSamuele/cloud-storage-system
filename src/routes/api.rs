use crate::{errors::SignupError, models::UsersModel};
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SignupJsonData {
    username: String,
    email: String,
    password: String,
}

pub fn api(pool: PgPool) -> Router {
    Router::new()
        .route("/signup", post(post_signup))
        .with_state(pool)
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
        Ok(_) => StatusCode::CREATED,
        Err(err) => match err {
            SignupError::UsernameExists | SignupError::EmailExists => StatusCode::CONFLICT,
            SignupError::InvalidUsername | SignupError::InvalidEmail => StatusCode::BAD_REQUEST,
            SignupError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        },
    }
}
