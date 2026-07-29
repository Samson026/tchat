use tchat::client::WebSocketConnection;

#[tokio::main]
async fn main() {
    let mut client = WebSocketConnection::connect("ws://127.0.0.1:3000/ws").await;

    client.send("hi").await;
    let resp = client.recv().await.expect("Error");
    println!("{resp}")
}
