mod auth;
mod db;
mod messages;
mod middleware;
mod state;
mod user;
mod websocket;

use axum::Router;
use protocol::{AUTH, GET_MESSAGES, GET_USERS, SERVER_ADDRESS, WEBSOCKET_PATH};
use std::io;
use tokio::net::TcpListener;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer, cookie::time::Duration};
use tracing::Level;

use crate::{auth::db::AuthDB, messages::db::MessagesDB, user::db::UserDB};

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let db = db::Database::connect("tchat.db")
        .await
        .expect("Could not connect to db");

    let user_db = UserDB::new(db.pool.clone());
    let message_db = MessagesDB::new(db.pool.clone());
    let auth_db = AuthDB::new(db.pool.clone());
    let app_state = state::AppState::new(user_db, message_db, auth_db);
    let store = MemoryStore::default();
    let session = SessionManagerLayer::new(store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(30)));

    let app = Router::new()
        .nest(GET_USERS, user::router())
        .nest(GET_MESSAGES, messages::router())
        .nest(WEBSOCKET_PATH, websocket::router())
        .nest(AUTH, auth::router())
        .layer(session)
        .layer(
            TraceLayer::new_for_http()
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(app_state);

    let listener = TcpListener::bind(SERVER_ADDRESS).await?;

    println!("server listening on ws://{SERVER_ADDRESS}{WEBSOCKET_PATH}");
    axum::serve(listener, app).await
}
