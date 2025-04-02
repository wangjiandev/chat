mod auth;
mod request_id;
mod server_time;

use axum::{middleware::from_fn, Router};
use request_id::request_id_middleware;
use server_time::ServerTimeLayer;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

pub use auth::verify_token;

const REQUEST_ID_HEADER: &str = "X-Request-Id";
const SERVER_TIME_HEADER: &str = "X-Server-Time";

pub fn set_layers(router: Router) -> Router {
    router.layer(
        ServiceBuilder::new()
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().include_headers(true))
                    .on_request(DefaultOnRequest::new().level(Level::INFO))
                    .on_response(
                        DefaultOnResponse::new()
                            .level(Level::INFO)
                            .latency_unit(tower_http::LatencyUnit::Micros),
                    ),
            )
            .layer(CompressionLayer::new().gzip(true).br(true).deflate(true))
            .layer(from_fn(request_id_middleware))
            .layer(ServerTimeLayer),
    )
}
