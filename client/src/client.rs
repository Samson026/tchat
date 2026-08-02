use std::io::Error;

use protocol::SERVER_URL;

use crate::api::{
    Client,
    models::{ChatMessage, User},
};
use crate::ws::WebSocketConnection;

#[derive(Debug)]
pub struct ClientApp {
    client: Client,
    ws: Option<WebSocketConnection>,
    pub user: Option<User>,
}

impl ClientApp {
    pub async fn new() -> Result<Self, Error> {
        let client = Client::new().await.map_err(Error::other)?;

        Ok(Self {
            client,
            ws: None,
            user: None,
        })
    }

    pub async fn login(&mut self, username: &str) -> Result<(), Error> {
        match self.client.login(username).await {
            Ok(user) => {
                self.user = Some(user);
                Ok(())
            }
            Err(error) => Err(std::io::Error::other(error)),
        }
    }

    pub async fn create_user(&mut self, username: &str) -> Result<(), Error> {
        match self.client.create_user(username).await {
            Ok(user) => {
                self.user = Some(user);
                Ok(())
            }
            Err(error) => Err(std::io::Error::other(error)),
        }
    }

    pub async fn connect_ws(&mut self) -> Result<(), Error> {
        let user = self.user.as_ref().unwrap();
        let url = format!("{SERVER_URL}?user_id={}", user.id);
        self.ws = Some(
            WebSocketConnection::connect(&url)
                .await
                .map_err(Error::other)?,
        );
        Ok(())
    }

    pub fn websocket_connected(&self) -> bool {
        self.ws.is_some()
    }

    pub async fn recv_msg(&mut self) -> Result<Option<ChatMessage>, Error> {
        let result = match self.ws.as_mut() {
            Some(ws) => ws.recv().await,
            None => return Err(Error::other("Websocket is not connected")),
        };

        match result {
            Ok(Some(json)) => {
                let message = serde_json::from_str::<ChatMessage>(&json).map_err(Error::other)?;
                Ok(Some(message))
            }
            Ok(None) => Ok(None),
            Err(error) => Err(Error::other(error)),
        }
    }

    pub async fn send_message(&mut self, receiver_id: &i64, message: &str) -> Result<(), Error> {
        let ws = self.ws.as_mut().unwrap();
        let user = self.user.as_ref().unwrap();
        let msg = ChatMessage {
            sender_id: user.id,
            recv_id: *receiver_id,
            content: message.to_string(),
        };
        let message = serde_json::to_string(&msg).map_err(Error::other)?;

        if let Err(error) = ws.send(&message).await {
            return Err(Error::other(error));
        }
        Ok(())
    }

    pub async fn get_messages(
        &mut self,
        receiver_id: &i64,
    ) -> Result<Vec<crate::api::models::ChatMessage>, Error> {
        let user = match self.user.as_ref() {
            Some(user) => user,
            None => {
                return Err(Error::other("User not logged in"));
            }
        };

        match self.client.get_message(&user.id, receiver_id).await {
            Ok(messages) => Ok(messages),
            Err(error) => Err(Error::other(error)),
        }
    }

    pub async fn get_users(&mut self) -> Result<Vec<User>, Error> {
        self.client.get_users().await.map_err(Error::other)
    }
}
