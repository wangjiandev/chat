use axum::response::IntoResponse;

pub(crate) async fn login_handler() -> impl IntoResponse {
    "login"
}

pub(crate) async fn register_handler() -> impl IntoResponse {
    "register"
}

pub(crate) async fn logout_handler() -> impl IntoResponse {
    "logout"
}
