mod api;
mod db;
mod state;

use axum::{Router, routing::get, routing::post};
use protocol::{
    CREATE_USER_PATH, GET_MESSAGES, GET_USERS, LOGIN_PATH, SERVER_ADDRESS, WEBSOCKET_PATH,
};
use std::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let db = db::Database::connect("tchat.db")
        .await
        .expect("Could not connect to db");

    let app_state = state::AppState::new(db);

    let app = Router::new()
        .route(LOGIN_PATH, post(api::login))
        .route(CREATE_USER_PATH, post(api::create_user))
        .route(WEBSOCKET_PATH, get(api::upgrade))
        .route(GET_MESSAGES, get(api::get_messages))
        .route(GET_USERS, get(api::get_users))
        .with_state(app_state);

    let listener = TcpListener::bind(SERVER_ADDRESS).await?;

    println!("server listening on ws://{SERVER_ADDRESS}{WEBSOCKET_PATH}");
    axum::serve(listener, app).await
}
