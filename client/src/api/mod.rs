pub mod models;

use models::{NewUserRequest, User};
use reqwest::Error;

use protocol::{CREATE_USER_PATH, GET_MESSAGES, GET_USERS, LOGIN_PATH, SERVER_ADDRESS};

use crate::api::models::ChatMessage;

#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub async fn new() -> Result<Self, Error> {
        let client = reqwest::Client::new();

        Ok(Self { client })
    }

    pub async fn create_user(&mut self, username: &str) -> Result<User, Error> {
        let body = NewUserRequest {
            username: username.to_string(),
        };

        let url = format!("http://{SERVER_ADDRESS}{CREATE_USER_PATH}");
        self.client
            .post(url)
            .json(&body)
            .send()
            .await?
            .json::<User>()
            .await
    }

    pub async fn login(&mut self, username: &str) -> Result<User, Error> {
        let body = NewUserRequest {
            username: username.to_string(),
        };

        let url = format!("http://{SERVER_ADDRESS}{LOGIN_PATH}");
        self.client
            .post(url)
            .json(&body)
            .send()
            .await?
            .json::<User>()
            .await
    }

    pub async fn get_message(
        &mut self,
        sender_id: &i64,
        receiver_id: &i64,
    ) -> Result<Vec<ChatMessage>, Error> {
        let url = format!("http://{SERVER_ADDRESS}{GET_MESSAGES}");

        self.client
            .get(url)
            .query(&[("sender_id", *sender_id), ("recv_id", *receiver_id)])
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<ChatMessage>>()
            .await
    }

    pub async fn get_users(&mut self) -> Result<Vec<User>, Error> {
        let url = format!("http://{SERVER_ADDRESS}{GET_USERS}");

        self.client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<User>>()
            .await
    }
}
