#[path = "../src/api/mod.rs"]
mod api;
#[path = "../src/db/mod.rs"]
mod db;
#[path = "../src/state.rs"]
mod state;

use axum::{Router, routing::{get, post}};
use futures_util::{SinkExt, StreamExt};
use protocol::{CREATE_USER_PATH, LOGIN_PATH, WEBSOCKET_PATH};
use sqlx::sqlite::SqlitePoolOptions;
use std::{fs, time::{Duration, SystemTime, UNIX_EPOCH}};
use tokio::{net::TcpListener, task::JoinHandle, time::{sleep, timeout}};
use tokio_tungstenite::{connect_async, tungstenite::Message};

fn test_db_path() -> String {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_nanos();

    std::env::temp_dir()
        .join(format!("tchat-server-test-{unique}.db"))
        .to_string_lossy()
        .into_owned()
}

async fn spawn_test_server() -> (JoinHandle<()>, String, i64, i64, String, String) {
    let db_path = test_db_path();
    let db = db::Database::connect(&db_path)
        .await
        .expect("could not connect test db");

    let sender_id = insert_user(&db_path, "sender-test-user")
        .await
        .expect("could not create sender user");
    let receiver_id = insert_user(&db_path, "receiver-test-user")
        .await
        .expect("could not create receiver user");

    let app_state = state::AppState::new(db);

    let app = Router::new()
        .route(LOGIN_PATH, post(api::login))
        .route(CREATE_USER_PATH, post(api::create_user))
        .route(WEBSOCKET_PATH, get(api::upgrade))
        .with_state(app_state);

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("could not bind test listener");
    let addr = listener
        .local_addr()
        .expect("could not read listener addr");

    let server = tokio::spawn(async move {
        axum::serve(listener, app)
            .await
            .expect("test server failed");
    });

    sleep(Duration::from_millis(50)).await;

    (
        server,
        db_path,
        sender_id,
        receiver_id,
        format!("ws://{addr}{WEBSOCKET_PATH}?user_id={sender_id}"),
        format!("ws://{addr}{WEBSOCKET_PATH}?user_id={receiver_id}"),
    )
}

async fn insert_user(db_path: &str, username: &str) -> Result<i64, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&format!("sqlite:{db_path}"))
        .await?;

    let result = sqlx::query("INSERT INTO users (username) VALUES (?)")
        .bind(username)
        .execute(&pool)
        .await?;

    pool.close().await;

    Ok(result.last_insert_rowid())
}

#[tokio::test]
async fn sending_a_message_delivers_it_to_the_other_user() {
    let (server, db_path, sender_id, receiver_id, sender_url, receiver_url) =
        spawn_test_server().await;

    let result = async {
        let (mut receiver_socket, _) = connect_async(&receiver_url)
            .await
            .expect("receiver failed to connect");
        sleep(Duration::from_millis(50)).await;

        let (mut sender_socket, _) = connect_async(&sender_url)
            .await
            .expect("sender failed to connect");
        sleep(Duration::from_millis(50)).await;

        let sent_message = api::models::ChatMessage {
            sender_id,
            recv_id: receiver_id,
            content: "integration message".to_string(),
        };

        sender_socket
            .send(Message::text(
                serde_json::to_string(&sent_message)
                    .expect("could not serialize sent message"),
            ))
            .await
            .expect("sender failed to send message");

        let received = timeout(Duration::from_secs(5), receiver_socket.next())
            .await
            .expect("timed out waiting for receiver message")
            .expect("receiver socket closed unexpectedly")
            .expect("receiver websocket returned an error");

        let Message::Text(text) = received else {
            panic!("receiver got a non-text websocket message");
        };

        let delivered: api::models::ChatMessage = serde_json::from_str(&text)
            .expect("could not deserialize delivered message");

        assert_eq!(delivered.sender_id, sent_message.sender_id);
        assert_eq!(delivered.recv_id, sent_message.recv_id);
        assert_eq!(delivered.content, sent_message.content);

        let _ = sender_socket.close(None).await;
        let _ = receiver_socket.close(None).await;
    }
    .await;

    server.abort();
    let _ = server.await;
    let _ = fs::remove_file(&db_path);

    result
}
