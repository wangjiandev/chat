mod config;
mod errors;
mod handlers;
mod middlewares;
mod models;
mod utils;

pub use config::AppConfig;
pub use errors::AppError;
pub use models::User;

use anyhow::Result;
use axum::{
    middleware::from_fn_with_state,
    routing::{get, patch, post},
    Router,
};
use handlers::{
    chat_create_handler, chat_delete_handler, chat_list_handler, chat_update_handler,
    index_handler, login_handler, logout_handler, message_create_handler, message_list_handler,
    register_handler,
};
use middlewares::{set_layers, verify_token};
use sqlx::PgPool;
use std::{
    fmt::{self, Debug},
    ops::Deref,
    sync::Arc,
};
use utils::{DecodingKey, EncodingKey};

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    inner: Arc<AppStateInner>,
}

#[allow(dead_code)]
pub(crate) struct AppStateInner {
    pub(crate) config: AppConfig,
    pub(crate) dk: DecodingKey,
    pub(crate) ek: EncodingKey,
    pub(crate) pool: PgPool,
}

impl AppState {
    pub async fn try_new(config: AppConfig) -> Result<Self, AppError> {
        let dk = DecodingKey::load(&config.auth.sk)?;
        let ek = EncodingKey::load(&config.auth.pk)?;
        let pool = PgPool::connect(&config.server.database_url).await?;
        Ok(Self {
            inner: Arc::new(AppStateInner {
                config,
                dk,
                ek,
                pool,
            }),
        })
    }
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Debug for AppStateInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AppStateInner")
    }
}

pub async fn get_router(config: AppConfig) -> Result<Router, AppError> {
    let state = AppState::try_new(config).await?;
    let api_router = Router::new()
        .route("/chat", get(chat_list_handler).post(chat_create_handler))
        .route(
            "/chat/{id}",
            patch(chat_update_handler).delete(chat_delete_handler),
        )
        .route(
            "/chat/{id}/message",
            get(message_list_handler).post(message_create_handler),
        )
        .layer(from_fn_with_state(state.clone(), verify_token))
        .route("/login", post(login_handler))
        .route("/logout", post(logout_handler))
        .route("/register", post(register_handler));

    let router = Router::new()
        .route("/", get(index_handler))
        .nest("/api", api_router)
        .with_state(state);

    Ok(set_layers(router))
}
