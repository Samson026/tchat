use reqwest::Error;

use protocol::{CHATS, GET_MESSAGES, SERVER_ADDRESS};

use crate::messages::models::{ChatMessage, User};

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
        sender_id: &i64,
        receiver_id: &i64,
    ) -> Result<Vec<ChatMessage>, Error> {
        let url = format!("http://{SERVER_ADDRESS}{GET_MESSAGES}");

        self.client
            .get(url)
            .query(&[("sender_id", *sender_id), ("recv_id", *receiver_id)])
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<ChatMessage>>()
            .await
    }

    pub async fn get_chats(&self, user_id: &i64) -> Result<Vec<User>, Error> {
        let url = format!("httpL//{SERVER_ADDRESS}{GET_MESSAGES}{CHATS}");
        self.client
            .get(url)
            .query(&[("user_id", *user_id)])
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
    sender_id: i64,
    receiver_id: i64,
) -> Result<Vec<ChatMessage>, String> {
    message_client
        .inner()
        .get_messages(&sender_id, &receiver_id)
        .await
        .map_err(|error| error.to_string())
}
