mod models;

use crate::db::{Database, User};
use axum::{
    Json, extract::{
        State,
        ws::{WebSocket, WebSocketUpgrade},
    }, http::StatusCode, response::{IntoResponse, Response},
};
use models::LoginRequest;

pub async fn create_user(
    State(mut db): State<Database>,
    Json(data): Json<LoginRequest>,
) -> Response {

    println!("create user called");

    match db.add_user(&data.username).await {
        Ok(user) => Json(user).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR, "Could not create user"
        ) 
        .into_response()
    }
}

pub async fn login(
    State(db): State<Database>,
    Json(data): Json<LoginRequest>,
) -> Result<Json<User>, (StatusCode, &'static str)> {

    println!("login called");

    match db.get_user(&data.username).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err((StatusCode::NOT_FOUND, "User not found")),
    }
}

pub async fn upgrade(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

pub async fn handle_socket(mut socket: WebSocket) {
    while let Some(message) = socket.recv().await {
        let Ok(message) = message else {
            return;
        };

        if socket.send(message).await.is_err() {
            return;
        }
    }
}
