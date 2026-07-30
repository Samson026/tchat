use std::io::Error;

use crate::api::{Client};
use crate::api::models::User;
use crate::ws::WebSocketConnection;


pub struct ClientApp {
    client: Client,
    ws: Option<WebSocketConnection>,
    user: Option<User>
}

impl ClientApp {
    pub async fn new() -> Result<Self, Error> {
        let client = Client::new()
            .await
            .map_err(Error::other)?;

        Ok(Self {
            client,
            ws: None,
            user: None
        })
    }

    pub async fn login(&mut self, username: &str) -> Result<(), Error> {
        match self.client.login(username).await {
            Ok(user) => {
                self.user = Some(user);
                Ok(())
            }
            Err(error) => {
                Err(std::io::Error::other(error))
            }
        }
    }

    


}