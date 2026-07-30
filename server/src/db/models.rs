use serde::Serialize;
use sqlx::FromRow;
use sqlx::types::chrono::{DateTime, Utc};

#[derive(FromRow, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
}

#[derive(FromRow)]
pub struct Message {
    pub id: i64,
    pub sender_id: i64,
    pub recv_id: i64,
    pub content: String,
    pub time: DateTime<Utc>,
}
