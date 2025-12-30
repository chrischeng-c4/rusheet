import { theme } from './theme';
import type { CellData as _CellData } from '../types';
import * as WasmBridge from '../core/WasmBridge';

interface Point {
  x: number;
  y: number;
}

interface Size {
  width: number;
  height: number;
}

interface CellPosition {
  row: number;
  col: number;
}

export default class GridRenderer {
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D;
  private scrollOffset: Point = { x: 0, y: 0 };
  private viewportSize: Size = { width: 0, height: 0 };
  private activeCell: CellPosition = { row: 0, col: 0 };

  constructor(canvas: HTMLCanvasElement) {
    this.canvas = canvas;
    const context = canvas.getContext('2d');
    if (!context) {
      throw new Error('Failed to get 2D context from canvas');
    }
    this.ctx = context;
    this.updateViewportSize();
  }

  /**
   * Update viewport size based on canvas dimensions
   */
  public updateViewportSize(): void {
    this.viewportSize = {
      width: this.canvas.width,
      height: this.canvas.height,
    };
  }

  /**
   * Set scroll offset
   */
  public setScrollOffset(x: number, y: number): void {
    this.scrollOffset = { x, y };
  }

  /**
   * Set active cell
   */
  public setActiveCell(row: number, col: number): void {
    this.activeCell = { row, col };
  }

  /**
   * Get active cell
   */
  public getActiveCell(): CellPosition {
    return { ...this.activeCell };
  }

  /**
   * Convert grid coordinates to screen coordinates
   */
  public gridToScreen(row: number, col: number): Point {
    let x = theme.headerWidth;
    let y = theme.headerHeight;

    // Calculate x position by summing column widths
    for (let c = 0; c < col; c++) {
      x += WasmBridge.getColWidth(c);
    }
    x -= this.scrollOffset.x;

    // Calculate y position by summing row heights
    for (let r = 0; r < row; r++) {
      y += WasmBridge.getRowHeight(r);
    }
    y -= this.scrollOffset.y;

    return { x, y };
  }

  /**
   * Convert screen coordinates to grid coordinates
   */
  public screenToGrid(x: number, y: number): CellPosition {
    // Adjust for scroll offset and headers
    const adjustedX = x - theme.headerWidth + this.scrollOffset.x;
    const adjustedY = y - theme.headerHeight + this.scrollOffset.y;

    let col = 0;
    let accumulatedWidth = 0;
    while (accumulatedWidth < adjustedX && col < 1000) {
      accumulatedWidth += WasmBridge.getColWidth(col);
      if (accumulatedWidth >= adjustedX) break;
      col++;
    }

    let row = 0;
    let accumulatedHeight = 0;
    while (accumulatedHeight < adjustedY && row < 1000) {
      accumulatedHeight += WasmBridge.getRowHeight(row);
      if (accumulatedHeight >= adjustedY) break;
      row++;
    }

    return { row, col };
  }

  /**
   * Calculate visible viewport range
   */
  private calculateVisibleRange(): {
    startRow: number;
    endRow: number;
    startCol: number;
    endCol: number;
  } {
    const startPos = this.screenToGrid(theme.headerWidth, theme.headerHeight);
    const endPos = this.screenToGrid(this.viewportSize.width, this.viewportSize.height);

    return {
      startRow: Math.max(0, startPos.row),
      endRow: Math.min(999, endPos.row + 1),
      startCol: Math.max(0, startPos.col),
      endCol: Math.min(999, endPos.col + 1),
    };
  }

  /**
   * Convert column index to letter (0 -> A, 25 -> Z, 26 -> AA, etc.)
   */
  private colToLetter(col: number): string {
    let result = '';
    let num = col;
    while (num >= 0) {
      result = String.fromCharCode(65 + (num % 26)) + result;
      num = Math.floor(num / 26) - 1;
      if (num < 0) break;
    }
    return result;
  }

  /**
   * Main render method - draws all layers
   */
  public render(): void {
    const { width, height } = this.viewportSize;
    const ctx = this.ctx;

    // Layer 1: White background
    ctx.fillStyle = theme.backgroundColor;
    ctx.fillRect(0, 0, width, height);

    // Calculate visible range
    const range = this.calculateVisibleRange();

    // Layer 2: Grid lines and cells
    this.renderGrid(range);

    // Layer 3: Cell content
    this.renderCellContent(range);

    // Layer 4: Headers
    this.renderHeaders(range);

    // Layer 5: Selection border
    this.renderSelection();
  }

  /**
   * Render grid lines
   */
  private renderGrid(range: {
    startRow: number;
    endRow: number;
    startCol: number;
    endCol: number;
  }): void {
    const ctx = this.ctx;
    ctx.strokeStyle = theme.gridLineColor;
    ctx.lineWidth = theme.gridLineWidth;

    // Draw vertical lines (columns)
    let x = theme.headerWidth;
    for (let col = 0; col <= range.endCol; col++) {
      const colWidth = WasmBridge.getColWidth(col);
      x += colWidth;
      const screenX = x - this.scrollOffset.x;

      if (screenX >= theme.headerWidth && screenX <= this.viewportSize.width) {
        ctx.beginPath();
        ctx.moveTo(screenX, theme.headerHeight);
        ctx.lineTo(screenX, this.viewportSize.height);
        ctx.stroke();
      }
    }

    // Draw horizontal lines (rows)
    let y = theme.headerHeight;
    for (let row = 0; row <= range.endRow; row++) {
      const rowHeight = WasmBridge.getRowHeight(row);
      y += rowHeight;
      const screenY = y - this.scrollOffset.y;

      if (screenY >= theme.headerHeight && screenY <= this.viewportSize.height) {
        ctx.beginPath();
        ctx.moveTo(theme.headerWidth, screenY);
        ctx.lineTo(this.viewportSize.width, screenY);
        ctx.stroke();
      }
    }
  }

  /**
   * Calculate the total width of merged columns
   */
  private getMergedWidth(startCol: number, colSpan: number): number {
    let width = 0;
    for (let c = startCol; c < startCol + colSpan; c++) {
      width += WasmBridge.getColWidth(c);
    }
    return width;
  }

  /**
   * Calculate the total height of merged rows
   */
  private getMergedHeight(startRow: number, rowSpan: number): number {
    let height = 0;
    for (let r = startRow; r < startRow + rowSpan; r++) {
      height += WasmBridge.getRowHeight(r);
    }
    return height;
  }

  /**
   * Render cell content
   */
  private renderCellContent(range: {
    startRow: number;
    endRow: number;
    startCol: number;
    endCol: number;
  }): void {
    const ctx = this.ctx;
    const cellsData = WasmBridge.getViewportData(
      range.startRow,
      range.endRow,
      range.startCol,
      range.endCol
    );

    // Get merged ranges for the viewport
    const mergedRanges = WasmBridge.getMergedRanges();

    // First, render backgrounds for merged cells (including empty merged cells)
    for (const merge of mergedRanges) {
      // Check if merge is in viewport
      if (merge.endRow < range.startRow || merge.startRow > range.endRow ||
          merge.endCol < range.startCol || merge.startCol > range.endCol) {
        continue;
      }

      const pos = this.gridToScreen(merge.startRow, merge.startCol);
      const mergedWidth = this.getMergedWidth(merge.startCol, merge.endCol - merge.startCol + 1);
      const mergedHeight = this.getMergedHeight(merge.startRow, merge.endRow - merge.startRow + 1);

      // Get master cell data for background
      const masterCell = cellsData.find(c => c.row === merge.startRow && c.col === merge.startCol);

      // Draw white background to cover grid lines inside merge
      ctx.fillStyle = masterCell?.format.backgroundColor || theme.backgroundColor;
      ctx.fillRect(pos.x + 1, pos.y + 1, mergedWidth - 1, mergedHeight - 1);
    }

    for (const cellData of cellsData) {
      const { row, col } = cellData;

      // Skip slave cells (non-master cells in a merged range)
      if (WasmBridge.isMergedSlave(row, col)) {
        continue;
      }

      // Get merge info to determine if this is a master cell
      const mergeInfo = WasmBridge.getMergeInfo(row, col);

      const pos = this.gridToScreen(row, col);
      let cellWidth: number;
      let cellHeight: number;

      if (mergeInfo) {
        // This is a master cell of a merged range
        cellWidth = this.getMergedWidth(mergeInfo.masterCol, mergeInfo.colSpan);
        cellHeight = this.getMergedHeight(mergeInfo.masterRow, mergeInfo.rowSpan);
      } else {
        // Regular cell
        cellWidth = WasmBridge.getColWidth(col);
        cellHeight = WasmBridge.getRowHeight(row);
      }

      // Apply cell background color if set (for non-merged cells, or re-apply for merged)
      if (cellData.format.backgroundColor) {
        ctx.fillStyle = cellData.format.backgroundColor;
        ctx.fillRect(pos.x, pos.y, cellWidth, cellHeight);
      }

      // Skip text rendering if no display value
      if (!cellData.displayValue) continue;

      // Set up text rendering
      let fontStyle = '';
      if (cellData.format.bold) fontStyle += 'bold ';
      if (cellData.format.italic) fontStyle += 'italic ';
      ctx.font = fontStyle + theme.cellFont;
      ctx.fillStyle = cellData.format.textColor || theme.cellTextColor;
      ctx.textBaseline = 'top';

      // Horizontal alignment
      let textX = pos.x + theme.cellPadding;
      const horizontalAlign = cellData.format.horizontalAlign || 'left';
      const metrics = ctx.measureText(cellData.displayValue);
      const textWidth = metrics.width;

      if (horizontalAlign === 'center') {
        textX = pos.x + (cellWidth - textWidth) / 2;
      } else if (horizontalAlign === 'right') {
        textX = pos.x + cellWidth - textWidth - theme.cellPadding;
      }

      // Vertical alignment
      let textY = pos.y + theme.cellPadding;
      const verticalAlign = cellData.format.verticalAlign || 'top';

      if (verticalAlign === 'middle') {
        textY = pos.y + (cellHeight - 13) / 2; // 13 is approximate font height
      } else if (verticalAlign === 'bottom') {
        textY = pos.y + cellHeight - 13 - theme.cellPadding;
      }

      // Clip text to cell bounds
      ctx.save();
      ctx.beginPath();
      ctx.rect(pos.x, pos.y, cellWidth, cellHeight);
      ctx.clip();

      // Draw text with underline if needed
      ctx.fillText(cellData.displayValue, textX, textY);

      if (cellData.format.underline) {
        ctx.strokeStyle = cellData.format.textColor || theme.cellTextColor;
        ctx.lineWidth = 1;
        ctx.beginPath();
        ctx.moveTo(textX, textY + 14);
        ctx.lineTo(textX + textWidth, textY + 14);
        ctx.stroke();
      }

      ctx.restore();
    }

    // Draw borders around merged cells
    ctx.strokeStyle = theme.gridLineColor;
    ctx.lineWidth = theme.gridLineWidth;
    for (const merge of mergedRanges) {
      if (merge.endRow < range.startRow || merge.startRow > range.endRow ||
          merge.endCol < range.startCol || merge.startCol > range.endCol) {
        continue;
      }

      const pos = this.gridToScreen(merge.startRow, merge.startCol);
      const mergedWidth = this.getMergedWidth(merge.startCol, merge.endCol - merge.startCol + 1);
      const mergedHeight = this.getMergedHeight(merge.startRow, merge.endRow - merge.startRow + 1);

      ctx.strokeRect(pos.x, pos.y, mergedWidth, mergedHeight);
    }
  }

  /**
   * Render row and column headers
   */
  private renderHeaders(range: {
    startRow: number;
    endRow: number;
    startCol: number;
    endCol: number;
  }): void {
    const ctx = this.ctx;

    // Draw header backgrounds
    ctx.fillStyle = theme.headerBackground;

    // Top-left corner
    ctx.fillRect(0, 0, theme.headerWidth, theme.headerHeight);

    // Column headers background
    ctx.fillRect(theme.headerWidth, 0, this.viewportSize.width - theme.headerWidth, theme.headerHeight);

    // Row headers background
    ctx.fillRect(0, theme.headerHeight, theme.headerWidth, this.viewportSize.height - theme.headerHeight);

    // Set up text style
    ctx.font = theme.headerFont;
    ctx.fillStyle = theme.headerText;
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';

    // Draw column headers (A, B, C, ...)
    for (let col = range.startCol; col <= range.endCol; col++) {
      const pos = this.gridToScreen(0, col);
      const colWidth = WasmBridge.getColWidth(col);
      const centerX = pos.x + colWidth / 2;
      const centerY = theme.headerHeight / 2;

      if (centerX >= theme.headerWidth && centerX <= this.viewportSize.width) {
        ctx.fillText(this.colToLetter(col), centerX, centerY);
      }
    }

    // Draw row headers (1, 2, 3, ...)
    for (let row = range.startRow; row <= range.endRow; row++) {
      const pos = this.gridToScreen(row, 0);
      const rowHeight = WasmBridge.getRowHeight(row);
      const centerX = theme.headerWidth / 2;
      const centerY = pos.y + rowHeight / 2;

      if (centerY >= theme.headerHeight && centerY <= this.viewportSize.height) {
        ctx.fillText(String(row + 1), centerX, centerY);
      }
    }

    // Draw header borders
    ctx.strokeStyle = theme.gridLineColor;
    ctx.lineWidth = theme.gridLineWidth;

    // Vertical line after row headers
    ctx.beginPath();
    ctx.moveTo(theme.headerWidth, 0);
    ctx.lineTo(theme.headerWidth, this.viewportSize.height);
    ctx.stroke();

    // Horizontal line after column headers
    ctx.beginPath();
    ctx.moveTo(0, theme.headerHeight);
    ctx.lineTo(this.viewportSize.width, theme.headerHeight);
    ctx.stroke();
  }

  /**
   * Render selection border around active cell
   */
  private renderSelection(): void {
    const ctx = this.ctx;
    const { row, col } = this.activeCell;

    // Check if the active cell is part of a merged range
    const mergeInfo = WasmBridge.getMergeInfo(row, col);

    let selectionRow = row;
    let selectionCol = col;
    let selectionWidth: number;
    let selectionHeight: number;

    if (mergeInfo) {
      // Selection covers the entire merged range
      selectionRow = mergeInfo.masterRow;
      selectionCol = mergeInfo.masterCol;
      selectionWidth = this.getMergedWidth(mergeInfo.masterCol, mergeInfo.colSpan);
      selectionHeight = this.getMergedHeight(mergeInfo.masterRow, mergeInfo.rowSpan);
    } else {
      // Regular cell selection
      selectionWidth = WasmBridge.getColWidth(col);
      selectionHeight = WasmBridge.getRowHeight(row);
    }

    const pos = this.gridToScreen(selectionRow, selectionCol);

    // Draw selection border
    ctx.strokeStyle = theme.activeCellBorder;
    ctx.lineWidth = theme.activeCellBorderWidth;
    ctx.strokeRect(pos.x, pos.y, selectionWidth, selectionHeight);
  }

  /**
   * Handle canvas resize
   */
  public resize(width: number, height: number): void {
    this.canvas.width = width;
    this.canvas.height = height;
    this.updateViewportSize();
    this.render();
  }
}
