use crate::{user::Client, ws::WsState, messages::MessageClient};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod user;
mod ws;
mod messages;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let http_client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .expect("Failed to build http");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Client::new(http_client.clone()))
        .manage(WsState::new())
        .manage(MessageClient::new(http_client.clone()))
        .invoke_handler(tauri::generate_handler![greet,
            ws::commands::connect_ws,
            ws::commands::send,
            user::commands::create_user,
            user::commands::login,
            user::commands::get_users,
            messages::commands::get_messages,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
