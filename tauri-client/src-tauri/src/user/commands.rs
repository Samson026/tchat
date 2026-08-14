use std::{
    fs::File,
    io::BufWriter,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use reqwest::Error;

use protocol::{CREATE_USER_PATH, GET_USERS, LOGIN_PATH};
use reqwest_cookie_store::CookieStoreMutex;
use tauri::Manager;

use crate::{
    constants::COOKIE_FILE,
    settings::SettingsWriter,
    user::models::{NewUserRequest, User},
};

#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }

    pub async fn create_user(
        &self,
        username: &str,
        password: &str,
        server_addr: &str,
    ) -> Result<User, Error> {
        let body = NewUserRequest {
            username: username.to_string(),
            password: password.to_string(),
        };

        let url = format!("http://{server_addr}{GET_USERS}{CREATE_USER_PATH}");
        self.client
            .post(url)
            .json(&body)
            .send()
            .await?
            .json::<User>()
            .await
    }

    pub async fn login(
        &self,
        username: &str,
        password: &str,
        server_addr: &str,
    ) -> Result<User, String> {
        let body = NewUserRequest {
            username: username.to_string(),
            password: password.to_string(),
        };

        let url = format!("http://{server_addr}{GET_USERS}{LOGIN_PATH}");
        let response = self
            .client
            .post(url)
            .json(&body)
            .send()
            .await
            .map_err(|error| error.to_string())?;

        if response.status().is_success() {
            response
                .json::<User>()
                .await
                .map_err(|error| error.to_string())
        } else {
            let message = response
                .text()
                .await
                .unwrap_or_else(|_| "Login failed".to_string());

            Err(message)
        }
    }

    pub async fn get_users(&self, server_addr: &str) -> Result<Vec<User>, Error> {
        let url = format!("http://{server_addr}{GET_USERS}");

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

    pub async fn get_user(
        &self,
        user_id: &i64,
        server_addr: &str
    ) -> Result<User, Error> {
        let url = format!("http://{server_addr}{GET_USERS}/{user_id}");

        self.client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<User>()
            .await
    }
}

#[tauri::command]
pub async fn create_user(
    client: tauri::State<'_, Client>,
    settings_writer: tauri::State<'_, Mutex<SettingsWriter>>,
    cookie_store: tauri::State<'_, Arc<CookieStoreMutex>>,
    app: tauri::AppHandle,
    username: String,
    password: String,
) -> Result<User, String> {
    let server_addr = settings_writer
        .lock()
        .map_err(|error| error.to_string())?
        .server_address();

    let res = client
        .inner()
        .create_user(&username, &password, &server_addr)
        .await
        .map_err(|error| error.to_string());

    let path = app
        .path()
        .app_cache_dir()
        .map_err(|error| error.to_string())?
        .join(COOKIE_FILE);

    // save cookies
    client
        .inner()
        .save_cookies(&path, cookie_store.inner())
        .map_err(|error| error.to_string())?;

    res
}

#[tauri::command]
pub async fn login(
    client: tauri::State<'_, Client>,
    settings_writer: tauri::State<'_, Mutex<SettingsWriter>>,
    cookie_store: tauri::State<'_, Arc<CookieStoreMutex>>,
    app: tauri::AppHandle,
    username: String,
    password: String,
) -> Result<User, String> {
    let server_addr = settings_writer
        .lock()
        .map_err(|error| error.to_string())?
        .server_address();

    let res = client
        .inner()
        .login(&username, &password, &server_addr)
        .await
        .map_err(|error| error.to_string());

    let path = app
        .path()
        .app_cache_dir()
        .map_err(|error| error.to_string())?
        .join(COOKIE_FILE);

    // save cookies
    client
        .inner()
        .save_cookies(&path, cookie_store.inner())
        .map_err(|error| error.to_string())?;

    res
}

#[tauri::command]
pub async fn get_users(
    client: tauri::State<'_, Client>,
    settings_writer: tauri::State<'_, Mutex<SettingsWriter>>,
) -> Result<Vec<User>, String> {
    let server_addr = settings_writer
        .lock()
        .map_err(|error| error.to_string())?
        .server_address();

    client
        .inner()
        .get_users(&server_addr)
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
        .app_cache_dir()
        .map_err(|error| error.to_string())?
        .join(COOKIE_FILE);

    let file = File::create(&path).map_err(|error| error.to_string())?;
    let mut writer = BufWriter::new(file);
    cookie_store::serde::json::save(&store, &mut writer).map_err(|error| error.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn get_user(
    client: tauri::State<'_, Client>,
    settings_writer: tauri::State<'_, Mutex<SettingsWriter>>,
    user_id: i64
) -> Result<User, String> {
    let server_addr = settings_writer
        .lock()
        .map_err(|error| error.to_string())?
        .server_address();
    
    client
        .inner()
        .get_user(&user_id, &server_addr)
        .await
        .map_err(|error| error.to_string())
}
