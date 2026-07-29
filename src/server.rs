use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket},
    routing::get,
    response::Response,
    Router,
};

use std::io;
use tokio::net::TcpListener;

pub async fn run() -> io::Result<()> {
    let app = Router::new().route("/ws", get(upgrade));
    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    axum::serve(listener, app).await
}

async fn upgrade(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };

        if socket.send(msg).await.is_err() {
            // client disconnected
            return;
        }
    }
}



