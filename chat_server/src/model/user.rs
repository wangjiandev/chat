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
    pub async fn create(user: &User, pool: &sqlx::PgPool) -> Result<Self, AppError> {
        let user = sqlx::query_as(
            r#"INSERT INTO users (fullname, password, email)
			VALUES ($1, $2, $3)
			RETURNING id, fullname, email, created_at"#,
        )
        .bind(&user.fullname)
        .bind(&user.password)
        .bind(&user.email)
        .fetch_one(pool)
        .await?;
        Ok(user)
    }
}
