'use client';

import { useEffect, useRef, useState } from 'react';

// Theme constants
const theme = {
  headerBackground: '#f3f4f6',
  headerFont: 'bold 12px Arial',
  headerTextColor: '#374151',
  cellFont: '13px Arial',
  cellTextColor: '#111827',
  gridLineColor: '#e5e7eb',
  activeCellBorder: '#2563eb',
  selectionBorderWidth: 2,
  defaultColWidth: 100,
  defaultRowHeight: 24,
  headerHeight: 24,
  headerWidth: 50,
  cellPadding: 4,
};

interface CellPosition {
  row: number;
  col: number;
}

export default function Spreadsheet() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const [engine, setEngine] = useState<any>(null);
  const [activeCell, setActiveCell] = useState<CellPosition>({ row: 0, col: 0 });
  const [isEditing, setIsEditing] = useState(false);
  const [editValue, setEditValue] = useState('');
  const [scrollOffset, setScrollOffset] = useState({ x: 0, y: 0 });

  // Initialize WASM
  useEffect(() => {
    async function initWasm() {
      try {
        const wasm = await import('../pkg/rusheet_wasm');
        await wasm.default();
        const eng = new wasm.SpreadsheetEngine();

        // Add sample data
        eng.set_cell_value(0, 0, 'Product');
        eng.set_cell_value(0, 1, 'Qty');
        eng.set_cell_value(0, 2, 'Price');
        eng.set_cell_value(0, 3, 'Total');

        eng.set_cell_value(1, 0, 'Apples');
        eng.set_cell_value(1, 1, '10');
        eng.set_cell_value(1, 2, '2.50');
        eng.set_cell_value(1, 3, '=B2*C2');

        eng.set_cell_value(2, 0, 'Oranges');
        eng.set_cell_value(2, 1, '15');
        eng.set_cell_value(2, 2, '3.00');
        eng.set_cell_value(2, 3, '=B3*C3');

        eng.set_cell_value(3, 0, 'Bananas');
        eng.set_cell_value(3, 1, '20');
        eng.set_cell_value(3, 2, '1.50');
        eng.set_cell_value(3, 3, '=B4*C4');

        eng.set_cell_value(5, 0, 'Total:');
        eng.set_cell_value(5, 3, '=SUM(D2:D4)');

        setEngine(eng);
      } catch (err) {
        console.error('Failed to load WASM:', err);
      }
    }
    initWasm();
  }, []);

  // Render grid
  useEffect(() => {
    if (!canvasRef.current || !engine) return;

    const canvas = canvasRef.current;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const render = () => {
      const { width, height } = canvas;

      // Clear canvas
      ctx.fillStyle = '#ffffff';
      ctx.fillRect(0, 0, width, height);

      // Calculate visible cells
      const startCol = Math.floor(scrollOffset.x / theme.defaultColWidth);
      const startRow = Math.floor(scrollOffset.y / theme.defaultRowHeight);
      const endCol = startCol + Math.ceil((width - theme.headerWidth) / theme.defaultColWidth) + 1;
      const endRow = startRow + Math.ceil((height - theme.headerHeight) / theme.defaultRowHeight) + 1;

      // Draw grid lines
      ctx.strokeStyle = theme.gridLineColor;
      ctx.lineWidth = 1;

      // Vertical lines
      for (let col = startCol; col <= endCol; col++) {
        const x = theme.headerWidth + col * theme.defaultColWidth - scrollOffset.x;
        ctx.beginPath();
        ctx.moveTo(x, theme.headerHeight);
        ctx.lineTo(x, height);
        ctx.stroke();
      }

      // Horizontal lines
      for (let row = startRow; row <= endRow; row++) {
        const y = theme.headerHeight + row * theme.defaultRowHeight - scrollOffset.y;
        ctx.beginPath();
        ctx.moveTo(theme.headerWidth, y);
        ctx.lineTo(width, y);
        ctx.stroke();
      }

      // Draw cell content
      ctx.font = theme.cellFont;
      ctx.fillStyle = theme.cellTextColor;
      ctx.textBaseline = 'middle';

      for (let row = startRow; row <= endRow; row++) {
        for (let col = startCol; col <= endCol; col++) {
          try {
            const cellData = engine.get_cell_data(row, col);
            if (cellData) {
              const data = JSON.parse(cellData);
              const x = theme.headerWidth + col * theme.defaultColWidth - scrollOffset.x + theme.cellPadding;
              const y = theme.headerHeight + row * theme.defaultRowHeight - scrollOffset.y + theme.defaultRowHeight / 2;

              // Clip text
              ctx.save();
              ctx.beginPath();
              ctx.rect(
                theme.headerWidth + col * theme.defaultColWidth - scrollOffset.x,
                theme.headerHeight + row * theme.defaultRowHeight - scrollOffset.y,
                theme.defaultColWidth,
                theme.defaultRowHeight
              );
              ctx.clip();
              ctx.fillText(data.display_value || '', x, y);
              ctx.restore();
            }
          } catch {
            // Cell might be empty
          }
        }
      }

      // Draw headers
      ctx.fillStyle = theme.headerBackground;
      ctx.fillRect(0, 0, theme.headerWidth, height);
      ctx.fillRect(0, 0, width, theme.headerHeight);

      ctx.font = theme.headerFont;
      ctx.fillStyle = theme.headerTextColor;
      ctx.textAlign = 'center';

      // Column headers
      for (let col = startCol; col <= endCol; col++) {
        const x = theme.headerWidth + col * theme.defaultColWidth - scrollOffset.x + theme.defaultColWidth / 2;
        const letter = String.fromCharCode(65 + (col % 26));
        ctx.fillText(letter, x, theme.headerHeight / 2);
      }

      // Row headers
      ctx.textAlign = 'center';
      for (let row = startRow; row <= endRow; row++) {
        const y = theme.headerHeight + row * theme.defaultRowHeight - scrollOffset.y + theme.defaultRowHeight / 2;
        ctx.fillText(String(row + 1), theme.headerWidth / 2, y);
      }

      // Draw active cell border
      const cellX = theme.headerWidth + activeCell.col * theme.defaultColWidth - scrollOffset.x;
      const cellY = theme.headerHeight + activeCell.row * theme.defaultRowHeight - scrollOffset.y;

      ctx.strokeStyle = theme.activeCellBorder;
      ctx.lineWidth = theme.selectionBorderWidth;
      ctx.strokeRect(cellX, cellY, theme.defaultColWidth, theme.defaultRowHeight);
    };

    render();
  }, [engine, activeCell, scrollOffset]);

  // Handle canvas resize
  useEffect(() => {
    if (!containerRef.current || !canvasRef.current) return;

    const resizeObserver = new ResizeObserver((entries) => {
      for (const entry of entries) {
        const { width, height } = entry.contentRect;
        if (canvasRef.current) {
          canvasRef.current.width = width;
          canvasRef.current.height = height;
        }
      }
    });

    resizeObserver.observe(containerRef.current);
    return () => resizeObserver.disconnect();
  }, []);

  // Handle click
  const handleClick = (e: React.MouseEvent<HTMLCanvasElement>) => {
    if (!canvasRef.current) return;

    const rect = canvasRef.current.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    if (x < theme.headerWidth || y < theme.headerHeight) return;

    const col = Math.floor((x - theme.headerWidth + scrollOffset.x) / theme.defaultColWidth);
    const row = Math.floor((y - theme.headerHeight + scrollOffset.y) / theme.defaultRowHeight);

    setActiveCell({ row, col });
    setIsEditing(false);
  };

  // Handle double click to edit
  const handleDoubleClick = () => {
    if (!engine) return;

    try {
      const cellData = engine.get_cell_data(activeCell.row, activeCell.col);
      if (cellData) {
        const data = JSON.parse(cellData);
        setEditValue(data.formula || data.display_value || '');
      } else {
        setEditValue('');
      }
    } catch {
      setEditValue('');
    }
    setIsEditing(true);
  };

  // Handle keyboard
  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (isEditing) {
      if (e.key === 'Enter') {
        if (engine) {
          engine.set_cell_value(activeCell.row, activeCell.col, editValue);
        }
        setIsEditing(false);
        setActiveCell({ row: activeCell.row + 1, col: activeCell.col });
      } else if (e.key === 'Escape') {
        setIsEditing(false);
      }
      return;
    }

    switch (e.key) {
      case 'ArrowUp':
        setActiveCell({ row: Math.max(0, activeCell.row - 1), col: activeCell.col });
        break;
      case 'ArrowDown':
        setActiveCell({ row: activeCell.row + 1, col: activeCell.col });
        break;
      case 'ArrowLeft':
        setActiveCell({ row: activeCell.row, col: Math.max(0, activeCell.col - 1) });
        break;
      case 'ArrowRight':
        setActiveCell({ row: activeCell.row, col: activeCell.col + 1 });
        break;
      case 'Enter':
        handleDoubleClick();
        break;
      case 'Delete':
      case 'Backspace':
        if (engine) {
          engine.set_cell_value(activeCell.row, activeCell.col, '');
        }
        break;
    }
  };

  // Handle scroll
  const handleWheel = (e: React.WheelEvent) => {
    e.preventDefault();
    setScrollOffset({
      x: Math.max(0, scrollOffset.x + e.deltaX),
      y: Math.max(0, scrollOffset.y + e.deltaY),
    });
  };

  const colLetter = String.fromCharCode(65 + (activeCell.col % 26));
  const cellAddress = `${colLetter}${activeCell.row + 1}`;

  return (
    <div className="flex flex-col h-full w-full bg-white">
      {/* Formula bar */}
      <div className="flex items-center h-8 border-b border-gray-300 bg-gray-50 px-2 gap-2">
        <span className="text-sm font-medium text-gray-600 w-12">{cellAddress}</span>
        <span className="text-gray-400">|</span>
        <input
          type="text"
          value={isEditing ? editValue : ''}
          onChange={(e) => setEditValue(e.target.value)}
          onFocus={handleDoubleClick}
          onKeyDown={handleKeyDown}
          placeholder="Enter value or formula..."
          className="flex-1 h-6 px-2 text-sm border border-gray-300 rounded focus:outline-none focus:border-blue-500"
        />
      </div>

      {/* Spreadsheet container */}
      <div
        ref={containerRef}
        className="flex-1 relative overflow-hidden"
        tabIndex={0}
        onKeyDown={handleKeyDown}
      >
        <canvas
          ref={canvasRef}
          onClick={handleClick}
          onDoubleClick={handleDoubleClick}
          onWheel={handleWheel}
          className="absolute inset-0"
        />

        {/* Inline editor */}
        {isEditing && (
          <textarea
            value={editValue}
            onChange={(e) => setEditValue(e.target.value)}
            onKeyDown={handleKeyDown}
            onBlur={() => {
              if (engine) {
                engine.set_cell_value(activeCell.row, activeCell.col, editValue);
              }
              setIsEditing(false);
            }}
            autoFocus
            className="absolute border-2 border-blue-500 p-1 text-sm resize-none focus:outline-none"
            style={{
              left: theme.headerWidth + activeCell.col * theme.defaultColWidth - scrollOffset.x,
              top: theme.headerHeight + activeCell.row * theme.defaultRowHeight - scrollOffset.y,
              width: theme.defaultColWidth,
              height: theme.defaultRowHeight,
            }}
          />
        )}
      </div>
    </div>
  );
}
