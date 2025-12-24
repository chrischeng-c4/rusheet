# Task 1.3: Input Controller

## Objective
Implement `src/ui/InputController.ts` to handle user interactions.

## Requirements
1.  **Mouse Events**:
    -   `mousedown`: Set `activeCell` based on coordinates.
    -   `wheel`: Update `GridRenderer` scroll offset.
2.  **Keyboard Events**:
    -   `ArrowUp/Down/Left/Right`: Move selection.
    -   `Enter`: Enter Edit Mode (Task 1.4).
    -   `Delete`/`Backspace`: Clear cell content (call `clearRange`).
    -   `Ctrl+Z` / `Ctrl+Y`: Trigger Undo/Redo.
3.  **Bridge Integration**:
    -   Call `WasmBridge.setActiveSheet()` etc.

## Implementation Details
-   **File**: `src/ui/InputController.ts`
-   **Dependencies**: `src/core/WasmBridge.ts`, `src/canvas/GridRenderer.ts`.

## Success Criteria
-   Clicking a cell highlights it.
-   Arrow keys move the selection.
-   Scrolling moves the grid.
