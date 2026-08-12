use std::{fs, path::PathBuf, sync::Mutex};

use futures_util::TryFutureExt;
use reqwest::{
    multipart::{Form, Part},
    Error,
};

use tauri::Manager;

use protocol::{CHATS, DOWNLOAD, GET_MESSAGES, READ, UPLOAD};
use tokio::fs::{create_dir_all, write};

use crate::{
    constants::ATTACHMENTS_DIR, messages::models::{Attachment, Chat, ChatMessage, DownloadReq, GetMessagesReq, UpdateLastReadReq, User}, settings::{self, SettingsWriter},
};

#[derive(Debug)]
pub struct MessageClient {
    client: reqwest::Client,
}

impl MessageClient {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }

    pub async fn get_messages(
        &self,
        receiver_id: &i64,
        server_addr: &str,
    ) -> Result<Vec<ChatMessage>, Error> {
        let url = format!("http://{server_addr}{GET_MESSAGES}");

        let params = GetMessagesReq {
            receiver: *receiver_id,
        };

        self.client
            .get(url)
            .query(&params)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<ChatMessage>>()
            .await
    }

    pub async fn get_chats(&self, server_addr: &str) -> Result<Vec<Chat>, Error> {
        let url = format!("http://{server_addr}{GET_MESSAGES}{CHATS}");
        self.client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<Chat>>()
            .await
    }

    pub async fn upload_image(
        &self,
        file_name: &str,
        file: Vec<u8>,
        server_addr: &str,
    ) -> Result<Attachment, String> {
        let part = Part::bytes(file)
            .file_name(file_name.to_owned())
            .mime_str("image/png")
            .map_err(|error| error.to_string())?;
        let form = Form::new()
            .text("fileName", file_name.to_owned())
            .text("chunkNumber", "0")
            .text("totalChunks", "1")
            .part("chunk", part);

        let url = format!("http://{server_addr}{GET_MESSAGES}{UPLOAD}");
        self.client
            .post(url)
            .multipart(form)
            .send()
            .await
            .map_err(|error| error.to_string())?
            .error_for_status()
            .map_err(|error| error.to_string())?
            .json::<Attachment>()
            .await
            .map_err(|error| error.to_string())
    }

    #[allow(dead_code)]
    pub async fn download_image(
        &self,
        file_id: &str,
        server_addr: &str,
        image_dir: &PathBuf,
    ) -> Result<Attachment, String> {
        let params = DownloadReq {
            file_id: file_id.to_string(),
        };

        let url = format!("http://{server_addr}{GET_MESSAGES}{DOWNLOAD}");

        let resp = self
            .client
            .get(url)
            .query(&params)
            .send()
            .await
            .map_err(|error| error.to_string())?;

        let file_data = resp.bytes().await.map_err(|error| error.to_string())?;

        create_dir_all(image_dir)
            .await
            .map_err(|error| error.to_string())?;

        let image = image_dir.join(file_id);

        write(&image, &file_data)
            .await
            .map_err(|error| error.to_string())?;

        Ok(Attachment {
            id: file_id.to_string(),
        })
    }

    pub async fn update_read(
        &self,
        chat_id: &i64,
        read_count: &i64,
        server_addr: &str
    ) -> Result<(), reqwest::Error> {
        let url = format!("http://{server_addr}{GET_MESSAGES}{READ}");
        let body = UpdateLastReadReq {
            chat_id: *chat_id,
            read_count: *read_count
        };
        
        self.client
            .post(url)
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

#[tauri::command]
pub async fn get_messages(
    message_client: tauri::State<'_, MessageClient>,
    settings_writer: tauri::State<'_, Mutex<SettingsWriter>>,
    receiver_id: i64,
) -> Result<Vec<ChatMessage>, String> {
    let server_addr = settings_writer
        .lock()
        .map_err(|error| error.to_string())?
        .server_address();

    message_client
        .inner()
        .get_messages(&receiver_id, &server_addr)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn get_chats(
    message_client: tauri::State<'_, MessageClient>,
    settings_writer: tauri::State<'_, Mutex<SettingsWriter>>,
) -> Result<Vec<Chat>, String> {
    let server_addr = settings_writer
        .lock()
        .map_err(|error| error.to_string())?
        .server_address();

    message_client
        .inner()
        .get_chats(&server_addr)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn upload_image(
    request: tauri::ipc::Request<'_>,
    message_client: tauri::State<'_, MessageClient>,
    settings_writer: tauri::State<'_, Mutex<SettingsWriter>>,
) -> Result<Attachment, String> {
    let server_addr = settings_writer
        .lock()
        .map_err(|error| error.to_string())?
        .server_address();

    let tauri::ipc::InvokeBody::Raw(image_data) = request.body() else {
        return Err("No image data...".into());
    };

    let file_name = request
        .headers()
        .get("x-file-name")
        .and_then(|value| value.to_str().ok())
        .ok_or("Missing filename")?
        .to_string();

    message_client
        .inner()
        .upload_image(&file_name, image_data.clone(), &server_addr)
        .await
}

#[tauri::command]
pub async fn download_image(
    app: tauri::AppHandle,
    message_client: tauri::State<'_, MessageClient>,
    settings_writer: tauri::State<'_, Mutex<SettingsWriter>>,
    file_id: String,
) -> Result<Attachment, String> {
    let image_dir = app
        .path()
        .app_cache_dir()
        .map_err(|error| error.to_string())?
        .join(ATTACHMENTS_DIR);

    create_dir_all(&image_dir)
        .map_err(|error| error.to_string())
        .await?;

    let server_addr = settings_writer
        .lock()
        .map_err(|error| error.to_string())?
        .server_address();

    let image_path = image_dir.join(&file_id);

    if fs::exists(&image_path).map_err(|error| error.to_string())? {
        return Ok(Attachment { id: file_id });
    }

    message_client
        .inner()
        .download_image(&file_id, &server_addr, &image_dir)
        .await
}

#[tauri::command]
pub async fn update_read(
    message_client: tauri::State<'_, MessageClient>,
    settings_writer: tauri::State<'_, Mutex<SettingsWriter>>,
    chat_id: i64,
    read_count: i64
) -> Result<(), String> {
    let server_addr = settings_writer
        .lock()
        .map_err(|error| error.to_string())?
        .server_address();
    
    message_client
        .inner()
        .update_read(&chat_id, &read_count, &server_addr)
        .await
        .map_err(|error| error.to_string())?;
    Ok(())
}
