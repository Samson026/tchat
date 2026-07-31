mod models;

use sqlx::Error;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::path::Path;

pub use models::{Message, User};

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn connect(db_path: &str) -> Result<Self, Error> {
        let path = Path::new(db_path);

        let options = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL UNIQUE
            )",
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                sender_id NOT NULL,
                receiver_id NOT NULL,
                content TEXT NOT NULL,
                time DATETIME DEFAULT CURRENT_TIMESTAMP,

                FOREIGN KEY (sender_id) REFERENCES users(id),
                FOREIGN KEY (receiver_id) REFERENCES users(id)
            )",
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }

    pub async fn add_user(&mut self, username: &str) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "INSERT INTO users (username)
                    VALUES(?)
            ",
        )
        .bind(username)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_user(&self, username: &str) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "
            SELECT * FROM users
            WHERE username == ?
        ",
        )
        .bind(username)
        .fetch_one(&self.pool)
        .await
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
        sender: &User,
        receiver: &User,
    ) -> Result<Vec<Message>, sqlx::Error> {
        sqlx::query_as::<_, Message>(
            "SELECT * FROM messages
            WHERE sender_id == ? AND receiver_id == ?",
        )
        .bind(sender.id)
        .bind(receiver.id)
        .fetch_all(&self.pool)
        .await
    }
}
