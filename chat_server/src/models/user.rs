use std::mem;

use crate::{AppError, User};
use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sqlx::PgPool;

impl User {
    /// Find a user by email
    pub async fn find_by_email(email: &str, pool: &PgPool) -> Result<Option<Self>, AppError> {
        let user = sqlx::query_as("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(pool)
            .await?;
        Ok(user)
    }

    /// Create a new user
    pub async fn create(
        email: &str,
        fullname: &str,
        password: &str,
        pool: &PgPool,
    ) -> Result<Self, AppError> {
        let password_hash = Self::hash_password(password)?;
        let user = sqlx::query_as(
            "INSERT INTO users (email, fullname, password_hash) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(email)
        .bind(fullname)
        .bind(password_hash)
        .fetch_one(pool)
        .await?;
        Ok(user)
    }

    /// Verify a user email and password
    pub async fn verify(email: &str, password: &str, pool: &PgPool) -> Result<bool, AppError> {
        let user = Self::find_by_email(email, pool).await?;
        match user {
            Some(mut user) => {
                let password_hash = mem::take(&mut user.password_hash);
                let is_valid = Self::verify_password(password, &password_hash.unwrap_or_default())?;
                if !is_valid {
                    return Ok(false);
                }
                Ok(true)
            }
            None => Ok(false),
        }
    }

    fn hash_password(password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();
        Ok(password_hash)
    }

    fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
        let argon2 = Argon2::default();
        let password_hash = PasswordHash::new(password_hash)?;
        let is_valid = argon2
            .verify_password(password.as_bytes(), &password_hash)
            .is_ok();
        Ok(is_valid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_user_should_succeed() {}
}
