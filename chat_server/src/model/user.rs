use std::mem;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::error::AppError;

use super::User;

impl User {
    /// **NOTE**: 通过邮箱查找用户,但是不包含密码
    pub async fn find_by_email(email: &str, pool: &sqlx::PgPool) -> Result<Option<Self>, AppError> {
        let user =
            sqlx::query_as("SELECT id, fullname, email, created_at FROM users WHERE email = $1")
                .bind(email)
                .fetch_optional(pool)
                .await?;
        Ok(user)
    }

    /// **NOTE**: 创建一个用户
    pub async fn create(
        fullname: &str,
        password: &str,
        email: &str,
        pool: &sqlx::PgPool,
    ) -> Result<Self, AppError> {
        let password = hash_password(password)?;

        let user = sqlx::query_as(
            r#"INSERT INTO users (fullname, password, email)
			VALUES ($1, $2, $3)
			RETURNING id, fullname, email, created_at"#,
        )
        .bind(fullname)
        .bind(password)
        .bind(email)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    /// Verify user email and password
    pub async fn verify(
        email: &str,
        password: &str,
        pool: &sqlx::PgPool,
    ) -> Result<Option<Self>, AppError> {
        let user: Option<User> = sqlx::query_as(
            "SELECT id, fullname, email, password, created_at FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;

        match user {
            Some(mut user) => {
                // 使用mem::take()方法可以将字段的值从结构体中取出并将字段的值设置为None
                let password_hash = mem::take(&mut user.password);
                let is_valid = verify_password(password, &password_hash.unwrap_or_default())?;
                if is_valid {
                    Ok(Some(user))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }
}

// **NOTE**: 生成密码哈希
fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

/// **NOTE**: 验证密码是否匹配
fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hash)?;
    let is_valid = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(is_valid)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use sqlx_db_tester::TestPg;

    use super::*;
    use anyhow::{Ok, Result};

    #[test]
    fn password_hash_should_work() -> Result<()> {
        let password = "password";
        let password_hash = hash_password(password)?;
        let is_valid = verify_password(password, &password_hash)?;
        assert!(is_valid);
        Ok(())
    }

    // #[tokio::test]
    #[allow(dead_code)]
    async fn create_user_should_work() -> Result<()> {
        let tdb = TestPg::new(
            "postgres://wangjian@localhost:5432".to_string(),
            Path::new("../migrations"),
        );
        let pool = tdb.get_pool().await;

        let fullname = "test";
        let password = "password";
        let email = "wangjian0504@gmail.com";

        let user = User::create(fullname, password, email, &pool).await?;
        assert_eq!(user.fullname, fullname);
        assert_eq!(user.email, email);

        Ok(())
    }
}
