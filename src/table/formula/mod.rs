//! Formula evaluation system for markdown tables
//!
//! This module provides spreadsheet-like formula support for markdown tables,
//! including scalar operations, vector operations, and matrix multiplication.
//!
//! # Features
//!
//! - **Scalar formulas**: `D2 = B2 * C2`
//! - **Vector operations**: `C_ = A_ + B_` (element-wise operations on columns)
//! - **Broadcasting**: `C_ = A_ * 0.5` (scalar applied to all vector elements)
//! - **Matrix multiplication**: `D1 = A_.T @ B_` (dot product using transpose operator)
//! - **Transpose operator**: `.T` to transpose vectors (e.g., `A_.T` converts column to row)
//! - **Functions**: `sum(A_)` to aggregate vector values
//! - **Operators**: `+`, `-`, `*`, `/`, `^`, `@` with proper precedence
//! - **Parentheses**: `(A_ + B_) * 2` for grouping
//!
//! # Cell References
//!
//! - `A1`, `B2`, etc. - Scalar cell references (column letter + row number)
//! - `A_`, `B_`, etc. - Column vector references (n×1 matrix, all rows in a column)
//! - `_1`, `_2`, etc. - Row vector references (1×n matrix, all columns in a row)
//! - `A_.T` - Transpose of column vector (converts n×1 to 1×n)
//! - `_1.T` - Transpose of row vector (converts 1×n to n×1)
//!
//! Row 1 refers to the first data row (headers and separators are not addressable).
//! Column references use alphabetic indexing (A=1st column, B=2nd, etc.) regardless of header text.
//!
//! # Matrix Operations
//!
//! Vectors are represented internally as matrices with dimension tracking:
//! - Column vectors: n×1 matrices (e.g., `A_` with 3 rows is a 3×1 matrix)
//! - Row vectors: 1×n matrices (e.g., `_1` with 3 columns is a 1×3 matrix)
//! - Matrix multiplication follows standard linear algebra rules: (m×n) @ (n×p) = (m×p)
//! - Dimension mismatch results in formula failure (no result computed)
//!
//! # Operator Precedence
//!
//! 1. Parentheses `()` and transpose `.T` (highest)
//! 2. Exponentiation `^`
//! 3. Matrix multiplication `@`, scalar multiplication `*`, and division `/`
//! 4. Addition `+` and subtraction `-` (lowest)
//!
//! # Examples
//!
//! ```text
//! <!-- md-table: D1 = B1 * C1 -->                 // Scalar formula
//! <!-- md-table: C_ = A_ + B_ -->                 // Vector addition
//! <!-- md-table: D_ = A_ * 0.08 -->               // Broadcast scalar
//! <!-- md-table: E1 = sum(A_ ^ 2) -->             // Sum of squares
//! <!-- md-table: F_ = (A_ + B_) / 2 -->           // Average of two columns
//! <!-- md-table: G1 = A_.T @ B_ -->               // Dot product (transpose then multiply)
//! <!-- md-table: H1 = _1 @ A_ -->                 // Row times column
//! <!-- md-table: I1 = (A_.T @ B_) + 10 -->        // Matrix mult in expression
//! ```

// Internal modules
mod types;
mod ast;
mod evaluator;
mod reference;
mod tokenizer;

// Re-export Span for use in error messages and public API
pub use types::Span;

// Internal imports
use types::{Value, Assignment, Statement};
use crate::table::error::FormulaError;
use types::{FIRST_DATA_ROW_INDEX, formula_row_to_table_index};
use std::collections::HashMap;
use ast::Parser;
use tokenizer::tokenize_expression;

/// Applies a column vector of values to a table column
/// Starts at first data row (after header and separator)
fn apply_column_vector_assignment(rows: &mut [Vec<String>], col: usize, value: &Value) {
    if let Value::Matrix { rows: _n_rows, cols: 1, data } = value {
        for (i, &val) in data.iter().enumerate() {
            let row_idx = FIRST_DATA_ROW_INDEX + i;
            if row_idx < rows.len() && col < rows[row_idx].len() {
                rows[row_idx][col] = val.to_string();
            }
        }
    }
}

/// Applies a row vector of values to a table row
fn apply_row_vector_assignment(rows: &mut [Vec<String>], row: usize, value: &Value) {
    if let Value::Matrix { rows: 1, cols: _n_cols, data } = value {
        for (i, &val) in data.iter().enumerate() {
            if row < rows.len() && i < rows[row].len() {
                rows[row][i] = val.to_string();
            }
        }
    }
}

/// Applies a matrix to a rectangular range in the table
fn apply_range_assignment(
    rows: &mut [Vec<String>],
    start_row: usize,
    start_col: usize,
    end_row: usize,
    end_col: usize,
    value: &Value
) {
    if let Value::Matrix { rows: num_rows, cols: num_cols, data } = value {
        let expected_rows = end_row - start_row + 1;
        let expected_cols = end_col - start_col + 1;

        // Check dimension match
        if *num_rows != expected_rows || *num_cols != expected_cols {
            return; // Dimension mismatch, skip assignment
        }

        // Apply values to range
        for r in 0..expected_rows {
            for c in 0..expected_cols {
                let table_row = start_row + r;
                let table_col = start_col + c;
                if table_row < rows.len() && table_col < rows[table_row].len() {
                    let data_idx = r * expected_cols + c;
                    rows[table_row][table_col] = data[data_idx].to_string();
                }
            }
        }
    }
}

/// Applies a matrix to a column range (A_:C_)
fn apply_column_range_assignment(
    rows: &mut [Vec<String>],
    start_col: usize,
    end_col: usize,
    value: &Value
) {
    if let Value::Matrix { rows: num_rows, cols: num_cols, data } = value {
        let expected_cols = end_col - start_col + 1;

        // Check column dimension match
        if *num_cols != expected_cols {
            return; // Dimension mismatch
        }

        // Apply values starting from first data row
        for r in 0..*num_rows {
            for c in 0..expected_cols {
                let table_row = FIRST_DATA_ROW_INDEX + r;
                let table_col = start_col + c;
                if table_row < rows.len() && table_col < rows[table_row].len() {
                    let data_idx = r * expected_cols + c;
                    rows[table_row][table_col] = data[data_idx].to_string();
                }
            }
        }
    }
}

/// Applies a matrix to a row range (_1:_5)
fn apply_row_range_assignment(
    rows: &mut [Vec<String>],
    start_row: usize,
    end_row: usize,
    value: &Value
) {
    if let Value::Matrix { rows: num_rows, cols: num_cols, data } = value {
        let expected_rows = end_row - start_row + 1;

        // Check row dimension match
        if *num_rows != expected_rows {
            return; // Dimension mismatch
        }

        // Apply values
        for r in 0..expected_rows {
            let table_row = formula_row_to_table_index(start_row + r);
            for c in 0..*num_cols {
                if table_row < rows.len() && c < rows[table_row].len() {
                    let data_idx = r * num_cols + c;
                    rows[table_row][c] = data[data_idx].to_string();
                }
            }
        }
    }
}

/// Applies spreadsheet-style formulas to table cells with access to other tables.
///
/// This version supports cross-table references via the from() function.
///
/// # Arguments
///
/// * `rows` - Mutable reference to the table rows (header, separator, then data rows)
/// * `formulas` - Slice of formula strings to evaluate
/// * `table_map` - Map of table IDs to their data for cross-table references
///
/// # Returns
///
/// A vector of Option<String> where each element corresponds to a formula.
/// None indicates the formula succeeded, Some(error) indicates it failed with the given error message.
pub fn apply_formulas_with_tables(
    rows: &mut Vec<Vec<String>>,
    formulas: &[String],
    table_map: &std::collections::HashMap<String, Vec<Vec<String>>>
) -> Vec<Option<String>> {
    let mut errors = Vec::new();
    let mut variable_map: HashMap<String, Value> = HashMap::new();

    for formula in formulas {
        let formula_trimmed = formula.trim();

        // Try to parse the statement (let or assignment)
        let (statement, expr) = match parse_statement(formula_trimmed) {
            Some(parsed) => parsed,
            None => {
                errors.push(Some(format!("Failed to parse statement '{}': invalid syntax (expected format: 'let VAR = EXPRESSION' or 'TARGET = EXPRESSION')", formula_trimmed)));
                continue;
            }
        };

        // Handle let statements - evaluate and store in variable map
        if let Statement::Let { name, span: _ } = &statement {
            // Try to evaluate the expression
            let value = match evaluate_expression_value_with_tables(&expr, rows, table_map, &variable_map) {
                Ok(v) => v,
                Err(error) => {
                    // Try to extract span information for better error messages
                    let error_msg = match extract_error_span(&expr, &error) {
                        Some(span) => {
                            format!("Failed to evaluate expression for variable '{}': \n{}", name, error.with_context(&expr, span))
                        }
                        None => {
                            format!("Failed to evaluate expression for variable '{}': {}", name, error)
                        }
                    };
                    errors.push(Some(error_msg));
                    continue;
                }
            };

            // Store the variable
            variable_map.insert(name.clone(), value);
            errors.push(None); // Success
            continue;
        }

        // Handle assignment statements - evaluate and apply to cells
        let assignment = match statement {
            Statement::Assignment(a) => a,
            _ => unreachable!("Already handled Let statements above"),
        };

        // Try to evaluate the expression (with table_map and variable_map)
        let value = match evaluate_expression_value_with_tables(&expr, rows, table_map, &variable_map) {
            Ok(v) => v,
            Err(error) => {
                // Try to extract span information by re-parsing for better error messages
                let error_msg = match extract_error_span(&expr, &error) {
                    Some(span) => {
                        // Use with_context to show visual position indicator
                        format!("Failed to evaluate expression:\n{}", error.with_context(&expr, span))
                    }
                    None => {
                        // Fallback to simple error message
                        format!("Failed to evaluate expression '{}': {}", expr, error)
                    }
                };
                errors.push(Some(error_msg));
                continue;
            }
        };

        // Try to apply the assignment
        let error = match assignment {
            Assignment::Scalar { row, col } => {
                // Scalar assignment: single cell update
                match value.as_scalar() {
                    Some(decimal) => {
                        if row >= rows.len() || col >= rows[row].len() {
                            Some(format!("Assignment failed for '{}': cell index out of bounds", formula_trimmed))
                        } else {
                            rows[row][col] = decimal.to_string();
                            None  // Success
                        }
                    }
                    None => {
                        Some(format!("Assignment failed for '{}': cannot assign matrix to scalar cell (use a cell vector assignment like C_ instead)", formula_trimmed))
                    }
                }
            }
            Assignment::ColumnVector { col } => {
                // Column vector assignment: update entire column
                if !value.is_column_vector() {
                    Some(format!("Assignment failed for '{}': expected column vector but got {} result",
                        formula_trimmed,
                        match value {
                            Value::Scalar(_) => "scalar",
                            Value::Matrix { rows: num_rows, cols: _, .. } => {
                                if num_rows == 1 {
                                    "row vector"
                                } else {
                                    "matrix"
                                }
                            }
                        }
                    ))
                } else if col >= rows.first().map(|r| r.len()).unwrap_or(0) {
                    Some(format!("Assignment failed for '{}': column index out of bounds", formula_trimmed))
                } else {
                    apply_column_vector_assignment(rows, col, &value);
                    None  // Success
                }
            }
            Assignment::RowVector { row } => {
                // Row vector assignment: update entire row
                let table_row = formula_row_to_table_index(row);
                match value {
                    Value::Matrix { rows: 1, cols: _, .. } => {
                        if table_row >= rows.len() {
                            Some(format!("Assignment failed for '{}': row index out of bounds", formula_trimmed))
                        } else {
                            apply_row_vector_assignment(rows, table_row, &value);
                            None  // Success
                        }
                    }
                    Value::Scalar(_) => {
                        Some(format!("Assignment failed for '{}': cannot assign scalar to row vector (expected row vector)", formula_trimmed))
                    }
                    Value::Matrix { rows: num_rows, .. } => {
                        Some(format!("Assignment failed for '{}': expected row vector but got matrix with {} rows", formula_trimmed, num_rows))
                    }
                }
            }
            Assignment::Range { start_row, start_col, end_row, end_col } => {
                // Range assignment: update rectangular region
                match value {
                    Value::Matrix { rows: num_rows, cols: num_cols, .. } => {
                        let expected_rows = end_row - start_row + 1;
                        let expected_cols = end_col - start_col + 1;

                        if num_rows != expected_rows || num_cols != expected_cols {
                            Some(format!("Assignment failed for '{}': dimension mismatch (expected {}×{} but got {}×{})",
                                formula_trimmed, expected_rows, expected_cols, num_rows, num_cols))
                        } else if end_row >= rows.len() {
                            Some(format!("Assignment failed for '{}': range extends beyond table bounds", formula_trimmed))
                        } else {
                            apply_range_assignment(rows, start_row, start_col, end_row, end_col, &value);
                            None  // Success
                        }
                    }
                    Value::Scalar(_) => {
                        Some(format!("Assignment failed for '{}': cannot assign scalar to range (expected matrix)", formula_trimmed))
                    }
                }
            }
            Assignment::ColumnRange { start_col, end_col } => {
                // Column range assignment: update multiple columns
                match value {
                    Value::Matrix { rows: _, cols: num_cols, .. } => {
                        let expected_cols = end_col - start_col + 1;

                        if num_cols != expected_cols {
                            Some(format!("Assignment failed for '{}': column dimension mismatch (expected {} columns but got {})",
                                formula_trimmed, expected_cols, num_cols))
                        } else if end_col >= rows.first().map(|r| r.len()).unwrap_or(0) {
                            Some(format!("Assignment failed for '{}': column range extends beyond table bounds", formula_trimmed))
                        } else {
                            apply_column_range_assignment(rows, start_col, end_col, &value);
                            None  // Success
                        }
                    }
                    Value::Scalar(_) => {
                        Some(format!("Assignment failed for '{}': cannot assign scalar to column range (expected matrix)", formula_trimmed))
                    }
                }
            }
            Assignment::RowRange { start_row, end_row } => {
                // Row range assignment: update multiple rows
                match value {
                    Value::Matrix { rows: num_rows, cols: _, .. } => {
                        let expected_rows = end_row - start_row + 1;

                        if num_rows != expected_rows {
                            Some(format!("Assignment failed for '{}': row dimension mismatch (expected {} rows but got {})",
                                formula_trimmed, expected_rows, num_rows))
                        } else {
                            let table_end_row = formula_row_to_table_index(end_row);
                            if table_end_row >= rows.len() {
                                Some(format!("Assignment failed for '{}': row range extends beyond table bounds", formula_trimmed))
                            } else {
                                apply_row_range_assignment(rows, start_row, end_row, &value);
                                None  // Success
                            }
                        }
                    }
                    Value::Scalar(_) => {
                        Some(format!("Assignment failed for '{}': cannot assign scalar to row range (expected matrix)", formula_trimmed))
                    }
                }
            }
        };

        errors.push(error);
    }

    errors
}

/// Parses an assignment target (left side of formula)
/// Supports: A1 (scalar), A_ (column vector), _1 (row vector), A1:C3 (range), A_:C_ (column range), _1:_5 (row range)
fn parse_assignment(target: &str) -> Option<Assignment> {
    use reference::parse_cell_reference;
    use types::CellReference;

    let target = target.trim();

    // Check if this is a range (contains ':')
    if target.contains(':') {
        let parts: Vec<&str> = target.split(':').collect();
        if parts.len() != 2 {
            return None;
        }

        let start_ref = parse_cell_reference(parts[0])?;
        let end_ref = parse_cell_reference(parts[1])?;

        // Match the type of range based on start and end references
        match (start_ref, end_ref) {
            (CellReference::Scalar { row: start_row, col: start_col },
             CellReference::Scalar { row: end_row, col: end_col }) => {
                Some(Assignment::Range { start_row, start_col, end_row, end_col })
            }
            (CellReference::ColumnVector { col: start_col },
             CellReference::ColumnVector { col: end_col }) => {
                Some(Assignment::ColumnRange { start_col, end_col })
            }
            (CellReference::RowVector { row: start_row },
             CellReference::RowVector { row: end_row }) => {
                Some(Assignment::RowRange { start_row, end_row })
            }
            _ => None, // Mixed types not allowed
        }
    } else {
        // Not a range, parse as single cell reference
        let cell_ref = parse_cell_reference(target)?;

        // Convert CellReference to Assignment
        match cell_ref {
            CellReference::Scalar { row, col } => {
                Some(Assignment::Scalar { row, col })
            }
            CellReference::ColumnVector { col } => {
                Some(Assignment::ColumnVector { col })
            }
            CellReference::RowVector { row } => {
                Some(Assignment::RowVector { row })
            }
            _ => None, // Ranges should have been caught above
        }
    }
}

/// Parses a formula like "A1 = B1 + C1" into (assignment, expression)
fn parse_formula(formula: &str) -> Option<(Assignment, String)> {
    let parts: Vec<&str> = formula.split('=').collect();
    if parts.len() == 2 {
        let assignment = parse_assignment(parts[0])?;
        Some((assignment, parts[1].trim().to_string()))
    } else {
        None
    }
}

/// Check if a variable name looks like a cell reference
/// Disallows: A1, B2, A_, B_, _1, _2, etc.
fn is_cell_reference_like(name: &str) -> bool {
    use crate::table::formula::reference::parse_cell_reference;
    // Try to parse as a cell reference - if it succeeds, it's not a valid variable name
    parse_cell_reference(name).is_some()
}

/// Parse a statement (either `let variable = expression` or `target = expression`)
fn parse_statement(formula: &str) -> Option<(Statement, String)> {
    let formula = formula.trim();

    // Check if this is a let statement
    if formula.starts_with("let ") {
        // Parse: let variable = expression
        let rest = formula[4..].trim(); // Skip "let "
        let parts: Vec<&str> = rest.splitn(2, '=').collect();

        if parts.len() != 2 {
            return None; // Invalid let syntax
        }

        let var_name = parts[0].trim().to_string();
        let expr = parts[1].trim().to_string();

        // Validate variable name
        if var_name.is_empty() {
            return None;
        }

        // Check if the variable name looks like a cell reference
        if is_cell_reference_like(&var_name) {
            return None; // Disallow names that look like cell references
        }

        // Create a Let statement
        let span = Span::new(0, formula.len()); // Full formula span
        Some((Statement::let_statement(var_name, span), expr))
    } else {
        // Parse as a regular assignment: target = expression
        let parts: Vec<&str> = formula.split('=').collect();
        if parts.len() == 2 {
            let assignment = parse_assignment(parts[0])?;
            Some((Statement::assignment(assignment), parts[1].trim().to_string()))
        } else {
            None
        }
    }
}

/// Attempts to extract span information from an error by analyzing the error type.
/// This allows us to provide visual position indicators even when the error doesn't
/// carry span information through the entire call chain.
///
/// # Strategy
///
/// For errors that reference specific tokens or positions, we can infer the span.
/// For other errors, we return None and fall back to simple error messages.
fn extract_error_span(expr: &str, error: &FormulaError) -> Option<Span> {
    match error {
        FormulaError::UnexpectedToken { position, .. } => {
            // Use the position from the error
            Some(Span::single(*position))
        }
        FormulaError::InvalidToken { token, .. } => {
            // Try to find the token in the expression
            if let Some(pos) = expr.find(token.as_str()) {
                Some(Span::new(pos, pos + token.len()))
            } else {
                None
            }
        }
        FormulaError::CellOutOfBounds { cell, .. } |
        FormulaError::ColumnOutOfBounds { column: cell, .. } => {
            // Try to find the cell reference in the expression
            if let Some(pos) = expr.find(cell.as_str()) {
                Some(Span::new(pos, pos + cell.len()))
            } else {
                None
            }
        }
        FormulaError::RowOutOfBounds { row, .. } => {
            // Try to find the row reference pattern (e.g., "_1", "_2")
            let row_ref = format!("_{}", row);
            if let Some(pos) = expr.find(&row_ref) {
                Some(Span::new(pos, pos + row_ref.len()))
            } else {
                None
            }
        }
        FormulaError::UnknownFunction { name, .. } => {
            // Try to find the function name in the expression
            if let Some(pos) = expr.find(name.as_str()) {
                Some(Span::new(pos, pos + name.len()))
            } else {
                None
            }
        }
        FormulaError::RuntimeError(msg) => {
            // For runtime errors, try to extract span from common patterns

            // Pattern 1: "unknown function: 'name'"
            if let Some(start_idx) = msg.find("unknown function: '") {
                let name_start = start_idx + "unknown function: '".len();
                if let Some(end_idx) = msg[name_start..].find('\'') {
                    let func_name = &msg[name_start..name_start + end_idx];
                    if let Some(pos) = expr.find(func_name) {
                        return Some(Span::new(pos, pos + func_name.len()));
                    }
                }
            }

            // Pattern 2: "cannot transpose a scalar" or other common messages
            // For now, we don't extract spans for these

            None
        }
        // For other error types, we could add more sophisticated span extraction
        // but for now we'll return None and use simple error messages
        _ => None,
    }
}

/// Evaluates a mathematical expression string and returns its computed value.
///
/// Supports cell references, numbers, operators, functions, and parentheses.
/// Handles both scalar and vector expressions with automatic type inference.
///
/// # Expression Components
///
/// - **Cell references**: `A1` (scalar), `A_` (column vector), `_1` (row vector)
/// - **Numbers**: `42`, `3.14`, `-5`
/// - **Operators**: `+`, `-`, `*`, `/`, `^`, `@` (with proper precedence)
/// - **Functions**: `sum(expression)`
/// - **Parentheses**: `(A_ + B_) * 2`
///
/// # Operator Precedence
///
/// 1. Parentheses `()` (highest)
/// 2. Exponentiation `^`
/// 3. Multiplication `*`, Division `/`, and Matrix multiplication `@`
/// 4. Addition `+` and Subtraction `-` (lowest)
///
/// # Arguments
///
/// * `expr` - The expression string to evaluate
/// * `rows` - Reference to the table rows for resolving cell references
///
/// # Returns
///
/// * `Ok(Value::Scalar)` for scalar results
/// * `Ok(Value::Matrix)` for matrix/vector results
/// * `Err(String)` with specific error message if evaluation fails
/// Evaluate an expression with access to other tables
fn evaluate_expression_value_with_tables(
    expr: &str,
    rows: &Vec<Vec<String>>,
    table_map: &std::collections::HashMap<String, Vec<Vec<String>>>,
    variable_map: &HashMap<String, Value>
) -> Result<Value, FormulaError> {
    // Step 1: Tokenize the expression
    let tokens = tokenize_expression(expr);

    // Step 2: Parse tokens into AST
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;

    // Step 3: Evaluate the AST with table_map and variable_map support
    evaluator::eval_ast_with_tables(&ast, rows, table_map, variable_map)
}

/// Evaluate an expression (backwards compatibility, no cross-table refs)
fn evaluate_expression_value(expr: &str, rows: &Vec<Vec<String>>) -> Result<Value, FormulaError> {
    use std::collections::HashMap;
    evaluate_expression_value_with_tables(expr, rows, &HashMap::new(), &HashMap::new())
}

/// Backwards-compatible apply_formulas (no cross-table refs)
pub fn apply_formulas(rows: &mut Vec<Vec<String>>, formulas: &[String]) -> Vec<Option<String>> {
    use std::collections::HashMap;
    apply_formulas_with_tables(rows, formulas, &HashMap::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use types::CellReference;

    #[test]
    fn test_parse_formula() {
        assert_eq!(
            parse_formula("A1 = B1 + C1"),
            Some((Assignment::Scalar { row: 2, col: 0 }, "B1 + C1".to_string()))
        );
        assert_eq!(
            parse_formula("D2 = B2 * C2"),
            Some((Assignment::Scalar { row: 3, col: 3 }, "B2 * C2".to_string()))
        );
        assert_eq!(
            parse_formula("C_ = A_ + B_"),
            Some((Assignment::ColumnVector { col: 2 }, "A_ + B_".to_string()))
        );
        assert_eq!(parse_formula("invalid"), None);
    }

    // Tests for vector/matrix operations

    #[test]
    fn test_parse_column_vector() {
        use reference::parse_cell_reference;
        assert_eq!(
            parse_cell_reference("A_"),
            Some(CellReference::ColumnVector { col: 0 })
        );
        assert_eq!(
            parse_cell_reference("B_"),
            Some(CellReference::ColumnVector { col: 1 })
        );
        assert_eq!(
            parse_cell_reference("Z_"),
            Some(CellReference::ColumnVector { col: 25 })
        );
        assert_eq!(parse_cell_reference("a_"), Some(CellReference::ColumnVector { col: 0 })); // lowercase
    }

    #[test]
    fn test_parse_row_vector() {
        use reference::parse_cell_reference;
        assert_eq!(
            parse_cell_reference("_1"),
            Some(CellReference::RowVector { row: 1 })
        );
        assert_eq!(
            parse_cell_reference("_2"),
            Some(CellReference::RowVector { row: 2 })
        );
        assert_eq!(
            parse_cell_reference("_10"),
            Some(CellReference::RowVector { row: 10 })
        );
    }

    #[test]
    fn test_parse_scalar_ref() {
        use reference::parse_cell_reference;
        assert_eq!(
            parse_cell_reference("A1"),
            Some(CellReference::Scalar { row: 2, col: 0 })
        );
        assert_eq!(
            parse_cell_reference("B2"),
            Some(CellReference::Scalar { row: 3, col: 1 })
        );
    }

    #[test]
    fn test_resolve_column_vector() {
        use reference::resolve_reference;
        let rows = vec![
            vec!["Name".to_string(), "Value".to_string()],
            vec!["---".to_string(), "---".to_string()],
            vec!["A".to_string(), "10".to_string()],
            vec!["B".to_string(), "20".to_string()],
            vec!["C".to_string(), "30".to_string()],
        ];

        let col_ref = CellReference::ColumnVector { col: 1 };
        let result = resolve_reference(&col_ref, &rows);

        assert_eq!(
            result,
            Ok(Value::column_vector(vec![
                Decimal::from(10),
                Decimal::from(20),
                Decimal::from(30)
            ]))
        );
    }

    #[test]
    fn test_resolve_row_vector() {
        use reference::resolve_reference;
        let rows = vec![
            vec!["Name".to_string(), "A".to_string(), "B".to_string()],
            vec!["---".to_string(), "---".to_string(), "---".to_string()],
            vec!["Values".to_string(), "10".to_string(), "20".to_string()],
        ];

        let row_ref = CellReference::RowVector { row: 1 };
        let result = resolve_reference(&row_ref, &rows);

        // Row 1 is first data row (index 2), includes all columns
        assert_eq!(
            result,
            Ok(Value::row_vector(vec![
                Decimal::ZERO,  // "Values" is non-numeric, treated as 0
                Decimal::from(10),
                Decimal::from(20)
            ]))
        );
    }

    #[test]
    fn test_resolve_empty_cells_as_zero() {
        use reference::resolve_reference;
        let rows = vec![
            vec!["Col".to_string()],
            vec!["---".to_string()],
            vec!["10".to_string()],
            vec!["".to_string()],  // empty
            vec!["text".to_string()],  // non-numeric
            vec!["30".to_string()],
        ];

        let col_ref = CellReference::ColumnVector { col: 0 };
        let result = resolve_reference(&col_ref, &rows);

        assert_eq!(
            result,
            Ok(Value::column_vector(vec![
                Decimal::from(10),
                Decimal::ZERO,  // empty treated as 0
                Decimal::ZERO,  // non-numeric treated as 0
                Decimal::from(30)
            ]))
        );
    }

    #[test]
    fn test_vector_addition() {
        use evaluator::evaluate_operation;
        // Column vector + Column vector
        let left = Value::column_vector(vec![Decimal::from(1), Decimal::from(2), Decimal::from(3)]);
        let right = Value::column_vector(vec![Decimal::from(4), Decimal::from(5), Decimal::from(6)]);

        let result = evaluate_operation('+', left, right);

        assert_eq!(
            result,
            Ok(Value::column_vector(vec![
                Decimal::from(5),
                Decimal::from(7),
                Decimal::from(9)
            ]))
        );
    }

    #[test]
    fn test_vector_scalar_multiply() {
        use evaluator::evaluate_operation;
        // Column vector * Scalar (broadcasting)
        let vec = Value::column_vector(vec![Decimal::from(2), Decimal::from(4), Decimal::from(6)]);
        let scalar = Value::Scalar(Decimal::from(3));

        let result = evaluate_operation('*', vec, scalar);

        assert_eq!(
            result,
            Ok(Value::column_vector(vec![
                Decimal::from(6),
                Decimal::from(12),
                Decimal::from(18)
            ]))
        );
    }

    #[test]
    fn test_broadcast_scalar() {
        use evaluator::evaluate_operation;
        // Scalar + Column vector (broadcasting)
        let scalar = Value::Scalar(Decimal::from(10));
        let vec = Value::column_vector(vec![Decimal::from(1), Decimal::from(2), Decimal::from(3)]);

        let result = evaluate_operation('+', scalar, vec);

        assert_eq!(
            result,
            Ok(Value::column_vector(vec![
                Decimal::from(11),
                Decimal::from(12),
                Decimal::from(13)
            ]))
        );
    }

    #[test]
    fn test_length_mismatch() {
        use evaluator::evaluate_operation;
        // Different length column vectors - should fail (dimensions must match for element-wise ops)
        let left = Value::column_vector(vec![Decimal::from(1), Decimal::from(2), Decimal::from(3)]);
        let right = Value::column_vector(vec![Decimal::from(10), Decimal::from(20)]);

        let result = evaluate_operation('+', left, right);

        // Dimensions don't match (3×1 vs 2×1), so operation fails
        assert!(result.is_err());
    }

    #[test]
    fn test_sum_vector() {
        use evaluator::eval_function;
        let vec = Value::column_vector(vec![Decimal::from(1), Decimal::from(2), Decimal::from(3)]);
        let result = eval_function("sum", vec);

        assert_eq!(result, Ok(Value::Scalar(Decimal::from(6))));
    }

    #[test]
    fn test_sum_scalar() {
        use evaluator::eval_function;
        let scalar = Value::Scalar(Decimal::from(42));
        let result = eval_function("sum", scalar);

        assert_eq!(result, Ok(Value::Scalar(Decimal::from(42))));
    }

    #[test]
    fn test_sum_empty_vector() {
        use evaluator::eval_function;
        let vec = Value::column_vector(vec![]);
        let result = eval_function("sum", vec);

        assert_eq!(result, Ok(Value::Scalar(Decimal::ZERO)));
    }

    #[test]
    fn test_power_operator() {
        use evaluator::evaluate_operation;
        let base = Value::Scalar(Decimal::from(2));
        let exp = Value::Scalar(Decimal::from(3));

        let result = evaluate_operation('^', base, exp);

        assert_eq!(result, Ok(Value::Scalar(Decimal::from(8))));
    }

    #[test]
    fn test_power_precedence() {
        // Test: 2 * 3^2 = 2 * 9 = 18
        let rows = vec![
            vec!["A".to_string()],
            vec!["---".to_string()],
        ];

        let result = evaluate_expression_value("2 * 3 ^ 2", &rows);

        assert_eq!(result, Ok(Value::Scalar(Decimal::from(18))));
    }

    #[test]
    fn test_power_in_expression() {
        // Test: (2+3)^2 = 5^2 = 25
        let rows = vec![
            vec!["A".to_string()],
            vec!["---".to_string()],
        ];

        let result = evaluate_expression_value("( 2 + 3 ) ^ 2", &rows);

        assert_eq!(result, Ok(Value::Scalar(Decimal::from(25))));
    }

    #[test]
    fn test_column_assignment() {
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            vec!["---".to_string(), "---".to_string(), "---".to_string()],
            vec!["1".to_string(), "2".to_string(), "0".to_string()],
            vec!["3".to_string(), "4".to_string(), "0".to_string()],
        ];

        let formulas = vec!["C_ = A_ + B_".to_string()];
        apply_formulas(&mut rows, &formulas);

        assert_eq!(rows[2][2], "3");  // 1 + 2
        assert_eq!(rows[3][2], "7");  // 3 + 4
    }

    #[test]
    fn test_vector_scalar_assignment() {
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["---".to_string(), "---".to_string()],
            vec!["10".to_string(), "0".to_string()],
            vec!["20".to_string(), "0".to_string()],
        ];

        let formulas = vec!["B_ = A_ * 0.5".to_string()];
        apply_formulas(&mut rows, &formulas);

        assert_eq!(rows[2][1], "5.0");   // 10 * 0.5
        assert_eq!(rows[3][1], "10.0");  // 20 * 0.5
    }

    #[test]
    fn test_sum_expression() {
        let mut rows = vec![
            vec!["A".to_string(), "Sum".to_string()],
            vec!["---".to_string(), "---".to_string()],
            vec!["10".to_string(), "0".to_string()],
            vec!["20".to_string(), "0".to_string()],
            vec!["30".to_string(), "0".to_string()],
        ];

        let formulas = vec!["B1 = sum(A_)".to_string()];
        apply_formulas(&mut rows, &formulas);

        assert_eq!(rows[2][1], "60");  // 10 + 20 + 30
    }

    #[test]
    fn test_sum_complex_expression() {
        // Test: sum(A_ * 2)
        let mut rows = vec![
            vec!["A".to_string(), "Result".to_string()],
            vec!["---".to_string(), "---".to_string()],
            vec!["1".to_string(), "0".to_string()],
            vec!["2".to_string(), "0".to_string()],
            vec!["3".to_string(), "0".to_string()],
        ];

        let formulas = vec!["B1 = sum(A_ * 2)".to_string()];
        apply_formulas(&mut rows, &formulas);

        assert_eq!(rows[2][1], "12");  // (1+2+3) * 2 = 6 * 2 = 12
    }

    #[test]
    fn test_existing_scalar_formulas_still_work() {
        // Ensure backward compatibility
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            vec!["---".to_string(), "---".to_string(), "---".to_string()],
            vec!["5".to_string(), "10".to_string(), "0".to_string()],
        ];

        let formulas = vec!["C1 = A1 + B1".to_string()];
        apply_formulas(&mut rows, &formulas);

        assert_eq!(rows[2][2], "15");  // 5 + 10
    }

    // Matrix multiplication tests

    #[test]
    fn test_dot_product_row_times_column() {
        use evaluator::evaluate_operation;
        // Test row vector @ column vector: (1×3) @ (3×1) = (1×1) = scalar
        // [1, 2, 3] @ [4; 5; 6] = 1*4 + 2*5 + 3*6 = 32
        let row = Value::row_vector(vec![Decimal::from(1), Decimal::from(2), Decimal::from(3)]);
        let col = Value::column_vector(vec![Decimal::from(4), Decimal::from(5), Decimal::from(6)]);

        let result = evaluate_operation('@', row, col);

        // Result is a 1×1 matrix
        assert_eq!(
            result,
            Ok(Value::Matrix {
                rows: 1,
                cols: 1,
                data: vec![Decimal::from(32)]
            })
        );
    }

    #[test]
    fn test_dot_product_dimension_mismatch() {
        use evaluator::evaluate_operation;
        // (1×3) @ (2×1) is invalid - inner dimensions don't match
        let row = Value::row_vector(vec![Decimal::from(1), Decimal::from(2), Decimal::from(3)]);
        let col = Value::column_vector(vec![Decimal::from(4), Decimal::from(5)]);

        let result = evaluate_operation('@', row, col);

        assert!(result.is_err());
    }

    #[test]
    fn test_matrix_mult_scalar_not_supported() {
        use evaluator::evaluate_operation;
        // Matrix multiplication with scalars is not defined
        let scalar = Value::Scalar(Decimal::from(5));
        let col = Value::column_vector(vec![Decimal::from(1), Decimal::from(2)]);

        let result = evaluate_operation('@', scalar.clone(), col.clone());
        assert!(result.is_err());

        let result2 = evaluate_operation('@', col, scalar);
        assert!(result2.is_err());
    }

    #[test]
    fn test_matrix_mult_scalar_scalar_invalid() {
        use evaluator::evaluate_operation;
        // Scalar @ Scalar is not valid for matrix multiplication
        let left = Value::Scalar(Decimal::from(5));
        let right = Value::Scalar(Decimal::from(10));

        let result = evaluate_operation('@', left, right);

        assert!(result.is_err());
    }

    #[test]
    fn test_column_column_invalid() {
        use evaluator::evaluate_operation;
        // Column @ Column is invalid: (3×1) @ (3×1) - inner dimensions don't match
        let left = Value::column_vector(vec![Decimal::from(1), Decimal::from(2), Decimal::from(3)]);
        let right = Value::column_vector(vec![Decimal::from(4), Decimal::from(5), Decimal::from(6)]);

        let result = evaluate_operation('@', left, right);

        assert!(result.is_err());
    }

    #[test]
    fn test_row_row_invalid() {
        use evaluator::evaluate_operation;
        // Row @ Row is invalid: (1×3) @ (1×3) - inner dimensions don't match
        let left = Value::row_vector(vec![Decimal::from(1), Decimal::from(2), Decimal::from(3)]);
        let right = Value::row_vector(vec![Decimal::from(4), Decimal::from(5), Decimal::from(6)]);

        let result = evaluate_operation('@', left, right);

        assert!(result.is_err());
    }

    #[test]
    fn test_row_vector_column_vector_dot_product() {
        // Test the example from user: _1 @ A_ in a square table
        // For this to work, we need a square data portion (n rows × n columns)
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            vec!["---".to_string(), "---".to_string(), "---".to_string()],
            vec!["2".to_string(), "3".to_string(), "4".to_string()],
            vec!["5".to_string(), "6".to_string(), "7".to_string()],
            vec!["8".to_string(), "9".to_string(), "10".to_string()],
        ];

        // C1 = _1 @ A_
        // _1 = [2, 3, 4] (first row, all 3 columns)
        // A_ = [2, 5, 8] (column A, all 3 rows)
        // Dot product = 2*2 + 3*5 + 4*8 = 4 + 15 + 32 = 51
        let formulas = vec!["C1 = _1 @ A_".to_string()];
        apply_formulas(&mut rows, &formulas);

        assert_eq!(rows[2][2], "51");
    }

    #[test]
    fn test_matrix_mult_with_expression() {
        // Test matrix multiplication with transpose in a complex expression
        let mut rows = vec![
            vec!["X".to_string(), "Y".to_string(), "Result".to_string()],
            vec!["---".to_string(), "---".to_string(), "---".to_string()],
            vec!["1".to_string(), "2".to_string(), "0".to_string()],
            vec!["3".to_string(), "4".to_string(), "0".to_string()],
            vec!["5".to_string(), "6".to_string(), "0".to_string()],
        ];

        // C1 = (A_.T @ B_) + 10
        // A_ = [1, 3, 5] (column vector 3×1)
        // A_.T = [1, 3, 5] (row vector 1×3)
        // B_ = [2, 4, 6] (column vector 3×1)
        // A_.T @ B_ = (1×3) @ (3×1) = 1*2 + 3*4 + 5*6 = 44
        // Result = 44 + 10 = 54
        let formulas = vec!["C1 = (A_.T @ B_) + 10".to_string()];
        apply_formulas(&mut rows, &formulas);

        assert_eq!(rows[2][2], "54");
    }

    #[test]
    fn test_matrix_mult_precedence() {
        // Test that @ has same precedence as *
        // 2 + 3 @ 4 should fail because 3 and 4 are scalars, not vectors
        let rows = vec![
            vec!["A".to_string()],
            vec!["---".to_string()],
        ];

        let result = evaluate_expression_value("2 + 3 @ 4", &rows);

        // Should return None because @ doesn't work with scalars
        assert!(result.is_err());
    }

    // Transpose operator (.T) tests

    #[test]
    fn test_transpose_column_to_row() {
        // Test column vector transposing to row vector
        let col = Value::column_vector(vec![Decimal::from(1), Decimal::from(2), Decimal::from(3)]);
        let result = col.transpose();

        assert_eq!(
            result,
            Some(Value::row_vector(vec![
                Decimal::from(1),
                Decimal::from(2),
                Decimal::from(3)
            ]))
        );
    }

    #[test]
    fn test_transpose_row_to_column() {
        // Test row vector transposing to column vector
        let row = Value::row_vector(vec![Decimal::from(4), Decimal::from(5)]);
        let result = row.transpose();

        assert_eq!(
            result,
            Some(Value::column_vector(vec![Decimal::from(4), Decimal::from(5)]))
        );
    }

    #[test]
    fn test_transpose_scalar_fails() {
        // Scalars cannot be transposed
        let scalar = Value::Scalar(Decimal::from(42));
        let result = scalar.transpose();

        assert_eq!(result, None);
    }

    #[test]
    fn test_transpose_operator_in_table() {
        // Test using .T operator in a real table formula
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string(), "Result".to_string()],
            vec!["---".to_string(), "---".to_string(), "---".to_string()],
            vec!["1".to_string(), "2".to_string(), "0".to_string()],
            vec!["3".to_string(), "4".to_string(), "0".to_string()],
            vec!["5".to_string(), "6".to_string(), "0".to_string()],
        ];

        // C1 = A_.T @ B_
        // A_ = [1, 3, 5] (column 3×1)
        // A_.T = [1, 3, 5] (row 1×3)
        // B_ = [2, 4, 6] (column 3×1)
        // A_.T @ B_ = 1*2 + 3*4 + 5*6 = 2 + 12 + 30 = 44
        let formulas = vec!["C1 = A_.T @ B_".to_string()];
        apply_formulas(&mut rows, &formulas);

        assert_eq!(rows[2][2], "44");
    }

    #[test]
    fn test_multiple_formulas_with_transpose() {
        // Test multiple formulas using matrix operations
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            vec!["---".to_string(), "---".to_string(), "---".to_string()],
            vec!["1".to_string(), "2".to_string(), "0".to_string()],
            vec!["4".to_string(), "5".to_string(), "0".to_string()],
            vec!["7".to_string(), "8".to_string(), "0".to_string()],
        ];

        // First: C_ = A_ + B_ (element-wise addition)
        // Then: C1 = _1 @ A_ (row dot column = 30)
        // _1 = [1, 2, 3] after first formula, A_ = [1, 4, 7]
        // _1 @ A_ = 1*1 + 2*4 + 3*7 = 1 + 8 + 21 = 30
        let formulas = vec![
            "C_ = A_ + B_".to_string(),
            "C1 = _1 @ A_".to_string(),
        ];
        apply_formulas(&mut rows, &formulas);

        // After C_ = A_ + B_: C1=3, C2=9, C3=15
        // After C1 = _1 @ A_: C1=30 (overwrites 3)
        assert_eq!(rows[2][2], "30");  // C1
        assert_eq!(rows[3][2], "9");   // C2
        assert_eq!(rows[4][2], "15");  // C3
    }

    #[test]
    fn test_matrix_dimensions_in_result() {
        use evaluator::evaluate_operation;
        // Verify that matrix multiplication preserves correct dimensions
        // (1×3) @ (3×1) should produce (1×1)
        let row = Value::row_vector(vec![Decimal::from(1), Decimal::from(2), Decimal::from(3)]);
        let col = Value::column_vector(vec![Decimal::from(4), Decimal::from(5), Decimal::from(6)]);

        let result = evaluate_operation('@', row, col);

        match result {
            Ok(Value::Matrix { rows: 1, cols: 1, data }) => {
                assert_eq!(data.len(), 1);
                assert_eq!(data[0], Decimal::from(32));
            }
            _ => panic!("Expected 1×1 matrix result"),
        }
    }

    #[test]
    fn test_as_scalar_extraction() {
        // Test that 1×1 matrices can be extracted as scalars
        let one_by_one = Value::Matrix {
            rows: 1,
            cols: 1,
            data: vec![Decimal::from(42)],
        };

        assert_eq!(one_by_one.as_scalar(), Some(Decimal::from(42)));

        // But larger matrices cannot
        let col = Value::column_vector(vec![Decimal::from(1), Decimal::from(2)]);
        assert_eq!(col.as_scalar(), None);
    }

    // ========== avg() function tests ==========

    #[test]
    fn test_avg_vector() {
        use evaluator::eval_function;
        let vec = Value::column_vector(vec![Decimal::from(10), Decimal::from(20), Decimal::from(30)]);
        let result = eval_function("avg", vec);

        assert_eq!(result, Ok(Value::Scalar(Decimal::from(20))));
    }

    #[test]
    fn test_avg_scalar() {
        use evaluator::eval_function;
        let scalar = Value::Scalar(Decimal::from(42));
        let result = eval_function("avg", scalar);

        assert_eq!(result, Ok(Value::Scalar(Decimal::from(42))));
    }

    #[test]
    fn test_avg_empty_vector() {
        use evaluator::eval_function;
        let vec = Value::column_vector(vec![]);
        let result = eval_function("avg", vec);

        assert_eq!(result, Ok(Value::Scalar(Decimal::ZERO)));
    }

    #[test]
    fn test_avg_matrix() {
        use evaluator::eval_function;
        // 2x2 matrix: [[1, 2], [3, 4]]
        let matrix = Value::Matrix {
            rows: 2,
            cols: 2,
            data: vec![Decimal::from(1), Decimal::from(2), Decimal::from(3), Decimal::from(4)],
        };
        let result = eval_function("avg", matrix);

        // Average of [1, 2, 3, 4] is 2.5
        assert_eq!(result, Ok(Value::Scalar(Decimal::new(25, 1))));
    }

    // ========== min() function tests ==========

    #[test]
    fn test_min_vector() {
        use evaluator::eval_function;
        let vec = Value::column_vector(vec![Decimal::from(30), Decimal::from(10), Decimal::from(20)]);
        let result = eval_function("min", vec);

        assert_eq!(result, Ok(Value::Scalar(Decimal::from(10))));
    }

    #[test]
    fn test_min_scalar() {
        use evaluator::eval_function;
        let scalar = Value::Scalar(Decimal::from(42));
        let result = eval_function("min", scalar);

        assert_eq!(result, Ok(Value::Scalar(Decimal::from(42))));
    }

    #[test]
    fn test_min_empty_vector() {
        use evaluator::eval_function;
        let vec = Value::column_vector(vec![]);
        let result = eval_function("min", vec);

        assert_eq!(result, Ok(Value::Scalar(Decimal::ZERO)));
    }

    #[test]
    fn test_min_with_negatives() {
        use evaluator::eval_function;
        let vec = Value::column_vector(vec![Decimal::from(5), Decimal::from(-3), Decimal::from(10)]);
        let result = eval_function("min", vec);

        assert_eq!(result, Ok(Value::Scalar(Decimal::from(-3))));
    }

    // ========== max() function tests ==========

    #[test]
    fn test_max_vector() {
        use evaluator::eval_function;
        let vec = Value::column_vector(vec![Decimal::from(30), Decimal::from(10), Decimal::from(20)]);
        let result = eval_function("max", vec);

        assert_eq!(result, Ok(Value::Scalar(Decimal::from(30))));
    }

    #[test]
    fn test_max_scalar() {
        use evaluator::eval_function;
        let scalar = Value::Scalar(Decimal::from(42));
        let result = eval_function("max", scalar);

        assert_eq!(result, Ok(Value::Scalar(Decimal::from(42))));
    }

    #[test]
    fn test_max_empty_vector() {
        use evaluator::eval_function;
        let vec = Value::column_vector(vec![]);
        let result = eval_function("max", vec);

        assert_eq!(result, Ok(Value::Scalar(Decimal::ZERO)));
    }

    #[test]
    fn test_max_with_negatives() {
        use evaluator::eval_function;
        let vec = Value::column_vector(vec![Decimal::from(-5), Decimal::from(-3), Decimal::from(-10)]);
        let result = eval_function("max", vec);

        assert_eq!(result, Ok(Value::Scalar(Decimal::from(-3))));
    }

    // ========== count() function tests ==========

    #[test]
    fn test_count_vector() {
        use evaluator::eval_function;
        let vec = Value::column_vector(vec![Decimal::from(10), Decimal::from(20), Decimal::from(30)]);
        let result = eval_function("count", vec);

        assert_eq!(result, Ok(Value::Scalar(Decimal::from(3))));
    }

    #[test]
    fn test_count_scalar() {
        use evaluator::eval_function;
        let scalar = Value::Scalar(Decimal::from(42));
        let result = eval_function("count", scalar);

        assert_eq!(result, Ok(Value::Scalar(Decimal::ONE)));
    }

    #[test]
    fn test_count_empty_vector() {
        use evaluator::eval_function;
        let vec = Value::column_vector(vec![]);
        let result = eval_function("count", vec);

        assert_eq!(result, Ok(Value::Scalar(Decimal::ZERO)));
    }

    #[test]
    fn test_count_matrix() {
        use evaluator::eval_function;
        // 2x3 matrix with 6 elements
        let matrix = Value::Matrix {
            rows: 2,
            cols: 3,
            data: vec![
                Decimal::from(1), Decimal::from(2), Decimal::from(3),
                Decimal::from(4), Decimal::from(5), Decimal::from(6),
            ],
        };
        let result = eval_function("count", matrix);

        assert_eq!(result, Ok(Value::Scalar(Decimal::from(6))));
    }

    // ========== prod() function tests ==========

    #[test]
    fn test_prod_vector() {
        use evaluator::eval_function;
        let vec = Value::column_vector(vec![Decimal::from(2), Decimal::from(3), Decimal::from(4)]);
        let result = eval_function("prod", vec);

        // 2 * 3 * 4 = 24
        assert_eq!(result, Ok(Value::Scalar(Decimal::from(24))));
    }

    #[test]
    fn test_prod_scalar() {
        use evaluator::eval_function;
        let scalar = Value::Scalar(Decimal::from(42));
        let result = eval_function("prod", scalar);

        assert_eq!(result, Ok(Value::Scalar(Decimal::from(42))));
    }

    #[test]
    fn test_prod_empty_vector() {
        use evaluator::eval_function;
        let vec = Value::column_vector(vec![]);
        let result = eval_function("prod", vec);

        // Product of empty vector is 1 (identity element for multiplication)
        assert_eq!(result, Ok(Value::Scalar(Decimal::ONE)));
    }

    #[test]
    fn test_prod_with_zero() {
        use evaluator::eval_function;
        let vec = Value::column_vector(vec![Decimal::from(5), Decimal::ZERO, Decimal::from(10)]);
        let result = eval_function("prod", vec);

        assert_eq!(result, Ok(Value::Scalar(Decimal::ZERO)));
    }

    // ========== Integration tests for new functions ==========

    #[test]
    fn test_functions_in_formulas() {
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string(), "F".to_string()],
            vec!["---".to_string(), "---".to_string(), "---".to_string(), "---".to_string(), "---".to_string(), "---".to_string()],
            vec!["10".to_string(), "20".to_string(), "0".to_string(), "0".to_string(), "0".to_string(), "0".to_string()],
            vec!["30".to_string(), "40".to_string(), "0".to_string(), "0".to_string(), "0".to_string(), "0".to_string()],
            vec!["50".to_string(), "60".to_string(), "0".to_string(), "0".to_string(), "0".to_string(), "0".to_string()],
        ];

        let formulas = vec![
            "C1 = avg(A_)".to_string(),      // avg([10, 30, 50]) = 30
            "D1 = min(A_)".to_string(),      // min([10, 30, 50]) = 10
            "E1 = max(B_)".to_string(),      // max([20, 40, 60]) = 60
            "F1 = count(A_)".to_string(),    // count([10, 30, 50]) = 3
            "C2 = prod(A_)".to_string(),     // prod([10, 30, 50]) = 15000
        ];

        let errors = apply_formulas(&mut rows, &formulas);
        assert!(errors.iter().all(|e| e.is_none()));

        assert_eq!(rows[2][2], "30");     // avg
        assert_eq!(rows[2][3], "10");     // min
        assert_eq!(rows[2][4], "60");     // max
        assert_eq!(rows[2][5], "3");      // count
        assert_eq!(rows[3][2], "15000");  // prod
    }

    // ========== Cell Range Tests ==========

    #[test]
    fn test_range_single_column() {
        // A1:A3 should create a 3x1 column vector
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["---".to_string(), "---".to_string()],
            vec!["1".to_string(), "0".to_string()],
            vec!["2".to_string(), "0".to_string()],
            vec!["3".to_string(), "0".to_string()],
        ];

        let formulas = vec!["B1 = sum(A1:A3)".to_string()];
        let errors = apply_formulas(&mut rows, &formulas);
        assert!(errors.iter().all(|e| e.is_none()));

        assert_eq!(rows[2][1], "6");  // 1 + 2 + 3 = 6
    }

    #[test]
    fn test_range_single_row() {
        // A1:C1 should create a 1x3 row vector
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string(), "Result".to_string()],
            vec!["---".to_string(), "---".to_string(), "---".to_string(), "---".to_string()],
            vec!["5".to_string(), "10".to_string(), "15".to_string(), "0".to_string()],
        ];

        let formulas = vec!["D1 = sum(A1:C1)".to_string()];
        let errors = apply_formulas(&mut rows, &formulas);
        assert!(errors.iter().all(|e| e.is_none()));

        assert_eq!(rows[2][3], "30");  // 5 + 10 + 15 = 30
    }

    #[test]
    fn test_range_matrix() {
        // A1:C2 should create a 2x3 matrix
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string(), "Result".to_string()],
            vec!["---".to_string(), "---".to_string(), "---".to_string(), "---".to_string()],
            vec!["1".to_string(), "2".to_string(), "3".to_string(), "0".to_string()],
            vec!["4".to_string(), "5".to_string(), "6".to_string(), "0".to_string()],
        ];

        let formulas = vec!["D1 = sum(A1:C2)".to_string()];
        let errors = apply_formulas(&mut rows, &formulas);
        assert!(errors.iter().all(|e| e.is_none()));

        assert_eq!(rows[2][3], "21");  // 1+2+3+4+5+6 = 21
    }

    #[test]
    fn test_range_single_cell() {
        // A1:A1 should return a scalar (1x1 matrix converted to scalar)
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["---".to_string(), "---".to_string()],
            vec!["42".to_string(), "0".to_string()],
        ];

        let formulas = vec!["B1 = A1:A1".to_string()];
        let errors = apply_formulas(&mut rows, &formulas);
        assert!(errors.iter().all(|e| e.is_none()));

        assert_eq!(rows[2][1], "42");
    }

    #[test]
    fn test_range_with_operations() {
        // Test ranges in arithmetic operations
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            vec!["---".to_string(), "---".to_string(), "---".to_string()],
            vec!["1".to_string(), "10".to_string(), "0".to_string()],
            vec!["2".to_string(), "20".to_string(), "0".to_string()],
            vec!["3".to_string(), "30".to_string(), "0".to_string()],
        ];

        // C1 = sum(A1:A3 + B1:B3) should be sum([11, 22, 33]) = 66
        let formulas = vec!["C1 = sum(A1:A3 + B1:B3)".to_string()];
        let errors = apply_formulas(&mut rows, &formulas);
        assert!(errors.iter().all(|e| e.is_none()));

        assert_eq!(rows[2][2], "66");
    }

    #[test]
    fn test_range_with_scalar_multiplication() {
        // Test range with scalar multiplication
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["---".to_string(), "---".to_string()],
            vec!["1".to_string(), "0".to_string()],
            vec!["2".to_string(), "0".to_string()],
            vec!["3".to_string(), "0".to_string()],
        ];

        let formulas = vec!["B1 = sum(A1:A3 * 2)".to_string()];
        let errors = apply_formulas(&mut rows, &formulas);
        assert!(errors.iter().all(|e| e.is_none()));

        assert_eq!(rows[2][1], "12");  // (1+2+3) * 2 = 12
    }

    #[test]
    fn test_range_all_functions() {
        // Test all functions with ranges
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string(), "F".to_string()],
            vec!["---".to_string(), "---".to_string(), "---".to_string(), "---".to_string(), "---".to_string(), "---".to_string()],
            vec!["10".to_string(), "0".to_string(), "0".to_string(), "0".to_string(), "0".to_string(), "0".to_string()],
            vec!["20".to_string(), "0".to_string(), "0".to_string(), "0".to_string(), "0".to_string(), "0".to_string()],
            vec!["30".to_string(), "0".to_string(), "0".to_string(), "0".to_string(), "0".to_string(), "0".to_string()],
        ];

        let formulas = vec![
            "B1 = sum(A1:A3)".to_string(),    // 10+20+30 = 60
            "C1 = avg(A1:A3)".to_string(),    // 60/3 = 20
            "D1 = min(A1:A3)".to_string(),    // 10
            "E1 = max(A1:A3)".to_string(),    // 30
            "F1 = count(A1:A3)".to_string(),  // 3
        ];

        let errors = apply_formulas(&mut rows, &formulas);
        assert!(errors.iter().all(|e| e.is_none()));

        assert_eq!(rows[2][1], "60");  // sum
        assert_eq!(rows[2][2], "20");  // avg
        assert_eq!(rows[2][3], "10");  // min
        assert_eq!(rows[2][4], "30");  // max
        assert_eq!(rows[2][5], "3");   // count
    }

    #[test]
    fn test_range_matrix_multiplication() {
        // Test matrix multiplication with ranges
        // [1, 2, 3] @ [4; 5; 6] = 1*4 + 2*5 + 3*6 = 32
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["---".to_string(), "---".to_string()],
            vec!["1".to_string(), "4".to_string()],
            vec!["2".to_string(), "5".to_string()],
            vec!["3".to_string(), "6".to_string()],
        ];

        // A1:A3 is a 3x1 column vector, need to transpose it to 1x3
        // B1:B3 is a 3x1 column vector
        let formulas = vec!["A1 = A1:A3.T @ B1:B3".to_string()];
        let errors = apply_formulas(&mut rows, &formulas);
        assert!(errors.iter().all(|e| e.is_none()));

        assert_eq!(rows[2][0], "32");
    }

    #[test]
    fn test_range_prod_function() {
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["---".to_string(), "---".to_string()],
            vec!["2".to_string(), "0".to_string()],
            vec!["3".to_string(), "0".to_string()],
            vec!["4".to_string(), "0".to_string()],
        ];

        let formulas = vec!["B1 = prod(A1:A3)".to_string()];
        let errors = apply_formulas(&mut rows, &formulas);
        assert!(errors.iter().all(|e| e.is_none()));

        assert_eq!(rows[2][1], "24");  // 2 * 3 * 4 = 24
    }

    #[test]
    fn test_range_rectangular_matrix() {
        // Test a rectangular 3x2 matrix
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            vec!["---".to_string(), "---".to_string(), "---".to_string()],
            vec!["1".to_string(), "2".to_string(), "0".to_string()],
            vec!["3".to_string(), "4".to_string(), "0".to_string()],
            vec!["5".to_string(), "6".to_string(), "0".to_string()],
        ];

        // A1:B3 is a 3x2 matrix, sum should be 1+2+3+4+5+6 = 21
        let formulas = vec!["C1 = sum(A1:B3)".to_string()];
        let errors = apply_formulas(&mut rows, &formulas);
        assert!(errors.iter().all(|e| e.is_none()));

        assert_eq!(rows[2][2], "21");
    }

    #[test]
    fn test_range_with_empty_cells() {
        // Test ranges with empty cells (should be treated as 0)
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["---".to_string(), "---".to_string()],
            vec!["1".to_string(), "0".to_string()],
            vec!["".to_string(), "0".to_string()],  // empty
            vec!["3".to_string(), "0".to_string()],
        ];

        let formulas = vec!["B1 = sum(A1:A3)".to_string()];
        let errors = apply_formulas(&mut rows, &formulas);
        assert!(errors.iter().all(|e| e.is_none()));

        assert_eq!(rows[2][1], "4");  // 1 + 0 + 3 = 4
    }

    // ========== Vector Range Tests (A_:C_, _1:_5) ==========

    #[test]
    fn test_column_range_basic() {
        // A_:C_ should extract all rows for columns A through C
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string(), "Sum".to_string()],
            vec!["---".to_string(), "---".to_string(), "---".to_string(), "---".to_string()],
            vec!["1".to_string(), "2".to_string(), "3".to_string(), "0".to_string()],
            vec!["4".to_string(), "5".to_string(), "6".to_string(), "0".to_string()],
            vec!["7".to_string(), "8".to_string(), "9".to_string(), "0".to_string()],
        ];

        // Sum of A_:C_ should be 1+2+3+4+5+6+7+8+9 = 45
        let formulas = vec!["D1 = sum(A_:C_)".to_string()];
        let errors = apply_formulas(&mut rows, &formulas);
        assert!(errors.iter().all(|e| e.is_none()));

        assert_eq!(rows[2][3], "45");
    }

    #[test]
    fn test_column_range_single() {
        // A_:A_ should be equivalent to A_
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["---".to_string(), "---".to_string()],
            vec!["10".to_string(), "0".to_string()],
            vec!["20".to_string(), "0".to_string()],
            vec!["30".to_string(), "0".to_string()],
        ];

        let formulas = vec!["B1 = sum(A_:A_)".to_string()];
        let errors = apply_formulas(&mut rows, &formulas);
        assert!(errors.iter().all(|e| e.is_none()));

        assert_eq!(rows[2][1], "60");  // 10+20+30
    }

    #[test]
    fn test_column_range_with_operations() {
        // Test arithmetic operations on column ranges
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string(), "Result".to_string()],
            vec!["---".to_string(), "---".to_string(), "---".to_string()],
            vec!["1".to_string(), "10".to_string(), "0".to_string()],
            vec!["2".to_string(), "20".to_string(), "0".to_string()],
            vec!["3".to_string(), "30".to_string(), "0".to_string()],
        ];

        // Sum of (A_:B_ * 2) should be (1+10+2+20+3+30)*2 = 132
        let formulas = vec!["C1 = sum(A_:B_ * 2)".to_string()];
        let errors = apply_formulas(&mut rows, &formulas);
        assert!(errors.iter().all(|e| e.is_none()));

        assert_eq!(rows[2][2], "132");
    }

    #[test]
    fn test_column_range_all_functions() {
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string(), "S".to_string(), "Av".to_string(), "Mn".to_string(), "Mx".to_string(), "C".to_string()],
            vec!["---".to_string(), "---".to_string(), "---".to_string(), "---".to_string(), "---".to_string(), "---".to_string(), "---".to_string()],
            vec!["10".to_string(), "20".to_string(), "0".to_string(), "0".to_string(), "0".to_string(), "0".to_string(), "0".to_string()],
            vec!["30".to_string(), "40".to_string(), "0".to_string(), "0".to_string(), "0".to_string(), "0".to_string(), "0".to_string()],
        ];

        let formulas = vec![
            "C1 = sum(A_:B_)".to_string(),    // 10+20+30+40 = 100
            "D1 = avg(A_:B_)".to_string(),    // 100/4 = 25
            "E1 = min(A_:B_)".to_string(),    // 10
            "F1 = max(A_:B_)".to_string(),    // 40
            "G1 = count(A_:B_)".to_string(),  // 4
        ];

        let errors = apply_formulas(&mut rows, &formulas);
        assert!(errors.iter().all(|e| e.is_none()));

        assert_eq!(rows[2][2], "100");  // sum
        assert_eq!(rows[2][3], "25");   // avg
        assert_eq!(rows[2][4], "10");   // min
        assert_eq!(rows[2][5], "40");   // max
        assert_eq!(rows[2][6], "4");    // count
    }

    #[test]
    fn test_row_range_basic() {
        // _1:_3 should extract rows 1 through 3, all columns
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            vec!["---".to_string(), "---".to_string(), "---".to_string()],
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
            vec!["4".to_string(), "5".to_string(), "6".to_string()],
            vec!["7".to_string(), "8".to_string(), "9".to_string()],
            vec!["0".to_string(), "0".to_string(), "0".to_string()],
        ];

        // Sum of _1:_3 should be 1+2+3+4+5+6+7+8+9 = 45
        let formulas = vec!["A4 = sum(_1:_3)".to_string()];
        let errors = apply_formulas(&mut rows, &formulas);
        assert!(errors.iter().all(|e| e.is_none()));

        assert_eq!(rows[5][0], "45");
    }

    #[test]
    fn test_row_range_single() {
        // _1:_1 should be equivalent to _1
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            vec!["---".to_string(), "---".to_string(), "---".to_string()],
            vec!["5".to_string(), "10".to_string(), "15".to_string()],
            vec!["0".to_string(), "0".to_string(), "0".to_string()],
        ];

        let formulas = vec!["A2 = sum(_1:_1)".to_string()];
        let errors = apply_formulas(&mut rows, &formulas);
        assert!(errors.iter().all(|e| e.is_none()));

        assert_eq!(rows[3][0], "30");  // 5+10+15
    }

    #[test]
    fn test_row_range_with_operations() {
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            vec!["---".to_string(), "---".to_string(), "---".to_string()],
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
            vec!["4".to_string(), "5".to_string(), "6".to_string()],
            vec!["0".to_string(), "0".to_string(), "0".to_string()],
        ];

        // Sum of (_1:_2 * 3) should be (1+2+3+4+5+6)*3 = 63
        let formulas = vec!["A3 = sum(_1:_2 * 3)".to_string()];
        let errors = apply_formulas(&mut rows, &formulas);
        assert!(errors.iter().all(|e| e.is_none()));

        assert_eq!(rows[4][0], "63");
    }

    #[test]
    fn test_row_range_all_functions() {
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["---".to_string(), "---".to_string()],
            vec!["10".to_string(), "20".to_string()],
            vec!["30".to_string(), "40".to_string()],
            vec!["0".to_string(), "0".to_string()],
            vec!["0".to_string(), "0".to_string()],
            vec!["0".to_string(), "0".to_string()],
            vec!["0".to_string(), "0".to_string()],
        ];

        let formulas = vec![
            "A3 = sum(_1:_2)".to_string(),    // 10+20+30+40 = 100
            "B3 = avg(_1:_2)".to_string(),    // 100/4 = 25
            "A4 = min(_1:_2)".to_string(),    // 10
            "B4 = max(_1:_2)".to_string(),    // 40
            "A5 = count(_1:_2)".to_string(),  // 4
        ];

        let errors = apply_formulas(&mut rows, &formulas);
        assert!(errors.iter().all(|e| e.is_none()));

        assert_eq!(rows[4][0], "100");  // sum
        assert_eq!(rows[4][1], "25");   // avg
        assert_eq!(rows[5][0], "10");   // min
        assert_eq!(rows[5][1], "40");   // max
        assert_eq!(rows[6][0], "4");    // count
    }

    #[test]
    fn test_mixed_range_types_error() {
        // A_:_5 should produce an error
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["---".to_string(), "---".to_string()],
            vec!["1".to_string(), "0".to_string()],
        ];

        let formulas = vec!["B1 = sum(A_:_5)".to_string()];
        let errors = apply_formulas(&mut rows, &formulas);

        // Should have an error
        assert!(errors.iter().any(|e| e.is_some()));
    }

    #[test]
    fn test_column_range_equivalence_to_individual_columns() {
        // A_:B_ should give same result as combining A_ and B_
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string(), "R1".to_string(), "R2".to_string()],
            vec!["---".to_string(), "---".to_string(), "---".to_string(), "---".to_string()],
            vec!["1".to_string(), "2".to_string(), "0".to_string(), "0".to_string()],
            vec!["3".to_string(), "4".to_string(), "0".to_string(), "0".to_string()],
            vec!["5".to_string(), "6".to_string(), "0".to_string(), "0".to_string()],
        ];

        let formulas = vec![
            "C1 = sum(A_:B_)".to_string(),
            "D1 = sum(A_) + sum(B_)".to_string(),
        ];

        let errors = apply_formulas(&mut rows, &formulas);
        assert!(errors.iter().all(|e| e.is_none()));

        // Both should give the same result
        assert_eq!(rows[2][2], rows[2][3]);
        assert_eq!(rows[2][2], "21");  // 1+2+3+4+5+6
    }

    #[test]
    fn test_row_range_equivalence_to_individual_rows() {
        // _1:_2 should give same result as combining _1 and _2
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            vec!["---".to_string(), "---".to_string(), "---".to_string()],
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
            vec!["4".to_string(), "5".to_string(), "6".to_string()],
            vec!["0".to_string(), "0".to_string(), "0".to_string()],
            vec!["0".to_string(), "0".to_string(), "0".to_string()],
        ];

        let formulas = vec![
            "A3 = sum(_1:_2)".to_string(),
            "B3 = sum(_1) + sum(_2)".to_string(),
        ];

        let errors = apply_formulas(&mut rows, &formulas);
        assert!(errors.iter().all(|e| e.is_none()));

        // Both should give the same result
        assert_eq!(rows[4][0], rows[4][1]);
        assert_eq!(rows[4][0], "21");  // 1+2+3+4+5+6
    }

    #[test]
    fn test_column_range_matrix_shape() {
        // Verify that A_:C_ creates the correct matrix dimensions
        use evaluator::eval_ast;
        use ast::Parser;
        use tokenizer::tokenize_expression;

        let rows = vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            vec!["---".to_string(), "---".to_string(), "---".to_string()],
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
            vec!["4".to_string(), "5".to_string(), "6".to_string()],
        ];

        let tokens = tokenize_expression("A_:C_");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        let result = eval_ast(&expr, &rows).unwrap();

        // Should be a 2x3 matrix (2 data rows, 3 columns)
        match result {
            Value::Matrix { rows: r, cols: c, data } => {
                assert_eq!(r, 2);
                assert_eq!(c, 3);
                assert_eq!(data.len(), 6);
            }
            _ => panic!("Expected Matrix, got {:?}", result),
        }
    }

    #[test]
    fn test_row_range_matrix_shape() {
        // Verify that _1:_2 creates the correct matrix dimensions
        use evaluator::eval_ast;
        use ast::Parser;
        use tokenizer::tokenize_expression;

        let rows = vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            vec!["---".to_string(), "---".to_string(), "---".to_string()],
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
            vec!["4".to_string(), "5".to_string(), "6".to_string()],
        ];

        let tokens = tokenize_expression("_1:_2");
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        let result = eval_ast(&expr, &rows).unwrap();

        // Should be a 2x3 matrix (2 rows, 3 columns)
        match result {
            Value::Matrix { rows: r, cols: c, data } => {
                assert_eq!(r, 2);
                assert_eq!(c, 3);
                assert_eq!(data.len(), 6);
            }
            _ => panic!("Expected Matrix, got {:?}", result),
        }
    }

    // Tests for let statements and variables

    #[test]
    fn test_parse_let_statement() {
        // Valid let statement with scalar
        assert_eq!(
            parse_statement("let x = 5"),
            Some((Statement::Let { name: "x".to_string(), span: Span::new(0, 9) }, "5".to_string()))
        );

        // Valid let statement with expression
        assert_eq!(
            parse_statement("let result = A1 + B1"),
            Some((Statement::Let { name: "result".to_string(), span: Span::new(0, 20) }, "A1 + B1".to_string()))
        );

        // Valid let statement with vector
        assert_eq!(
            parse_statement("let col = A_"),
            Some((Statement::Let { name: "col".to_string(), span: Span::new(0, 12) }, "A_".to_string()))
        );
    }

    #[test]
    fn test_parse_let_statement_invalid() {
        // Cell reference-like names should be rejected
        assert_eq!(parse_statement("let A1 = 5"), None);
        assert_eq!(parse_statement("let B_ = A_"), None);
        assert_eq!(parse_statement("let _2 = 10"), None);

        // No equals sign
        assert_eq!(parse_statement("let x"), None);

        // Empty variable name
        assert_eq!(parse_statement("let = 5"), None);
    }

    #[test]
    fn test_let_statement_basic() {
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            vec!["---".to_string(), "---".to_string(), "---".to_string()],
            vec!["5".to_string(), "10".to_string(), "0".to_string()],
        ];

        let formulas = vec![
            "let x = 15".to_string(),
            "C1 = x".to_string(),
        ];

        let errors = apply_formulas(&mut rows, &formulas);
        assert_eq!(errors, vec![None, None]);
        assert_eq!(rows[2][2], "15");
    }

    #[test]
    fn test_let_statement_with_expression() {
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            vec!["---".to_string(), "---".to_string(), "---".to_string()],
            vec!["5".to_string(), "10".to_string(), "0".to_string()],
        ];

        let formulas = vec![
            "let sum = A1 + B1".to_string(),
            "C1 = sum".to_string(),
        ];

        let errors = apply_formulas(&mut rows, &formulas);
        assert_eq!(errors, vec![None, None]);
        assert_eq!(rows[2][2], "15");
    }

    #[test]
    fn test_let_statement_with_vector() {
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            vec!["---".to_string(), "---".to_string(), "---".to_string()],
            vec!["1".to_string(), "2".to_string(), "0".to_string()],
            vec!["3".to_string(), "4".to_string(), "0".to_string()],
        ];

        let formulas = vec![
            "let col_a = A_".to_string(),
            "C_ = col_a".to_string(),
        ];

        let errors = apply_formulas(&mut rows, &formulas);
        assert_eq!(errors, vec![None, None]);
        assert_eq!(rows[2][2], "1");
        assert_eq!(rows[3][2], "3");
    }

    #[test]
    fn test_multiple_let_statements() {
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            vec!["---".to_string(), "---".to_string(), "---".to_string()],
            vec!["5".to_string(), "10".to_string(), "0".to_string()],
        ];

        let formulas = vec![
            "let x = A1".to_string(),
            "let y = B1".to_string(),
            "let result = x + y".to_string(),
            "C1 = result".to_string(),
        ];

        let errors = apply_formulas(&mut rows, &formulas);
        assert_eq!(errors, vec![None, None, None, None]);
        assert_eq!(rows[2][2], "15");
    }

    #[test]
    fn test_let_statement_error_undefined_variable() {
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            vec!["---".to_string(), "---".to_string(), "---".to_string()],
            vec!["5".to_string(), "10".to_string(), "0".to_string()],
        ];

        let formulas = vec![
            "C1 = undefined_var".to_string(),
        ];

        let errors = apply_formulas(&mut rows, &formulas);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].is_some());
        assert!(errors[0].as_ref().unwrap().contains("undefined variable"));
    }

    #[test]
    fn test_let_statement_error_cell_reference_like_name() {
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["---".to_string(), "---".to_string()],
            vec!["5".to_string(), "10".to_string()],
        ];

        let formulas = vec![
            "let A1 = 5".to_string(),
        ];

        let errors = apply_formulas(&mut rows, &formulas);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].is_some());
        assert!(errors[0].as_ref().unwrap().contains("Failed to parse statement"));
    }

    #[test]
    fn test_variable_in_arithmetic() {
        let mut rows = vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            vec!["---".to_string(), "---".to_string(), "---".to_string()],
            vec!["5".to_string(), "10".to_string(), "0".to_string()],
        ];

        let formulas = vec![
            "let factor = 2".to_string(),
            "C1 = A1 * factor + B1".to_string(),
        ];

        let errors = apply_formulas(&mut rows, &formulas);
        assert_eq!(errors, vec![None, None]);
        assert_eq!(rows[2][2], "20"); // (5 * 2) + 10 = 20
    }
}

