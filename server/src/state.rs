use std::{collections::HashMap, sync::Arc};

use crate::{auth::db::AuthDB, messages::db::MessagesDB, user::db::UserDB, websocket::ChatMessage};

use tokio::sync::{RwLock, mpsc};

type ConnectionRegistry = Arc<RwLock<HashMap<i64, mpsc::UnboundedSender<ChatMessage>>>>;

#[derive(Clone)]
pub struct AppState {
    pub user_db: UserDB,
    pub message_db: MessagesDB,
    pub auth_db: AuthDB,
    pub connections: ConnectionRegistry,
}

impl AppState {
    pub fn new(user_db: UserDB, message_db: MessagesDB, auth_db: AuthDB) -> Self {
        Self {
            user_db,
            message_db,
            auth_db,
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
