use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ChatMessage {
    pub sender_id: i64,
    pub recv_id: i64,
    pub content: String,
    pub attachment: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
}

#[derive(Serialize)]
pub struct GetMessagesReq {
    pub receiver: i64,
}

#[allow(dead_code)]
#[derive(Serialize)]
pub struct DownloadReq {
    pub file_name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Attachment {
    pub id: String,
}
