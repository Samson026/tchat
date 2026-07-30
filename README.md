# tchat

A small WebSocket chat project organized as a Cargo workspace.

## Project layout

```text
.
├── client/            # WebSocket client executable
├── crates/
│   └── protocol/      # Configuration shared by client and server
└── server/            # Axum WebSocket server executable
```

## Running the project

Start the server:

```bash
cargo run -p server
```

In another terminal, run the client:

```bash
cargo run -p client
```

The example client sends `hi`, and the server echoes it back.
