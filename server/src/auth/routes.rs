use axum::{Router, http::StatusCode, middleware, routing::post};
use protocol::AUTH;

use crate::{middleware::auth_middleware, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(AUTH, post(auth))
        .route_layer(middleware::from_fn(auth_middleware))
}

async fn auth() -> StatusCode {
    StatusCode::OK
}
