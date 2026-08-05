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
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer, cookie::time::Duration};

use crate::{messages::db::MessagesDB, user::db::UserDB};

#[tokio::main]
async fn main() -> io::Result<()> {
    let db = db::Database::connect("tchat.db")
        .await
        .expect("Could not connect to db");

    let user_db = UserDB::new(db.pool.clone());
    let message_db = MessagesDB::new(db.pool.clone());
    let app_state = state::AppState::new(user_db, message_db);
    let store = MemoryStore::default();
    let session =
        SessionManagerLayer::new(store).with_expiry(Expiry::OnInactivity(Duration::days(30)));

    let app = Router::new()
        .nest(GET_USERS, user::router())
        .nest(GET_MESSAGES, messages::router())
        .nest(WEBSOCKET_PATH, websocket::router())
        .nest(AUTH, auth::router())
        .layer(session)
        .with_state(app_state);

    let listener = TcpListener::bind(SERVER_ADDRESS).await?;

    println!("server listening on ws://{SERVER_ADDRESS}{WEBSOCKET_PATH}");
    axum::serve(listener, app).await
}
