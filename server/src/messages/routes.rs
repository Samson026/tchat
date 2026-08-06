use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::get,
};
use protocol::{BASE_ROUTE, CHATS};

use crate::{
    messages::models::{ChatHistoryReq, ChatMessage, ChatsReq},
    middleware::auth_middleware,
    state::AppState,
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
) -> Response {
    match app_state
        .message_db
        .get_messages(&params.sender_id, &params.recv_id)
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
    Query(params): Query<ChatsReq>,
) -> Response {
    match app_state.message_db.get_chats(params.user_id).await {
        Ok(users) => Json(users).into_response(),
        Err(error) => (StatusCode::NOT_FOUND, error.to_string()).into_response(),
    }
}
