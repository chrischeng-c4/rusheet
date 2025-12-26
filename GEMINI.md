# RuSheet

## Project Overview

RuSheet is a high-performance spreadsheet application built with Rust and WebAssembly (WASM). It leverages Rust for core logic, formula evaluation, and state management, while using TypeScript and the HTML5 Canvas API for a responsive frontend interface.

### Architecture

The project is a monorepo structure containing a Rust workspace and a TypeScript frontend.

**Rust Workspace (`crates/`):**
*   **`rusheet-core`**: Core spreadsheet data structures (Grid, Cell, Workbook) and logic.
*   **`rusheet-formula`**: Formula parsing, lexing, and evaluation engine.
*   **`rusheet-history`**: Undo/Redo command stack and history management.
*   **`rusheet-wasm`**: The bridge between Rust core logic and the JavaScript/Browser world, exposing APIs via `wasm-bindgen`.

**Frontend (`src/`, `index.html`):**
*   **Technology Stack**: TypeScript, Vite.
*   **Rendering**: Custom GridRenderer using HTML5 Canvas for performance.
*   **Integration**: Imports the compiled WASM module (`pkg/`) to drive application state.

## Setup and Development

This project uses [`just`](https://github.com/casey/just) as a command runner to simplify development workflows.

### Prerequisites
*   Node.js (v18+)
*   Rust (stable toolchain)
*   `wasm-pack`: `cargo install wasm-pack`
*   `just` (optional but recommended): `cargo install just`

### Common Commands

If you have `just` installed, use the following commands. If not, refer to the `justfile` or `package.json` for the underlying shell commands.

| Action | Command | Description |
| :--- | :--- | :--- |
| **Start Dev Server** | `just dev` | Builds WASM and starts Vite dev server (includes hot reload for TS, manual restart for Rust changes). |
| **Build WASM** | `just build-wasm` | Compiles the Rust crates to WebAssembly (outputs to `pkg/`). |
| **Build Production** | `just build` | Builds both WASM and the optimized frontend bundle. |
| **Type Check** | `just check` | Runs `cargo check` and `tsc --noEmit`. |
| **Run Rust Tests** | `just test-rust` | Executes unit tests in the Rust workspace. |
| **Format Code** | `just fmt` | Formats Rust code using `rustfmt`. |
| **Lint Code** | `just lint` | Runs `clippy` on Rust code. |

## Development Workflow

1.  **Rust Changes**: When modifying files in `crates/`, you typically need to re-run `just build-wasm` (or `just dev` which handles the initial build) for changes to take effect in the browser.
2.  **Frontend Changes**: Vite handles HMR (Hot Module Replacement) for TypeScript/CSS changes automatically.
3.  **Testing**:
    *   Logic-heavy features should be tested in Rust (`crates/*/src/**/*.rs`).
    *   UI/Integration logic can be tested with Vitest (`npm run test`).

## Key Files & Directories

*   `crates/rusheet-core/src/lib.rs`: Entry point for core logic.
*   `crates/rusheet-wasm/src/lib.rs`: WASM API definitions.
*   `src/main.ts`: Frontend entry point.
*   `src/canvas/GridRenderer.ts`: Main rendering logic.
*   `specs/`: Detailed documentation on architecture, formulas, and performance.
*   `CLAUDE.md`: AI-specific context and agent guidelines (reference for project scope).
