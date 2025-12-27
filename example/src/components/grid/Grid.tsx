'use client';

import React, { KeyboardEvent } from 'react';
import { useGrid } from './hooks/useGrid';
import Toolbar, { CellFormat } from '../Toolbar'; // Re-use existing Toolbar
import { defaultTheme } from './types';

export default function Grid() {
  const {
    canvasRef,
    containerRef,
    scrollerRef,
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

  const formulaBarRef = React.useRef<HTMLInputElement>(null);
  const [formulaBarValue, setFormulaBarValue] = React.useState('');

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
    if (engine && activeCell) {
        // Update undo/redo state
        setCanUndo(engine.canUndo());
        setCanRedo(engine.canRedo());

        // Fetch and sync format for active cell
        try {
          const cellData = engine.getCellData(activeCell.row, activeCell.col);

          if (cellData) {
            // Handle both raw objects and JSON strings
            const data = typeof cellData === 'string' ? JSON.parse(cellData) : cellData;

            // Build format object with defaults
            const newFormat: CellFormat = {
              bold: data.format?.bold ?? false,
              italic: data.format?.italic ?? false,
              underline: data.format?.underline ?? false,
              fontSize: data.format?.fontSize,
              textColor: data.format?.textColor,
              backgroundColor: data.format?.backgroundColor,
              horizontalAlign: data.format?.horizontalAlign,
              verticalAlign: data.format?.verticalAlign,
            };

            setCurrentFormat(newFormat);
          } else {
            // Empty cell: reset to defaults
            setCurrentFormat({
              bold: false,
              italic: false,
              underline: false,
            });
          }
        } catch (e) {
          console.warn('[Grid] Failed to sync cell format from engine:', e);
          setCurrentFormat({
            bold: false,
            italic: false,
            underline: false,
          });
        }
    }
  }, [activeCell, engine]);

  // Sync formula bar with active cell (when NOT editing)
  React.useEffect(() => {
    if (!isEditing && engine && activeCell) {
      try {
        const cellData = engine.getCellData(activeCell.row, activeCell.col);
        if (cellData) {
          const data = typeof cellData === 'string' ? JSON.parse(cellData) : cellData;
          // Show formula if present, otherwise show display value
          const content = data.formula || data.displayValue || '';
          setFormulaBarValue(content);
        } else {
          setFormulaBarValue('');
        }
      } catch (e) {
        console.warn('[Grid] Failed to get cell data for formula bar:', e);
        setFormulaBarValue('');
      }
    }
  }, [activeCell, isEditing, engine]);

  // Auto-focus formula bar when editing starts
  React.useEffect(() => {
    if (isEditing && formulaBarRef.current) {
      formulaBarRef.current.focus();
      formulaBarRef.current.setSelectionRange(
        formulaBarRef.current.value.length,
        formulaBarRef.current.value.length
      );
    }
  }, [isEditing]);

  // Handle formula bar focus - enter edit mode
  const handleFormulaBarFocus = () => {
    if (!isEditing && engine) {
      try {
        const cellData = engine.getCellData(activeCell.row, activeCell.col);
        let value = '';
        if (cellData) {
          const data = typeof cellData === 'string' ? JSON.parse(cellData) : cellData;
          value = data.formula || data.displayValue || '';
        }
        setEditValue(value);
        setIsEditing(true);
      } catch (e) {
        console.warn('[Grid] Failed to start editing from formula bar:', e);
        setEditValue('');
        setIsEditing(true);
      }
    }
  };

  // Handle formula bar keyboard shortcuts
  const handleFormulaBarKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      e.preventDefault();
      if (engine && controller) {
        console.log('[Grid] Setting cell value from formula bar:', activeCell.row, activeCell.col, editValue);
        engine.setCellValue(activeCell.row, activeCell.col, editValue);
        controller.render();
      }
      setIsEditing(false);
      // Move down after Enter (Google Sheets behavior)
      if (controller) {
        const newRow = Math.min(activeCell.row + 1, 1999); // maxRows - 1
        controller.setActiveCell(newRow, activeCell.col);
      }
    } else if (e.key === 'Escape') {
      e.preventDefault();
      setIsEditing(false);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    // If user starts typing when NOT editing, start editing with that character
    if (!isEditing && e.key.length === 1 && !e.ctrlKey && !e.metaKey && !e.altKey) {
      e.preventDefault();
      setEditValue(e.key);
      setIsEditing(true);
      setTimeout(() => formulaBarRef.current?.focus(), 0);
      return;
    }

    if (isEditing) {
        if (e.key === 'Enter') {
            if (engine && controller) {
                console.log('[Grid] Setting cell value:', activeCell.row, activeCell.col, editValue);
                engine.setCellValue(activeCell.row, activeCell.col, editValue);
                controller.render();
                console.log('[Grid] Cell value set and render called');
            } else {
                console.warn('[Grid] Cannot set cell value - engine or controller is null');
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
    e.stopPropagation(); // Prevent bubbling to avoid double-processing
  };

  const handleFormatChange = (fmt: Partial<CellFormat>) => {
      if (!engine || !controller) {
          console.warn('[Grid] Cannot change format - engine or controller is null');
          return;
      }

      // If currently editing, commit the value first
      if (isEditing && editValue) {
          console.log('[Grid] Committing edit before format change');
          engine.setCellValue(activeCell.row, activeCell.col, editValue);
          setIsEditing(false);
      }

      // Convert and apply format (use camelCase for WASM serialization)
      const apiFormat: any = {};
      if (fmt.bold !== undefined) apiFormat.bold = fmt.bold;
      if (fmt.italic !== undefined) apiFormat.italic = fmt.italic;
      if (fmt.underline !== undefined) apiFormat.underline = fmt.underline;
      if (fmt.textColor !== undefined) apiFormat.textColor = fmt.textColor;
      if (fmt.backgroundColor !== undefined) apiFormat.backgroundColor = fmt.backgroundColor;
      if (fmt.horizontalAlign !== undefined) apiFormat.horizontalAlign = fmt.horizontalAlign;

      console.log('[Grid] Setting cell format:', activeCell.row, activeCell.col, apiFormat);
      engine.setCellFormat(activeCell.row, activeCell.col, JSON.stringify(apiFormat));
      controller.render();
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
            autoFocus
            className="absolute z-20 border-2 border-blue-500 p-1 text-sm resize-none focus:outline-none"
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
        onUndo={() => {
          engine?.undo();
          controller?.render();
          setCanUndo(engine?.canUndo() ?? false);
          setCanRedo(engine?.canRedo() ?? false);
        }}
        onRedo={() => {
          engine?.redo();
          controller?.render();
          setCanUndo(engine?.canUndo() ?? false);
          setCanRedo(engine?.canRedo() ?? false);
        }}
        canUndo={canUndo}
        canRedo={canRedo}
        onFormatChange={handleFormatChange}
        currentFormat={currentFormat}
      />
      
      {/* Formula Bar */}
      <div className="flex items-center h-8 border-b border-gray-300 bg-gray-50 px-2 gap-2">
         <span className="text-xs text-gray-500 font-semibold min-w-[40px]">
            {String.fromCharCode(65 + activeCell.col)}{activeCell.row + 1}
         </span>
         <input
            ref={formulaBarRef}
            className="flex-1 text-sm text-gray-900 border px-2 py-1 rounded focus:outline-none focus:ring-1 focus:ring-blue-500"
            value={isEditing ? editValue : formulaBarValue}
            onChange={(e) => setEditValue(e.target.value)}
            onFocus={handleFormulaBarFocus}
            onKeyDown={handleFormulaBarKeyDown}
            placeholder="Enter value or formula..."
         />
      </div>

      <div 
        ref={containerRef} 
        className="flex-1 relative overflow-hidden outline-none"
        tabIndex={0}
        onKeyDown={handleKeyDown}
      >
        <canvas ref={canvasRef} className="absolute inset-0 block" />
        
        {/* Virtual Scroller (The Interaction Layer) */}
        <div
            ref={scrollerRef}
            className="absolute inset-0 overflow-auto pointer-events-auto"
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
