use axum::{
    Router,
    extract::{
        Query, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::Response,
    routing::get,
};
use protocol::WEBSOCKET_PATH;

use crate::{
    state::AppState,
    websocket::models::{ChatMessage, WebSocketParams},
};

pub fn router() -> Router<AppState> {
    Router::new().route(WEBSOCKET_PATH, get(upgrade))
}

pub async fn upgrade(
    State(app_state): State<AppState>,
    Query(params): Query<WebSocketParams>,
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, params.user_id, app_state))
}

pub async fn handle_socket(mut socket: WebSocket, user_id: i64, mut app_state: AppState) {
    let (outgoing_tx, mut outgoing_rx) = tokio::sync::mpsc::unbounded_channel::<ChatMessage>();

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
                      let parsed: ChatMessage = match serde_json::from_str(&text) {
                          Ok(message) => message,
                          Err(error) => {
                              eprintln!("Invalid websocket message: {error}");
                              continue;
                          }
                      };
                      println!(
                          "message received: sender_id={}, recv_id={}, content={}",
                          parsed.sender_id, parsed.recv_id, parsed.content
                      );

                      // add msg to db

                      match app_state.message_db.add_message(&parsed.content, &parsed.sender_id, &parsed.recv_id).await {
                        Ok(_) => {
                            println!("saved msg to db");
                        },
                        Err(error) => {
                            eprintln!("Error saving msg to db: {error}");
                        }
                      }

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
