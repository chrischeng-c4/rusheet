'use client';

import React, { KeyboardEvent } from 'react';
import { useGrid } from './hooks/useGrid';
import Toolbar, { CellFormat } from '../../Toolbar'; // Re-use existing Toolbar
import { defaultTheme } from './types';

export default function Grid() {
  const {
    canvasRef,
    containerRef,
    controller,
    activeCell,
    isEditing,
    setIsEditing,
    editValue,
    setEditValue,
    engine,
    totalDimensions,
    scrollCallbackRef
  } = useGrid();

  const scrollerRef = React.useRef<HTMLDivElement>(null);

  // Sync Controller Scroll -> DOM Scroll
  React.useEffect(() => {
      scrollCallbackRef.current = (x, y) => {
          if (scrollerRef.current) {
              scrollerRef.current.scrollLeft = x;
              scrollerRef.current.scrollTop = y;
          }
      };
  }, [scrollCallbackRef]);

  // Sync DOM Scroll -> Controller Scroll
  const handleScroll = (e: React.UIEvent<HTMLDivElement>) => {
      const target = e.currentTarget;
      controller?.setScroll(target.scrollLeft, target.scrollTop);
  };

  // Toolbar State Helpers (Simplified for now)
  const [canUndo, setCanUndo] = React.useState(false);
  const [canRedo, setCanRedo] = React.useState(false);
  const [currentFormat, setCurrentFormat] = React.useState<CellFormat>({
    bold: false, italic: false, underline: false
  });

  // Sync Toolbar State
  React.useEffect(() => {
    if (engine) {
        setCanUndo(engine.canUndo());
        setCanRedo(engine.canRedo());
        // Fetch format for active cell
        // ... (Similar logic to previous Spreadsheet.tsx)
    }
  }, [activeCell, engine]);

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (isEditing) {
        if (e.key === 'Enter') {
            if (engine) {
                engine.setCellValue(activeCell.row, activeCell.col, editValue);
                controller?.render();
            }
            setIsEditing(false);
            controller?.handleKeyDown(e.nativeEvent); // Move selection
        } else if (e.key === 'Escape') {
            setIsEditing(false);
        }
        return;
    }

    // Pass to controller
    controller?.handleKeyDown(e.nativeEvent);
  };

  const handleFormatChange = (fmt: Partial<CellFormat>) => {
      if (!engine) return;
      // Convert and apply format
      const apiFormat: any = {};
      if (fmt.bold !== undefined) apiFormat.bold = fmt.bold;
      if (fmt.italic !== undefined) apiFormat.italic = fmt.italic;
      if (fmt.underline !== undefined) apiFormat.underline = fmt.underline;
      if (fmt.textColor !== undefined) apiFormat.text_color = fmt.textColor;
      if (fmt.backgroundColor !== undefined) apiFormat.background_color = fmt.backgroundColor;
      if (fmt.horizontalAlign !== undefined) apiFormat.horizontal_align = fmt.horizontalAlign;
      
      engine.setCellFormat(activeCell.row, activeCell.col, JSON.stringify(apiFormat));
      controller?.render();
  };

  // Render Editor Overlay
  const renderEditor = () => {
      if (!isEditing || !controller || !scrollerRef.current) return null;

      // Positioning logic using scroll offsets
      const scrollX = scrollerRef.current.scrollLeft;
      const scrollY = scrollerRef.current.scrollTop;
      
      return (
          <textarea
            value={editValue}
            onChange={(e) => setEditValue(e.target.value)}
            onKeyDown={handleKeyDown}
            autoFocus
            className="absolute z-10 border-2 border-blue-500 p-1 text-sm resize-none focus:outline-none"
            style={{
                left: defaultTheme.headerWidth + activeCell.col * defaultTheme.defaultColWidth - scrollX, 
                top: defaultTheme.headerHeight + activeCell.row * defaultTheme.defaultRowHeight - scrollY,
                width: defaultTheme.defaultColWidth,
                height: defaultTheme.defaultRowHeight,
            }}
          />
      );
  };

  return (
    <div className="flex flex-col h-full w-full bg-white">
      <Toolbar 
        onUndo={() => { engine?.undo(); controller?.render(); }}
        onRedo={() => { engine?.redo(); controller?.render(); }}
        canUndo={canUndo}
        canRedo={canRedo}
        onFormatChange={handleFormatChange}
        currentFormat={currentFormat}
      />
      
      {/* Formula Bar */}
      <div className="flex items-center h-8 border-b border-gray-300 bg-gray-50 px-2 gap-2">
         <span className="text-xs text-gray-500">
            {String.fromCharCode(65 + activeCell.col)}{activeCell.row + 1}
         </span>
         <input 
            className="flex-1 text-sm border px-1"
            value={isEditing ? editValue : ''}
            onChange={(e) => setEditValue(e.target.value)}
            placeholder="Formula..."
         />
      </div>

      <div 
        ref={containerRef} 
        className="flex-1 relative overflow-hidden outline-none"
        tabIndex={0}
        onKeyDown={handleKeyDown}
      >
        <canvas ref={canvasRef} className="absolute inset-0 block pointer-events-none" />
        
        {/* Virtual Scroller (The Interaction Layer) */}
        <div 
            ref={scrollerRef}
            className="absolute inset-0 overflow-auto"
            onScroll={handleScroll}
        >
            <div style={{ 
                width: Math.max(totalDimensions.width + defaultTheme.headerWidth, 1), 
                height: Math.max(totalDimensions.height + defaultTheme.headerHeight, 1) 
            }} />
        </div>

        {renderEditor()}
      </div>
    </div>
  );
}
