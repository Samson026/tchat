use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(FromRow, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
}
