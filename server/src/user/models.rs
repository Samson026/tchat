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