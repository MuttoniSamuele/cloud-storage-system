use crate::models::UsersModel;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserQuery {
    email: String,
}

pub fn api(state: PgPool) -> Router {
    Router::new()
        .route("/user", get(get_user))
        .with_state(state)
}

async fn get_user(
    Query(user): Query<UserQuery>,
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    // let pool = state.lock().await;
    let users_model = UsersModel::new(&pool);
    let user = users_model.get_by_email(user.email).await.unwrap();
    (StatusCode::OK, user.get_username())
}
