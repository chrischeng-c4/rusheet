# Getting Started

RuSheet is a high-performance spreadsheet component built with Rust, WebAssembly, and React. It provides Excel-like features including formulas, formatting, and real-time collaboration.

## Installation

```bash
npm install rusheet
# or
pnpm add rusheet
```

## Basic Usage

The easiest way to use RuSheet is via the React component wrapper.

```tsx
import { useRef } from 'react';
import { RuSheet, type RuSheetRef } from 'rusheet/react';

function App() {
  const sheetRef = useRef<RuSheetRef>(null);

  // Initial data (optional)
  const initialData = [
    ['Name', 'Price', 'Quantity', 'Total'],
    ['Apple', 1.5, 10, '=B2*C2'],
    ['Banana', 0.8, 20, '=B3*C3'],
  ];

  return (
    <div style={{ height: '600px', width: '100%' }}>
      <RuSheet
        ref={sheetRef}
        initialData={initialData}
        onChange={(event) => {
          console.log('Cell changed:', event);
        }}
        // Default grid size is 1000 rows x 26 columns (A-Z)
      />
    </div>
  );
}
```

## Imperative API

You can control the spreadsheet programmatically using the `ref`:

```tsx
// Set a cell value
sheetRef.current?.setCellValue(0, 0, 'New Value');

// Get data from a specific cell
const cell = sheetRef.current?.getCellData(0, 0);

// Format a range
sheetRef.current?.setRangeFormat(0, 0, 0, 3, {
  bold: true,
  backgroundColor: '#f0f0f0'
});

// Import CSV
sheetRef.current?.importCSV('A,B,C\n1,2,3');
```

## Enabling Collaboration

To enable real-time collaboration, you need to connect to a running `rusheet-server` instance.

1.  **Start the Server**:
    ```bash
    # From the project root (requires Docker)
    just db-up
    just server
    ```

2.  **Configure the Client**:

    ```tsx
    <RuSheet
      collaboration={{
        url: 'ws://localhost:3000/ws',
        workbookId: 'my-shared-workbook',
        user: {
          name: 'Alice',
          color: '#ff0000' // Cursor color
        }
      }}
    />
    ```

## Default Dimensions

By default, a new sheet is initialized with:
*   **1000 Rows**
*   **26 Columns** (A to Z)

You can add more rows or columns dynamically via the API (`insertRows`, `insertCols`) or let the spreadsheet expand automatically when importing data.

```