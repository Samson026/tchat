mod api;
mod client;
mod tui;
mod ws;

use client::ClientApp;
use tokio_tungstenite::tungstenite::Result;
use tui::App;

#[tokio::main]
async fn main() -> Result<()> {
    let client = ClientApp::new().await.expect("Error");

    // client.login("Sammi").await?;

    // client.connect_ws().await?;

    let mut tui = App::new(client);
    let mut terminal = ratatui::init();

    match terminal.clear() {
        Ok(()) => tui.run(&mut terminal).await?,
        Err(error) => {
            eprint!("Error: {error}");
        }
    }
    ratatui::restore();
    Ok(())
}
