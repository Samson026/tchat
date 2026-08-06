use axum::{
    Extension, Json, Router,
    extract::State,
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::post,
};
use protocol::BASE_ROUTE;

use crate::{middleware::auth_middleware, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(BASE_ROUTE, post(auth))
        .route_layer(middleware::from_fn(auth_middleware))
}

async fn auth(State(app_state): State<AppState>, Extension(user_id): Extension<i64>) -> Response {
    match app_state.auth_db.get_user(&user_id).await {
        Ok(user) => Json(user).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}
