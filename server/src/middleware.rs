use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use tower_sessions::Session;

pub async fn auth_middleware(
    session: Session,
    request: Request,
    next: Next
) -> Result<Response, StatusCode> {
    let user_id: Option<i64> = session
        .get::<i64>("user_id")
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if (user_id.is_none()) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(request).await)
}