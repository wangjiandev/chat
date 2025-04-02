use crate::AppState;
use axum::response::IntoResponse;
use axum::{
    extract::{FromRequestParts, Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use tracing::warn;

pub async fn verify_token(State(state): State<AppState>, req: Request, next: Next) -> Response {
    let (mut parts, body) = req.into_parts();
    let req =
        match TypedHeader::<Authorization<Bearer>>::from_request_parts(&mut parts, &state).await {
            Ok(TypedHeader(Authorization(bearer))) => {
                let token = bearer.token();
                match state.dk.verify(token) {
                    Ok(claims) => {
                        let mut req = Request::from_parts(parts, body);
                        req.extensions_mut().insert(claims);
                        req
                    }
                    Err(e) => {
                        let msg = format!("Failed to verify token: {}", e);
                        warn!(msg);
                        return (StatusCode::UNAUTHORIZED, msg).into_response();
                    }
                }
            }
            Err(e) => {
                let msg = format!("Failed to verify token: {}", e);
                warn!(msg);
                return (StatusCode::UNAUTHORIZED, msg).into_response();
            }
        };
    next.run(req).await
}
