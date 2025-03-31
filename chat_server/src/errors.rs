use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("sql error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("argon2 error: {0}")]
    PasswordHasherError(#[from] argon2::password_hash::Error),
}
