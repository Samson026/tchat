use futures_util::{SinkExt, StreamExt};
use protocol::SERVER_URL;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async,
    tungstenite::{Message, Result},
};

struct WebSocketConnection {
    socket: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl WebSocketConnection {
    async fn connect(url: &str) -> Result<Self> {
        let (socket, _) = connect_async(url).await?;
        Ok(Self { socket })
    }

    async fn send(&mut self, message: &str) -> Result<()> {
        self.socket.send(Message::text(message)).await
    }

    async fn recv(&mut self) -> Result<Option<String>> {
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

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = WebSocketConnection::connect(SERVER_URL).await?;

    client.send("hi").await?;

    if let Some(response) = client.recv().await? {
        println!("{response}");
    }

    Ok(())
}
