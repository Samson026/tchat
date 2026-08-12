use sqlx::Error;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::path::Path;

#[derive(Clone)]
pub struct Database {
    pub pool: SqlitePool,
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
                username TEXT NOT NULL UNIQUE,
                password TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS attachments (
                id TEXT PRIMARY KEY NOT NULL UNIQUE,
                filelocation TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                chat_id NOT NULL,
                sender_id NOT NULL,
                receiver_id NOT NULL,
                content TEXT NOT NULL,
                attachment_id TEXT,
                time DATETIME DEFAULT CURRENT_TIMESTAMP,

                FOREIGN KEY (chat_id) REFERENCES chats(id)
                FOREIGN KEY (sender_id) REFERENCES users(id),
                FOREIGN KEY (receiver_id) REFERENCES users(id),
                FOREIGN KEY (attachment_id) REFERENCES attachments(id)
            )",
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS chats (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_1_id NOT NULL,
                user_2_id NOT NULL,
                user_1_read_count INTEGER,
                user_2_read_count INTEGER,

                CHECK (user_1_id < user_2_id),
                UNIQUE (user_1_id, user_2_id),
                FOREIGN KEY (user_1_id) REFERENCES users(id),
                FOREIGN KEY (user_2_id) REFERENCES users(id)
            )",
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }
}
