use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),
}
