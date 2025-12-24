# Rusheet Development Roadmap

This document outlines the progressive development plan to build a fully functional spreadsheet application.

## Phase 1: Frontend Bootstrap & Basic Interaction (Current Focus)
**Goal:** Establish a working UI where users can select cells, enter values, and see them rendered.

- [ ] **Task 1.1: Application Entry Point (`src/main.ts`)**
    - Initialize WASM.
    - Setup Event Listeners.
    - Start the Render Loop.
- [ ] **Task 1.2: Canvas Grid Renderer (`src/canvas/GridRenderer.ts`)**
    - Draw grid lines, headers, and cell content.
    - Handle scrolling (offset calculations).
- [ ] **Task 1.3: Input Controller (`src/ui/InputController.ts`)**
    - Handle Mouse Clicks (Selection).
    - Handle Keyboard Navigation (Arrow keys).
    - Handle Double-Click (Enter Edit Mode).
- [ ] **Task 1.4: Cell Editor (`src/ui/CellEditor.ts`)**
    - Overlay an `<input>` element over the active cell.
    - Commit changes to WASM on Enter/Blur.
- [ ] **Task 1.5: UI Components Sync**
    - Sync Formula Bar with active cell.
    - Sync Sheet Tabs with active sheet.

## Phase 2: Formula Engine Expansion
**Goal:** Support a standard set of spreadsheet functions.

- [ ] **Task 2.1: Logical Functions**
    - Ensure `IF`, `AND`, `OR`, `NOT`, `IFERROR` are implemented.
- [ ] **Task 2.2: Lookup Functions**
    - Implement `VLOOKUP`, `HLOOKUP`, `MATCH`, `INDEX`.
- [ ] **Task 2.3: Date & Time Functions**
    - Implement `TODAY`, `NOW`, `DATE`, `YEAR`, `MONTH`, `DAY`.
- [ ] **Task 2.4: Text Functions**
    - Implement `CONCATENATE` (or `&`), `LEFT`, `RIGHT`, `MID`, `LEN`, `TRIM`.

## Phase 3: Formatting & Advanced UI
**Goal:** Allow users to style their sheets.

- [ ] **Task 3.1: Toolbar Implementation**
    - Connect Bold/Italic/Underline buttons to WASM.
    - Connect Color pickers.
    - Connect Alignment buttons.
- [ ] **Task 3.2: Range Operations**
    - Multi-cell selection rendering.
    - Delete Key clears range.
    - Copy/Paste support.
- [ ] **Task 3.3: Row/Column Resizing**
    - Drag handles on headers.

## Phase 4: Persistence & Polish
**Goal:** Save work and optimize performance.

- [ ] **Task 4.1: Persistence**
    - Save Workbook to LocalStorage (auto-save).
    - Export/Import JSON files.
- [ ] **Task 4.2: Optimization**
    - Virtual scrolling refinement.
    - Canvas rendering optimization (dirty rects).
