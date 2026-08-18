# tchat

A real-time chat app with a Rust/Axum backend and Vue 3 Tauri desktop client. Features a WebSocket server, user authentication, and SQLite persistence.

## Tech Stack

- **Language**: Rust
- **Server**: Axum + Tokio + WebSocket
- **Database**: SQLite (`tchat.db`)
- **Desktop Client**: Tauri + Vue 3 + TypeScript
- **Frontend**: Vite + Tailwind CSS + Pinia
- **Layout**: Cargo workspace with `server/`, `client/`, and `crates/protocol/`

## Installation & Setup

### Prerequisites

- Rust installed ([rustup.rs](https://rustup.rs))
- Bun installed ([bun.sh](https://bun.sh))

### Build & Run Server

```bash
# Start the WebSocket server (default: ws://localhost:3000/ws)
cargo run -p server
```

### Run Tauri Client

In a separate terminal:

```bash
cd tauri-client
bun install
bun run tauri dev
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

## Development

Run the project checks from the repository root:

```bash
make lint
cargo test --workspace
```

## Project Structure

```text
├── server/           # Axum WebSocket server + routes + DB
├── client/           # Standalone WebSocket client (demo)
├── tauri-client/     # Tauri desktop application
└── crates/protocol/  # Shared constants: endpoints, paths
```
