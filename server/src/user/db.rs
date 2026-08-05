use sqlx::SqlitePool;

use crate::user::models::User;

#[derive(Clone)]
pub struct UserDB {
    pool: SqlitePool,
}

impl UserDB {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
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

    pub async fn get_users(&mut self) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&self.pool)
            .await
    }
}
