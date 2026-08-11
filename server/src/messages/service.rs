use std::{fs::File, io::Error, path::PathBuf};

use crate::messages::db::MessagesDB;

pub async fn save_image(file_path: &PathBuf, message_db: &MessagesDB) -> Result<i64, Error> {
    
    todo!()
}
