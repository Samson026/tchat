pub mod models;

use crate::{api::models::ChatMessage, db::{User}, state::AppState};
use axum::{
    Json, extract::{
        Query, State, ws::{Message, WebSocket, WebSocketUpgrade, },
    }, http::StatusCode, response::{IntoResponse, Response},
};

use models::{WebSocketParams, LoginRequest};

pub async fn create_user(
    State(mut app_state): State<AppState>,
    Json(data): Json<LoginRequest>,
) -> Response {

    println!("create user called");

    match app_state.db.add_user(&data.username).await {
        Ok(user) => Json(user).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR, "Could not create user"
        ) 
        .into_response()
    }
}

pub async fn login(
    State(app_state): State<AppState>,
    Json(data): Json<LoginRequest>,
) -> Result<Json<User>, (StatusCode, &'static str)> {

    println!("login called");

    match app_state.db.get_user(&data.username).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err((StatusCode::NOT_FOUND, "User not found")),
    }
}

pub async fn upgrade(State(app_state): State<AppState>, Query(params): Query<WebSocketParams>, ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, params.user_id, app_state))
}

pub async fn handle_socket(mut socket: WebSocket, user_id: i64, app_state: AppState) {
    let (outgoing_tx, mut outgoing_rx) =
        tokio::sync::mpsc::unbounded_channel::<ChatMessage>();

    app_state
        .connections
        .write()
        .await
        .insert(user_id, outgoing_tx);

    while let Some(message) = socket.recv().await {
        
    }
}
