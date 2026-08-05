use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::types::chrono::{DateTime, Utc};

#[derive(Deserialize, Serialize)]
pub struct ChatMessage {
    pub sender_id: i64,
    pub recv_id: i64,
    pub content: String,
}

#[allow(dead_code)]
#[derive(FromRow)]
pub struct Message {
    pub id: i64,
    pub chat_id: i64,
    pub sender_id: i64,
    pub recv_id: i64,
    pub content: String,
    pub time: DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
pub struct ChatHistoryReq {
    pub sender_id: i64,
    pub recv_id: i64,
}

#[derive(FromRow, Serialize)]
pub struct Chat {
    pub id: i64,
    pub user_1_id: i64,
    pub user_2_id: i64
}

#[derive(Deserialize, Serialize)]
pub struct ChatsReq {
    pub user_id: i64,
}

#[derive(FromRow, Serialize)]
pub struct ChatDB {
    pub username: String
}
