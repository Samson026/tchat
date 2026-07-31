pub mod models;

use models::{NewUserRequest, User};
use reqwest::Error;
use serde::de::Unexpected::Other;

use protocol::{CREATE_USER_PATH, LOGIN_PATH, SERVER_ADDRESS};

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

        println!("got here");

        let url = format!("http://{SERVER_ADDRESS}{CREATE_USER_PATH}");
        println!("{url}");
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
}
