use sqlx::SqlitePool;

use crate::messages::models::{Chat, Message, User};

#[derive(Clone)]
pub struct MessagesDB {
    pool: SqlitePool,
}

impl MessagesDB {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
    pub async fn add_message(
        &mut self,
        message: &str,
        sender: &i64,
        receiver: &i64,
    ) -> Result<(), sqlx::Error> {
        // add msg to db
        // check if chat exists
        let user_1_id = sender.min(receiver);
        let user_2_id = sender.max(receiver);

        let chat_id = match self.is_chat(user_1_id, user_2_id).await {
            Ok(Some(chat)) => chat.id,
            Ok(None) => {
                let c = self.create_chat(user_1_id, user_2_id).await?;
                c.id
            }
            Err(error) => {
                return Err(error);
            }
        };

        sqlx::query(
            "
            INSERT INTO messages (chat_id, sender_id, receiver_id, content)
            VALUES (?, ?, ?, ?)
        ",
        )
        .bind(chat_id)
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
            "SELECT id, chat_id, sender_id, receiver_id AS recv_id, content, time FROM messages
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

    pub async fn get_chats(&self, user_id: i64) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "
            SELECT
                CASE
                    WHEN u1.id = ? THEN u2.id
                    ELSE u1.id
                END AS id,
                CASE
                    WHEN u1.id = ? THEN u2.username
                    ELSE u1.username
                END AS username
            FROM chats
            JOIN users u1 ON chats.user_1_id = u1.id
            JOIN users u2 ON chats.user_2_id = u2.id
            WHERE u1.id = ? OR u2.id = ?
            ",
        )
        .bind(user_id)
        .bind(user_id)
        .bind(user_id)
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn is_chat(
        &self,
        user_1_id: &i64,
        user_2_id: &i64,
    ) -> Result<Option<Chat>, sqlx::Error> {
        sqlx::query_as::<_, Chat>(
            "
                SELECT * FROM chats
                WHERE user_1_id = ? AND user_2_id = ?
            ",
        )
        .bind(user_1_id)
        .bind(user_2_id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn create_chat(
        &mut self,
        user_id: &i64,
        receiver_id: &i64,
    ) -> Result<Chat, sqlx::Error> {
        sqlx::query_as::<_, Chat>(
            "
                INSERT INTO chats (user_1_id, user_2_id)
                VALUES (?, ?)
                RETURNING id, user_1_id, user_2_id
            ",
        )
        .bind(user_id)
        .bind(receiver_id)
        .fetch_one(&self.pool)
        .await
    }
}
