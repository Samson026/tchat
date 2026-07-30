mod db;

use axum::{
    Router,
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::Response,
    routing::get,
};
use protocol::{SERVER_ADDRESS, WEBSOCKET_PATH};
use std::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let app = Router::new().route(WEBSOCKET_PATH, get(upgrade));
    let listener = TcpListener::bind(SERVER_ADDRESS).await?;

    println!("server listening on ws://{SERVER_ADDRESS}{WEBSOCKET_PATH}");
    axum::serve(listener, app).await
}

async fn upgrade(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(message) = socket.recv().await {
        let Ok(message) = message else {
            return;
        };

        if socket.send(message).await.is_err() {
            return;
        }
    }
}
