use reqwest::Error;

use protocol::{CHATS, GET_MESSAGES, SERVER_ADDRESS};

use crate::messages::models::{ChatMessage, GetMessagesReq, User};

#[derive(Debug)]
pub struct MessageClient {
    client: reqwest::Client,
}

impl MessageClient {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }

    pub async fn get_messages(&self, receiver_id: &i64) -> Result<Vec<ChatMessage>, Error> {
        let url = format!("http://{SERVER_ADDRESS}{GET_MESSAGES}");

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

    pub async fn get_chats(&self) -> Result<Vec<User>, Error> {
        let url = format!("http://{SERVER_ADDRESS}{GET_MESSAGES}{CHATS}");
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
    receiver_id: i64,
) -> Result<Vec<ChatMessage>, String> {
    message_client
        .inner()
        .get_messages(&receiver_id)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn get_chats(
    message_client: tauri::State<'_, MessageClient>,
) -> Result<Vec<User>, String> {
    message_client
        .inner()
        .get_chats()
        .await
        .map_err(|error| error.to_string())
}
