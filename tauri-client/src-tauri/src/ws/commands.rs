use std::sync::{Arc, Mutex};

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use protocol::WEBSOCKET_PATH;
use reqwest::Url;
use reqwest_cookie_store::CookieStoreMutex;
use tauri::Emitter;
use tokio::{net::TcpStream, sync::Mutex as TokioMutex};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        client::IntoClientRequest,
        http::{header::COOKIE, HeaderValue},
        Message, Result,
    },
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
    cookie_store: tauri::State<'_, Arc<CookieStoreMutex>>,
) -> Result<(), String> {
    let server_addr = settings_writer
        .lock()
        .map_err(|error| error.to_string())?
        .server_address();
    let url = format!("ws://{server_addr}{WEBSOCKET_PATH}");
    let cookie_url = Url::parse(&format!("http://{server_addr}{WEBSOCKET_PATH}"))
        .map_err(|error| error.to_string())?;

    let cookie_header = {
        let store = cookie_store.lock().map_err(|error| error.to_string())?;

        let header = store
            .get_request_values(&cookie_url)
            .map(|(name, value)| format!("{name}={value}"))
            .collect::<Vec<_>>()
            .join("; ");

        if header.is_empty() {
            None
        } else {
            Some(header)
        }
    };

    let mut request = url
        .as_str()
        .into_client_request()
        .map_err(|error| error.to_string())?;

    if let Some(cookie_header) = cookie_header {
        request.headers_mut().insert(
            COOKIE,
            HeaderValue::from_str(&cookie_header).map_err(|error| error.to_string())?,
        );
    }

    let (socket, _) = connect_async(request)
        .await
        .map_err(|error| error.to_string())?;

    let ws = WebSocketConnection { socket };

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
