use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    server::run().await
}
