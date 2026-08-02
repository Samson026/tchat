use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
}

#[derive(FromRow, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
}

#[derive(Deserialize, Serialize)]
pub struct ChatHistoryReq {
    pub sender_id: i64,
    pub recv_id: i64,
}
