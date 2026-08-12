use sqlx::{Error, SqlitePool};

use crate::messages::models::{Attachment, Chat, ClientChat, Message, User};

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
        attachment: Option<&str>,
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
            INSERT INTO messages (chat_id, sender_id, receiver_id, content, attachment_id)
            VALUES (?, ?, ?, ?, ?)
        ",
        )
        .bind(chat_id)
        .bind(sender)
        .bind(receiver)
        .bind(message)
        .bind(attachment)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
    pub async fn get_messages(
        &mut self,
        user_1: &i64,
        user_2: &i64,
    ) -> Result<Vec<Message>, sqlx::Error> {
        sqlx::query_as::<_, Message>(
            "SELECT id, chat_id, sender_id, receiver_id AS recv_id, content, attachment_id AS attachment, time FROM messages
            WHERE (sender_id = ? AND receiver_id = ?)
                OR (sender_id = ? AND receiver_id = ?)
            ORDER BY time ASC, id ASC",
        )
        .bind(user_1)
        .bind(user_2)
        .bind(user_2)
        .bind(user_1)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_chats(&self, user_id: &i64) -> Result<Vec<ClientChat>, sqlx::Error> {
        sqlx::query_as::<_, ClientChat>(
            "
            SELECT
                chats.id,
                CASE
                    WHEN u1.id = ? THEN u2.id
                    ELSE u1.id
                END AS user_id,
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

    pub async fn create_attachment(
        &self,
        attachment_id: &str,
        file_location: &str,
    ) -> Result<Attachment, sqlx::Error> {
        sqlx::query_as::<_, Attachment>(
            "
                INSERT INTO attachments (id, filelocation)
                VALUES (?, ?)
                RETURNING id, filelocation
            ",
        )
        .bind(attachment_id)
        .bind(file_location)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_attachment(&self, file_id: &str) -> Result<Attachment, Error> {
        sqlx::query_as::<_, Attachment>(
            "SELECT * FROM attachments
            WHERE id = ? 
            ",
        )
        .bind(file_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn set_read_message(&self, chat_id: &i64, user_id: &i64, last_read_message_id: &i64) -> Result<(), sqlx::Error> {
        sqlx::query(
            "
                UPDATE chats
                SET 
                    user_1_last_read_id = CASE
                        WHEN user_1_id = ? THEN ?
                        ELSE user_1_last_read_id
                    END,
                    user_2_last_read_id = CASE
                        WHEN user_2_id = ? THEN ?
                        ELSE user_2_last_read_id
                    END
                WHERE id = ?
            ",
        )
        .bind(user_id)
        .bind(last_read_message_id)
        .bind(user_id)
        .bind(last_read_message_id)
        .bind(chat_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
