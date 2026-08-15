use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::types::chrono::{DateTime, Utc};

#[derive(Deserialize, Serialize)]
pub struct ChatMessage {
    pub sender_id: i64,
    pub recv_id: i64,
    pub content: String,
    pub attachment: Option<String>,
}

#[allow(dead_code)]
#[derive(FromRow)]
pub struct Message {
    pub id: i64,
    pub chat_id: i64,
    pub sender_id: i64,
    pub recv_id: i64,
    pub content: String,
    pub attachment: Option<String>,
    pub time: DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
pub struct ChatHistoryReq {
    pub receiver: i64,
}

#[derive(FromRow, Serialize)]
pub struct Chat {
    pub id: i64,
    pub user_1_id: i64,
    pub user_2_id: i64,
}

#[derive(FromRow, Serialize)]
pub struct ClientChat {
    pub id: i64,
    pub username: String,
    pub user_id: i64,
    pub read_count: i64,
}

#[derive(Deserialize)]
pub struct DownloadReq {
    pub file_id: String,
}

#[derive(FromRow, Deserialize)]
pub struct Attachment {
    pub id: String,
    pub filelocation: String,
}

#[derive(Serialize)]
pub struct AttachmentUser {
    pub id: String,
}

impl From<Attachment> for AttachmentUser {
    fn from(attachment: Attachment) -> Self {
        Self { id: attachment.id }
    }
}

#[derive(Deserialize)]
pub struct UpdateLastReadReq {
    pub chat_id: i64,
    pub read_count: i64,
}

#[derive(Deserialize)]
pub struct GetChatByIdParams {
    pub receiver_id: i64
}
