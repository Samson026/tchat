use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use tower_sessions::Session;

pub async fn auth_middleware(
    session: Session,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let user_id: Option<i64> = session
        .get::<i64>("user_id")
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match user_id {
        Some(id) => {
            request.extensions_mut().insert(id);
            Ok(next.run(request).await)
        }
        None => Err(StatusCode::UNAUTHORIZED),
    }
}
