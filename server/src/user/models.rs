use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(FromRow, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct GetUserParams {
    pub id: i64
}