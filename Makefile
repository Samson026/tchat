.PHONY: help lint lint-rust lint-rust-all lint-rust-format lint-client lint-server lint-protocol lint-tauri-rust lint-tauri-client fix fix-rust fix-rust-all fix-rust-format fix-client fix-server fix-protocol fix-tauri-rust fix-tauri-client

help:
	@echo "Available targets:"
	@echo "  make lint               Run linters for lint-ready subprojects"
	@echo "  make lint-rust          Lint lint-ready Rust workspace crates"
	@echo "  make lint-rust-all      Check formatting and lint all Rust workspace crates"
	@echo "  make lint-rust-format   Check Rust formatting"
	@echo "  make lint-client        Lint the Rust terminal client"
	@echo "  make lint-server        Lint the Rust server"
	@echo "  make lint-protocol      Lint the shared Rust protocol crate"
	@echo "  make lint-tauri-rust    Lint the Tauri Rust crate"
	@echo "  make lint-tauri-client  Run Biome checks for the Tauri frontend"
	@echo "  make fix                Auto-fix all fixable lint issues"
	@echo "  make fix-rust           Auto-fix Rust lint issues"
	@echo "  make fix-rust-all       Auto-fix Rust formatting and lint issues"
	@echo "  make fix-rust-format    Format all Rust workspace crates"
	@echo "  make fix-client         Auto-fix Rust terminal client lint issues"
	@echo "  make fix-server         Auto-fix Rust server lint issues"
	@echo "  make fix-protocol       Auto-fix shared Rust protocol lint issues"
	@echo "  make fix-tauri-rust     Auto-fix Tauri Rust crate lint issues"
	@echo "  make fix-tauri-client   Auto-fix Tauri frontend issues"

lint: lint-rust-format lint-rust lint-tauri-client

lint-rust: lint-server lint-client lint-protocol lint-tauri-rust

lint-rust-all: lint-rust-format
	cargo clippy --workspace --all-targets --all-features -- -D warnings

lint-rust-format:
	cargo fmt --all -- --check

lint-client:
	cargo clippy -p client --all-targets --all-features -- -D warnings

lint-server:
	cargo clippy -p server --all-targets --all-features -- -D warnings

lint-protocol:
	cargo clippy -p protocol --all-targets --all-features -- -D warnings

lint-tauri-rust:
	cargo clippy -p tChat --all-targets --all-features -- -D warnings

lint-tauri-client:
	cd tauri-client && bun run check

fix: fix-rust-format fix-rust fix-tauri-client

fix-rust: fix-server fix-client fix-protocol fix-tauri-rust

fix-rust-all: fix-rust-format
	cargo clippy --workspace --all-targets --all-features --fix --allow-dirty --allow-staged

fix-rust-format:
	cargo fmt --all

fix-client:
	cargo clippy -p client --all-targets --all-features --fix --allow-dirty --allow-staged

fix-server:
	cargo clippy -p server --all-targets --all-features --fix --allow-dirty --allow-staged

fix-protocol:
	cargo clippy -p protocol --all-targets --all-features --fix --allow-dirty --allow-staged

fix-tauri-rust:
	cargo clippy -p tChat --all-targets --all-features --fix --allow-dirty --allow-staged

fix-tauri-client:
	cd tauri-client && bun run fix
