use sqlx::SqlitePool;

use crate::messages::models::Message;

#[derive(Clone)]
pub struct MessagesDB {
    pool: SqlitePool,
}

impl MessagesDB {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool: pool }
    }
    pub async fn add_message(
        &mut self,
        message: &str,
        sender: &i64,
        receiver: &i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "
            INSERT INTO messages (sender_id, receiver_id, content)
            VALUES (?, ?, ?)
        ",
        )
        .bind(sender)
        .bind(receiver)
        .bind(message)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
    pub async fn get_messages(
        &mut self,
        sender_id: &i64,
        receiver_id: &i64,
    ) -> Result<Vec<Message>, sqlx::Error> {
        sqlx::query_as::<_, Message>(
            "SELECT id, sender_id, receiver_id AS recv_id, content, time FROM messages
            WHERE (sender_id = ? AND receiver_id = ?)
                OR (sender_id = ? AND receiver_id = ?)
            ORDER BY time ASC, id ASC",
        )
        .bind(sender_id)
        .bind(receiver_id)
        .bind(receiver_id)
        .bind(sender_id)
        .fetch_all(&self.pool)
        .await
    }
}
