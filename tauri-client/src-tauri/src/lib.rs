use std::sync::Mutex;

use crate::{
    auth::AuthClient, messages::MessageClient, settings::SettingsWriter, user::Client, ws::WsState,
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod auth;
mod messages;
mod settings;
mod user;
mod ws;

use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Load an existing set of cookies, serialized as json, if it is available

    tauri::Builder::default()
        .setup(|app| {
            let data_path = app.path().app_data_dir()?;
            let cookie_path = data_path.join("cookies.json");

            std::fs::create_dir_all(cookie_path.parent().expect("Cookie path has no parent"))?;

            println!("Cookie path: {}", cookie_path.display());

            let cookie_store = {
                if let Ok(file) = std::fs::File::open(&cookie_path).map(std::io::BufReader::new) {
                    cookie_store::serde::json::load(file).unwrap()
                } else {
                    reqwest_cookie_store::CookieStore::new()
                }
            };
            let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
            let cookie_store = std::sync::Arc::new(cookie_store);

            let http_client = reqwest::Client::builder()
                .cookie_provider(std::sync::Arc::clone(&cookie_store))
                .build()
                .expect("Failed to build http");

            let settings_writer = SettingsWriter::new(&data_path)?;

            app.manage(Client::new(http_client.clone()));
            app.manage(MessageClient::new(http_client.clone()));
            app.manage(AuthClient::new(http_client));
            app.manage(cookie_store);
            app.manage(Mutex::new(settings_writer));

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .manage(WsState::new())
        .invoke_handler(tauri::generate_handler![
            greet,
            ws::commands::connect_ws,
            ws::commands::send,
            user::commands::create_user,
            user::commands::login,
            user::commands::get_users,
            user::commands::logout,
            messages::commands::get_messages,
            messages::commands::get_chats,
            auth::commands::auth,
            settings::commands::update_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
