use futures_util::{SinkExt, StreamExt};
use protocol::SERVER_URL;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async,
    tungstenite::{Message, Result},
};

pub struct WebSocketConnection {
    socket: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl WebSocketConnection {
    pub async fn connect(url: &str) -> Result<Self> {
        let (socket, _) = connect_async(url).await?;
        Ok(Self { socket })
    }

    pub async fn send(&mut self, message: &str) -> Result<()> {
        self.socket.send(Message::text(message)).await
    }

    pub async fn recv(&mut self) -> Result<Option<String>> {
        while let Some(message) = self.socket.next().await {
            match message? {
                Message::Text(text) => return Ok(Some(text.to_string())),
                Message::Close(_) => return Ok(None),
                _ => {}
            }
        }

        Ok(None)
    }
}
