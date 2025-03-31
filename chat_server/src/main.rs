use anyhow::Result;
use chat_server::{get_router, AppConfig};
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let config = AppConfig::load()?;
    let addr = format!("{}:{}", config.server.host, config.server.port);
    info!("Starting server at {}", addr);
    let app = get_router(config);
    axum::serve(TcpListener::bind(&addr).await?, app.into_make_service()).await?;

    Ok(())
}
