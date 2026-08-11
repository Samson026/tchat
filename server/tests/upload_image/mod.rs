use axum::{
    body::Body,
    extract::{FromRequest, Multipart, State},
    http::{Request, StatusCode, header::CONTENT_TYPE},
};
use sqlx::sqlite::SqlitePoolOptions;

use super::upload_image;
use crate::{auth::db::AuthDB, messages::db::MessagesDB, state::AppState, user::db::UserDB};

#[tokio::test]
async fn uploads_a_single_image_chunk() {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::query(
        "CREATE TABLE attachments (
            id TEXT PRIMARY KEY NOT NULL UNIQUE,
            filelocation TEXT NOT NULL UNIQUE
        )",
    )
    .execute(&pool)
    .await
    .unwrap();
    let state = AppState::new(
        UserDB::new(pool.clone()),
        MessagesDB::new(pool.clone()),
        AuthDB::new(pool),
    );

    let boundary = "upload-test-boundary";
    let file_name = format!("upload-test-{}.png", std::process::id());
    let image = b"test image bytes";
    let mut body = format!(
        "--{boundary}\r\n\
         Content-Disposition: form-data; name=\"fileName\"\r\n\r\n\
         {file_name}\r\n\
         --{boundary}\r\n\
         Content-Disposition: form-data; name=\"chunkNumber\"\r\n\r\n\
         0\r\n\
         --{boundary}\r\n\
         Content-Disposition: form-data; name=\"totalChunks\"\r\n\r\n\
         1\r\n\
         --{boundary}\r\n\
         Content-Disposition: form-data; name=\"chunk\"; filename=\"{file_name}\"\r\n\
         Content-Type: image/png\r\n\r\n"
    )
    .into_bytes();
    body.extend_from_slice(image);
    body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());

    let request = Request::builder()
        .header(
            CONTENT_TYPE,
            format!("multipart/form-data; boundary={boundary}"),
        )
        .body(Body::from(body))
        .unwrap();
    let multipart = Multipart::from_request(request, &()).await.unwrap();
    let response = upload_image(State(state), multipart).await;

    let upload_path = std::env::current_dir()
        .unwrap()
        .join("server_data/output")
        .join(file_name);
    let uploaded_image = tokio::fs::read(&upload_path).await;
    let _ = tokio::fs::remove_file(&upload_path).await;

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        uploaded_image.expect("upload should assemble the image chunks into a file"),
        image
    );
}
