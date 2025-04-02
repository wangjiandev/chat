use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("sql error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("argon2 error: {0}")]
    PasswordHasherError(#[from] argon2::password_hash::Error),

    #[error("jwt error: {0}")]
    JwtError(#[from] jwt_simple::Error),

    #[error("unauthorized")]
    Unauthorized,

    #[error("not found: {0}")]
    NotFound(String),

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("internal server error: {0}")]
    InternalServerError(String),

    #[error("conflict: {0}")]
    Conflict(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<axum::body::Body> {
        let status = match &self {
            Self::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::PasswordHasherError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::JwtError(_) => StatusCode::FORBIDDEN,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Conflict(_) => StatusCode::CONFLICT,
        };
        (status, self.to_string()).into_response()
    }
}
