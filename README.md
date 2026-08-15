# tchat

A real-time chat application built with Rust. Features a WebSocket server, Tauri desktop client, user authentication, and SQLite persistence.

## Tech Stack

- **Language**: Rust
- **Server**: Axum + Tokio + WebSocket
- **Database**: SQLite (`tchat.db`)
- **Desktop Client**: Tauri
- **Layout**: Cargo workspace with `server/`, `client/`, and `crates/protocol/`

## Installation & Setup

### Prerequisites

- Rust installed ([rustup.rs](https://rustup.rs))

### Build & Run Server

```bash
cargo build --release

# Start the WebSocket server (default: ws://localhost:3000/ws)
cargo run -p server
```

### Run Tauri Client

In a separate terminal:

```bash
cd tauri-client/src-tauri
cargo run
```

Alternatively, build and run with:

```bash
cargo build --release --bin client  # standalone client
cargo run -p client               # example demo client
```

## Features

- Real-time WebSocket messaging
- User authentication and accounts
- Message history in SQLite
- File upload and download support
- Cross-platform Tauri desktop client

## Project Structure

```text
├── server/           # Axum WebSocket server + routes + DB
├── client/           # Standalone WebSocket client (demo)
├── tauri-client/     # Tauri desktop application
└── crates/protocol/  # Shared constants: endpoints, paths
```

## License

MIT License
