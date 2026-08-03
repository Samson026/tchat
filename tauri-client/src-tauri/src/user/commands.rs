
use reqwest::Error;

use protocol::{CREATE_USER_PATH, LOGIN_PATH, SERVER_ADDRESS};

use crate::user::models::{NewUserRequest, User};



#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Result<Self, Error> {
        let client = reqwest::Client::new();

        Ok(Self { client })
    }

    pub async fn create_user(&self, username: &str) -> Result<User, Error> {
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

#[tauri::command]
pub async fn create_user(
    client: tauri::State<'_, Client>,
    username: String
) -> Result<User, String> {
    client
        .inner()
        .create_user(&username)
        .await
        .map_err(|error| error.to_string())
}
