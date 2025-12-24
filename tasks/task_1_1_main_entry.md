# Task 1.1: Application Entry Point

## Objective
Create the main entry point `src/main.ts` to bootstrap the application.

## Requirements
1.  **WASM Initialization**: Call `initWasm()` from `src/core/WasmBridge.ts`.
2.  **DOM Element Access**: Get references to:
    - `#spreadsheet-canvas`
    - `#formula-input`
    - `#sheet-tabs`
    - `#toolbar`
3.  **Controller Initialization**:
    - Instantiate `GridRenderer` (to be created in Task 1.2).
    - Instantiate `InputController` (to be created in Task 1.3).
4.  **Resize Handling**: Listen to window resize events and adjust canvas size.
5.  **Render Loop**: Use `requestAnimationFrame` to drive the `GridRenderer`.

## Implementation Details
-   **File**: `src/main.ts`
-   **Dependencies**: `src/core/WasmBridge.ts`, `src/canvas/GridRenderer.ts` (stub), `src/ui/InputController.ts` (stub).

## Success Criteria
-   The application loads without errors in the console.
-   The canvas resizes to fill the container.
-   The WASM module is successfully loaded (log confirmation).
