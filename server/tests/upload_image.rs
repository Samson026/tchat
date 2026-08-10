use axum::{
    body::Body,
    extract::{FromRequest, Multipart},
    http::{Request, StatusCode, header::CONTENT_TYPE},
};
use server::messages::upload_image;

#[tokio::test]
async fn uploads_a_single_image_chunk() {
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
    let response = upload_image(multipart).await;

    let upload_path = std::env::current_dir()
        .unwrap()
        .join("server_data/output")
        .join(file_name);
    let uploaded_image = tokio::fs::read(&upload_path).await;
    if upload_path.is_dir() {
        let _ = tokio::fs::remove_dir_all(&upload_path).await;
    } else {
        let _ = tokio::fs::remove_file(&upload_path).await;
    }

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        uploaded_image.expect("upload should assemble the image chunks into a file"),
        image
    );
}
