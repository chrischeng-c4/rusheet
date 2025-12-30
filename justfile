# RuSheet Justfile
# Ensures rustup toolchain is used for WASM builds

# Set PATH to prioritize rustup's cargo/rustc
export PATH := env_var("HOME") + "/.cargo/bin:" + env_var("PATH")

# Default recipe
default: dev

# Install dependencies
install:
    pnpm install

# Build WASM module
build-wasm:
    wasm-pack build crates/rusheet-wasm --target web --out-dir ../../pkg

# Build everything (WASM + frontend)
build: build-wasm
    pnpm run build

# Development server
dev: build-wasm
    pnpm run dev

# Type check TypeScript
check-ts:
    npx tsc --noEmit

# Check Rust code
check-rust:
    cargo check --workspace

# Check all
check: check-rust check-ts

# Clean build artifacts
clean:
    rm -rf pkg dist target node_modules

# Clean and rebuild
rebuild: clean install build

# Run Rust tests
test-rust:
    cargo test --workspace

# Run unit tests (no WASM required)
test-unit:
    pnpm vitest run --exclude '**/*.integration.test.ts'

# Run integration tests (browser mode, real WASM)
test-integration:
    pnpm vitest run --config vite.config.browser.ts

# Run all tests
test-all: test-rust test-unit test-integration

# Format Rust code
fmt:
    cargo fmt --all

# Lint Rust code
lint:
    cargo clippy --workspace

# Update rustup toolchain
update-rust:
    rustup update stable
    rustup target add wasm32-unknown-unknown

# Run the example
example: build-wasm
    cp -r pkg example/src/pkg
    cd example && pnpm install && pnpm run dev -- -p 3300

# Documentation commands
docs-dev:
    pnpm docs:dev

docs-build:
    pnpm docs:build

docs-preview:
    pnpm docs:preview

# Server commands
# Start PostgreSQL (for local development without devcontainer)
db-up:
    docker compose up -d postgres

# Stop PostgreSQL
db-down:
    docker compose down

# Build the server
build-server:
    cargo build --package rusheet-server

# Run the server (requires PostgreSQL)
server:
    cargo run --package rusheet-server

# Run server in watch mode (requires cargo-watch)
server-watch:
    cargo watch -x "run --package rusheet-server"

# Run server tests
test-server:
    cargo test --package rusheet-server

# Full dev environment (starts DB, builds WASM, runs frontend + server in parallel)
dev-full: build-wasm
    @echo "Starting PostgreSQL..."
    docker compose up -d postgres
    @echo "Waiting for PostgreSQL..."
    sleep 3
    @echo "Starting server and frontend in parallel..."
    (cargo run --package rusheet-server &) && pnpm run dev
