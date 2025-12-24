# Task 1.2: Canvas Grid Renderer

## Objective
Implement `src/canvas/GridRenderer.ts` to draw the spreadsheet grid.

## Requirements
1.  **State Management**: Store `scrollOffset` (x, y) and `viewportSize` (width, height).
2.  **Drawing Layers**:
    -   **Background**: White fill.
    -   **Grid Lines**: Loop through visible rows/cols and draw lines using `theme.gridLineColor`.
    -   **Headers**: Draw Row Headers (1, 2, 3...) and Column Headers (A, B, C...) with distinct background.
    -   **Cells**: Fetch visible data using `getViewportData` and draw text.
    -   **Selection**: Draw a border around the `activeCell`.
3.  **Coordinate Conversion**:
    -   `gridToScreen(row, col)`: Returns x, y.
    -   `screenToGrid(x, y)`: Returns row, col (for clicks).

## Implementation Details
-   **File**: `src/canvas/GridRenderer.ts`
-   **Dependencies**: `src/core/WasmBridge.ts`, `src/canvas/theme.ts`.

## Success Criteria
-   An empty grid is rendered.
-   Headers (A-Z, 1-100) are visible.
-   Hardcoded test data in WASM appears on the grid.
