use crate::settings::settings::{self, Settings, SettingsWriter};

#[tauri::command]
pub async fn update_settings(
    settings_writer: tauri::State<'_, SettingsWriter>,
    settings: Settings,
) -> Result<Settings, String> {
    match settings_writer.inner().update_settings(&settings) {
        Ok(_) => Ok(settings),
        Err(error) => Err(error.to_string()),
    }
}
