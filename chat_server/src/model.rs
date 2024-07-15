mod user;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub fullname: String,
    #[sqlx(default)]
    #[serde(skip)]
    pub password: Option<String>,
    pub email: String,
    pub created_at: DateTime<Utc>,
}
