
use reqwest::Error;

use protocol::{CREATE_USER_PATH, GET_USERS, LOGIN_PATH, SERVER_ADDRESS};

use crate::user::models::{NewUserRequest, User};



#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new(client: reqwest::Client) -> Self {

        Self { client }
    }

    pub async fn create_user(&self, username: &str, password: &str) -> Result<User, Error> {
        let body = NewUserRequest {
            username: username.to_string(),
            password: password.to_string()
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

    pub async fn login(&self, username: &str, password: &str) -> Result<User, Error> {
        let body = NewUserRequest {
            username: username.to_string(),
            password: password.to_string(),
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

    pub async fn get_users(&self) -> Result<Vec<User>, Error> {
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

#[tauri::command]
pub async fn create_user(
    client: tauri::State<'_, Client>,
    username: String,
    password: String
) -> Result<User, String> {
    client
        .inner()
        .create_user(&username, &password)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn login(
    client: tauri::State<'_, Client>,
    username: String,
    password: String
) -> Result<User, String> {
    client
        .inner()
        .login(&username, &password)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn get_users(
    client: tauri::State<'_, Client>
) -> Result<Vec<User>, String> {
    client
        .inner()
        .get_users()
        .await
        .map_err(|error| error.to_string())
}
