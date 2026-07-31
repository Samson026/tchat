use std::{collections::HashMap, sync::Arc};

use crate::{api::models::ChatMessage, db::Database};

use tokio::sync::{RwLock, mpsc};

type ConnectionRegistry =
    Arc<RwLock<HashMap<i64, mpsc::UnboundedSender<ChatMessage>>>>;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub connections: ConnectionRegistry
}

  impl AppState {
      pub fn new(db: Database) -> Self {
          Self {
              db,
              connections: Arc::new(RwLock::new(HashMap::new())),
          }
      }
  }