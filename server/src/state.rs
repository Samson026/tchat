use std::{collections::HashMap, sync::Arc};

use crate::{messages::db::MessagesDB, user::db::UserDB, websocket::ChatMessage};

use tokio::sync::{RwLock, mpsc};

type ConnectionRegistry = Arc<RwLock<HashMap<i64, mpsc::UnboundedSender<ChatMessage>>>>;

#[derive(Clone)]
pub struct AppState {
    pub userDB: UserDB,
    pub messageDB: MessagesDB,
    pub connections: ConnectionRegistry,
}

impl AppState {
    pub fn new(userDB: UserDB, messageDB: MessagesDB) -> Self {
        Self {
            userDB,
            messageDB,
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
