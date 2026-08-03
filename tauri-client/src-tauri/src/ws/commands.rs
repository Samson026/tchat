

use futures_util::{SinkExt, StreamExt, stream::{SplitSink, SplitStream}};
use protocol::{SERVER_URL, WEBSOCKET_PATH};
use tauri::Emitter;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async,
    tungstenite::{Message, Result},
};

type Socket = WebSocketStream<MaybeTlsStream<TcpStream>>;
type Writer = SplitSink<Socket, Message>;
type Reader = SplitStream<Socket>;

#[derive(Debug)]
pub struct WebSocketConnection {
    socket: Socket,
}

impl WebSocketConnection {
    pub async fn connect(url: &str) -> Result<Self> {
        let (socket, _) = connect_async(url).await?;
        Ok(Self { socket })
    }

    pub fn split(self) -> (Writer, Reader) {
        self.split()
    }
}

pub struct WsState {
    pub connection: Mutex<Option<Writer>>,
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
    app: tauri::AppHandle,
    state: tauri::State<'_, WsState>,
) -> Result<(), String> {
    let url = format!("{SERVER_URL}{WEBSOCKET_PATH}");
    let ws = WebSocketConnection::connect(&url).await
        .map_err(|error| error.to_string())?;

    let (writer, mut reader) = ws.split();

    let mut connection = state.connection.lock().await;
    *connection = Some(writer);

    tauri::async_runtime::spawn(async move {
        while let Some(result) = reader.next().await {
            match result {
                Ok(Message::Text(message)) => {
                    let _ = app.emit("ws-message", message.to_string());
                },
                Ok(Message::Close(_)) => {
                    let _ = app.emit("ws-disconnected", ());
                    break;
                },
                Err(error) => {
                    let _ = app.emit("ws-error", error.to_string());
                    break;
                },
                _ => {}
            }
        }
    });

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

    ws.send(Message::text(message)).await.map_err(|error| error.to_string())?;
    Ok(())
}


