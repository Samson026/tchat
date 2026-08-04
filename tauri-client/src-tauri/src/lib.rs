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
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Client::new())
        .manage(WsState::new())
        .manage(MessageClient::new())
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
