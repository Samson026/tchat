use std::{fs::{OpenOptions, read_dir}, io::{Error, Write}, path::PathBuf};

use axum::{
    Extension, Json, Router, extract::{Multipart, Query, State}, http::StatusCode, middleware, response::{IntoResponse, Response}, routing::get,
};
use protocol::{BASE_ROUTE, CHATS};
use tokio::{fs::{self, File, create_dir_all}, io::AsyncWriteExt};

use crate::{
    messages::models::{ChatHistoryReq, ChatMessage}, middleware::auth_middleware, path::get_app_dir, state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(BASE_ROUTE, get(get_messages))
        .route(CHATS, get(get_chats))
        .route_layer(middleware::from_fn(auth_middleware))
}

pub async fn get_messages(
    State(mut app_state): State<AppState>,
    Query(params): Query<ChatHistoryReq>,
    Extension(user_id): Extension<i64>,
) -> Response {
    match app_state
        .message_db
        .get_messages(&user_id, &params.receiver)
        .await
    {
        Ok(message) => Json(
            message
                .into_iter()
                .map(|msg| ChatMessage {
                    sender_id: msg.sender_id,
                    recv_id: msg.recv_id,
                    content: msg.content,
                })
                .collect::<Vec<_>>(),
        )
        .into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Messages not found").into_response(),
    }
}

// return users who are being chatted with
pub async fn get_chats(
    State(app_state): State<AppState>,
    Extension(user_id): Extension<i64>,
) -> Response {
    match app_state.message_db.get_chats(&user_id).await {
        Ok(users) => Json(users).into_response(),
        Err(error) => (StatusCode::NOT_FOUND, error.to_string()).into_response(),
    }
}

pub async fn upload_image(
    State(app_state): State<AppState>,
    mut mutlipart: Multipart
) -> Response {
    let file_name = String::new();
    let mut file_name = String::new();
    let mut chunk_number = 0;
    let mut total_chunks = 0;
    let mut chunk_data = Vec::new();
    
    while let Some(field) = match mutlipart.next_field().await {
        Ok(f) => f,
        Err(error) => {
            eprintln!("Error: {error}");
            return StatusCode::BAD_REQUEST.into_response();
        }
    } {
        let field_name = field.name().unwrap_or_default().to_string();
            match field_name.as_str() {
            "fileName" => file_name = field.text().await.unwrap_or_default(),
            "chunkNumber" => chunk_number = field.text().await.unwrap_or_default().parse().unwrap_or(0),
            "totalChunks" => total_chunks = field.text().await.unwrap_or_default().parse().unwrap_or(0),
            "chunk" => chunk_data = field.bytes().await.unwrap_or_default().to_vec(),
            _ => {}
        }
    }
    if file_name.is_empty() || chunk_data.is_empty() {
        return StatusCode::BAD_REQUEST.into_response()
    }

    let temp_dir = match get_app_dir().await {
        Ok(path) => path,
        Err(_) => {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };
    let temp_dir = temp_dir.join(&file_name);
    create_dir_all(&temp_dir).await
        .map_err(|_| return StatusCode::INTERNAL_SERVER_ERROR.into_response());
    let chunk_path = temp_dir.join(chunk_number.to_string());
    let mut file = match File::create(&chunk_path).await {
        Ok(file) => file,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if file.write_all(&chunk_data).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }

    if is_upload_complete(&temp_dir, total_chunks) {
        assemble_file(&temp_dir, &file_name, total_chunks).await;
    }
    StatusCode::OK.into_response()
}

fn is_upload_complete(temp_dir: &PathBuf, total_chunks: usize) -> bool {
    match std::fs::read_dir(temp_dir) {
        Ok(entries) =>  entries.count() == total_chunks,
        Err(_) => false
    }
}

async fn assemble_file(temp_dir: &PathBuf, file_name: &str, total__chunks: usize) -> Result<(), Error> {
    let output_path = match get_app_dir().await {
        Ok(path) => path,
        Err(error) => {
            return Err(error);
        }
    };
    let mut output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&output_path)?;

    for chunk_number in 0..total__chunks {
        let chunk_path = temp_dir.join(chunk_number.to_string());
        let chunk_data = std::fs::read(&chunk_path)?;
        output_file.write_all(&chunk_data)?;
    }
    fs::remove_dir_all(temp_dir).await?;
    Ok(())
}
