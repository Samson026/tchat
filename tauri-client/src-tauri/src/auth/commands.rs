use std::sync::Mutex;

use protocol::AUTH;
use reqwest::Error;
use tauri::State;

use crate::{auth::models::User, settings::SettingsWriter};

pub struct AuthClient {
    client: reqwest::Client,
}

impl AuthClient {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }

    pub async fn auth(&self, server_addr: &str) -> Result<User, Error> {
        let url = format!("http://{server_addr}{AUTH}");
        self.client
            .post(url)
            .send()
            .await?
            .error_for_status()?
            .json::<User>()
            .await
    }
}

#[tauri::command]
pub async fn auth(
    state: State<'_, AuthClient>,
    settings_writer: State<'_, Mutex<SettingsWriter>>,
) -> Result<User, String> {
    let server_addr = settings_writer
        .lock()
        .map_err(|error| error.to_string())?
        .server_address();

    state
        .inner()
        .auth(&server_addr)
        .await
        .map_err(|error| error.to_string())
}
