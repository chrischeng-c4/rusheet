import React, {
  useRef,
  useEffect,
  useImperativeHandle,
  forwardRef,
  useState,
  useCallback,
} from 'react';
import { rusheet, type CellChangeEvent, type SelectionChangeEvent } from '../core/RusheetAPI';
import GridRenderer from '../canvas/GridRenderer';
import InputController from '../ui/InputController';
import CellEditor from '../ui/CellEditor';
import {
  initCollaboration,
  disconnectCollaboration,
  type CollaborationConfig,
} from '../collab';
import type { CellData, CellFormat } from '../types';

export interface RuSheetProps {
  /** Initial data as 2D array */
  initialData?: (string | number | null)[][];
  /** Initial JSON state (from serialize()) */
  initialState?: string;
  /** Callback when any cell changes */
  onChange?: (event: CellChangeEvent) => void;
  /** Callback when selection changes */
  onSelectionChange?: (event: SelectionChangeEvent) => void;
  /** Callback when spreadsheet is ready */
  onReady?: () => void;
  /** Collaboration configuration */
  collaboration?: CollaborationConfig;
  /** Width of the component */
  width?: number | string;
  /** Height of the component */
  height?: number | string;
  /** Custom class name */
  className?: string;
  /** Custom styles */
  style?: React.CSSProperties;
  /** Show formula bar */
  showFormulaBar?: boolean;
  /** Show sheet tabs */
  showSheetTabs?: boolean;
  /** Read-only mode */
  readOnly?: boolean;
}

export interface RuSheetRef {
  /** Get cell data */
  getCellData: (row: number, col: number) => CellData | null;
  /** Set cell value */
  setCellValue: (row: number, col: number, value: string) => void;
  /** Set cell format */
  setCellFormat: (row: number, col: number, format: CellFormat) => void;
  /** Set range format */
  setRangeFormat: (
    startRow: number,
    startCol: number,
    endRow: number,
    endCol: number,
    format: CellFormat
  ) => void;
  /** Clear a range */
  clearRange: (startRow: number, startCol: number, endRow: number, endCol: number) => void;
  /** Insert rows */
  insertRows: (atRow: number, count: number) => void;
  /** Delete rows */
  deleteRows: (atRow: number, count: number) => void;
  /** Insert columns */
  insertCols: (atCol: number, count: number) => void;
  /** Delete columns */
  deleteCols: (atCol: number, count: number) => void;
  /** Add a sheet */
  addSheet: (name: string) => number;
  /** Delete a sheet */
  deleteSheet: (index: number) => boolean;
  /** Get sheet names */
  getSheetNames: () => string[];
  /** Set active sheet */
  setActiveSheet: (index: number) => boolean;
  /** Undo */
  undo: () => void;
  /** Redo */
  redo: () => void;
  /** Check if can undo */
  canUndo: () => boolean;
  /** Check if can redo */
  canRedo: () => boolean;
  /** Serialize to JSON */
  serialize: () => string;
  /** Deserialize from JSON */
  deserialize: (json: string) => boolean;
  /** Get all data as 2D array */
  getData: (startRow?: number, endRow?: number, startCol?: number, endCol?: number) => (string | null)[][];
  /** Set data from 2D array */
  setData: (data: (string | number | null)[][]) => void;
  /** Force re-render */
  render: () => void;
}

/**
 * RuSheet React Component
 *
 * A high-performance spreadsheet component built with Rust/WASM.
 *
 * @example
 * ```tsx
 * import { RuSheet, RuSheetRef } from 'rusheet/react';
 *
 * function App() {
 *   const sheetRef = useRef<RuSheetRef>(null);
 *
 *   return (
 *     <RuSheet
 *       ref={sheetRef}
 *       initialData={[
 *         ['Name', 'Age', 'City'],
 *         ['Alice', 30, 'NYC'],
 *         ['Bob', 25, 'LA'],
 *       ]}
 *       onChange={(e) => console.log('Changed:', e)}
 *       width="100%"
 *       height={500}
 *     />
 *   );
 * }
 * ```
 */
export const RuSheet = forwardRef<RuSheetRef, RuSheetProps>(function RuSheet(
  {
    initialData,
    initialState,
    onChange,
    onSelectionChange,
    onReady,
    collaboration,
    width = '100%',
    height = 400,
    className,
    style,
    showFormulaBar = true,
    showSheetTabs = true,
    readOnly = false,
  },
  ref
) {
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const formulaInputRef = useRef<HTMLInputElement>(null);
  const rendererRef = useRef<GridRenderer | null>(null);
  const cellEditorRef = useRef<CellEditor | null>(null);
  const [isReady, setIsReady] = useState(false);
  const [cellAddress, setCellAddress] = useState('A1');

  // Convert column index to letter
  const colToLetter = useCallback((col: number): string => {
    let result = '';
    let num = col;
    while (num >= 0) {
      result = String.fromCharCode(65 + (num % 26)) + result;
      num = Math.floor(num / 26) - 1;
      if (num < 0) break;
    }
    return result;
  }, []);

  // Initialize the spreadsheet
  useEffect(() => {
    let mounted = true;
    const unsubscribers: (() => void)[] = [];

    const init = async () => {
      if (!canvasRef.current || !containerRef.current) return;

      try {
        // Initialize WASM
        await rusheet.init();

        if (!mounted) return;

        // Create renderer
        const renderer = new GridRenderer(canvasRef.current);
        rendererRef.current = renderer;

        // Create cell editor
        if (formulaInputRef.current) {
          const editor = new CellEditor(
            containerRef.current,
            renderer,
            formulaInputRef.current
          );
          cellEditorRef.current = editor;

          // Set up edit callback
          editor.setCellChangeCallback((row: number, col: number) => {
            setCellAddress(`${colToLetter(col)}${row + 1}`);
            renderer.render();
          });
        }

        // Create input controller (if not read-only)
        if (!readOnly && cellEditorRef.current) {
          new InputController(canvasRef.current, renderer, (row, col) => {
            cellEditorRef.current?.activate(row, col);
          });
        }

        // Load initial state
        if (initialState) {
          rusheet.deserialize(initialState);
        } else if (initialData) {
          rusheet.setData(initialData);
        }

        // Subscribe to events
        if (onChange) {
          unsubscribers.push(rusheet.onChange(onChange));
        }

        if (onSelectionChange) {
          unsubscribers.push(rusheet.onSelectionChange((event) => {
            setCellAddress(`${colToLetter(event.col)}${event.row + 1}`);
            onSelectionChange(event);
          }));
        } else {
          unsubscribers.push(rusheet.onSelectionChange((event) => {
            setCellAddress(`${colToLetter(event.col)}${event.row + 1}`);
          }));
        }

        // Initialize collaboration if configured
        if (collaboration) {
          initCollaboration(collaboration);
        }

        // Initial render
        renderer.render();
        setIsReady(true);

        if (onReady) {
          onReady();
        }
      } catch (error) {
        console.error('[RuSheet] Initialization failed:', error);
      }
    };

    init();

    return () => {
      mounted = false;
      unsubscribers.forEach((unsub) => unsub());
      if (collaboration) {
        disconnectCollaboration();
      }
    };
  }, []); // Only run once on mount

  // Handle resize
  useEffect(() => {
    if (!canvasRef.current || !containerRef.current || !rendererRef.current) return;

    const handleResize = () => {
      if (!canvasRef.current || !containerRef.current || !rendererRef.current) return;

      const rect = containerRef.current.getBoundingClientRect();
      const headerHeight = showFormulaBar ? 32 : 0;
      const footerHeight = showSheetTabs ? 32 : 0;

      canvasRef.current.width = rect.width;
      canvasRef.current.height = rect.height - headerHeight - footerHeight;

      rendererRef.current.updateViewportSize();
      rendererRef.current.render();
      cellEditorRef.current?.updatePosition();
    };

    // Initial resize
    handleResize();

    // Listen for resize
    const resizeObserver = new ResizeObserver(handleResize);
    resizeObserver.observe(containerRef.current);

    return () => {
      resizeObserver.disconnect();
    };
  }, [showFormulaBar, showSheetTabs, isReady]);

  // Expose API via ref
  useImperativeHandle(ref, () => ({
    getCellData: (row, col) => rusheet.getCellData(row, col),
    setCellValue: (row, col, value) => {
      rusheet.setCellValue(row, col, value, 'api');
      rendererRef.current?.render();
    },
    setCellFormat: (row, col, format) => {
      rusheet.setCellFormat(row, col, format, 'api');
      rendererRef.current?.render();
    },
    setRangeFormat: (startRow, startCol, endRow, endCol, format) => {
      rusheet.setRangeFormat(startRow, startCol, endRow, endCol, format, 'api');
      rendererRef.current?.render();
    },
    clearRange: (startRow, startCol, endRow, endCol) => {
      rusheet.clearRange(startRow, startCol, endRow, endCol);
      rendererRef.current?.render();
    },
    insertRows: (atRow, count) => {
      rusheet.insertRows(atRow, count, 'api');
      rendererRef.current?.render();
    },
    deleteRows: (atRow, count) => {
      rusheet.deleteRows(atRow, count, 'api');
      rendererRef.current?.render();
    },
    insertCols: (atCol, count) => {
      rusheet.insertCols(atCol, count, 'api');
      rendererRef.current?.render();
    },
    deleteCols: (atCol, count) => {
      rusheet.deleteCols(atCol, count, 'api');
      rendererRef.current?.render();
    },
    addSheet: (name) => rusheet.addSheet(name, 'api'),
    deleteSheet: (index) => rusheet.deleteSheet(index, 'api'),
    getSheetNames: () => rusheet.getSheetNames(),
    setActiveSheet: (index) => {
      const result = rusheet.setActiveSheet(index, 'api');
      rendererRef.current?.render();
      return result;
    },
    undo: () => {
      rusheet.undo();
      rendererRef.current?.render();
    },
    redo: () => {
      rusheet.redo();
      rendererRef.current?.render();
    },
    canUndo: () => rusheet.canUndo(),
    canRedo: () => rusheet.canRedo(),
    serialize: () => rusheet.serialize(),
    deserialize: (json) => {
      const result = rusheet.deserialize(json);
      rendererRef.current?.render();
      return result;
    },
    getData: (startRow = 0, endRow = 999, startCol = 0, endCol = 25) =>
      rusheet.getData(startRow, endRow, startCol, endCol),
    setData: (data) => {
      rusheet.setData(data);
      rendererRef.current?.render();
    },
    render: () => rendererRef.current?.render(),
  }), []);

  const containerStyle: React.CSSProperties = {
    width,
    height,
    display: 'flex',
    flexDirection: 'column',
    fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
    fontSize: 13,
    border: '1px solid #e0e0e0',
    borderRadius: 4,
    overflow: 'hidden',
    ...style,
  };

  const headerStyle: React.CSSProperties = {
    display: showFormulaBar ? 'flex' : 'none',
    alignItems: 'center',
    height: 32,
    padding: '0 8px',
    borderBottom: '1px solid #e0e0e0',
    backgroundColor: '#f8f9fa',
    gap: 8,
  };

  const cellAddressStyle: React.CSSProperties = {
    minWidth: 60,
    padding: '4px 8px',
    backgroundColor: '#fff',
    border: '1px solid #ddd',
    borderRadius: 2,
    textAlign: 'center',
    fontWeight: 500,
  };

  const formulaInputStyle: React.CSSProperties = {
    flex: 1,
    padding: '4px 8px',
    border: '1px solid #ddd',
    borderRadius: 2,
    outline: 'none',
  };

  const canvasContainerStyle: React.CSSProperties = {
    flex: 1,
    position: 'relative',
    overflow: 'hidden',
  };

  const footerStyle: React.CSSProperties = {
    display: showSheetTabs ? 'flex' : 'none',
    alignItems: 'center',
    height: 32,
    padding: '0 8px',
    borderTop: '1px solid #e0e0e0',
    backgroundColor: '#f8f9fa',
    gap: 4,
  };

  return (
    <div ref={containerRef} className={className} style={containerStyle}>
      {/* Formula Bar */}
      <div style={headerStyle}>
        <span style={cellAddressStyle}>{cellAddress}</span>
        <input
          ref={formulaInputRef}
          type="text"
          style={formulaInputStyle}
          placeholder="Enter value or formula"
          readOnly={readOnly}
        />
      </div>

      {/* Canvas */}
      <div style={canvasContainerStyle}>
        <canvas ref={canvasRef} style={{ display: 'block' }} />
      </div>

      {/* Sheet Tabs */}
      <div style={footerStyle}>
        <div
          style={{
            padding: '4px 12px',
            backgroundColor: '#fff',
            border: '1px solid #ddd',
            borderBottom: 'none',
            borderRadius: '4px 4px 0 0',
            cursor: 'pointer',
          }}
        >
          Sheet1
        </div>
      </div>
    </div>
  );
});

export default RuSheet;
