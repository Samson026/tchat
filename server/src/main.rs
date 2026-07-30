mod api;
mod db;

use axum::{Router, routing::get, routing::post};
use protocol::{LOGIN_PATH, SERVER_ADDRESS, WEBSOCKET_PATH, CREATE_USER_PATH};
use std::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let db = db::Database::connect("tchat.db")
        .await
        .expect("Could not connect to db");

    let app = Router::new()
        .route(LOGIN_PATH, post(api::login))
        .with_state(db)
        .route(CREATE_USER_PATH, post(api::create_user))
        .with_state(db)
        .route(WEBSOCKET_PATH, get(api::upgrade));

    let listener = TcpListener::bind(SERVER_ADDRESS).await?;

    println!("server listening on ws://{SERVER_ADDRESS}{WEBSOCKET_PATH}");
    axum::serve(listener, app).await
}
