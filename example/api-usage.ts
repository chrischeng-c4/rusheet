/**
 * Example usage of the RusheetAPI
 * This demonstrates the event-driven API for Rusheet
 */

import { rusheet } from '../src/core';
import type { CellChangeEvent, SelectionChangeEvent, CellEditEvent } from '../src/core';

// Example 1: Listen to cell changes
const unsubscribeChange = rusheet.onChange((event: CellChangeEvent) => {
  console.log(`Cell changed at (${event.row}, ${event.col}):`, {
    oldValue: event.oldValue,
    newValue: event.newValue,
    source: event.source
  });
});

// Example 2: Listen to selection changes
const unsubscribeSelection = rusheet.onSelectionChange((event: SelectionChangeEvent) => {
  console.log(`Selection changed from (${event.previousRow}, ${event.previousCol}) to (${event.row}, ${event.col})`);
});

// Example 3: Listen to cell edit events
const unsubscribeEdit = rusheet.onCellEdit((event: CellEditEvent) => {
  console.log(`Cell edit ${event.phase} at (${event.row}, ${event.col}): "${event.value}"`);
});

// Example 4: Initialize and use the API
async function main() {
  // Initialize WASM
  await rusheet.init();

  // Set cell values
  rusheet.setCellValue(0, 0, 'Hello');
  rusheet.setCellValue(0, 1, '=A1 & " World"');

  // Get cell data
  const cellA1 = rusheet.getCellData(0, 0);
  const cellB1 = rusheet.getCellData(0, 1);
  console.log('A1:', cellA1);
  console.log('B1:', cellB1);

  // Set selection
  rusheet.setSelection(0, 1);

  // Batch load data
  rusheet.setData([
    ['Name', 'Age', 'City'],
    ['Alice', '30', 'NYC'],
    ['Bob', '25', 'LA']
  ]);

  // Get data back
  const data = rusheet.getData(0, 2, 0, 2);
  console.log('Data:', data);

  // Clean up
  unsubscribeChange();
  unsubscribeSelection();
  unsubscribeEdit();
}

// Run example
// main().catch(console.error);
