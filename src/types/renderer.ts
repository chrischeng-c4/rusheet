/**
 * Common interface for grid renderers.
 * Both GridRenderer (direct canvas) and RenderController (offscreen worker)
 * implement this interface.
 */

export interface Point {
  x: number;
  y: number;
}

export interface CellPosition {
  row: number;
  col: number;
}

export interface IGridRenderer {
  /**
   * Update viewport size based on canvas dimensions
   */
  updateViewportSize(): void;

  /**
   * Set scroll offset
   */
  setScrollOffset(x: number, y: number): void;

  /**
   * Set active cell
   */
  setActiveCell(row: number, col: number): void;

  /**
   * Get active cell
   */
  getActiveCell(): CellPosition;

  /**
   * Convert grid coordinates to screen coordinates
   */
  gridToScreen(row: number, col: number): Point;

  /**
   * Convert screen coordinates to grid coordinates
   */
  screenToGrid(x: number, y: number): CellPosition;

  /**
   * Trigger a render
   */
  render(): void;

  /**
   * Handle canvas resize
   */
  resize(width: number, height: number): void;
}
