use std::{
    fs::OpenOptions,
    io::{Error, Write},
    path::PathBuf,
};

use axum::{
    Extension, Json, Router,
    extract::{DefaultBodyLimit, Multipart, Path, Query, State},
    http::{StatusCode, header},
    middleware,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use protocol::{BASE_ROUTE, CHATS, DOWNLOAD, READ, RECEIVER_ID_PARAM, UPLOAD};
use tokio::{
    fs::{self, File as AsyncFile, create_dir_all},
    io::AsyncWriteExt,
};

use crate::{
    messages::{
        models::{
            AttachmentUser, ChatHistoryReq, ChatMessage, DownloadReq, GetChatByIdParams,
            UpdateLastReadReq,
        },
        service::save_image,
    },
    middleware::auth_middleware,
    path::get_app_dir,
    state::AppState,
};

#[cfg(test)]
#[path = "../../tests/upload_image/mod.rs"]
mod tests;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(BASE_ROUTE, get(get_messages))
        .route(CHATS, get(get_chats))
        .route(UPLOAD, post(upload_image))
        .route(DOWNLOAD, get(download_image))
        .route(READ, post(update_last_read_message))
        .route(
            &format!("{CHATS}{RECEIVER_ID_PARAM}"),
            get(get_chat_from_ids),
        )
        .route_layer(middleware::from_fn(auth_middleware))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024))
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
                    chat_id: msg.chat_id,
                    sender_id: msg.sender_id,
                    recv_id: msg.recv_id,
                    content: msg.content,
                    attachment: msg.attachment,
                })
                .collect::<Vec<_>>(),
        )
        .into_response(),
        Err(error) => {
            eprintln!("Error: {error}");
            (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
        }
    }
}

// return users who are being chatted with
pub async fn get_chats(
    State(app_state): State<AppState>,
    Extension(user_id): Extension<i64>,
) -> Response {
    match app_state.message_db.get_chats(&user_id).await {
        Ok(chats) => Json(chats).into_response(),
        Err(error) => {
            eprintln!("Error: {error}");
            (StatusCode::NOT_FOUND, error.to_string()).into_response()
        }
    }
}

pub async fn upload_image(State(app_state): State<AppState>, mut mutlipart: Multipart) -> Response {
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
            "chunkNumber" => {
                chunk_number = field.text().await.unwrap_or_default().parse().unwrap_or(0)
            }
            "totalChunks" => {
                total_chunks = field.text().await.unwrap_or_default().parse().unwrap_or(0)
            }
            "chunk" => chunk_data = field.bytes().await.unwrap_or_default().to_vec(),
            _ => {}
        }
    }
    if file_name.is_empty() || chunk_data.is_empty() {
        return StatusCode::BAD_REQUEST.into_response();
    }

    let temp_dir = match get_app_dir().await {
        Ok(path) => path,
        Err(_) => {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };
    let temp_dir = temp_dir.join(&file_name);
    let _ = create_dir_all(&temp_dir)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response());
    let chunk_path = temp_dir.join(chunk_number.to_string());
    let mut file = match AsyncFile::create(&chunk_path).await {
        Ok(file) => file,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if file.write_all(&chunk_data).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    if is_upload_complete(&temp_dir, total_chunks)
        && let Ok(file) = assemble_file(&temp_dir, &file_name, total_chunks).await
    {
        match save_image(&file, &app_state.message_db).await {
            Ok(attachment) => {
                return Json(AttachmentUser::from(attachment)).into_response();
            }
            Err(error) => {
                eprintln!("error {error}");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };
    }
    StatusCode::OK.into_response()
}

fn is_upload_complete(temp_dir: &PathBuf, total_chunks: usize) -> bool {
    match std::fs::read_dir(temp_dir) {
        Ok(entries) => entries.count() == total_chunks,
        Err(_) => false,
    }
}

async fn assemble_file(
    temp_dir: &PathBuf,
    file_name: &str,
    total_chunks: usize,
) -> Result<PathBuf, Error> {
    let mut output_path = match get_app_dir().await {
        Ok(path) => path,
        Err(error) => return Err(error),
    };
    output_path = output_path.join("output");
    create_dir_all(&output_path).await?;

    let mut output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output_path.join(file_name))?;

    for chunk_number in 0..total_chunks {
        let chunk_path = temp_dir.join(chunk_number.to_string());
        let chunk_data = std::fs::read(&chunk_path)?;
        output_file.write_all(&chunk_data)?;
    }
    fs::remove_dir_all(temp_dir).await?;
    Ok(output_path.join(file_name))
}

pub async fn download_image(
    State(app_state): State<AppState>,
    Query(params): Query<DownloadReq>,
) -> Response {
    let attachment = match app_state.message_db.get_attachment(&params.file_id).await {
        Ok(attachment) => attachment,
        Err(_) => {
            return StatusCode::BAD_REQUEST.into_response();
        }
    };

    let file_path = PathBuf::from(attachment.filelocation);

    let file = match fs::read(&file_path).await {
        Ok(file) => file,
        Err(error) => {
            println!("this error");
            eprintln!("Error: {error}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let extension = file_path
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    let content_type = match extension.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        _ => "application/octet-stream",
    };

    ([(header::CONTENT_TYPE, content_type)], file).into_response()
}

pub async fn update_last_read_message(
    State(app_state): State<AppState>,
    Extension(user_id): Extension<i64>,
    Json(data): Json<UpdateLastReadReq>,
) -> Response {
    match app_state
        .message_db
        .set_read_message(&data.chat_id, &user_id, &data.read_count)
        .await
    {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_chat_from_ids(
    State(app_state): State<AppState>,
    Extension(user_id): Extension<i64>,
    Path(params): Path<GetChatByIdParams>,
) -> Response {
    let (user_1, user_2) = if user_id < params.receiver_id {
        (user_id, params.receiver_id)
    } else {
        (params.receiver_id, user_id)
    };

    match app_state.message_db.get_chat_by_ids(&user_1, &user_2).await {
        Ok(chat) => Json(chat).into_response(),
        Err(sqlx::Error::RowNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
