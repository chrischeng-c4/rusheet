import { useEffect, useRef, useState } from 'react';
import { GridController } from '../controller/GridController';
import { defaultTheme, CellPosition } from '../types';

export function useGrid() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const controllerRef = useRef<GridController | null>(null);
  const [activeCell, setActiveCell] = useState<CellPosition>({ row: 0, col: 0 });
  const [isEditing, setIsEditing] = useState(false);
  const [editValue, setEditValue] = useState('');
  const [engine, setEngine] = useState<any>(null);
  const [totalDimensions, setTotalDimensions] = useState({ width: 0, height: 0 });
  
  const scrollCallbackRef = useRef<(x: number, y: number) => void>(() => {});

  // Initialize Engine
  useEffect(() => {
    async function initWasm() {
      try {
        const wasm = await import('../../../../../pkg/rusheet_wasm');
        await wasm.default();
        const eng = new wasm.SpreadsheetEngine();
        
        // Setup initial data
        eng.setCellValue(0, 0, "Test");
        
        setEngine(eng);
      } catch (err) {
        console.error('Failed to load WASM:', err);
      }
    }
    initWasm();
  }, []);

  // Sync Dimensions
  useEffect(() => {
      if (engine) {
          try {
              const dimJson = engine.getDimensions();
              const dim = JSON.parse(dimJson);
              setTotalDimensions({ width: dim.width, height: dim.height });
          } catch(e) {
              console.warn("Failed to get dimensions", e);
          }
      }
  }, [engine]);

  // Initialize Controller
  useEffect(() => {
    if (!canvasRef.current || !engine) return;

    const controller = new GridController(
      canvasRef.current,
      defaultTheme,
      engine,
      {
        onActiveCellChange: (pos) => setActiveCell(pos),
        onEditStart: (pos, val) => {
            setEditValue(val);
            setIsEditing(true);
        },
        onScroll: (x, y) => scrollCallbackRef.current(x, y)
      }
    );

    controllerRef.current = controller;

    // Initial Resize
    if (containerRef.current) {
        const { width, height } = containerRef.current.getBoundingClientRect();
        controller.resize(width, height);
    }

    return () => {
        // cleanup if needed
    };
  }, [engine]);

  // Resize Observer
  useEffect(() => {
    if (!containerRef.current || !controllerRef.current) return;

    const observer = new ResizeObserver(entries => {
      for (const entry of entries) {
        const { width, height } = entry.contentRect;
        controllerRef.current?.resize(width, height);
      }
    });

    observer.observe(containerRef.current);
    return () => observer.disconnect();
  }, [engine]); // Dependency on engine ensures controller exists

  return {
    canvasRef,
    containerRef,
    controller: controllerRef.current,
    activeCell,
    isEditing,
    setIsEditing,
    editValue,
    setEditValue,
    engine,
    totalDimensions,
    scrollCallbackRef
  };
}
