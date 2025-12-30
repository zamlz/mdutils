use crate::table::error::FormulaError;
use crate::table::formula::types::{CellReference, Value, FIRST_DATA_ROW_INDEX, formula_row_to_table_index, col_index_to_letter};
use rust_decimal::Decimal;
use std::str::FromStr;

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
pub(crate) fn parse_cell_reference(token: &str) -> Option<CellReference> {
    let token = token.trim().to_uppercase();
    if token.is_empty() {
        return None;
    }

    // Check for row vector pattern: _N (underscore followed by number)
    if let Some(row_str) = token.strip_prefix('_') {
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
pub(crate) fn resolve_reference(cell_ref: &CellReference, rows: &[Vec<String>]) -> Result<Value, FormulaError> {
    match cell_ref {
        CellReference::Scalar { row, col } => {
            // Get single cell value
            if *row >= rows.len() {
                let col_letter = col_index_to_letter(*col);
                let cell = format!("{}{}", col_letter, row + 1);
                return Err(FormulaError::cell_out_of_bounds(
                    cell,
                    format!("row {} does not exist (table has {} rows)", row + 1, rows.len()),
                ));
            }
            if *col >= rows[*row].len() {
                let col_letter = col_index_to_letter(*col);
                let cell = format!("{}{}", col_letter, row + 1);
                return Err(FormulaError::cell_out_of_bounds(
                    cell,
                    format!("column {} does not exist (row has {} columns)", col_letter, rows[*row].len()),
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
                return Err(FormulaError::column_out_of_bounds(
                    col_letter,
                    format!("table has no data rows (only {} rows total)", rows.len()),
                ));
            }

            if *col >= rows[FIRST_DATA_ROW_INDEX].len() {
                return Err(FormulaError::column_out_of_bounds(
                    col_letter.clone(),
                    format!("column {} does not exist (table has {} columns)", col_letter, rows[FIRST_DATA_ROW_INDEX].len()),
                ));
            }

            let mut data = Vec::new();
            for row in rows.iter().skip(FIRST_DATA_ROW_INDEX) {
                if *col < row.len() {
                    let cell_value = &row[*col];
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
                return Err(FormulaError::row_out_of_bounds(
                    *row,
                    format!("row {} does not exist (table has {} rows)", row, rows.len()),
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
        CellReference::Range { start_row, start_col, end_row, end_col } => {
            // Extract a submatrix from the table
            // Validate bounds
            if *end_row >= rows.len() {
                let start_col_letter = col_index_to_letter(*start_col);
                let end_col_letter = col_index_to_letter(*end_col);
                return Err(FormulaError::cell_out_of_bounds(
                    format!("{}{}:{}{}", start_col_letter, start_row + 1, end_col_letter, end_row + 1),
                    format!("end row {} does not exist (table has {} rows)", end_row + 1, rows.len()),
                ));
            }

            // Check if columns exist in all rows of the range
            for row_idx in *start_row..=*end_row {
                if *end_col >= rows[row_idx].len() {
                    let start_col_letter = col_index_to_letter(*start_col);
                    let end_col_letter = col_index_to_letter(*end_col);
                    return Err(FormulaError::cell_out_of_bounds(
                        format!("{}{}:{}{}", start_col_letter, start_row + 1, end_col_letter, end_row + 1),
                        format!("column {} does not exist in row {} (row has {} columns)",
                            end_col_letter, row_idx + 1, rows[row_idx].len()),
                    ));
                }
            }

            // Extract the submatrix
            let num_rows = end_row - start_row + 1;
            let num_cols = end_col - start_col + 1;
            let mut data = Vec::with_capacity(num_rows * num_cols);

            for row_idx in *start_row..=*end_row {
                for col_idx in *start_col..=*end_col {
                    let cell_value = &rows[row_idx][col_idx];
                    if let Ok(decimal) = Decimal::from_str(cell_value) {
                        data.push(decimal);
                    } else {
                        // Empty or non-numeric cells are treated as 0
                        data.push(Decimal::ZERO);
                    }
                }
            }

            // Special case: if it's a 1x1 range, return a scalar
            if num_rows == 1 && num_cols == 1 {
                Ok(Value::Scalar(data[0]))
            } else {
                Ok(Value::Matrix {
                    rows: num_rows,
                    cols: num_cols,
                    data,
                })
            }
        }
        CellReference::ColumnRange { start_col, end_col } => {
            // Extract multiple columns (A_:C_) as a matrix
            // All data rows, columns start_col through end_col

            // Check if table has data rows
            if rows.len() <= FIRST_DATA_ROW_INDEX {
                let start_col_letter = col_index_to_letter(*start_col);
                let end_col_letter = col_index_to_letter(*end_col);
                return Err(FormulaError::column_out_of_bounds(
                    format!("{}_:{}_", start_col_letter, end_col_letter),
                    format!("table has no data rows (only {} rows total)", rows.len()),
                ));
            }

            // Validate columns exist
            if *end_col >= rows[FIRST_DATA_ROW_INDEX].len() {
                let start_col_letter = col_index_to_letter(*start_col);
                let end_col_letter = col_index_to_letter(*end_col);
                return Err(FormulaError::column_out_of_bounds(
                    format!("{}_:{}_", start_col_letter, end_col_letter),
                    format!("column {} does not exist (table has {} columns)",
                        end_col_letter, rows[FIRST_DATA_ROW_INDEX].len()),
                ));
            }

            // Extract the data
            let num_rows = rows.len() - FIRST_DATA_ROW_INDEX;
            let num_cols = end_col - start_col + 1;
            let mut data = Vec::with_capacity(num_rows * num_cols);

            for row in rows.iter().skip(FIRST_DATA_ROW_INDEX) {
                for col_idx in *start_col..=*end_col {
                    if col_idx < row.len() {
                        let cell_value = &row[col_idx];
                        if let Ok(decimal) = Decimal::from_str(cell_value) {
                            data.push(decimal);
                        } else {
                            data.push(Decimal::ZERO);
                        }
                    } else {
                        data.push(Decimal::ZERO);
                    }
                }
            }

            // Special case: single column range is equivalent to a column vector
            if num_cols == 1 {
                Ok(Value::column_vector(data))
            } else {
                Ok(Value::Matrix {
                    rows: num_rows,
                    cols: num_cols,
                    data,
                })
            }
        }
        CellReference::RowRange { start_row, end_row } => {
            // Extract multiple rows (_1:_5) as a matrix
            // Rows start_row through end_row, all columns

            let start_row_idx = formula_row_to_table_index(*start_row);
            let end_row_idx = formula_row_to_table_index(*end_row);

            // Validate rows exist
            if end_row_idx >= rows.len() {
                return Err(FormulaError::row_out_of_bounds(
                    *end_row,
                    format!("row {} does not exist (table has {} rows)", end_row, rows.len()),
                ));
            }

            // Determine number of columns (use the first row in range)
            let num_cols = rows[start_row_idx].len();
            let num_rows = end_row_idx - start_row_idx + 1;
            let mut data = Vec::with_capacity(num_rows * num_cols);

            for row_idx in start_row_idx..=end_row_idx {
                for col_idx in 0..num_cols {
                    if col_idx < rows[row_idx].len() {
                        let cell_value = &rows[row_idx][col_idx];
                        if let Ok(decimal) = Decimal::from_str(cell_value) {
                            data.push(decimal);
                        } else {
                            data.push(Decimal::ZERO);
                        }
                    } else {
                        data.push(Decimal::ZERO);
                    }
                }
            }

            // Special case: single row range is equivalent to a row vector
            if num_rows == 1 {
                Ok(Value::row_vector(data))
            } else {
                Ok(Value::Matrix {
                    rows: num_rows,
                    cols: num_cols,
                    data,
                })
            }
        }
    }
}

/// Converts an entire table to a matrix (all data rows, all columns)
pub(crate) fn table_to_matrix(rows: &[Vec<String>]) -> Result<Value, FormulaError> {
    if rows.len() < FIRST_DATA_ROW_INDEX {
        return Err(FormulaError::RuntimeError(
            "table has no data rows".to_string()
        ));
    }

    let num_rows = rows.len() - FIRST_DATA_ROW_INDEX;
    if num_rows == 0 {
        return Ok(Value::Matrix {
            rows: 0,
            cols: 0,
            data: vec![],
        });
    }

    let num_cols = rows[FIRST_DATA_ROW_INDEX].len();
    let mut data = Vec::new();

    for row_idx in FIRST_DATA_ROW_INDEX..rows.len() {
        for col_idx in 0..num_cols {
            if col_idx < rows[row_idx].len() {
                let cell = &rows[row_idx][col_idx];
                if let Ok(decimal) = Decimal::from_str(cell) {
                    data.push(decimal);
                } else {
                    data.push(Decimal::ZERO);
                }
            } else {
                data.push(Decimal::ZERO);
            }
        }
    }

    Ok(Value::Matrix {
        rows: num_rows,
        cols: num_cols,
        data,
    })
}
