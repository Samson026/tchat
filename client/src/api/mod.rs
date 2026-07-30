mod models; 

use models::{User, NewUserRequest};
use serde::de::Unexpected::Other;
use reqwest::Error;

use protocol::SERVER_ADDRESS;

pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub async fn new() -> Result<Self, Error> {
        let client = reqwest::Client::new();

        Ok(Self {client})
    }

    pub async fn create_user(&mut self, username: &str) -> Result<User, Error> {
        let body = NewUserRequest {
            username: username.to_string()
        };

        self.client
        .post(SERVER_ADDRESS)
        .json(&body)
        .send()
        .await?
        .json::<User>()
        .await
    }

    pub async fn login(&mut self, username: &str) -> Result<User, Error> {
        let body = NewUserRequest {
            username: username.to_string()
        };

        self.client
        .post(SERVER_ADDRESS)
        .json(&body)
        .send()
        .await?
        .json::<User>()
        .await
    }
}
