use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::{
    models::{CreateUser, LoginUser},
    AppError, AppState, User,
};

pub(crate) async fn login_handler(
    State(state): State<AppState>,
    Json(input): Json<LoginUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = User::verify(&input.email, &input.password, &state.pool).await?;
    match user {
        Some(user) => {
            let token = state.ek.sign(user)?;
            Ok((StatusCode::OK, Json(json!({ "token": token }))).into_response())
        }
        None => Err(AppError::Unauthorized),
    }
}

pub(crate) async fn register_handler(
    State(state): State<AppState>,
    Json(input): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = User::create(&input, &state.pool).await?;
    let token = state.ek.sign(user)?;
    Ok((StatusCode::CREATED, token))
}

pub(crate) async fn logout_handler() -> impl IntoResponse {
    "logout"
}
