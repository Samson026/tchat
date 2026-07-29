use axum::routing::connect;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async, tungstenite::{Error, Message},
};
use tokio::net::TcpStream;

pub struct WebSocketConnection {
    socket: WebSocketStream<MaybeTlsStream<TcpStream>>
}

impl WebSocketConnection {
    pub async fn connect(url: &str) -> Self {
        let (socket, _) = connect_async(url)
            .await
            .expect("Failed to connect");

        Self { socket }
    }

    pub async fn send(&mut self, message: &str) {
        self.socket
            .send(Message::text(message))
            .await
            .expect("Error")
    }

    pub async fn recv(&mut self) -> Option<String> {
        while let Some(message) = self.socket.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    return Some(text.to_string());
                },
                Ok(Message::Close(_)) => {
                    return None;
                },
                Ok(_) => {}
                Err(err) => {
                    eprint!("{err}");
                    return None;
                }
            }
        }
        None
    }
}