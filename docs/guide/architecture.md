# Architecture

## Overview

```
┌─────────────────────────────────────┐
│         Your Application            │
├─────────────────────────────────────┤
│         RusheetAPI (Facade)         │
│  ┌───────────┬───────────────────┐  │
│  │  Events   │   WasmBridge      │  │
│  └───────────┴───────────────────┘  │
├─────────────────────────────────────┤
│            WASM Module              │
│  ┌─────────┬─────────┬──────────┐   │
│  │  Core   │ Formula │ History  │   │
│  │ (Rust)  │ (Rust)  │ (Rust)   │   │
│  └─────────┴─────────┴──────────┘   │
└─────────────────────────────────────┘
```

## Data Flow

1. **User Input** → RusheetAPI
2. **RusheetAPI** → WasmBridge → WASM
3. **WASM** computes & returns affected cells
4. **Events** emitted to subscribers
5. **Renderer** updates canvas

## Performance Optimizations

- **64x64 Morton Chunks**: Z-order curve indexing for cache-friendly iteration
- **Zero-Copy Viewport**: TypedArray views directly into WASM memory
- **Virtual Scrolling**: Only visible cells are rendered
- **OffscreenCanvas**: Optional worker-based rendering
