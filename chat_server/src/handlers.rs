pub(crate) mod auth;
pub(crate) mod chat;
pub(crate) mod message;

use axum::response::IntoResponse;

pub(crate) async fn index_handler() -> impl IntoResponse {
    "index"
}
