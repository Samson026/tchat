use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
}

#[derive(Serialize)]
pub struct NewUserRequest {
    pub username: String
}