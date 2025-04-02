use axum::{response::IntoResponse, Extension};
use tracing::info;

use crate::User;

pub(crate) async fn chat_list_handler(Extension(user): Extension<User>) -> impl IntoResponse {
    info!("user: {:?}", user);
    "chat_list"
}

pub(crate) async fn chat_create_handler() -> impl IntoResponse {
    "chat_create"
}

pub(crate) async fn chat_update_handler() -> impl IntoResponse {
    "chat_update"
}

pub(crate) async fn chat_delete_handler() -> impl IntoResponse {
    "chat_delete"
}
