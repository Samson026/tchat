use std::{collections::HashMap, sync::Arc};

use crate::{messages::db::MessagesDB, user::db::UserDB, websocket::ChatMessage};

use tokio::sync::{RwLock, mpsc};

type ConnectionRegistry = Arc<RwLock<HashMap<i64, mpsc::UnboundedSender<ChatMessage>>>>;

#[derive(Clone)]
pub struct AppState {
    pub user_db: UserDB,
    pub message_db: MessagesDB,
    pub connections: ConnectionRegistry,
}

impl AppState {
    pub fn new(user_db: UserDB, message_db: MessagesDB) -> Self {
        Self {
            user_db,
            message_db,
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
