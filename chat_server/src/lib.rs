mod config;
mod errors;
mod handlers;
mod models;

use axum::{
    routing::{get, patch, post},
    Router,
};
use handlers::{
    chat_create_handler, chat_delete_handler, chat_list_handler, chat_update_handler,
    index_handler, login_handler, logout_handler, message_create_handler, message_list_handler,
    register_handler,
};
use std::{ops::Deref, sync::Arc};

pub use config::AppConfig;
pub use errors::AppError;
pub use models::User;

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    inner: Arc<AppStateInner>,
}

#[allow(unused)]
#[derive(Debug)]
pub(crate) struct AppStateInner {
    pub(crate) config: AppConfig,
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        Self {
            inner: Arc::new(AppStateInner { config }),
        }
    }
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub fn get_router(config: AppConfig) -> Router {
    let state = AppState::new(config);

    let api_router = Router::new()
        .route("/login", post(login_handler))
        .route("/logout", post(logout_handler))
        .route("/register", post(register_handler))
        .route("/chat", get(chat_list_handler).post(chat_create_handler))
        .route(
            "/chat/{id}",
            patch(chat_update_handler).delete(chat_delete_handler),
        )
        .route(
            "/chat/{id}/message",
            get(message_list_handler).post(message_create_handler),
        );

    Router::new()
        .route("/", get(index_handler))
        .nest("/api", api_router)
        .with_state(state)
}
