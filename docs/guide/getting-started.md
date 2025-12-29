# Getting Started

## Installation

```bash
npm install rusheet
# or
pnpm add rusheet
```

## Basic Usage

```typescript
import { rusheet } from 'rusheet';

// Initialize
await rusheet.init();

// Set cell values
rusheet.setCellValue(0, 0, 'Hello');
rusheet.setCellValue(0, 1, '100');
rusheet.setCellValue(0, 2, '=B1*2'); // Formula

// Listen to changes
rusheet.onChange((event) => {
  console.log(`Cell ${event.row},${event.col} changed to ${event.newValue}`);
});

// Batch load data
rusheet.setData([
  ['Name', 'Price', 'Quantity', 'Total'],
  ['Apple', 1.5, 10, '=B2*C2'],
  ['Banana', 0.8, 20, '=B3*C3'],
]);

// Get all data
const data = rusheet.getData(0, 10, 0, 5);
```

## With React

```tsx
import { useEffect, useRef } from 'react';
import { rusheet } from 'rusheet';

function Spreadsheet() {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    rusheet.init().then(() => {
      // Initialize your spreadsheet
    });

    return () => rusheet.destroy();
  }, []);

  return <canvas ref={canvasRef} />;
}
```
