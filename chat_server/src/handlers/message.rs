use axum::response::IntoResponse;

pub(crate) async fn message_list_handler() -> impl IntoResponse {
    "message_list"
}

pub(crate) async fn message_create_handler() -> impl IntoResponse {
    "message_create"
}
