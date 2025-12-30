pub mod ast;
pub mod dependency;
pub mod evaluator;
pub mod functions;
pub mod lexer;
pub mod parser;
pub mod parser_nom;
pub mod reference_shifter;

pub use ast::{BinaryOp, Expr, UnaryOp};
pub use dependency::DependencyGraph;
pub use evaluator::Evaluator;
pub use lexer::{Lexer, Token};
pub use parser::Parser;
pub use parser_nom::NomParser;
pub use reference_shifter::{shift_formula_cols, shift_formula_rows};

use rusheet_core::{CellError, CellValue};

/// Parse and evaluate a formula expression
///
/// Uses the nom-based parser for robust parsing.
pub fn evaluate_formula(
    expression: &str,
    get_cell_value: impl Fn(u32, u32) -> CellValue,
) -> CellValue {
    // Use the nom parser
    let parser = NomParser::new();
    let ast = match parser.parse(expression) {
        Ok(ast) => ast,
        Err(_) => return CellValue::Error(CellError::InvalidValue),
    };

    // Evaluator
    let evaluator = Evaluator::new(get_cell_value);
    evaluator.evaluate(&ast)
}

/// Extract cell references from a formula expression
///
/// Uses the nom-based parser for robust parsing.
pub fn extract_references(expression: &str) -> Vec<(u32, u32)> {
    let parser = NomParser::new();
    let ast = match parser.parse(expression) {
        Ok(ast) => ast,
        Err(_) => return vec![],
    };

    collect_references(&ast)
}

/// Recursively collect cell references from an AST
fn collect_references(expr: &Expr) -> Vec<(u32, u32)> {
    let mut refs = Vec::new();

    match expr {
        Expr::CellRef { row, col, .. } => {
            refs.push((*row, *col));
        }
        Expr::Range { start, end } => {
            if let (
                Expr::CellRef {
                    row: r1, col: c1, ..
                },
                Expr::CellRef {
                    row: r2, col: c2, ..
                },
            ) = (start.as_ref(), end.as_ref())
            {
                for row in *r1..=*r2 {
                    for col in *c1..=*c2 {
                        refs.push((row, col));
                    }
                }
            }
        }
        Expr::Binary { left, right, .. } => {
            refs.extend(collect_references(left));
            refs.extend(collect_references(right));
        }
        Expr::Unary { operand, .. } => {
            refs.extend(collect_references(operand));
        }
        Expr::FunctionCall { args, .. } => {
            for arg in args {
                refs.extend(collect_references(arg));
            }
        }
        Expr::Grouped(inner) => {
            refs.extend(collect_references(inner));
        }
        _ => {}
    }

    refs
}
