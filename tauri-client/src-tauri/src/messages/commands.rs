
use reqwest::Error;

use protocol::{GET_MESSAGES, GET_USERS, SERVER_ADDRESS};
use tauri::async_runtime::Receiver;

use crate::messages::models::{ChatMessage, User};


#[derive(Debug)]
pub struct MessageClient {
    client: reqwest::Client,
}

impl MessageClient {
    pub fn new() -> Result<Self, Error> {
        let client = reqwest::Client::new();

        Ok(Self { client })
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

    pub async fn get_users(&self) -> Result<Vec<User>, Error> {
        let url = format!("http://{SERVER_ADDRESS}{GET_USERS}");

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
    messageClient: tauri::State<'_, MessageClient>,
    sender_id: i64,
    receiver_id: i64
) -> Result<Vec<ChatMessage>, String> {
    messageClient
        .inner()
        .get_messages(&sender_id, &receiver_id)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn get_users(
    messageClient: tauri::State<'_, MessageClient>
) -> Result<Vec<User>, String> {
    messageClient
        .inner()
        .get_users()
        .await
        .map_err(|error| error.to_string())
}
