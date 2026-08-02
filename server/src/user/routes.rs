use axum::{Json, extract::State, http::StatusCode, response::{IntoResponse, Response}};

use crate::{state::AppState, user::models::{LoginRequest, User}};



pub async fn create_user(
    State(mut app_state): State<AppState>,
    Json(data): Json<LoginRequest>,
) -> Response {
    println!("create user called");

    match app_state.userDB.add_user(&data.username).await {
        Ok(user) => Json(user).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Could not create user").into_response(),
    }
}

pub async fn get_users(State(mut app_state): State<AppState>) -> Response {
    match app_state.userDB.get_users().await {
        Ok(users) => Json(users).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Messages not found").into_response(),
    }
}

pub async fn login(
    State(app_state): State<AppState>,
    Json(data): Json<LoginRequest>,
) -> Result<Json<User>, (StatusCode, &'static str)> {
    println!("login called");

    match app_state.userDB.get_user(&data.username).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err((StatusCode::NOT_FOUND, "User not found")),
    }
}