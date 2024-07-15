use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use sse::event_handler;

pub mod sse;

pub fn get_router() -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/events", get(event_handler))
}

const INDEX_HTML: &str = include_str!("../index.html");

async fn index_handler() -> impl IntoResponse {
    Html(INDEX_HTML)
}
