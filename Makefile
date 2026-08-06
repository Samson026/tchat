.PHONY: help lint lint-rust lint-rust-all lint-client lint-server lint-protocol lint-tauri-rust lint-tauri-client

help:
	@echo "Available targets:"
	@echo "  make lint               Run linters for lint-ready subprojects"
	@echo "  make lint-rust          Lint lint-ready Rust workspace crates"
	@echo "  make lint-rust-all      Lint all Rust workspace crates, including client"
	@echo "  make lint-client        Lint the Rust terminal client"
	@echo "  make lint-server        Lint the Rust server"
	@echo "  make lint-protocol      Lint the shared Rust protocol crate"
	@echo "  make lint-tauri-rust    Lint the Tauri Rust crate"
	@echo "  make lint-tauri-client  Run Biome checks for the Tauri frontend"

lint: lint-rust lint-tauri-client

lint-rust: lint-server lint-protocol lint-tauri-rust

lint-rust-all:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

lint-client:
	cargo clippy -p client --all-targets --all-features -- -D warnings

lint-server:
	cargo clippy -p server --all-targets --all-features -- -D warnings

lint-protocol:
	cargo clippy -p protocol --all-targets --all-features -- -D warnings

lint-tauri-rust:
	cargo clippy -p tauri-client --all-targets --all-features -- -D warnings

lint-tauri-client:
	cd tauri-client && bun run check
