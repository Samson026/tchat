mod api;
mod db;
mod state;
pub mod user;

use axum::{Router, routing::get};
use protocol::{GET_MESSAGES, SERVER_ADDRESS, WEBSOCKET_PATH};
use std::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let db = db::Database::connect("tchat.db")
        .await
        .expect("Could not connect to db");

    let app_state = state::AppState::new(db);

    let app = Router::new()
        .merge(user::router())
        .route(WEBSOCKET_PATH, get(api::upgrade))
        .route(GET_MESSAGES, get(api::get_messages))
        .with_state(app_state);

    let listener = TcpListener::bind(SERVER_ADDRESS).await?;

    println!("server listening on ws://{SERVER_ADDRESS}{WEBSOCKET_PATH}");
    axum::serve(listener, app).await
}
