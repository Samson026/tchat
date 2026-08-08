use std::{fs::File, io::BufWriter, path::PathBuf, sync::Arc};

use reqwest::Error;

use protocol::{CREATE_USER_PATH, GET_USERS, LOGIN_PATH, SERVER_ADDRESS};
use reqwest_cookie_store::CookieStoreMutex;
use tauri::Manager;

use crate::user::models::{NewUserRequest, User};

#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }

    pub async fn create_user(&self, username: &str, password: &str, server_addr: &str) -> Result<User, Error> {
        let body = NewUserRequest {
            username: username.to_string(),
            password: password.to_string(),
        };

        println!("got here");

        let url = format!("http://{server_addr}{GET_USERS}{CREATE_USER_PATH}");
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

        let url = format!("http://{SERVER_ADDRESS}{GET_USERS}{LOGIN_PATH}");
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

    pub fn save_cookies(
        &self,
        path: &PathBuf,
        cookie_store: &CookieStoreMutex,
    ) -> Result<(), std::io::Error> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        let store = cookie_store
            .lock()
            .map_err(|error| error.to_string())
            .unwrap();
        cookie_store::serde::json::save(&store, &mut writer).map_err(std::io::Error::other)
    }
}

#[tauri::command]
pub async fn create_user(
    client: tauri::State<'_, Client>,
    username: String,
    password: String,
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
    cookie_store: tauri::State<'_, Arc<CookieStoreMutex>>,
    app: tauri::AppHandle,
    username: String,
    password: String,
) -> Result<User, String> {
    let res = client
        .inner()
        .login(&username, &password)
        .await
        .map_err(|error| error.to_string());

    let path = app
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?
        .join("cookies.json");

    // save cookies
    client
        .inner()
        .save_cookies(&path, cookie_store.inner())
        .map_err(|error| error.to_string())?;

    res
}

#[tauri::command]
pub async fn get_users(client: tauri::State<'_, Client>) -> Result<Vec<User>, String> {
    client
        .inner()
        .get_users()
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn logout(
    cookie_store: tauri::State<'_, Arc<CookieStoreMutex>>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let mut store = cookie_store
        .inner()
        .lock()
        .map_err(|error| error.to_string())?;

    store.clear();

    let path = app
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?
        .join("cookies.json");

    let file = File::create(&path).map_err(|error| error.to_string())?;
    let mut writer = BufWriter::new(file);
    cookie_store::serde::json::save(&store, &mut writer).map_err(|error| error.to_string())?;

    Ok(())
}
