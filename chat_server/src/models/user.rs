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
    pub async fn verify(
        email: &str,
        password: &str,
        pool: &PgPool,
    ) -> Result<Option<Self>, AppError> {
        let user = Self::find_by_email(email, pool).await?;
        match user {
            Some(mut user) => {
                let password_hash = mem::take(&mut user.password_hash);
                let is_valid = Self::verify_password(password, &password_hash.unwrap_or_default())?;
                if !is_valid {
                    return Ok(None);
                }
                Ok(Some(user))
            }
            None => Ok(None),
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
    use std::path::Path;

    use super::*;
    use anyhow::Result;
    use sqlx_db_tester::TestPg;

    #[tokio::test]
    async fn hash_password_and_verify_should_work() -> Result<()> {
        let password = "password";
        let password_hash = User::hash_password(password)?;
        assert_eq!(password_hash.len(), 97);
        let is_valid = User::verify_password(password, &password_hash)?;
        assert!(is_valid);
        Ok(())
    }

    #[tokio::test]
    async fn create_and_verify_user_should_work() -> Result<()> {
        let test_pg = TestPg::new(
            "postgres://wangjian:@localhost:5432".to_string(),
            Path::new("../migrations"),
        );
        let pool = test_pg.get_pool().await;
        let email = "test@test.com";
        let fullname = "Test User";
        let password = "password";
        let user = User::create(email, fullname, password, &pool).await?;
        assert_eq!(user.email, email);
        assert_eq!(user.fullname, fullname);
        assert!(user.password_hash.is_some());
        assert!(user.id.is_positive());

        let user = User::verify(email, password, &pool).await?;
        assert!(user.is_some());
        Ok(())
    }
}
