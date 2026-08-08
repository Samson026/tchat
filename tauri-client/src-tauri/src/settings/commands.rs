use std::sync::Mutex;

use crate::settings::settings_writer::{Settings, SettingsWriter};

#[tauri::command]
pub async fn update_settings(
    settings_writer: tauri::State<'_, Mutex<SettingsWriter>>,
    settings: Settings,
) -> Result<Settings, String> {
    let mut settings_w = settings_writer.lock().map_err(|error| error.to_string())?;

    match settings_w.update_settings(settings) {
        Ok(_) => Ok(settings_w.settings.clone()),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
pub async fn get_settings(
    settings_writer: tauri::State<'_, Mutex<SettingsWriter>>,
) -> Result<Settings, String> {
    let settings = settings_writer
        .lock()
        .map_err(|error| error.to_string())?
        .get_settings();

    Ok(settings)
}
