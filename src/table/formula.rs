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

/// Converts a column index to its letter representation (0 -> A, 1 -> B, etc.)
fn col_index_to_letter(col: usize) -> String {
    let col_char = (b'A' + col as u8) as char;
    col_char.to_string()
}

/// Represents a value in a formula - either a scalar or a matrix
#[derive(Debug, Clone, PartialEq)]
enum Value {
    Scalar(Decimal),
    Matrix {
        rows: usize,
        cols: usize,
        data: Vec<Decimal>, // stored in row-major order
    },
}

impl Value {
    /// Creates a row vector (1×n matrix)
    fn row_vector(data: Vec<Decimal>) -> Self {
        let cols = data.len();
        Value::Matrix { rows: 1, cols, data }
    }

    /// Creates a column vector (n×1 matrix)
    fn column_vector(data: Vec<Decimal>) -> Self {
        let rows = data.len();
        Value::Matrix { rows, cols: 1, data }
    }

    /// Transposes a matrix (swaps rows and cols)
    fn transpose(self) -> Option<Self> {
        match self {
            Value::Scalar(_) => None, // Cannot transpose a scalar
            Value::Matrix { rows, cols, data } => {
                // Transpose by converting row-major to column-major
                let mut transposed = Vec::with_capacity(data.len());
                for col in 0..cols {
                    for row in 0..rows {
                        transposed.push(data[row * cols + col]);
                    }
                }
                Some(Value::Matrix {
                    rows: cols,
                    cols: rows,
                    data: transposed,
                })
            }
        }
    }

    /// Checks if this is a 1×1 matrix (can be extracted as scalar)
    fn as_scalar(&self) -> Option<Decimal> {
        match self {
            Value::Scalar(d) => Some(*d),
            Value::Matrix { rows: 1, cols: 1, data } => data.get(0).copied(),
            _ => None,
        }
    }

    /// Checks if this is a column vector (n×1 matrix)
    fn is_column_vector(&self) -> bool {
        matches!(self, Value::Matrix { cols: 1, .. })
    }
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

/// Applies a column vector of values to a table column
/// Starts at first data row (after header and separator)
fn apply_column_vector_assignment(rows: &mut Vec<Vec<String>>, col: usize, value: &Value) {
    if let Value::Matrix { rows: _n_rows, cols: 1, data } = value {
        for (i, &val) in data.iter().enumerate() {
            let row_idx = FIRST_DATA_ROW_INDEX + i;
            if row_idx < rows.len() && col < rows[row_idx].len() {
                rows[row_idx][col] = val.to_string();
            }
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
/// # Returns
///
/// A vector of Option<String> where each element corresponds to a formula.
/// None indicates the formula succeeded, Some(error) indicates it failed with the given error message.
pub fn apply_formulas(rows: &mut Vec<Vec<String>>, formulas: &[String]) -> Vec<Option<String>> {
    let mut errors = Vec::new();

    for formula in formulas {
        let formula_trimmed = formula.trim();

        // Try to parse the formula
        let (assignment, expr) = match parse_formula(formula_trimmed) {
            Some(parsed) => parsed,
            None => {
                errors.push(Some(format!("Failed to parse formula '{}': invalid syntax (expected format: TARGET = EXPRESSION)", formula_trimmed)));
                continue;
            }
        };

        // Try to evaluate the expression
        let value = match evaluate_expression_value(&expr, rows) {
            Ok(v) => v,
            Err(error_msg) => {
                errors.push(Some(format!("Failed to evaluate expression '{}': {}", expr, error_msg)));
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
                } else if col >= rows.get(0).map(|r| r.len()).unwrap_or(0) {
                    Some(format!("Assignment failed for '{}': column index out of bounds", formula_trimmed))
                } else {
                    apply_column_vector_assignment(rows, col, &value);
                    None  // Success
                }
            }
        };

        errors.push(error);
    }

    errors
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
/// # Behavior Details
///
/// **Scalar references** (e.g., "A1", "B2", "Z99"):
/// - Column letter is converted to a zero-based index (A=0, B=1, etc.)
/// - Row number must be a positive integer
/// - Returns the table row index (accounting for header and separator rows)
///
/// **Column vector references** (e.g., "A_", "B_", "Z_"):
/// - Represents all data rows in the specified column
/// - Column letter is converted to zero-based index
/// - Used for operations on entire columns
///
/// **Row vector references** (e.g., "_1", "_2", "_99"):
/// - Represents all columns in the specified row
/// - Row number is 1-based where 1 refers to the first data row
/// - Used for operations on entire rows
///
/// Invalid tokens return None, including: empty strings, numbers without letters,
/// invalid formats, or row number 0.
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
fn resolve_reference(cell_ref: &CellReference, rows: &Vec<Vec<String>>) -> Result<Value, String> {
    match cell_ref {
        CellReference::Scalar { row, col } => {
            // Get single cell value
            if *row >= rows.len() {
                let col_letter = col_index_to_letter(*col);
                return Err(format!(
                    "cell reference {}{} is out of bounds: row {} does not exist (table has {} rows)",
                    col_letter, row + 1, row + 1, rows.len()
                ));
            }
            if *col >= rows[*row].len() {
                let col_letter = col_index_to_letter(*col);
                return Err(format!(
                    "cell reference {}{} is out of bounds: column {} does not exist (row has {} columns)",
                    col_letter, row + 1, col_letter, rows[*row].len()
                ));
            }

            let cell_value = &rows[*row][*col];
            if let Ok(decimal) = Decimal::from_str(cell_value) {
                Ok(Value::Scalar(decimal))
            } else {
                // Empty or non-numeric cells are treated as 0
                Ok(Value::Scalar(Decimal::ZERO))
            }
        }
        CellReference::ColumnVector { col } => {
            // Get all values in the column (starting from first data row)
            // Returns a column vector (n×1 matrix)
            let col_letter = col_index_to_letter(*col);

            // Check if column exists in at least the first data row
            if rows.len() <= FIRST_DATA_ROW_INDEX {
                return Err(format!(
                    "column vector {}_ cannot be resolved: table has no data rows (only {} rows total)",
                    col_letter, rows.len()
                ));
            }

            if *col >= rows[FIRST_DATA_ROW_INDEX].len() {
                return Err(format!(
                    "column vector {}_ is out of bounds: column {} does not exist (table has {} columns)",
                    col_letter, col_letter, rows[FIRST_DATA_ROW_INDEX].len()
                ));
            }

            let mut data = Vec::new();
            for row_idx in FIRST_DATA_ROW_INDEX..rows.len() {
                if *col < rows[row_idx].len() {
                    let cell_value = &rows[row_idx][*col];
                    if let Ok(decimal) = Decimal::from_str(cell_value) {
                        data.push(decimal);
                    } else {
                        // Empty or non-numeric cells are treated as 0
                        data.push(Decimal::ZERO);
                    }
                }
            }
            Ok(Value::column_vector(data))
        }
        CellReference::RowVector { row } => {
            // Get all values in the row
            // Row 1 means first data row
            // Returns a row vector (1×n matrix)
            let row_idx = formula_row_to_table_index(*row);
            if row_idx >= rows.len() {
                return Err(format!(
                    "row vector _{} is out of bounds: row {} does not exist (table has {} rows)",
                    row, row, rows.len()
                ));
            }

            let mut data = Vec::new();
            for cell_value in &rows[row_idx] {
                if let Ok(decimal) = Decimal::from_str(cell_value) {
                    data.push(decimal);
                } else {
                    // Empty or non-numeric cells are treated as 0
                    data.push(Decimal::ZERO);
                }
            }
            Ok(Value::row_vector(data))
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
/// * `@` - Matrix multiplication (dot product for vectors)
///
/// # Arguments
///
/// * `op` - The operator character
/// * `left` - Left operand (Scalar or Vector)
/// * `right` - Right operand (Scalar or Vector)
///
/// # Returns
///
/// * `Ok(Value)` if the operation succeeds
/// * `Err(String)` with a specific error message if the operation fails
fn evaluate_operation(op: char, left: Value, right: Value) -> Result<Value, String> {
    // Handle matrix multiplication (@) - uses proper matrix multiplication rules
    if op == '@' {
        return match (&left, &right) {
            (Value::Matrix { rows: m, cols: n, data: left_data },
             Value::Matrix { rows: n2, cols: p, data: right_data }) => {
                // Check dimension compatibility: (m×n) @ (n2×p) requires n == n2
                if n != n2 {
                    return Err(format!(
                        "matrix multiplication dimension mismatch: cannot multiply ({}×{}) @ ({}×{}) - inner dimensions {} and {} must match",
                        m, n, n2, p, n, n2
                    ));
                }

                // Perform matrix multiplication
                let mut result = Vec::with_capacity(m * p);
                for i in 0..(*m) {
                    for j in 0..(*p) {
                        let mut sum = Decimal::ZERO;
                        for k in 0..(*n) {
                            sum += left_data[i * n + k] * right_data[k * p + j];
                        }
                        result.push(sum);
                    }
                }

                // Return result as (m×p) matrix
                Ok(Value::Matrix {
                    rows: *m,
                    cols: *p,
                    data: result,
                })
            }
            (Value::Scalar(_), Value::Scalar(_)) => {
                Err("cannot use matrix multiplication (@) with two scalar values - use * for scalar multiplication".to_string())
            }
            (Value::Scalar(_), Value::Matrix { rows, cols, .. }) => {
                Err(format!("cannot use matrix multiplication (@) with scalar on left side and ({}×{}) matrix on right side", rows, cols))
            }
            (Value::Matrix { rows, cols, .. }, Value::Scalar(_)) => {
                Err(format!("cannot use matrix multiplication (@) with ({}×{}) matrix on left side and scalar on right side", rows, cols))
            }
        };
    }

    // Handle other operators (+, -, *, /, ^)
    match (left, right) {
        // Scalar op Scalar
        (Value::Scalar(l), Value::Scalar(r)) => {
            let result = apply_scalar_op(op, l, r)
                .ok_or_else(|| format!("division by zero in scalar operation: {} {} {}", l, op, r))?;
            Ok(Value::Scalar(result))
        }

        // Matrix op Matrix (element-wise) - must have same dimensions
        (Value::Matrix { rows: m1, cols: n1, data: data1 },
         Value::Matrix { rows: m2, cols: n2, data: data2 }) => {
            // For element-wise operations, dimensions must match
            if m1 != m2 || n1 != n2 {
                return Err(format!(
                    "element-wise operation '{}' requires matching dimensions: got ({}×{}) and ({}×{})",
                    op, m1, n1, m2, n2
                ));
            }

            let mut result = Vec::with_capacity(data1.len());
            for (i, (&a, &b)) in data1.iter().zip(data2.iter()).enumerate() {
                let value = apply_scalar_op(op, a, b)
                    .ok_or_else(|| format!("division by zero in element-wise operation at position {}", i))?;
                result.push(value);
            }

            Ok(Value::Matrix {
                rows: m1,
                cols: n1,
                data: result,
            })
        }

        // Matrix op Scalar (broadcast scalar to all elements)
        (Value::Matrix { rows, cols, data }, Value::Scalar(scalar)) => {
            let mut result = Vec::with_capacity(data.len());
            for (i, &v) in data.iter().enumerate() {
                let value = apply_scalar_op(op, v, scalar)
                    .ok_or_else(|| format!("division by zero when broadcasting scalar to matrix at position {}", i))?;
                result.push(value);
            }

            Ok(Value::Matrix { rows, cols, data: result })
        }

        // Scalar op Matrix (broadcast scalar to all elements)
        (Value::Scalar(scalar), Value::Matrix { rows, cols, data }) => {
            let mut result = Vec::with_capacity(data.len());
            for (i, &v) in data.iter().enumerate() {
                let value = apply_scalar_op(op, scalar, v)
                    .ok_or_else(|| format!("division by zero when broadcasting scalar to matrix at position {}", i))?;
                result.push(value);
            }

            Ok(Value::Matrix { rows, cols, data: result })
        }
    }
}

/// Helper function to apply a scalar operation to two Decimal values
fn apply_scalar_op(op: char, left: Decimal, right: Decimal) -> Option<Decimal> {
    match op {
        '+' => Some(left + right),
        '-' => Some(left - right),
        '*' => Some(left * right),
        '/' => {
            if right == Decimal::ZERO {
                None
            } else {
                Some(left / right)
            }
        }
        '^' => decimal_pow(left, right),
        _ => None,
    }
}

/// Evaluates a function call (e.g., sum(...))
/// Currently supports: sum
fn evaluate_function_call(name: &str, arg_value: Value) -> Result<Value, String> {
    match name.to_lowercase().as_str() {
        "sum" => {
            match arg_value {
                Value::Scalar(s) => Ok(Value::Scalar(s)), // sum of scalar is itself
                Value::Matrix { data, .. } => {
                    let sum = data.iter().fold(Decimal::ZERO, |acc, &x| acc + x);
                    Ok(Value::Scalar(sum))
                }
            }
        }
        _ => Err(format!("unknown function: '{}' (supported functions: sum)", name))
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
/// * `Some(Value::Scalar)` for scalar results
/// * `Some(Value::Vector)` for vector results
/// * `None` if the expression is invalid or contains errors
fn evaluate_expression_value(expr: &str, rows: &Vec<Vec<String>>) -> Result<Value, String> {
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
fn eval_tokens_value(tokens: &[String], rows: &Vec<Vec<String>>) -> Result<Value, String> {
    if tokens.is_empty() {
        return Err("empty expression".to_string());
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
            return Ok(Value::Scalar(decimal));
        }

        return Err(format!("invalid token: '{}' is not a valid number or cell reference", token));
    }

    // Handle parentheses first
    if let Some(processed) = process_parentheses_value(tokens, rows)? {
        return eval_tokens_value(&processed, rows);
    }

    // Handle .T transpose operator (postfix operator with highest precedence)
    // Check if the expression ends with a ".T" pattern
    if tokens.len() >= 3 && tokens[tokens.len() - 1] == "T" && tokens[tokens.len() - 2] == "." {
        // Evaluate everything before ".T"
        let value = eval_tokens_value(&tokens[..tokens.len() - 2], rows)?;
        // Apply transpose
        match value {
            Value::Scalar(_) => {
                return Err("cannot transpose a scalar value - only matrices can be transposed".to_string());
            }
            Value::Matrix { .. } => {
                return value.transpose().ok_or_else(|| "transpose operation failed".to_string());
            }
        }
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
            let op = token.chars().next().unwrap();
            return evaluate_operation(op, left, right);
        }
    }

    // Find next precedence operator (*, /, @) at the top level
    depth = 0;
    for (i, token) in tokens.iter().enumerate().rev() {
        if token == ")" {
            depth += 1;
        } else if token == "(" {
            depth -= 1;
        } else if depth == 0 && (token == "*" || token == "/" || token == "@") && i > 0 {
            let left = eval_tokens_value(&tokens[..i], rows)?;
            let right = eval_tokens_value(&tokens[i + 1..], rows)?;
            let op = token.chars().next().unwrap();
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

    Err(format!("could not evaluate expression with tokens: {:?}", tokens))
}

/// Processes parentheses by finding the first pair and evaluating the content
fn process_parentheses_value(tokens: &[String], rows: &Vec<Vec<String>>) -> Result<Option<Vec<String>>, String> {
    // Find the first opening parenthesis
    let open_idx = match tokens.iter().position(|t| t == "(") {
        Some(idx) => idx,
        None => return Ok(None), // No parentheses found
    };

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

    let close_idx = match close_idx {
        Some(idx) => idx,
        None => return Err("unmatched opening parenthesis '(' - missing closing ')'".to_string()),
    };

    // Evaluate the content inside the parentheses
    let result = eval_tokens_value(&tokens[open_idx + 1..close_idx], rows)?;

    // Convert result back to string token
    let result_str = match result {
        Value::Scalar(d) => d.to_string(),
        Value::Matrix { rows, cols, .. } => {
            // Try to convert 1×1 matrix to scalar
            if let Some(scalar) = result.as_scalar() {
                scalar.to_string()
            } else {
                // Can't reduce non-1×1 matrix in expression context
                return Err(format!(
                    "cannot use ({}×{}) matrix result from parenthesized expression as a value in larger expression - only scalars and 1×1 matrices can be used",
                    rows, cols
                ));
            }
        }
    };

    // Build new token list with the parentheses replaced by the result
    let mut new_tokens = Vec::new();
    new_tokens.extend(tokens[..open_idx].iter().map(|s| s.to_string()));
    new_tokens.push(result_str);
    new_tokens.extend(tokens[close_idx + 1..].iter().map(|s| s.to_string()));

    Ok(Some(new_tokens))
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
/// - An operator: `+`, `-`, `*`, `/`, `^`, `@`
/// - A parenthesis: `(`, `)`
/// - A number: `42`, `3.14`
/// - An identifier: `A1`, `B_`, `sum`
///
/// # Tokenization Rules
///
/// **Operators** are split into individual tokens:
/// - Arithmetic: `+`, `-`, `*`, `/`, `^` (exponentiation)
/// - Matrix operations: `@` (matrix multiplication)
/// - Each operator becomes a single-character token
///
/// **Parentheses** are split into individual tokens:
/// - Opening `(` and closing `)` parentheses
/// - Used for grouping expressions and function arguments
///
/// **The dot operator** for transpose (`.T`) is handled specially:
/// - The dot `.` and `T` are kept as separate tokens
/// - Allows the parser to recognize the `.T` transpose operator
///
/// **Numbers** are kept together as single tokens:
/// - Integer literals: "42", "100"
/// - Decimal literals: "3.14", "0.5"
/// - Scientific notation is not currently supported
///
/// **Identifiers** (cell references and function names) are kept together:
/// - Cell references: "A1", "B2", "A_", "_1"
/// - Function names: "sum", "avg"
/// - Alphanumeric characters and underscores
///
/// **Whitespace** is ignored except as a token separator. Multiple spaces are
/// treated the same as a single space.
///
/// For example, "A1 + B2 * 3" becomes ["A1", "+", "B2", "*", "3"], and
/// "sum(A_)" becomes ["sum", "(", "A_", ")"].
fn tokenize_expression(expr: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let chars: Vec<char> = expr.chars().collect();

    let mut i = 0;
    while i < chars.len() {
        let ch = chars[i];

        match ch {
            '+' | '-' | '*' | '/' | '^' | '@' | '(' | ')' => {
                if !current_token.is_empty() {
                    tokens.push(current_token.trim().to_string());
                    current_token.clear();
                }
                tokens.push(ch.to_string());
                i += 1;
            }
            '.' => {
                // Check if this is a decimal point in a number
                // It's a decimal point if:
                // 1. We're building a numeric token AND
                // 2. The next character is a digit
                let is_decimal_point = !current_token.is_empty()
                    && current_token.chars().all(|c| c.is_ascii_digit())
                    && i + 1 < chars.len()
                    && chars[i + 1].is_ascii_digit();

                if is_decimal_point {
                    // Include the decimal point in the current number token
                    current_token.push(ch);
                    i += 1;
                } else {
                    // It's the transpose operator - treat as separate token
                    if !current_token.is_empty() {
                        tokens.push(current_token.trim().to_string());
                        current_token.clear();
                    }
                    tokens.push(ch.to_string());
                    i += 1;
                }
            }
            ' ' => {
                if !current_token.is_empty() {
                    tokens.push(current_token.trim().to_string());
                    current_token.clear();
                }
                i += 1;
            }
            _ => {
                current_token.push(ch);
                i += 1;
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
            Ok(Value::column_vector(vec![
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
            Ok(Value::row_vector(vec![
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
        // Different length column vectors - should fail (dimensions must match for element-wise ops)
        let left = Value::column_vector(vec![Decimal::from(1), Decimal::from(2), Decimal::from(3)]);
        let right = Value::column_vector(vec![Decimal::from(10), Decimal::from(20)]);

        let result = evaluate_operation('+', left, right);

        // Dimensions don't match (3×1 vs 2×1), so operation fails
        assert!(result.is_err());
    }

    #[test]
    fn test_sum_vector() {
        let vec = Value::column_vector(vec![Decimal::from(1), Decimal::from(2), Decimal::from(3)]);
        let result = evaluate_function_call("sum", vec);

        assert_eq!(result, Ok(Value::Scalar(Decimal::from(6))));
    }

    #[test]
    fn test_sum_scalar() {
        let scalar = Value::Scalar(Decimal::from(42));
        let result = evaluate_function_call("sum", scalar);

        assert_eq!(result, Ok(Value::Scalar(Decimal::from(42))));
    }

    #[test]
    fn test_sum_empty_vector() {
        let vec = Value::column_vector(vec![]);
        let result = evaluate_function_call("sum", vec);

        assert_eq!(result, Ok(Value::Scalar(Decimal::ZERO)));
    }

    #[test]
    fn test_power_operator() {
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
        // (1×3) @ (2×1) is invalid - inner dimensions don't match
        let row = Value::row_vector(vec![Decimal::from(1), Decimal::from(2), Decimal::from(3)]);
        let col = Value::column_vector(vec![Decimal::from(4), Decimal::from(5)]);

        let result = evaluate_operation('@', row, col);

        assert!(result.is_err());
    }

    #[test]
    fn test_matrix_mult_scalar_not_supported() {
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
        // Scalar @ Scalar is not valid for matrix multiplication
        let left = Value::Scalar(Decimal::from(5));
        let right = Value::Scalar(Decimal::from(10));

        let result = evaluate_operation('@', left, right);

        assert!(result.is_err());
    }

    #[test]
    fn test_column_column_invalid() {
        // Column @ Column is invalid: (3×1) @ (3×1) - inner dimensions don't match
        let left = Value::column_vector(vec![Decimal::from(1), Decimal::from(2), Decimal::from(3)]);
        let right = Value::column_vector(vec![Decimal::from(4), Decimal::from(5), Decimal::from(6)]);

        let result = evaluate_operation('@', left, right);

        assert!(result.is_err());
    }

    #[test]
    fn test_row_row_invalid() {
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
}
