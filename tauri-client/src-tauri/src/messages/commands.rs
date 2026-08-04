
use reqwest::Error;

use protocol::{GET_MESSAGES, SERVER_ADDRESS};

use crate::messages::models::ChatMessage;


#[derive(Debug)]
pub struct MessageClient {
    client: reqwest::Client,
}

impl MessageClient {
    pub fn new() -> Self {
        let client = reqwest::Client::new();

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
