use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct IncomingChatMessage {
    pub sender_id: i64,
    pub recv_id: i64,
    pub content: String,
    pub attachment: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ChatMessage {
    pub chat_id: i64,
    pub sender_id: i64,
    pub recv_id: i64,
    pub content: String,
    pub attachment: Option<String>,
}
