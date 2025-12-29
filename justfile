# RuSheet Justfile
# Ensures rustup toolchain is used for WASM builds

# Set PATH to prioritize rustup's cargo/rustc
export PATH := env_var("HOME") + "/.cargo/bin:" + env_var("PATH")

# Default recipe
default: dev

# Install dependencies
install:
    npm install

# Build WASM module
build-wasm:
    wasm-pack build crates/rusheet-wasm --target web --out-dir ../../pkg

# Build everything (WASM + frontend)
build: build-wasm
    npm run build

# Development server
dev: build-wasm
    npm run dev

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
    cd example && npm install && npm run dev -- -p 3300
