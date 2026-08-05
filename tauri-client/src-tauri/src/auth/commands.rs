use protocol::{AUTH, SERVER_ADDRESS};
use reqwest::Error;
use tauri::State;

pub struct AuthClient {
    client: reqwest::Client,
}

impl AuthClient {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client: client }
    }

    pub async fn auth(&self) -> Result<(), Error> {
        let url = format!("http://{SERVER_ADDRESS}{AUTH}");
        self.client
            .post(url)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }
}

#[tauri::command]
pub async fn auth(state: State<'_, AuthClient>) -> Result<(), String> {
    state
        .inner()
        .auth()
        .await
        .map_err(|error| error.to_string())
}
