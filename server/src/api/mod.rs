pub mod models;

use crate::{api::models::ChatMessage, db::User, state::AppState};
use axum::{
    Json,
    extract::{
        Query, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    http::StatusCode,
    response::{IntoResponse, Response},
};

use models::{ChatHistoryReq, LoginRequest, WebSocketParams};

pub async fn get_messages(
    State(mut app_state): State<AppState>,
    Query(params): Query<ChatHistoryReq>,
) -> Response {
    match app_state
        .db
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

pub async fn get_users(State(mut app_state): State<AppState>) -> Response {
    match app_state.db.get_users().await {
        Ok(users) => Json(users).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Messages not found").into_response(),
    }
}
