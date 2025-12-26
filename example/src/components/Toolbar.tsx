import React from 'react';

interface ToolbarProps {
  onUndo: () => void;
  onRedo: () => void;
  canUndo: boolean;
  canRedo: boolean;
  onFormatChange: (format: Partial<CellFormat>) => void;
  currentFormat: CellFormat;
}

export interface CellFormat {
  bold: boolean;
  italic: boolean;
  underline: boolean;
  fontSize?: number;
  textColor?: string;
  backgroundColor?: string;
  horizontalAlign?: 'left' | 'center' | 'right';
  verticalAlign?: 'top' | 'middle' | 'bottom';
}

export default function Toolbar({
  onUndo,
  onRedo,
  canUndo,
  canRedo,
  onFormatChange,
  currentFormat,
}: ToolbarProps) {
  return (
    <div className="flex items-center gap-2 p-2 bg-gray-100 border-b border-gray-300 overflow-x-auto">
      <div className="flex items-center gap-1 border-r border-gray-300 pr-2">
        <button
          onClick={onUndo}
          disabled={!canUndo}
          className={`px-2 py-1 rounded text-sm ${
            !canUndo ? 'text-gray-400 cursor-not-allowed' : 'hover:bg-gray-200 text-gray-700'
          }`}
          title="Undo"
        >
          ↶
        </button>
        <button
          onClick={onRedo}
          disabled={!canRedo}
          className={`px-2 py-1 rounded text-sm ${
            !canRedo ? 'text-gray-400 cursor-not-allowed' : 'hover:bg-gray-200 text-gray-700'
          }`}
          title="Redo"
        >
          ↷
        </button>
      </div>

      <div className="flex items-center gap-1 border-r border-gray-300 pr-2">
        <button
          onClick={() => onFormatChange({ bold: !currentFormat.bold })}
          className={`px-3 py-1 rounded text-sm font-bold ${
            currentFormat.bold ? 'bg-gray-300 text-black' : 'hover:bg-gray-200 text-gray-700'
          }`}
          title="Bold"
        >
          B
        </button>
        <button
          onClick={() => onFormatChange({ italic: !currentFormat.italic })}
          className={`px-3 py-1 rounded text-sm italic ${
            currentFormat.italic ? 'bg-gray-300 text-black' : 'hover:bg-gray-200 text-gray-700'
          }`}
          title="Italic"
        >
          I
        </button>
        <button
          onClick={() => onFormatChange({ underline: !currentFormat.underline })}
          className={`px-3 py-1 rounded text-sm underline ${
            currentFormat.underline ? 'bg-gray-300 text-black' : 'hover:bg-gray-200 text-gray-700'
          }`}
          title="Underline"
        >
          U
        </button>
      </div>

      <div className="flex items-center gap-1 border-r border-gray-300 pr-2">
        <button
          onClick={() => onFormatChange({ horizontalAlign: 'left' })}
          className={`px-2 py-1 rounded text-sm ${
            currentFormat.horizontalAlign === 'left' || !currentFormat.horizontalAlign
              ? 'bg-gray-300 text-black'
              : 'hover:bg-gray-200 text-gray-700'
          }`}
          title="Align Left"
        >
          ⇤
        </button>
        <button
          onClick={() => onFormatChange({ horizontalAlign: 'center' })}
          className={`px-2 py-1 rounded text-sm ${
            currentFormat.horizontalAlign === 'center'
              ? 'bg-gray-300 text-black'
              : 'hover:bg-gray-200 text-gray-700'
          }`}
          title="Align Center"
        >
          ↔
        </button>
        <button
          onClick={() => onFormatChange({ horizontalAlign: 'right' })}
          className={`px-2 py-1 rounded text-sm ${
            currentFormat.horizontalAlign === 'right'
              ? 'bg-gray-300 text-black'
              : 'hover:bg-gray-200 text-gray-700'
          }`}
          title="Align Right"
        >
          ⇥
        </button>
      </div>

       <div className="flex items-center gap-2">
          <div className="flex items-center gap-1">
             <span className="text-xs text-gray-500">Text:</span>
             <input
                type="color"
                value={currentFormat.textColor || '#000000'}
                onChange={(e) => onFormatChange({ textColor: e.target.value })}
                className="w-6 h-6 p-0 border-0 rounded cursor-pointer"
                title="Text Color"
             />
          </div>
          <div className="flex items-center gap-1">
             <span className="text-xs text-gray-500">Bg:</span>
             <input
                type="color"
                value={currentFormat.backgroundColor || '#ffffff'}
                onChange={(e) => onFormatChange({ backgroundColor: e.target.value })}
                className="w-6 h-6 p-0 border-0 rounded cursor-pointer"
                title="Background Color"
             />
          </div>
       </div>
    </div>
  );
}
