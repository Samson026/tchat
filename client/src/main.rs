mod api;
mod client;
mod ws;
mod tui;

use client::ClientApp;
use tokio_tungstenite::tungstenite::Result;
use tui::App;

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = ClientApp::new().await.expect("Error");

    // client.login("Sammi").await?;

    // let user = client.user.as_ref().unwrap();
    // println!("{}", user.id);
    // println!("{}", user.username);

    // client.connect_ws().await?;

    let mut tui = App::new(client);
    let mut terminal = ratatui::init();
    
    match terminal.clear() {
        Ok(()) => tui.run(&mut terminal).await?,
        Err(error) => {eprint!("Error: {error}");}
    }
    ratatui::restore();
    Ok(())
}
