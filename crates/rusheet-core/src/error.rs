use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents possible cell errors (Excel-compatible)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CellError {
    /// #DIV/0! - Division by zero
    DivisionByZero,
    /// #VALUE! - Invalid value type
    InvalidValue,
    /// #REF! - Invalid cell reference
    InvalidReference,
    /// #NAME? - Unrecognized function or name
    InvalidName,
    /// #NULL! - Null intersection
    NullError,
    /// #NUM! - Invalid numeric value
    NumError,
    /// #N/A - Value not available
    NotAvailable,
    /// Circular reference detected
    CircularReference,
}

impl fmt::Display for CellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CellError::DivisionByZero => write!(f, "#DIV/0!"),
            CellError::InvalidValue => write!(f, "#VALUE!"),
            CellError::InvalidReference => write!(f, "#REF!"),
            CellError::InvalidName => write!(f, "#NAME?"),
            CellError::NullError => write!(f, "#NULL!"),
            CellError::NumError => write!(f, "#NUM!"),
            CellError::NotAvailable => write!(f, "#N/A"),
            CellError::CircularReference => write!(f, "#CIRCULAR!"),
        }
    }
}
