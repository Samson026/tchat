mod ws;
mod api;
mod client;

use tokio_tungstenite::tungstenite::Result;
use client::ClientApp;



#[tokio::main]
async fn main() -> Result<()> {
    let mut client = ClientApp::new().await.expect("Error");

    client.login("Sammi").await?;

    let user = client.user.as_ref().unwrap();
    println!("{}", user.id);
    println!("{}", user.username);

    client.connect_ws().await?;

    
    Ok(())
}
