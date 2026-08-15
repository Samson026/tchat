use axum::{
    Extension, Router,
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    middleware,
    response::Response,
    routing::get,
};

use futures_util::sink::SinkExt;

use protocol::BASE_ROUTE;

use super::models::IncomingChatMessage;

use crate::{
    middleware::auth_middleware,
    state::AppState,
    websocket::models::ChatMessage,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(BASE_ROUTE, get(upgrade))
        .route_layer(middleware::from_fn(auth_middleware))
}

pub async fn upgrade(
    State(app_state): State<AppState>,
    Extension(user_id): Extension<i64>,
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, user_id, app_state))
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
                      let parsed: IncomingChatMessage = match serde_json::from_str(&text) {
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

                      match app_state.message_db.add_message(&parsed.content, &parsed.sender_id, &parsed.recv_id, parsed.attachment.as_deref()).await {
                        Ok(chat_id) => {
                            println!("saved msg to db");

                            let outgoing = ChatMessage {
                                chat_id,
                                sender_id: parsed.sender_id,
                                recv_id: parsed.recv_id,
                                content: parsed.content,
                                attachment: parsed.attachment,
                            };

                            let recv = {
                                let connections = app_state.connections.read().await;
                                connections.get(&outgoing.recv_id).cloned()
                            };

                            if let Some(recv) = recv {
                                let _ = recv.send(outgoing);
                            }
                        },
                        Err(error) => {
                            eprintln!("Error saving msg to db: {error}");
                        }
                      }
                  },
                  Some(Ok(Message::Close(_))) => {
                      if let Err(error) = socket.flush().await {
                          eprintln!("Error: {error}");
                      }
                      break;
                  },
                  None => {
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
