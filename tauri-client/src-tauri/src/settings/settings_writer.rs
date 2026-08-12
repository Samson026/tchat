use std::{
    fs::{self, File},
    io::{BufWriter, Error},
    path::{Path, PathBuf},
};

use protocol::SERVER_ADDRESS;
use serde::{Deserialize, Serialize};

use crate::constants::SETTINGS_FILE;

pub struct SettingsWriter {
    file_location: PathBuf,
    pub settings: Settings,
}

impl SettingsWriter {
    pub fn new(data_dir: &Path) -> Result<Self, Error> {
        // create file if not exist
        let path = data_dir.join(SETTINGS_FILE);

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

        Ok(Self {
            settings,
            file_location: path,
        })
    }

    pub fn update_settings(&mut self, settings: Settings) -> Result<(), Error> {
        let file = File::create(&self.file_location)?;
        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, &settings)?;

        self.settings = settings;
        Ok(())
    }

    pub fn get_settings(&self) -> Settings {
        self.settings.clone()
    }

    pub fn server_address(&self) -> String {
        self.settings.server_address.clone()
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
