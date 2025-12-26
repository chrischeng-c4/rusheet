# Rusheet Architectural Roadmap

This document outlines the roadmap for upgrading `rusheet` to a high-performance, industry-standard spreadsheet engine.

## 1. Core Architecture (Rust)

### Current Status
*   ✅ **Sparse Matrix:** Implemented `ChunkedGrid` (HashMap of 16x16 blocks).
*   ✅ **Spatial Index:** Implemented `SpatialIndex` using `FenwickTree` for O(log N) row/col lookups.
*   ✅ **Basic State:** `Sheet` struct holds data and dimensions.
*   ✅ **Hit Testing API:** `getCellFromPixel` exposed to WASM.
*   ✅ **Dimensions API:** `getDimensions` exposed to WASM.

### Gaps & Tasks
*   [ ] **Command Pattern:** Ensure all state mutations (edits, formatting, resizing) go through a uniform `Command` enum that supports Undo/Redo.
*   [ ] **Dependency Graph:** Verify `rusheet-formula` handles DAGs efficiently for recalculation.
*   [ ] **Dirty Rectangles:** The `Sheet` or `Engine` needs to return a list of "Dirty Rects" after a mutation, rather than requiring a full re-render.

## 2. Rendering Engine (TypeScript/WASM)

### Current Status
*   ✅ **Modular Architecture:** Split into `GridRenderer`, `GridController`, `useGrid`, and `Grid`.
*   ✅ **Hybrid Rendering:** Canvas for Grid, DOM for Editor Overlay and Scrollbars.
*   ✅ **Virtual Scroll:** Implemented `onScroll` sync between DOM Scroller and Canvas Controller.
*   ⚠️ **Direct Canvas API:** Rendering commands are issued one-by-one from JS. Need `getViewportData` optimization.

### Tasks
*   [ ] **WASM Bridge Optimization:** Implement `getViewportData(rect)` that returns a serialized buffer or FlatBuffer.
*   [ ] **Render Loop:** Move from `useEffect` to an explicit `requestAnimationFrame` loop driven by "Dirty" flags.
*   [ ] **Border Rendering:** Implement rendering of cell borders (Top/Bottom/Left/Right).

## 3. Interaction & UX (Goal: 60-70% Google Sheets)

### Current Status
*   ✅ **Hit Testing:** O(log N) hit testing using `SpatialIndex`.
*   ✅ **Basic Selection:** Single cell selection.
*   ✅ **Basic Navigation:** Arrow keys.
*   ✅ **Scrolling:** Virtualized scrolling with native feel.

### Tasks (Priority Order)

#### 3.1 Selection & Range
*   [ ] **Range Selection:** Implement drag-to-select logic in `GridController`.
*   [ ] **Shift+Select:** Expand selection range.
*   [ ] **Visual Feedback:** Draw blue selection overlay in `GridRenderer`.

#### 3.2 Resizing
*   [ ] **Header Interaction:** Detect hover over row/column separators.
*   [ ] **Drag Logic:** Implement resizing interaction.
*   [ ] **API:** Connect to `setRowHeight` / `setColWidth`.

#### 3.3 Sheet Management
*   [ ] **Tab Bar UI:** React component for Sheet Tabs.
*   [ ] **Operations:** Add, Rename, Delete, Switch Sheet.

#### 3.4 Context Menus
*   [ ] **Header Menu:** Right-click on Row/Col headers (Insert, Delete, Hide).
*   [ ] **Cell Menu:** Cut, Copy, Paste, Format.

#### 3.5 Clipboard
*   [ ] **Internal Serializer:** Implement TSV and HTML export in `SpreadsheetEngine`.
*   [ ] **Frontend Handling:** `Ctrl+C`/`Ctrl+V` listeners in `GridController`.
*   [ ] **Paste Logic:** Parser for TSV/HTML to insert data into `Sheet`.

#### 3.6 Persistence
*   [ ] **JSON Schema:** Define stable JSON format for `Workbook`.
*   [ ] **Import/Export:** Add "Save" and "Load" buttons to Toolbar.
*   [ ] **Auto-save:** Implement debounce save to `localStorage`.

## 4. Testing Strategy

### Core Logic (Rust)
*   **Unit Tests:** Continue strict testing of `ChunkedGrid` and `SpatialIndex`.
*   **Integration Tests:** Test the `Command` stack (Undo/Redo) and Formula Engine.

### Visual/Canvas (TypeScript)
*   **Interaction Tests (Vitest + Mock Canvas):**
    *   Mock the `CanvasRenderingContext2D`.
    *   Assert that `renderer.draw()` calls `ctx.fillRect` with correct coordinates.
*   **E2E (Playwright):**
    *   "Golden Image" comparison for pixel-perfect rendering checks.