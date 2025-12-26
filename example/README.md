# RuSheet Next.js Example

A Next.js example demonstrating the RuSheet Rust WASM spreadsheet component.

## Prerequisites

Before running this example, build the WASM module from the parent directory:

```bash
# From the rusheet root directory
just build-wasm

# Copy pkg to example
cp -r pkg example/src/pkg
```

## Getting Started

```bash
# Install dependencies
npm install

# Run development server on port 3300
npm run dev -- -p 3300
```

Open [http://localhost:3300](http://localhost:3300) to see the spreadsheet.

## Features

- Canvas-based grid rendering
- Cell selection with mouse click
- Keyboard navigation (Arrow keys, Tab, Enter)
- Inline cell editing (double-click or Enter)
- Formula bar with cell address display
- Formula support (SUM, multiplication, etc.)
- Scroll support

## Project Structure

```
example/
├── src/
│   ├── app/
│   │   ├── page.tsx          # Main page
│   │   └── layout.tsx        # Root layout
│   ├── components/
│   │   └── Spreadsheet.tsx   # Spreadsheet React component
│   └── pkg/                  # WASM bindings (copied from parent)
└── next.config.ts            # Next.js config with WASM support
```
