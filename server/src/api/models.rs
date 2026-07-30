use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
}
