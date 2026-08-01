use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
}

#[derive(Deserialize, Serialize)]
pub struct ChatMessage {
    pub sender_id: i64,
    pub recv_id: i64,
    pub content: String,
}

#[derive(Deserialize)]
pub struct WebSocketParams {
    pub user_id: i64,
}

#[derive(Deserialize, Serialize)]
pub struct ChatHistoryReq {
    pub sender_id: i64,
    pub recv_id: i64,
}
