import { CellPosition, CellRange, GridTheme, ViewportState } from '../types';
import { GridRenderer } from '../renderer/GridRenderer';

export class GridController {
  private renderer: GridRenderer;
  private canvas: HTMLCanvasElement;
  private theme: GridTheme;
  private engine: any;

  // State
  private viewport: ViewportState = { scrollX: 0, scrollY: 0, width: 0, height: 0 };
  private activeCell: CellPosition = { row: 0, col: 0 };
  private selectionRange: CellRange | null = null;
  private isDragging = false;
  
  // Callbacks
  private onActiveCellChange?: (pos: CellPosition) => void;
  private onEditStart?: (pos: CellPosition, value: string) => void;
  private onScroll?: (x: number, y: number) => void;

  constructor(
    canvas: HTMLCanvasElement,
    theme: GridTheme,
    engine: any,
    callbacks: {
        onActiveCellChange?: (pos: CellPosition) => void;
        onEditStart?: (pos: CellPosition, value: string) => void;
        onScroll?: (x: number, y: number) => void;
    }
  ) {
    this.canvas = canvas;
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
    this.canvas.addEventListener('mousedown', this.handleMouseDown.bind(this));
    this.canvas.addEventListener('dblclick', this.handleDoubleClick.bind(this));
    this.canvas.addEventListener('wheel', this.handleWheel.bind(this));
    // Keyboard events are handled by the container (React) or global listener
  }

  private handleMouseDown(e: MouseEvent) {
    const rect = this.canvas.getBoundingClientRect();
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
                value = data.formula || data.display_value || '';
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

      switch(e.key) {
        case 'ArrowUp': row = Math.max(0, row - 1); break;
        case 'ArrowDown': row++; break;
        case 'ArrowLeft': col = Math.max(0, col - 1); break;
        case 'ArrowRight': col++; break;
        case 'Enter': 
            // If not editing, move down. If Shift, move up.
            if (e.shiftKey) row = Math.max(0, row - 1); else row++;
            break;
        case 'Tab':
            if (e.shiftKey) col = Math.max(0, col - 1); else col++;
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
      e.preventDefault();
  }
}
