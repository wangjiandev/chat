mod auth;
mod chat;
mod message;

use axum::response::IntoResponse;

pub(crate) use auth::*;
pub(crate) use chat::*;
pub(crate) use message::*;

pub(crate) async fn index_handler() -> impl IntoResponse {
    "index"
}
