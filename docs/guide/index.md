# Introduction

Rusheet is a high-performance spreadsheet engine built with Rust and compiled to WebAssembly. It provides a library API for building spreadsheet applications.

## Features

- **High Performance**: 64x64 Morton-indexed chunks with bitvec sparse storage
- **Zero-Copy Data Transfer**: Direct TypedArray access to WASM memory
- **Formula Engine**: Nom-based parser supporting 20+ functions
- **Event-Driven API**: Subscribe to changes, selections, and edits
- **Undo/Redo**: Full history management
- **Multi-Sheet Support**: Multiple worksheets in a single workbook

## Target Use Cases

- Embedded spreadsheet components
- Data entry applications
- Financial modeling tools
- Report builders
