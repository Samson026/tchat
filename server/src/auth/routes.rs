use axum::{Router, http::StatusCode, middleware, routing::post};
use protocol::{AUTH, BASE_ROUTE};

use crate::{middleware::auth_middleware, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(BASE_ROUTE, post(auth))
        .route_layer(middleware::from_fn(auth_middleware))
}

async fn auth() -> StatusCode {
    StatusCode::OK
}
