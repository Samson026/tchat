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
