use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
use tracing::warn;
use uuid::Uuid;

use super::REQUEST_ID_HEADER;

pub async fn request_id_middleware(mut req: Request, next: Next) -> Response {
    let id = match req.headers().get(REQUEST_ID_HEADER) {
        Some(v) => Some(v.clone()),
        None => {
            let request_id = Uuid::now_v7().to_string();
            match HeaderValue::from_str(&request_id) {
                Ok(v) => {
                    req.headers_mut().insert(REQUEST_ID_HEADER, v.clone());
                    Some(v)
                }
                Err(e) => {
                    warn!("Failed to parse request ID: {}", e);
                    None
                }
            }
        }
    };

    let mut res = next.run(req).await;

    let Some(id) = id else {
        return res;
    };
    res.headers_mut().insert(REQUEST_ID_HEADER, id);
    res
}
