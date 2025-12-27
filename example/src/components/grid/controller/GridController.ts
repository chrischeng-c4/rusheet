import { CellPosition, CellRange, GridTheme, ViewportState } from '../types';
import { GridRenderer } from '../renderer/GridRenderer';

export class GridController {
  private renderer: GridRenderer;
  private canvas: HTMLCanvasElement;
  private interactionTarget: HTMLElement;
  private theme: GridTheme;
  private engine: any;

  // State
  private viewport: ViewportState = { scrollX: 0, scrollY: 0, width: 0, height: 0 };
  private activeCell: CellPosition = { row: 0, col: 0 };
  private selectionRange: CellRange | null = null;
  private isDragging = false;
  private maxRows: number = 2000;
  private maxCols: number = 100;

  // Callbacks
  private onActiveCellChange?: (pos: CellPosition) => void;
  private onEditStart?: (pos: CellPosition, value: string) => void;
  private onScroll?: (x: number, y: number) => void;

  constructor(
    canvas: HTMLCanvasElement,
    interactionTarget: HTMLElement,
    theme: GridTheme,
    engine: any,
    callbacks: {
        onActiveCellChange?: (pos: CellPosition) => void;
        onEditStart?: (pos: CellPosition, value: string) => void;
        onScroll?: (x: number, y: number) => void;
    }
  ) {
    this.canvas = canvas;
    this.interactionTarget = interactionTarget;
    this.theme = theme;
    this.engine = engine;
    this.renderer = new GridRenderer(canvas, theme, engine);
    this.onActiveCellChange = callbacks.onActiveCellChange;
    this.onEditStart = callbacks.onEditStart;
    this.onScroll = callbacks.onScroll;

    this.bindEvents();
  }

  public resize(width: number, height: number) {
    this.viewport.width = width;
    this.viewport.height = height;
    this.renderer.resize(width, height);
    this.render();
  }

  public setEngine(engine: any) {
    this.engine = engine;
    // Re-instantiate renderer with new engine
    this.renderer = new GridRenderer(this.canvas, this.theme, engine);
    this.render();
  }

  public setMaxBounds(maxRows: number, maxCols: number) {
    this.maxRows = maxRows;
    this.maxCols = maxCols;
  }

  public getActiveCell(): CellPosition {
    return this.activeCell;
  }

  public setActiveCell(row: number, col: number) {
    this.activeCell = { row, col };
    this.onActiveCellChange?.(this.activeCell);
    this.render();
  }

  public setScroll(x: number, y: number) {
      this.viewport.scrollX = x;
      this.viewport.scrollY = y;
      this.render();
  }

  public render() {
    this.renderer.render(this.viewport, this.activeCell, this.selectionRange || undefined);
  }

  private bindEvents() {
    this.interactionTarget.addEventListener('mousedown', this.handleMouseDown.bind(this));
    this.interactionTarget.addEventListener('dblclick', this.handleDoubleClick.bind(this));
    this.interactionTarget.addEventListener('wheel', this.handleWheel.bind(this));
    // Keyboard events are handled by the container (React) or global listener
  }

  private handleMouseDown(e: MouseEvent) {
    const rect = this.interactionTarget.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    if (x < this.theme.headerWidth || y < this.theme.headerHeight) {
        // Handle Header Click (Selection)
        return;
    }

    const gridX = x - this.theme.headerWidth + this.viewport.scrollX;
    const gridY = y - this.theme.headerHeight + this.viewport.scrollY;

    if (this.engine && this.engine.getCellFromPixel) {
        // Use efficient spatial index from Rust
        // Note: getCellFromPixel returns Uint32Array, which JS sees as [row, col]
        const [row, col] = this.engine.getCellFromPixel(gridX, gridY);
        this.setActiveCell(row, col);
    } else {
        // Fallback for when engine is not ready (or testing)
        const col = Math.floor(gridX / this.theme.defaultColWidth);
        const row = Math.floor(gridY / this.theme.defaultRowHeight);
        this.setActiveCell(row, col);
    }
    
    // TODO: Drag start logic
  }

  private handleDoubleClick(e: MouseEvent) {
    // Trigger Edit Mode
    if (this.engine) {
        try {
            const cellData = this.engine.getCellData(this.activeCell.row, this.activeCell.col);
            let value = '';
            if (cellData) {
                // Ensure we get the raw formula if present
                const data = typeof cellData === 'string' ? JSON.parse(cellData) : cellData;
                value = data.formula || data.displayValue || '';
            }
            this.onEditStart?.(this.activeCell, value);
        } catch(e) {
            this.onEditStart?.(this.activeCell, '');
        }
    }
  }

  private handleWheel(e: WheelEvent) {
    e.preventDefault();
    this.viewport.scrollX = Math.max(0, this.viewport.scrollX + e.deltaX);
    this.viewport.scrollY = Math.max(0, this.viewport.scrollY + e.deltaY);
    
    this.onScroll?.(this.viewport.scrollX, this.viewport.scrollY);
    this.render();
  }

  public handleKeyDown(e: KeyboardEvent) {
      // Basic Navigation
      let { row, col } = this.activeCell;

      // Prevent default for navigation keys BEFORE processing
      const navKeys = ['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight', 'Tab', 'Enter'];
      if (navKeys.includes(e.key)) {
        e.preventDefault();
      }

      switch(e.key) {
        case 'ArrowUp': row = Math.max(0, row - 1); break;
        case 'ArrowDown': row = Math.min(this.maxRows - 1, row + 1); break;
        case 'ArrowLeft': col = Math.max(0, col - 1); break;
        case 'ArrowRight': col = Math.min(this.maxCols - 1, col + 1); break;
        case 'Enter':
            // If not editing, move down. If Shift, move up.
            if (e.shiftKey) {
              row = Math.max(0, row - 1);
            } else {
              row = Math.min(this.maxRows - 1, row + 1);
            }
            break;
        case 'Tab':
            if (e.shiftKey) {
              col = Math.max(0, col - 1);
            } else {
              col = Math.min(this.maxCols - 1, col + 1);
            }
            break;
        case 'Delete':
        case 'Backspace':
            if (this.engine) {
                this.engine.setCellValue(row, col, '');
                this.render();
            }
            return; // Don't move
        default:
            return; // Allow other keys
      }

      this.setActiveCell(row, col);
  }

  public async handleCopy(): Promise<void> {
    // Copy active cell value to system clipboard
    const { row, col } = this.activeCell;

    if (!this.engine) return;

    try {
      // Get cell data
      const cellData = this.engine.getCellData(row, col);

      if (!cellData) {
        // Empty cell - copy empty string
        await navigator.clipboard.writeText('');
        console.log('[GridController] Copied empty cell');
        return;
      }

      // Parse cell data
      const data = typeof cellData === 'string' ? JSON.parse(cellData) : cellData;
      const value = data.displayValue || '';

      // Write to system clipboard
      await navigator.clipboard.writeText(value);
      console.log('[GridController] Copied cell value:', value);
    } catch (error) {
      console.error('[GridController] Copy failed:', error);
    }
  }

  public async handlePaste(): Promise<void> {
    // Paste from system clipboard to active cell
    const { row, col } = this.activeCell;

    if (!this.engine) return;

    try {
      // Read from system clipboard
      const clipboardText = await navigator.clipboard.readText();

      // For MVP: treat as single value (no tab/newline parsing)
      const value = clipboardText.trim();

      // Set cell value
      this.engine.setCellValue(row, col, value);
      this.render();
      console.log('[GridController] Pasted value:', value);
    } catch (error) {
      console.error('[GridController] Paste failed:', error);

      // Handle common clipboard errors
      if (error instanceof DOMException) {
        if (error.name === 'NotAllowedError') {
          console.warn('[GridController] Clipboard access denied - check browser permissions');
        }
      }
    }
  }
}
