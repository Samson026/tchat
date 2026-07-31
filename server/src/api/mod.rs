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

    loop {
      tokio::select! {
          incoming = socket.recv() => {
              // Handle data received from this client.
              match incoming {
                Some(Ok(Message::Text(text))) => {
                    let parsed: ChatMessage = serde_json::from_str(&text).unwrap();
                    
                    let recv = {
                        let connections = app_state.connections.read().await;
                        connections.get(&parsed.recv_id).cloned()
                    };

                    if let Some(recv) = recv {
                        let _ = recv.send(parsed);
                    }
                },
                Some(Ok(Message::Close(_))) | None => {
                    break;
                }
                Some(Err(error)) => {
                    eprint!("Error: {error}");
                },
                Some(Ok(_)) => {}
              }
          }

          Some(outgoing) = outgoing_rx.recv() => {
              match serde_json::to_string(&outgoing) {
                Ok(json) => {
                    if let Err(error) = socket.send(Message::Text(json.into())).await {
                        eprint!("Error: {error}");
                    }
                },
                Err(error) => {
                    eprint!("Error: {error}");
                }
              };
          }
      }
  }
}
