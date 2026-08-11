use std::{io::Error, path::PathBuf};

use ulid::Ulid;

use crate::messages::{db::MessagesDB, models::Attachment};

pub async fn save_image(file_path: &PathBuf, message_db: &MessagesDB) -> Result<Attachment, Error> {
    let attachment_id = Ulid::generate();
    let file_location = file_path
        .to_str()
        .ok_or_else(|| Error::other("path is not valid str"))?;

    message_db
        .create_attachment(&attachment_id.to_string(), file_location)
        .await
        .map_err(|error| Error::other(error.to_string()))
}
