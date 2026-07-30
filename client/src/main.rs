mod ws;
mod api;
mod client;

use tokio_tungstenite::tungstenite::Result;
use api::Client;



#[tokio::main]
async fn main() -> Result<()> {
    let mut client = Client::new().await.expect("Error");

    match client.login("Sammi").await {
        Ok(user) => {
            println!("{}", user.id);
            println!("{}", user.username);
        }
        Err(error) => {
            eprint!("Error {error}");
        }
    };
    Ok(())
}
