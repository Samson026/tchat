mod api;
mod db;
mod messages;
mod state;
mod user;
mod websocket;

use axum::Router;
use protocol::{SERVER_ADDRESS, WEBSOCKET_PATH};
use std::io;
use tokio::net::TcpListener;

use crate::{messages::db::MessagesDB, user::db::UserDB};

#[tokio::main]
async fn main() -> io::Result<()> {
    let db = db::Database::connect("tchat.db")
        .await
        .expect("Could not connect to db");

    let user_db = UserDB::new(db.pool.clone());
    let message_db = MessagesDB::new(db.pool.clone());
    let app_state = state::AppState::new(user_db, message_db);

    let app = Router::new()
        .merge(user::router())
        .merge(messages::router())
        .merge(websocket::router())
        .with_state(app_state);

    let listener = TcpListener::bind(SERVER_ADDRESS).await?;

    println!("server listening on ws://{SERVER_ADDRESS}{WEBSOCKET_PATH}");
    axum::serve(listener, app).await
}
