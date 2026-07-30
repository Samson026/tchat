mod ws;
mod api;
mod client;

use protocol::SERVER_URL;
use ws::WebSocketConnection;
use tokio_tungstenite::tungstenite::Result;



#[tokio::main]
async fn main() -> Result<()> {
    let mut client = WebSocketConnection::connect(SERVER_URL).await?;

    client.send("").await?;

    if let Some(response) = client.recv().await? {
        println!("{response}");
    }

    Ok(())
}
