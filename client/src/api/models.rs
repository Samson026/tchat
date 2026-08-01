use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
}

#[derive(Serialize)]
pub struct NewUserRequest {
    pub username: String,
}

#[derive(Deserialize, Serialize)]
pub struct ChatMessage {
    pub sender_id: i64,
    pub receiver_id: i64,
    pub content: String,
}
