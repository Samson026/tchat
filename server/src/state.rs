use std::{collections::HashMap, sync::Arc};

use crate::{api::models::ChatMessage, user::db::UserDB};

use tokio::sync::{RwLock, mpsc};

type ConnectionRegistry = Arc<RwLock<HashMap<i64, mpsc::UnboundedSender<ChatMessage>>>>;

#[derive(Clone)]
pub struct AppState {
    pub userDB: UserDB,
    pub connections: ConnectionRegistry,
}

impl AppState {
    pub fn new(userDB: UserDB) -> Self {
        Self {
            userDB,
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
