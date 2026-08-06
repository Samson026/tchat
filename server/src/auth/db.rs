use sqlx::SqlitePool;

use crate::auth::models::User;


#[derive(Clone)]
pub struct AuthDB {
    pool: SqlitePool,
}

impl AuthDB {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_user(&self, user_id: &i64) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "
            SELECT id, username FROM users
            WHERE id == ?
        ",
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
    }
}
