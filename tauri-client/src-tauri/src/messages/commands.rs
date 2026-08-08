use std::sync::Mutex;

use reqwest::Error;

use protocol::{CHATS, GET_MESSAGES};

use crate::{
    messages::models::{ChatMessage, GetMessagesReq, User},
    settings::SettingsWriter,
};

#[derive(Debug)]
pub struct MessageClient {
    client: reqwest::Client,
}

impl MessageClient {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }

    pub async fn get_messages(
        &self,
        receiver_id: &i64,
        server_addr: &str,
    ) -> Result<Vec<ChatMessage>, Error> {
        let url = format!("http://{server_addr}{GET_MESSAGES}");

        let params = GetMessagesReq {
            receiver: *receiver_id,
        };

        self.client
            .get(url)
            .query(&params)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<ChatMessage>>()
            .await
    }

    pub async fn get_chats(&self, server_addr: &str) -> Result<Vec<User>, Error> {
        let url = format!("http://{server_addr}{GET_MESSAGES}{CHATS}");
        self.client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<User>>()
            .await
    }
}

#[tauri::command]
pub async fn get_messages(
    message_client: tauri::State<'_, MessageClient>,
    settings_writer: tauri::State<'_, Mutex<SettingsWriter>>,
    receiver_id: i64,
) -> Result<Vec<ChatMessage>, String> {
    let server_addr = settings_writer
        .lock()
        .map_err(|error| error.to_string())?
        .server_address();

    message_client
        .inner()
        .get_messages(&receiver_id, &server_addr)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn get_chats(
    message_client: tauri::State<'_, MessageClient>,
    settings_writer: tauri::State<'_, Mutex<SettingsWriter>>,
) -> Result<Vec<User>, String> {
    let server_addr = settings_writer
        .lock()
        .map_err(|error| error.to_string())?
        .server_address();

    message_client
        .inner()
        .get_chats(&server_addr)
        .await
        .map_err(|error| error.to_string())
}
