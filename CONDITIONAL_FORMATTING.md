# Conditional Formatting Implementation

## Overview

This document describes the conditional formatting feature implemented for RuSheet. Conditional formatting allows cells to dynamically change their appearance based on their values or content.

## Features Implemented

### 1. Value-Based Rules
Compare numeric cell values against thresholds:
- **Operators**: Greater Than, Greater Than or Equal, Less Than, Less Than or Equal, Equal, Not Equal, Between, Not Between
- **Use Cases**: Highlight high/low values, identify outliers, mark specific ranges

### 2. Text-Based Rules
Match text patterns in cells:
- **Operators**: Contains, Not Contains, Starts With, Ends With, Is Empty, Is Not Empty
- **Case Sensitivity**: Configurable
- **Use Cases**: Highlight specific keywords, identify empty cells, pattern matching

### 3. Color Scales
Create gradients based on value ranges:
- **2-Color Scale**: Min to Max gradient
- **3-Color Scale**: Min to Mid to Max gradient
- **Automatic Range**: Calculates min/max from sheet data
- **Use Cases**: Heatmaps, data visualization, trend identification

## Architecture

### Core Components

#### `conditional_format.rs`
Located at: `crates/rusheet-core/src/conditional_format.rs`

Key types:
- `ConditionalFormattingRule`: Complete rule with ID, range, priority, and enabled state
- `ConditionalRule`: Enum of rule types (ValueBased, TextBased, ColorScale)
- `ConditionalFormat`: Format to apply (colors, bold, italic, underline)
- `ComparisonOperator`: Numeric comparison operators
- `TextOperator`: Text matching operators

#### `sheet.rs` Integration
The `Sheet` struct now includes:
- `conditional_formatting: Vec<ConditionalFormattingRule>` field
- Methods:
  - `add_conditional_formatting()`: Add a new rule
  - `remove_conditional_formatting()`: Remove a rule by ID
  - `get_conditional_formatting_rules()`: Get all rules
  - `get_effective_format()`: Calculate final format with conditional rules applied

### Priority System

Rules are sorted by priority (higher number = applied later = overrides earlier rules). When multiple rules match a cell:
1. Base cell format is used as starting point
2. Rules are applied in priority order (lowest to highest)
3. Each matching rule overlays its format on top of previous formats

## Usage Example

```rust
use rusheet_core::{
    Cell, CellCoord, CellRange, Color, ComparisonOperator,
    ConditionalFormat, ConditionalFormattingRule, ConditionalRule, Sheet,
};

let mut sheet = Sheet::new("Sales");

// Add data
sheet.set_cell(CellCoord::new(0, 0), Cell::number(75.0));
sheet.set_cell(CellCoord::new(0, 1), Cell::number(30.0));

// Create rule: highlight values > 50 in green
let rule = ConditionalFormattingRule::new(
    "high_values".to_string(),
    CellRange::new(CellCoord::new(0, 0), CellCoord::new(10, 10)),
    ConditionalRule::ValueBased {
        operator: ComparisonOperator::GreaterThan,
        value1: 50.0,
        value2: None,
        format: ConditionalFormat {
            background_color: Some(Color::GREEN),
            bold: Some(true),
            ..Default::default()
        },
    },
);

sheet.add_conditional_formatting(rule);

// Get effective format for a cell
let base_format = CellFormat::default();
let value = sheet.get_cell_value(CellCoord::new(0, 0));
let effective = sheet.get_effective_format(0, 0, &base_format, value);
// Cell at (0,0) will have green background and bold text
```

See `crates/rusheet-core/examples/conditional_format_example.rs` for a complete working example.

## Testing

All functionality is thoroughly tested:

### Unit Tests
- `conditional_format.rs`: 6 tests covering all rule types
- Color interpolation
- Range containment
- Rule evaluation

### Integration Tests
- `sheet.rs`: 4 tests covering:
  - Adding/removing rules
  - Effective format calculation
  - Priority system
  - Color scale rendering

Run tests:
```bash
cargo test -p rusheet-core conditional_format
```

## Serialization

Conditional formatting rules are automatically serialized with the sheet:
- Rules are saved as part of the sheet JSON
- Deserialization restores all rules with their priorities
- Empty rule lists are omitted from serialization

## Performance Considerations

1. **Min/Max Calculation**: Color scales calculate range min/max on each evaluation. Consider caching for large sheets.
2. **Rule Evaluation**: Rules are evaluated in priority order for each cell. Keep rule count reasonable for large ranges.
3. **Priority Sorting**: Rules are sorted on addition, so adding many rules sequentially has O(n log n) cost.

## Future Enhancements

Potential improvements:
- [ ] Formula-based rules (e.g., compare cell value to another cell)
- [ ] Icon sets (arrows, traffic lights)
- [ ] Data bars (in-cell bar charts)
- [ ] Top/Bottom N rules
- [ ] Duplicate value detection
- [ ] Custom formula expressions
- [ ] Performance optimization with caching

## Files Modified

1. **Created**:
   - `crates/rusheet-core/src/conditional_format.rs` (354 lines)
   - `crates/rusheet-core/examples/conditional_format_example.rs` (149 lines)

2. **Modified**:
   - `crates/rusheet-core/src/lib.rs`: Added exports
   - `crates/rusheet-core/src/sheet.rs`: Added field and methods
   - `crates/rusheet-core/Cargo.toml`: No changes needed (no new dependencies)

## API Reference

### `ConditionalFormattingRule`
```rust
pub struct ConditionalFormattingRule {
    pub id: String,
    pub range: CellRange,
    pub rule: ConditionalRule,
    pub priority: i32,
    pub enabled: bool,
}
```

### `ConditionalRule`
```rust
pub enum ConditionalRule {
    ValueBased {
        operator: ComparisonOperator,
        value1: f64,
        value2: Option<f64>,
        format: ConditionalFormat,
    },
    TextBased {
        operator: TextOperator,
        pattern: Option<String>,
        case_sensitive: bool,
        format: ConditionalFormat,
    },
    ColorScale {
        min_color: Color,
        max_color: Color,
        mid_color: Option<Color>,
    },
}
```

### `ConditionalFormat`
```rust
pub struct ConditionalFormat {
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underline: Option<bool>,
    pub text_color: Option<Color>,
    pub background_color: Option<Color>,
}
```

## Summary

The conditional formatting implementation provides a complete, extensible system for dynamic cell formatting in RuSheet. The feature is:
- ✅ Fully tested (310 tests passing)
- ✅ Type-safe with Rust's type system
- ✅ Serializable for persistence
- ✅ Well-documented with examples
- ✅ Following existing code patterns

The implementation seamlessly integrates with RuSheet's existing architecture and provides a foundation for future enhancements.
