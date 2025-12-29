//! Formula evaluation system for markdown tables
//!
//! This module provides spreadsheet-like formula support for markdown tables,
//! including both scalar and vector operations.
//!
//! # Features
//!
//! - **Scalar formulas**: `D2 = B2 * C2`
//! - **Vector operations**: `C_ = A_ + B_` (element-wise operations on columns)
//! - **Broadcasting**: `C_ = A_ * 0.5` (scalar applied to all vector elements)
//! - **Functions**: `sum(A_)` to aggregate vector values
//! - **Operators**: `+`, `-`, `*`, `/`, `^` with proper precedence
//! - **Parentheses**: `(A_ + B_) * 2` for grouping
//!
//! # Cell References
//!
//! - `A1`, `B2`, etc. - Scalar cell references (column letter + row number)
//! - `A_`, `B_`, etc. - Column vector references (entire column)
//! - `_1`, `_2`, etc. - Row vector references (entire row)
//!
//! Row 1 refers to the first data row (headers and separators are not addressable).
//!
//! # Operator Precedence
//!
//! 1. Parentheses `()` (highest)
//! 2. Exponentiation `^`
//! 3. Multiplication `*` and Division `/`
//! 4. Addition `+` and Subtraction `-` (lowest)
//!
//! # Examples
//!
//! ```text
//! <!-- md-table: D1 = B1 * C1 -->           // Scalar formula
//! <!-- md-table: C_ = A_ + B_ -->           // Vector addition
//! <!-- md-table: D_ = A_ * 0.08 -->         // Broadcast scalar
//! <!-- md-table: E1 = sum(A_ ^ 2) -->       // Sum of squares
//! <!-- md-table: F_ = (A_ + B_) / 2 -->     // Average of two columns
//! ```

use rust_decimal::Decimal;
use rust_decimal::prelude::{ToPrimitive, FromPrimitive};
use std::str::FromStr;

// Table row index constant
// Markdown tables have a header row, separator row, then data rows starting at index 2
const FIRST_DATA_ROW_INDEX: usize = 2;

/// Converts a formula row number (1-based) to actual table index
/// Formula row 1 = first data row (table index 2)
/// Formula row 2 = second data row (table index 3), etc.
fn formula_row_to_table_index(row_num: usize) -> usize {
    FIRST_DATA_ROW_INDEX + (row_num - 1)
}

/// Represents a value in a formula - either a scalar or a vector
#[derive(Debug, Clone, PartialEq)]
enum Value {
    Scalar(Decimal),
    Vector(Vec<Decimal>),
}

/// Represents different types of cell references
#[derive(Debug, Clone, PartialEq)]
enum CellReference {
    Scalar { row: usize, col: usize },  // A1, B2, etc.
    ColumnVector { col: usize },         // A_, B_, etc.
    RowVector { row: usize },            // _1, _2, etc.
}

/// Represents the left side of a formula assignment
#[derive(Debug, Clone, PartialEq)]
enum Assignment {
    Scalar { row: usize, col: usize },   // D2 = ...
    ColumnVector { col: usize },         // D_ = ...
}

/// Applies a vector of values to a column
/// Starts at first data row (after header and separator)
fn apply_vector_assignment(rows: &mut Vec<Vec<String>>, col: usize, values: Vec<Decimal>) {
    for (i, value) in values.iter().enumerate() {
        let row_idx = FIRST_DATA_ROW_INDEX + i;
        if row_idx < rows.len() && col < rows[row_idx].len() {
            rows[row_idx][col] = value.to_string();
        }
    }
}

/// Applies spreadsheet-style formulas to table cells.
///
/// Formulas are evaluated in order, allowing later formulas to reference
/// cells updated by earlier formulas. Each formula follows the pattern:
/// `TARGET = EXPRESSION` where TARGET can be a scalar cell (e.g., `D2`) or
/// a column vector (e.g., `D_`).
///
/// # Arguments
///
/// * `rows` - Mutable reference to the table rows (header, separator, then data rows)
/// * `formulas` - Slice of formula strings to evaluate (e.g., `["D1 = B1 * C1", "D_ = A_ + B_"]`)
///
/// # Examples
///
/// ```
/// // Scalar formula: D2 = B2 * C2
/// // Vector formula: D_ = A_ + B_
/// // Sum function: E1 = sum(A_)
/// ```
///
/// # Note
///
/// Invalid formulas are silently ignored. Formulas that reference out-of-bounds
/// cells or contain syntax errors will not modify the table.
pub fn apply_formulas(rows: &mut Vec<Vec<String>>, formulas: &[String]) {
    for formula in formulas {
        if let Some((assignment, expr)) = parse_formula(formula) {
            // Evaluate the expression to get a Value
            if let Some(value) = evaluate_expression_value(&expr, rows) {
                match assignment {
                    Assignment::Scalar { row, col } => {
                        // Scalar assignment: single cell update
                        if let Value::Scalar(decimal) = value {
                            if row < rows.len() && col < rows[row].len() {
                                rows[row][col] = decimal.to_string();
                            }
                        }
                    }
                    Assignment::ColumnVector { col } => {
                        // Vector assignment: update entire column
                        if let Value::Vector(values) = value {
                            apply_vector_assignment(rows, col, values);
                        }
                    }
                }
            }
        }
    }
}

/// Parses an assignment target (left side of formula)
/// Supports: A1 (scalar), A_ (column vector)
fn parse_assignment(target: &str) -> Option<Assignment> {
    let target = target.trim().to_uppercase();
    if target.is_empty() {
        return None;
    }

    // Check for column vector pattern: A_
    if target.ends_with('_') {
        let col_str = &target[..target.len() - 1];
        if col_str.len() == 1 {
            let col_char = col_str.chars().next()?;
            if col_char.is_ascii_alphabetic() {
                let col_idx = (col_char as u32 - 'A' as u32) as usize;
                return Some(Assignment::ColumnVector { col: col_idx });
            }
        }
        return None;
    }

    // Check for scalar pattern: A1
    let mut chars = target.chars();
    let first = chars.next()?;
    if !first.is_ascii_alphabetic() {
        return None;
    }

    let rest: String = chars.collect();
    if rest.is_empty() {
        return None;
    }

    // Verify rest is all digits
    if !rest.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    let col_idx = (first as u32 - 'A' as u32) as usize;
    let row_num: usize = rest.parse().ok()?;

    if row_num == 0 {
        return None;
    }

    let row_idx = formula_row_to_table_index(row_num);
    Some(Assignment::Scalar { row: row_idx, col: col_idx })
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

/// Converts cell reference (like "A1", "B2") to (row_index, col_index)
/// Note: Row 1 is the first data row (headers are not addressable)
/// The separator row (|---|---|) is automatically skipped
fn cell_ref_to_index(cell_ref: &str) -> Option<(usize, usize)> {
    let cell_ref = cell_ref.trim().to_uppercase();
    let mut chars = cell_ref.chars();

    // Get column letter (A, B, C, etc.)
    let col_char = chars.next()?;
    if !col_char.is_ascii_alphabetic() {
        return None;
    }

    // Convert A=0, B=1, C=2, etc.
    let col_idx = (col_char as u32 - 'A' as u32) as usize;

    // Get row number (1, 2, 3, etc.)
    let row_str: String = chars.collect();
    let row_num: usize = row_str.parse().ok()?;

    // Convert to 0-based index
    // Row 1 = first data row, Row 2 = second data row, etc.
    if row_num == 0 {
        return None;
    }

    let row_idx = formula_row_to_table_index(row_num);

    Some((row_idx, col_idx))
}

/// Parses a cell reference string into a structured `CellReference`.
///
/// Recognizes three types of references:
/// - **Scalar**: `A1`, `B2`, etc. (column letter + row number)
/// - **Column Vector**: `A_`, `B_`, etc. (column letter + underscore)
/// - **Row Vector**: `_1`, `_2`, etc. (underscore + row number)
///
/// # Arguments
///
/// * `token` - The reference string to parse (case-insensitive)
///
/// # Returns
///
/// * `Some(CellReference)` if the token matches a valid pattern
/// * `None` if the token is invalid or doesn't match any pattern
///
/// # Examples
///
/// ```
/// parse_cell_reference("A1")  // Some(Scalar { row: 2, col: 0 })
/// parse_cell_reference("B_")  // Some(ColumnVector { col: 1 })
/// parse_cell_reference("_1")  // Some(RowVector { row: 1 })
/// parse_cell_reference("123") // None
/// ```
fn parse_cell_reference(token: &str) -> Option<CellReference> {
    let token = token.trim().to_uppercase();
    if token.is_empty() {
        return None;
    }

    // Check for row vector pattern: _N (underscore followed by number)
    if token.starts_with('_') {
        let row_str = &token[1..];
        if let Ok(row_num) = row_str.parse::<usize>() {
            if row_num > 0 {
                return Some(CellReference::RowVector { row: row_num });
            }
        }
        return None;
    }

    // Check for column vector pattern: A_ (letter followed by underscore)
    if token.ends_with('_') {
        let col_str = &token[..token.len() - 1];
        if col_str.len() == 1 {
            let col_char = col_str.chars().next()?;
            if col_char.is_ascii_alphabetic() {
                let col_idx = (col_char as u32 - 'A' as u32) as usize;
                return Some(CellReference::ColumnVector { col: col_idx });
            }
        }
        return None;
    }

    // Check for scalar pattern: A1 (letter followed by number)
    let mut chars = token.chars();
    let first = chars.next()?;
    if !first.is_ascii_alphabetic() {
        return None;
    }

    let rest: String = chars.collect();
    if rest.is_empty() {
        return None;
    }

    // Verify rest is all digits
    if !rest.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    let col_idx = (first as u32 - 'A' as u32) as usize;
    let row_num: usize = rest.parse().ok()?;

    if row_num == 0 {
        return None;
    }

    let row_idx = formula_row_to_table_index(row_num);
    Some(CellReference::Scalar { row: row_idx, col: col_idx })
}

/// Resolves a cell reference to its actual value(s) from the table.
///
/// Extracts data from the table based on the reference type:
/// - **Scalar**: Returns a single cell value as `Value::Scalar`
/// - **Column Vector**: Returns all values in a column (from first data row onwards) as `Value::Vector`
/// - **Row Vector**: Returns all values in a row as `Value::Vector`
///
/// # Empty Cell Handling
///
/// Empty cells and non-numeric cells are automatically treated as `Decimal::ZERO`.
/// This allows formulas to work seamlessly with sparse or partially-filled tables.
///
/// # Arguments
///
/// * `cell_ref` - The parsed cell reference to resolve
/// * `rows` - Reference to the table rows
///
/// # Returns
///
/// * `Some(Value)` if the reference is valid and within table bounds
/// * `None` if the reference is out of bounds
///
/// # Examples
///
/// ```
/// // Given a table with values in column A: [10, "", "text", 30]
/// // ColumnVector resolves to: Vector([10, 0, 0, 30])
/// ```
fn resolve_reference(cell_ref: &CellReference, rows: &Vec<Vec<String>>) -> Option<Value> {
    match cell_ref {
        CellReference::Scalar { row, col } => {
            // Get single cell value
            if *row < rows.len() && *col < rows[*row].len() {
                let cell_value = &rows[*row][*col];
                if let Ok(decimal) = Decimal::from_str(cell_value) {
                    Some(Value::Scalar(decimal))
                } else {
                    // Empty or non-numeric cells are treated as 0
                    Some(Value::Scalar(Decimal::ZERO))
                }
            } else {
                None
            }
        }
        CellReference::ColumnVector { col } => {
            // Get all values in the column (starting from first data row)
            let mut values = Vec::new();
            for row_idx in FIRST_DATA_ROW_INDEX..rows.len() {
                if *col < rows[row_idx].len() {
                    let cell_value = &rows[row_idx][*col];
                    if let Ok(decimal) = Decimal::from_str(cell_value) {
                        values.push(decimal);
                    } else {
                        // Empty or non-numeric cells are treated as 0
                        values.push(Decimal::ZERO);
                    }
                }
            }
            Some(Value::Vector(values))
        }
        CellReference::RowVector { row } => {
            // Get all values in the row
            // Row 1 means first data row
            let row_idx = formula_row_to_table_index(*row);
            if row_idx < rows.len() {
                let mut values = Vec::new();
                for cell_value in &rows[row_idx] {
                    if let Ok(decimal) = Decimal::from_str(cell_value) {
                        values.push(decimal);
                    } else {
                        // Empty or non-numeric cells are treated as 0
                        values.push(Decimal::ZERO);
                    }
                }
                Some(Value::Vector(values))
            } else {
                None
            }
        }
    }
}

/// Helper function to compute decimal power for integer exponents
fn decimal_pow(base: Decimal, exp: Decimal) -> Option<Decimal> {
    // Try to convert exponent to i64 for integer power
    if let Some(exp_i64) = exp.to_i64() {
        if exp_i64 >= 0 {
            // Positive integer exponent: compute by repeated multiplication
            let mut result = Decimal::ONE;
            for _ in 0..exp_i64 {
                result *= base;
            }
            return Some(result);
        } else {
            // Negative exponent: base^(-n) = 1 / base^n
            let positive_exp = exp_i64.unsigned_abs();
            let mut result = Decimal::ONE;
            for _ in 0..positive_exp {
                result *= base;
            }
            return Some(Decimal::ONE / result);
        }
    }

    // For non-integer exponents, we'd need to convert to f64, compute, and convert back
    // This loses precision but is necessary for fractional powers
    if let Some(base_f64) = base.to_f64() {
        if let Some(exp_f64) = exp.to_f64() {
            let result = base_f64.powf(exp_f64);
            return Decimal::from_f64(result);
        }
    }

    None
}

/// Evaluates a binary operation with automatic broadcasting support.
///
/// Supports all four combinations of scalar and vector operands:
/// - **Scalar ○ Scalar**: Standard arithmetic
/// - **Vector ○ Vector**: Element-wise operation (uses minimum length if sizes differ)
/// - **Vector ○ Scalar**: Broadcasts scalar to each vector element
/// - **Scalar ○ Vector**: Broadcasts scalar to each vector element
///
/// # Broadcasting Rules
///
/// When combining a scalar with a vector, the scalar is automatically applied
/// to every element of the vector. For example:
/// - `[1, 2, 3] + 10` → `[11, 12, 13]`
/// - `5 * [2, 4, 6]` → `[10, 20, 30]`
///
/// # Supported Operators
///
/// * `+` - Addition
/// * `-` - Subtraction
/// * `*` - Multiplication
/// * `/` - Division (returns `None` on division by zero)
/// * `^` - Exponentiation
///
/// # Arguments
///
/// * `op` - The operator character
/// * `left` - Left operand (Scalar or Vector)
/// * `right` - Right operand (Scalar or Vector)
///
/// # Returns
///
/// * `Some(Value)` if the operation succeeds
/// * `None` if division by zero occurs or the operator is invalid
fn evaluate_operation(op: char, left: Value, right: Value) -> Option<Value> {
    match (left, right) {
        // Scalar op Scalar
        (Value::Scalar(l), Value::Scalar(r)) => {
            let result = match op {
                '+' => l + r,
                '-' => l - r,
                '*' => l * r,
                '/' => {
                    if r == Decimal::ZERO {
                        return None; // Division by zero
                    }
                    l / r
                }
                '^' => decimal_pow(l, r)?,
                _ => return None,
            };
            Some(Value::Scalar(result))
        }

        // Vector op Vector (element-wise)
        (Value::Vector(l_vec), Value::Vector(r_vec)) => {
            let min_len = l_vec.len().min(r_vec.len());
            let mut result = Vec::with_capacity(min_len);

            for i in 0..min_len {
                let val = match op {
                    '+' => l_vec[i] + r_vec[i],
                    '-' => l_vec[i] - r_vec[i],
                    '*' => l_vec[i] * r_vec[i],
                    '/' => {
                        if r_vec[i] == Decimal::ZERO {
                            return None; // Division by zero
                        }
                        l_vec[i] / r_vec[i]
                    }
                    '^' => decimal_pow(l_vec[i], r_vec[i])?,
                    _ => return None,
                };
                result.push(val);
            }

            Some(Value::Vector(result))
        }

        // Vector op Scalar (broadcast scalar to each element)
        (Value::Vector(vec), Value::Scalar(scalar)) => {
            let mut result = Vec::with_capacity(vec.len());

            for v in vec {
                let val = match op {
                    '+' => v + scalar,
                    '-' => v - scalar,
                    '*' => v * scalar,
                    '/' => {
                        if scalar == Decimal::ZERO {
                            return None; // Division by zero
                        }
                        v / scalar
                    }
                    '^' => decimal_pow(v, scalar)?,
                    _ => return None,
                };
                result.push(val);
            }

            Some(Value::Vector(result))
        }

        // Scalar op Vector (broadcast scalar to each element)
        (Value::Scalar(scalar), Value::Vector(vec)) => {
            let mut result = Vec::with_capacity(vec.len());

            for v in vec {
                let val = match op {
                    '+' => scalar + v,
                    '-' => scalar - v,
                    '*' => scalar * v,
                    '/' => {
                        if v == Decimal::ZERO {
                            return None; // Division by zero
                        }
                        scalar / v
                    }
                    '^' => decimal_pow(scalar, v)?,
                    _ => return None,
                };
                result.push(val);
            }

            Some(Value::Vector(result))
        }
    }
}

/// Evaluates a function call (e.g., sum(...))
/// Currently supports: sum
fn evaluate_function_call(name: &str, arg_value: Value) -> Option<Value> {
    match name.to_lowercase().as_str() {
        "sum" => {
            match arg_value {
                Value::Scalar(s) => Some(Value::Scalar(s)), // sum of scalar is itself
                Value::Vector(vec) => {
                    let sum = vec.iter().fold(Decimal::ZERO, |acc, &x| acc + x);
                    Some(Value::Scalar(sum))
                }
            }
        }
        _ => None, // Unknown function
    }
}

/// Helper to detect if expression starts with a function call
/// Returns (function_name, arg_start_idx, arg_end_idx) if found
fn detect_function_call(tokens: &[String]) -> Option<(String, usize, usize)> {
    if tokens.len() < 3 {
        return None;
    }

    // Check if first token looks like a function name and second is '('
    if tokens[1] == "(" {
        let func_name = tokens[0].to_lowercase();
        // Find matching closing parenthesis
        let mut depth = 1;
        for (i, token) in tokens.iter().enumerate().skip(2) {
            if token == "(" {
                depth += 1;
            } else if token == ")" {
                depth -= 1;
                if depth == 0 {
                    return Some((func_name, 2, i));
                }
            }
        }
    }
    None
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
/// - **Operators**: `+`, `-`, `*`, `/`, `^` (with proper precedence)
/// - **Functions**: `sum(expression)`
/// - **Parentheses**: `(A_ + B_) * 2`
///
/// # Operator Precedence
///
/// 1. Parentheses `()` (highest)
/// 2. Exponentiation `^`
/// 3. Multiplication `*` and Division `/`
/// 4. Addition `+` and Subtraction `-` (lowest)
///
/// # Arguments
///
/// * `expr` - The expression string to evaluate
/// * `rows` - Reference to the table rows for resolving cell references
///
/// # Returns
///
/// * `Some(Value::Scalar)` for scalar results
/// * `Some(Value::Vector)` for vector results
/// * `None` if the expression is invalid or contains errors
fn evaluate_expression_value(expr: &str, rows: &Vec<Vec<String>>) -> Option<Value> {
    let tokens = tokenize_expression(expr);

    // Check for function call at the start
    if let Some((func_name, arg_start, arg_end)) = detect_function_call(&tokens) {
        // Extract argument tokens
        let arg_tokens = &tokens[arg_start..arg_end];
        // Recursively evaluate the argument
        let arg_expr = arg_tokens.join(" ");
        let arg_value = evaluate_expression_value(&arg_expr, rows)?;
        // Apply the function
        return evaluate_function_call(&func_name, arg_value);
    }

    // Convert tokens to Values (resolve references and parse numbers)
    eval_tokens_value(&tokens, rows)
}

/// Evaluates tokens with proper operator precedence, supporting Value enum
fn eval_tokens_value(tokens: &[String], rows: &Vec<Vec<String>>) -> Option<Value> {
    if tokens.is_empty() {
        return None;
    }

    // Base case: single token
    if tokens.len() == 1 {
        let token = &tokens[0];

        // Try to parse as cell reference (scalar or vector)
        if let Some(cell_ref) = parse_cell_reference(token) {
            return resolve_reference(&cell_ref, rows);
        }

        // Try to parse as number
        if let Ok(decimal) = Decimal::from_str(token) {
            return Some(Value::Scalar(decimal));
        }

        return None;
    }

    // Handle parentheses first
    if let Some(processed) = process_parentheses_value(tokens, rows) {
        return eval_tokens_value(&processed, rows);
    }

    // Find lowest precedence operator (+ or -) at the top level (evaluated last)
    let mut depth = 0;
    for (i, token) in tokens.iter().enumerate().rev() {
        if token == ")" {
            depth += 1;
        } else if token == "(" {
            depth -= 1;
        } else if depth == 0 && (token == "+" || token == "-") && i > 0 {
            let left = eval_tokens_value(&tokens[..i], rows)?;
            let right = eval_tokens_value(&tokens[i + 1..], rows)?;
            let op = token.chars().next()?;
            return evaluate_operation(op, left, right);
        }
    }

    // Find next precedence operator (* or /) at the top level
    depth = 0;
    for (i, token) in tokens.iter().enumerate().rev() {
        if token == ")" {
            depth += 1;
        } else if token == "(" {
            depth -= 1;
        } else if depth == 0 && (token == "*" || token == "/") && i > 0 {
            let left = eval_tokens_value(&tokens[..i], rows)?;
            let right = eval_tokens_value(&tokens[i + 1..], rows)?;
            let op = token.chars().next()?;
            return evaluate_operation(op, left, right);
        }
    }

    // Find highest precedence operator (^) at the top level (evaluated first)
    depth = 0;
    for (i, token) in tokens.iter().enumerate().rev() {
        if token == ")" {
            depth += 1;
        } else if token == "(" {
            depth -= 1;
        } else if depth == 0 && token == "^" && i > 0 {
            let left = eval_tokens_value(&tokens[..i], rows)?;
            let right = eval_tokens_value(&tokens[i + 1..], rows)?;
            return evaluate_operation('^', left, right);
        }
    }

    None
}

/// Processes parentheses by finding the first pair and evaluating the content
fn process_parentheses_value(tokens: &[String], rows: &Vec<Vec<String>>) -> Option<Vec<String>> {
    // Find the first opening parenthesis
    let open_idx = tokens.iter().position(|t| t == "(")?;

    // Find the matching closing parenthesis
    let mut depth = 1;
    let mut close_idx = None;
    for (i, token) in tokens.iter().enumerate().skip(open_idx + 1) {
        if token == "(" {
            depth += 1;
        } else if token == ")" {
            depth -= 1;
            if depth == 0 {
                close_idx = Some(i);
                break;
            }
        }
    }

    let close_idx = close_idx?;

    // Evaluate the content inside the parentheses
    let result = eval_tokens_value(&tokens[open_idx + 1..close_idx], rows)?;

    // Convert result back to string token
    let result_str = match result {
        Value::Scalar(d) => d.to_string(),
        Value::Vector(_) => return None, // Can't reduce vector in parentheses context yet
    };

    // Build new token list with the parentheses replaced by the result
    let mut new_tokens = Vec::new();
    new_tokens.extend(tokens[..open_idx].iter().map(|s| s.to_string()));
    new_tokens.push(result_str);
    new_tokens.extend(tokens[close_idx + 1..].iter().map(|s| s.to_string()));

    Some(new_tokens)
}

/// Tokenizes a mathematical expression string into individual components.
///
/// Splits the expression into tokens while preserving operators, parentheses,
/// numbers, and identifiers (cell references, function names). Whitespace is
/// ignored except as a separator.
///
/// # Arguments
///
/// * `expr` - The expression string to tokenize
///
/// # Returns
///
/// A vector of token strings. Each token is one of:
/// - An operator: `+`, `-`, `*`, `/`, `^`
/// - A parenthesis: `(`, `)`
/// - A number: `42`, `3.14`
/// - An identifier: `A1`, `B_`, `sum`
///
/// # Examples
///
/// ```
/// tokenize_expression("A1 + B2 * 3")  // ["A1", "+", "B2", "*", "3"]
/// tokenize_expression("sum(A_)")      // ["sum", "(", "A_", ")"]
/// tokenize_expression("(2 + 3) ^ 2")  // ["(", "2", "+", "3", ")", "^", "2"]
/// ```
fn tokenize_expression(expr: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();

    for ch in expr.chars() {
        match ch {
            '+' | '-' | '*' | '/' | '^' | '(' | ')' => {
                if !current_token.is_empty() {
                    tokens.push(current_token.trim().to_string());
                    current_token.clear();
                }
                tokens.push(ch.to_string());
            }
            ' ' => {
                if !current_token.is_empty() {
                    tokens.push(current_token.trim().to_string());
                    current_token.clear();
                }
            }
            _ => {
                current_token.push(ch);
            }
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token.trim().to_string());
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_ref_to_index() {
        assert_eq!(cell_ref_to_index("A1"), Some((2, 0))); // First data row (skips header at 0 and separator at 1)
        assert_eq!(cell_ref_to_index("B1"), Some((2, 1))); // First data row, column B
        assert_eq!(cell_ref_to_index("A2"), Some((3, 0))); // Second data row
        assert_eq!(cell_ref_to_index("D3"), Some((4, 3))); // Third data row, column D
        assert_eq!(cell_ref_to_index("Z10"), Some((11, 25))); // 10th data row, column Z
        assert_eq!(cell_ref_to_index("a1"), Some((2, 0))); // lowercase
        assert_eq!(cell_ref_to_index("A0"), None); // row 0 invalid
        assert_eq!(cell_ref_to_index("1A"), None); // wrong format
    }

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
            Some(Value::Vector(vec![
                Decimal::from(10),
                Decimal::from(20),
                Decimal::from(30)
            ]))
        );
    }

    #[test]
    fn test_resolve_row_vector() {
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
            Some(Value::Vector(vec![
                Decimal::ZERO,  // "Values" is non-numeric, treated as 0
                Decimal::from(10),
                Decimal::from(20)
            ]))
        );
    }

    #[test]
    fn test_resolve_empty_cells_as_zero() {
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
            Some(Value::Vector(vec![
                Decimal::from(10),
                Decimal::ZERO,  // empty treated as 0
                Decimal::ZERO,  // non-numeric treated as 0
                Decimal::from(30)
            ]))
        );
    }

    #[test]
    fn test_vector_addition() {
        let left = Value::Vector(vec![Decimal::from(1), Decimal::from(2), Decimal::from(3)]);
        let right = Value::Vector(vec![Decimal::from(4), Decimal::from(5), Decimal::from(6)]);

        let result = evaluate_operation('+', left, right);

        assert_eq!(
            result,
            Some(Value::Vector(vec![
                Decimal::from(5),
                Decimal::from(7),
                Decimal::from(9)
            ]))
        );
    }

    #[test]
    fn test_vector_scalar_multiply() {
        let vec = Value::Vector(vec![Decimal::from(2), Decimal::from(4), Decimal::from(6)]);
        let scalar = Value::Scalar(Decimal::from(3));

        let result = evaluate_operation('*', vec, scalar);

        assert_eq!(
            result,
            Some(Value::Vector(vec![
                Decimal::from(6),
                Decimal::from(12),
                Decimal::from(18)
            ]))
        );
    }

    #[test]
    fn test_broadcast_scalar() {
        let scalar = Value::Scalar(Decimal::from(10));
        let vec = Value::Vector(vec![Decimal::from(1), Decimal::from(2), Decimal::from(3)]);

        let result = evaluate_operation('+', scalar, vec);

        assert_eq!(
            result,
            Some(Value::Vector(vec![
                Decimal::from(11),
                Decimal::from(12),
                Decimal::from(13)
            ]))
        );
    }

    #[test]
    fn test_length_mismatch() {
        let left = Value::Vector(vec![Decimal::from(1), Decimal::from(2), Decimal::from(3)]);
        let right = Value::Vector(vec![Decimal::from(10), Decimal::from(20)]);

        let result = evaluate_operation('+', left, right);

        // Should use minimum length (2)
        assert_eq!(
            result,
            Some(Value::Vector(vec![
                Decimal::from(11),
                Decimal::from(22)
            ]))
        );
    }

    #[test]
    fn test_sum_vector() {
        let vec = Value::Vector(vec![Decimal::from(1), Decimal::from(2), Decimal::from(3)]);
        let result = evaluate_function_call("sum", vec);

        assert_eq!(result, Some(Value::Scalar(Decimal::from(6))));
    }

    #[test]
    fn test_sum_scalar() {
        let scalar = Value::Scalar(Decimal::from(42));
        let result = evaluate_function_call("sum", scalar);

        assert_eq!(result, Some(Value::Scalar(Decimal::from(42))));
    }

    #[test]
    fn test_sum_empty_vector() {
        let vec = Value::Vector(vec![]);
        let result = evaluate_function_call("sum", vec);

        assert_eq!(result, Some(Value::Scalar(Decimal::ZERO)));
    }

    #[test]
    fn test_power_operator() {
        let base = Value::Scalar(Decimal::from(2));
        let exp = Value::Scalar(Decimal::from(3));

        let result = evaluate_operation('^', base, exp);

        assert_eq!(result, Some(Value::Scalar(Decimal::from(8))));
    }

    #[test]
    fn test_power_precedence() {
        // Test: 2 * 3^2 = 2 * 9 = 18
        let rows = vec![
            vec!["A".to_string()],
            vec!["---".to_string()],
        ];

        let result = evaluate_expression_value("2 * 3 ^ 2", &rows);

        assert_eq!(result, Some(Value::Scalar(Decimal::from(18))));
    }

    #[test]
    fn test_power_in_expression() {
        // Test: (2+3)^2 = 5^2 = 25
        let rows = vec![
            vec!["A".to_string()],
            vec!["---".to_string()],
        ];

        let result = evaluate_expression_value("( 2 + 3 ) ^ 2", &rows);

        assert_eq!(result, Some(Value::Scalar(Decimal::from(25))));
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
}
