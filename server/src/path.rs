use std::{io::{Error}, path::PathBuf};

use tokio::fs::create_dir_all;

pub async fn get_app_dir() -> Result<PathBuf, Error> {
    let path = match std::env::current_dir() {
        Ok(path) => path,
        Err(error) => return Err(error),
    }
    .join("server_data/tmp");

    match create_dir_all(&path).await {
        Ok(_) => Ok(path),
        Err(error) => Err(error)
    }
}
