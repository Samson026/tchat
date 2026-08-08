use std::{
    fs::{self, File},
    io::{BufWriter, Error},
    path::PathBuf,
};

use protocol::SERVER_ADDRESS;
use serde::{Deserialize, Serialize};

pub struct SettingsWriter {
    file_location: PathBuf,
    pub settings: Settings,
}

impl SettingsWriter {
    pub fn new(data_dir: &PathBuf) -> Result<Self, Error> {
        // create file if not exist
        let path = data_dir.join("settings.json");

        if !path.exists() {
            let file = File::create(&path)?;
            let writter = BufWriter::new(file);
            let settings = Settings::new();

            serde_json::to_writer_pretty(writter, &settings)?;

            return Ok(Self {
                file_location: path,
                settings,
            });
        }

        // read from file
        let json = fs::read_to_string(&path)?;
        let settings: Settings = serde_json::from_str(&json)?;

        return Ok(Self {
            settings,
            file_location: path,
        });
    }

    pub fn update_settings(&mut self, settings: Settings) -> Result<(), Error> {
        let file = File::create(&self.file_location)?;
        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, &settings)?;

        self.settings = settings;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    pub server_address: String,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            server_address: SERVER_ADDRESS.to_string(),
        }
    }
}
