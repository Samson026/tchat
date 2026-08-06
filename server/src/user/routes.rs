use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use protocol::{BASE_ROUTE, CREATE_USER_PATH, LOGIN_PATH};
use tower_sessions::Session;

use crate::{state::AppState, user::models::LoginRequest};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(CREATE_USER_PATH, post(create_user))
        .route(LOGIN_PATH, post(login))
        .route(BASE_ROUTE, get(get_users))
}

pub async fn create_user(
    State(mut app_state): State<AppState>,
    Json(data): Json<LoginRequest>,
) -> Response {
    println!("create user called");

    match app_state.user_db.add_user(&data.username).await {
        Ok(user) => Json(user).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Could not create user").into_response(),
    }
}

pub async fn get_users(State(mut app_state): State<AppState>) -> Response {
    match app_state.user_db.get_users().await {
        Ok(users) => Json(users).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Messages not found").into_response(),
    }
}

pub async fn login(
    State(app_state): State<AppState>,
    session: Session,
    Json(data): Json<LoginRequest>,
) -> Response {
    match app_state.user_db.get_user(&data.username).await {
        Ok(user) => {
            if data.password == user.password {
                session.insert("user_id", user.id).await.unwrap();
                Json(user).into_response()
            } else {
                (StatusCode::UNAUTHORIZED, "Invalid password").into_response()
            }
        }
        Err(_) => (StatusCode::NOT_FOUND, "User not found").into_response(),
    }
}
