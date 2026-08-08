use std::sync::Mutex;

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use protocol::WEBSOCKET_PATH;
use tauri::Emitter;
use tokio::{net::TcpStream, sync::Mutex as TokioMutex};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Message, Result},
    MaybeTlsStream, WebSocketStream,
};

use crate::{settings::SettingsWriter, ws::models::ChatMessage};

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
        self.socket.split()
    }
}

pub struct WsState {
    pub connection: TokioMutex<Option<Writer>>,
}

impl WsState {
    pub fn new() -> Self {
        Self {
            connection: TokioMutex::new(None),
        }
    }
}

#[tauri::command]
pub async fn connect_ws(
    app: tauri::AppHandle,
    state: tauri::State<'_, WsState>,
    settings_writer: tauri::State<'_, Mutex<SettingsWriter>>,
) -> Result<(), String> {
    let server_addr = settings_writer
        .lock()
        .map_err(|error| error.to_string())?
        .server_address();

    let url = format!("ws://{server_addr}{WEBSOCKET_PATH}");
    let ws = WebSocketConnection::connect(&url)
        .await
        .map_err(|error| error.to_string())?;

    let (writer, mut reader) = ws.split();

    let mut connection = state.connection.lock().await;
    *connection = Some(writer);

    tauri::async_runtime::spawn(async move {
        while let Some(result) = reader.next().await {
            match result {
                Ok(Message::Text(message)) => {
                    let _ = app.emit("ws-message", message.to_string());
                }
                Ok(Message::Close(_)) => {
                    let _ = app.emit("ws-disconnected", ());
                    break;
                }
                Err(error) => {
                    let _ = app.emit("ws-error", error.to_string());
                    break;
                }
                _ => {}
            }
        }
    });

    Ok(())
}

#[tauri::command]
pub async fn send(state: tauri::State<'_, WsState>, message: ChatMessage) -> Result<(), String> {
    let mut connection = state.connection.lock().await;

    let ws = connection
        .as_mut()
        .ok_or_else(|| "Websocket is not connected".to_string())?;

    let json = serde_json::to_string(&message).map_err(|error| error.to_string())?;

    ws.send(Message::text(json))
        .await
        .map_err(|error| error.to_string())?;
    Ok(())
}
