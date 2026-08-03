

use futures_util::{SinkExt, StreamExt};
use protocol::{SERVER_URL, WEBSOCKET_PATH};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async,
    tungstenite::{Message, Result},
};

#[derive(Debug)]
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

    #[allow(dead_code)]
    pub async fn recv(&self) -> Result<Option<String>> {
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

pub struct WsState {
    pub connection: Mutex<Option<WebSocketConnection>>,
}

impl WsState {
    pub fn new() -> Self {
        Self {
            connection: Mutex::new(None)
        }
    }
}

#[tauri::command]
pub async fn connect_ws(
    state: tauri::State<'_, WsState>,
) -> Result<(), String> {
    let url = format!("{SERVER_URL}{WEBSOCKET_PATH}");
    let ws = WebSocketConnection::connect(&url).await
        .map_err(|error| error.to_string())?;

    let mut connection = state.connection.lock().await;
    *connection = Some(ws);

    Ok(())
}

#[tauri::command]
pub async fn send(
    state: tauri::State<'_, WsState>,
    message: String
) -> Result<(), String> {
    let mut connection = state.connection.lock().await;

    let ws = connection
        .as_mut()
        .ok_or_else(|| "Websocket is not connected".to_string())?;

    ws.send(&message)
        .await
        .map_err(|error| error.to_string())
}
