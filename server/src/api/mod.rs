mod models;

use crate::db::{Database, User};
use axum::{
    Json,
    extract::{
        State,
        ws::{WebSocket, WebSocketUpgrade},
    },
    http::StatusCode,
    response::Response,
};
use models::LoginRequest;

pub async fn create_user(
    State(db): State<Database>,
    Json(data): Json<LoginRequest>,
) -> Result<Json<User>, (StatusCode, &'static str)> {
    match db.add_user(&data.username).await {
        Ok(user) => Json(user),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not create user"))
    }
}

pub async fn login(
    State(db): State<Database>,
    Json(data): Json<LoginRequest>,
) -> Result<Json<User>, (StatusCode, &'static str)> {
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
