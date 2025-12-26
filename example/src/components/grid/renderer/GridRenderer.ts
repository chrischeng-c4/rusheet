import { CellPosition, CellRange, GridTheme, ViewportState } from '../types';

export class GridRenderer {
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D;
  private theme: GridTheme;
  private engine: any; // WASM Engine type
  private dpr: number = 1;

  constructor(canvas: HTMLCanvasElement, theme: GridTheme, engine: any) {
    this.canvas = canvas;
    this.ctx = canvas.getContext('2d', { alpha: false })!;
    this.theme = theme;
    this.engine = engine;
    
    this.setupHighDPI();
  }

  private setupHighDPI() {
    this.dpr = window.devicePixelRatio || 1;
    // Note: Actual resizing happens in resize()
  }

  public resize(width: number, height: number) {
    this.canvas.style.width = `${width}px`;
    this.canvas.style.height = `${height}px`;
    
    this.canvas.width = width * this.dpr;
    this.canvas.height = height * this.dpr;
    
    this.ctx.scale(this.dpr, this.dpr);
  }

  public render(
    viewport: ViewportState,
    activeCell: CellPosition,
    selectionRange?: CellRange
  ) {
    if (!this.engine) return;

    const { width, height } = viewport;
    const { scrollX, scrollY } = viewport;

    // 1. Clear Background
    this.ctx.fillStyle = '#ffffff';
    this.ctx.fillRect(0, 0, width, height);

    // Calculate visible range (naive calculation for now, will use Fenwick in future)
    const startCol = Math.floor(scrollX / this.theme.defaultColWidth);
    const startRow = Math.floor(scrollY / this.theme.defaultRowHeight);
    const endCol = startCol + Math.ceil((width - this.theme.headerWidth) / this.theme.defaultColWidth) + 1;
    const endRow = startRow + Math.ceil((height - this.theme.headerHeight) / this.theme.defaultRowHeight) + 1;

    // 2. Draw Grid & Content
    this.drawGridAndContent(startRow, endRow, startCol, endCol, scrollX, scrollY, width, height);

    // 3. Draw Headers (Fixed)
    this.drawHeaders(startRow, endRow, startCol, endCol, scrollX, scrollY, width, height);

    // 4. Draw Selection & Active Cell
    this.drawSelection(activeCell, scrollX, scrollY);
  }

  private drawGridAndContent(
    startRow: number, endRow: number, 
    startCol: number, endCol: number,
    scrollX: number, scrollY: number,
    width: number, height: number
  ) {
    // Grid Lines
    this.ctx.strokeStyle = this.theme.gridLineColor;
    this.ctx.lineWidth = 1;

    // Vertical
    for (let col = startCol; col <= endCol; col++) {
      const x = this.theme.headerWidth + col * this.theme.defaultColWidth - scrollX;
      if (x >= this.theme.headerWidth) {
        this.ctx.beginPath();
        this.ctx.moveTo(x, this.theme.headerHeight);
        this.ctx.lineTo(x, height);
        this.ctx.stroke();
      }
    }

    // Horizontal
    for (let row = startRow; row <= endRow; row++) {
      const y = this.theme.headerHeight + row * this.theme.defaultRowHeight - scrollY;
      if (y >= this.theme.headerHeight) {
        this.ctx.beginPath();
        this.ctx.moveTo(this.theme.headerWidth, y);
        this.ctx.lineTo(width, y);
        this.ctx.stroke();
      }
    }

    // Content
    this.ctx.font = this.theme.cellFont;
    this.ctx.textBaseline = 'middle';

    // Batch fetch data ideally, but for now iterate
    for (let row = startRow; row <= endRow; row++) {
      for (let col = startCol; col <= endCol; col++) {
        try {
          const cellDataRaw = this.engine.getCellData(row, col);
          if (cellDataRaw) {
             // Handle raw object directly (serde_wasm_bindgen returns JS object)
             let data = cellDataRaw; 
             // If it happens to be a string (from JSON.stringify), parse it
             if (typeof cellDataRaw === 'string') {
                 data = JSON.parse(cellDataRaw);
             }

             const cellX = this.theme.headerWidth + col * this.theme.defaultColWidth - scrollX;
             const cellY = this.theme.headerHeight + row * this.theme.defaultRowHeight - scrollY;

             // Background
             if (data.format?.background_color) {
                 this.ctx.fillStyle = data.format.background_color;
                 this.ctx.fillRect(cellX, cellY, this.theme.defaultColWidth, this.theme.defaultRowHeight);
             }

             // Text
             let fontStyle = '';
             if (data.format?.bold) fontStyle += 'bold ';
             if (data.format?.italic) fontStyle += 'italic ';
             const fontSize = data.format?.font_size || 13;
             this.ctx.font = `${fontStyle}${fontSize}px Arial`;
             this.ctx.fillStyle = data.format?.text_color || this.theme.cellTextColor;

             const hAlign = data.format?.horizontal_align || 'left';
             this.ctx.textAlign = hAlign as CanvasTextAlign;

             let textX = cellX + this.theme.cellPadding;
             if (hAlign === 'center') textX = cellX + this.theme.defaultColWidth / 2;
             if (hAlign === 'right') textX = cellX + this.theme.defaultColWidth - this.theme.cellPadding;

             const textY = cellY + this.theme.defaultRowHeight / 2;

             // Clipping
             this.ctx.save();
             this.ctx.beginPath();
             this.ctx.rect(cellX, cellY, this.theme.defaultColWidth, this.theme.defaultRowHeight);
             this.ctx.clip();
             
             this.ctx.fillText(data.display_value || '', textX, textY);
             
             // Underline
             if (data.format?.underline) {
                 const textWidth = this.ctx.measureText(data.display_value || '').width;
                 let lineX = textX;
                 if (hAlign === 'center') lineX -= textWidth / 2;
                 if (hAlign === 'right') lineX -= textWidth;
                 this.ctx.fillRect(lineX, textY + fontSize/2 + 1, textWidth, 1);
             }

             this.ctx.restore();
          }
        } catch (e) {
          // Ignore empty cells
        }
      }
    }
  }

  private drawHeaders(
    startRow: number, endRow: number,
    startCol: number, endCol: number,
    scrollX: number, scrollY: number,
    width: number, height: number
  ) {
    // Fill Header Backgrounds
    this.ctx.fillStyle = this.theme.headerBackground;
    this.ctx.fillRect(0, 0, this.theme.headerWidth, height); // Row Header BG
    this.ctx.fillRect(0, 0, width, this.theme.headerHeight); // Col Header BG
    this.ctx.fillRect(0, 0, this.theme.headerWidth, this.theme.headerHeight); // Corner BG

    this.ctx.font = this.theme.headerFont;
    this.ctx.fillStyle = this.theme.headerTextColor;
    this.ctx.textAlign = 'center';
    this.ctx.textBaseline = 'middle';

    // Column Headers (A, B, C...)
    for (let col = startCol; col <= endCol; col++) {
      const x = this.theme.headerWidth + col * this.theme.defaultColWidth - scrollX + this.theme.defaultColWidth / 2;
      const letter = String.fromCharCode(65 + (col % 26)); // Simplified
      this.ctx.fillText(letter, x, this.theme.headerHeight / 2);
      
      // Separator
      const lineX = this.theme.headerWidth + (col + 1) * this.theme.defaultColWidth - scrollX;
      this.ctx.beginPath();
      this.ctx.strokeStyle = '#d1d5db';
      this.ctx.moveTo(lineX, 0);
      this.ctx.lineTo(lineX, this.theme.headerHeight);
      this.ctx.stroke();
    }

    // Row Headers (1, 2, 3...)
    for (let row = startRow; row <= endRow; row++) {
      const y = this.theme.headerHeight + row * this.theme.defaultRowHeight - scrollY + this.theme.defaultRowHeight / 2;
      this.ctx.fillText(String(row + 1), this.theme.headerWidth / 2, y);

      // Separator
      const lineY = this.theme.headerHeight + (row + 1) * this.theme.defaultRowHeight - scrollY;
      this.ctx.beginPath();
      this.ctx.strokeStyle = '#d1d5db';
      this.ctx.moveTo(0, lineY);
      this.ctx.lineTo(this.theme.headerWidth, lineY);
      this.ctx.stroke();
    }
  }

  private drawSelection(activeCell: CellPosition, scrollX: number, scrollY: number) {
    const cellX = this.theme.headerWidth + activeCell.col * this.theme.defaultColWidth - scrollX;
    const cellY = this.theme.headerHeight + activeCell.row * this.theme.defaultRowHeight - scrollY;

    this.ctx.strokeStyle = this.theme.activeCellBorder;
    this.ctx.lineWidth = this.theme.selectionBorderWidth;
    this.ctx.strokeRect(cellX, cellY, this.theme.defaultColWidth, this.theme.defaultRowHeight);
    
    // Fill Handle
    const size = 6;
    this.ctx.fillStyle = this.theme.activeCellBorder;
    this.ctx.fillRect(
        cellX + this.theme.defaultColWidth - size/2,
        cellY + this.theme.defaultRowHeight - size/2,
        size, size
    );
  }
}
