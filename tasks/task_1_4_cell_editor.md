# Task 1.4: Cell Editor

## Objective
Implement `src/ui/CellEditor.ts` for in-place editing.

## Requirements
1.  **Overlay Input**:
    -   Create a dynamic `<textarea>` or `<input>` positioned exactly over the active cell.
    -   Match font/size of the cell.
2.  **Lifecycle**:
    -   **Activate**: On `Enter` key or Double Click. Populate with current cell formula/value.
    -   **Commit**: On `Enter` or Blur. Call `WasmBridge.setCellValue`.
    -   **Cancel**: On `Escape`. Discard changes.
3.  **Formula Bar Sync**:
    -   Typing in the cell editor should update the formula bar input (and vice-versa).

## Implementation Details
-   **File**: `src/ui/CellEditor.ts`
-   **Dependencies**: `src/core/WasmBridge.ts`.

## Success Criteria
-   Double-clicking a cell shows an input box.
-   Typing "Hello" and pressing Enter saves it to the grid.
-   Typing "=SUM(1,2)" calculates and shows "3".
