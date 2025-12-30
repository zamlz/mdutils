use rust_decimal::Decimal;

/// Represents a span of characters in the source expression
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }

    pub fn single(pos: usize) -> Self {
        Span { start: pos, end: pos + 1 }
    }

    pub(crate) fn merge(&self, other: &Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

// Table row index constant
// Markdown tables have a header row, separator row, then data rows starting at index 2
pub(crate) const FIRST_DATA_ROW_INDEX: usize = 2;

/// Converts a formula row number (1-based) to actual table index
/// Formula row 1 = first data row (table index 2)
/// Formula row 2 = second data row (table index 3), etc.
pub(crate) fn formula_row_to_table_index(row_num: usize) -> usize {
    FIRST_DATA_ROW_INDEX + (row_num - 1)
}

/// Converts a column index to its letter representation (0 -> A, 1 -> B, etc.)
pub(crate) fn col_index_to_letter(col: usize) -> String {
    let col_char = (b'A' + col as u8) as char;
    col_char.to_string()
}

/// Represents a value in a formula - either a scalar or a matrix
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Value {
    Scalar(Decimal),
    Matrix {
        rows: usize,
        cols: usize,
        data: Vec<Decimal>, // stored in row-major order
    },
}

impl Value {
    /// Creates a row vector (1×n matrix)
    pub(crate) fn row_vector(data: Vec<Decimal>) -> Self {
        let cols = data.len();
        Value::Matrix { rows: 1, cols, data }
    }

    /// Creates a column vector (n×1 matrix)
    pub(crate) fn column_vector(data: Vec<Decimal>) -> Self {
        let rows = data.len();
        Value::Matrix { rows, cols: 1, data }
    }

    /// Transposes a matrix (swaps rows and cols)
    pub(crate) fn transpose(self) -> Option<Self> {
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
    pub(crate) fn as_scalar(&self) -> Option<Decimal> {
        match self {
            Value::Scalar(d) => Some(*d),
            Value::Matrix { rows: 1, cols: 1, data } => data.first().copied(),
            _ => None,
        }
    }

    /// Checks if this is a column vector (n×1 matrix)
    pub(crate) fn is_column_vector(&self) -> bool {
        matches!(self, Value::Matrix { cols: 1, .. })
    }
}

/// Represents different types of cell references
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CellReference {
    Scalar { row: usize, col: usize },  // A1, B2, etc.
    ColumnVector { col: usize },         // A_, B_, etc.
    RowVector { row: usize },            // _1, _2, etc.
    Range {
        start_row: usize,
        start_col: usize,
        end_row: usize,
        end_col: usize
    },  // A1:C5, B2:B10, etc.
    ColumnRange {
        start_col: usize,
        end_col: usize
    },  // A_:C_ (all rows, columns A through C)
    RowRange {
        start_row: usize,
        end_row: usize
    },  // _1:_5 (all columns, rows 1 through 5)
}

/// Represents the left side of a formula assignment
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Assignment {
    Scalar { row: usize, col: usize },   // D2 = ...
    ColumnVector { col: usize },         // D_ = ...
    RowVector { row: usize },            // _1 = ...
    Range {
        start_row: usize,
        start_col: usize,
        end_row: usize,
        end_col: usize
    },  // A1:C3 = ...
    ColumnRange {
        start_col: usize,
        end_col: usize
    },  // A_:C_ = ...
    RowRange {
        start_row: usize,
        end_row: usize
    },  // _1:_5 = ...
}

/// Represents a statement in a formula - either a variable definition or an assignment
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Statement {
    /// Variable definition: let x = expression
    Let {
        name: String,
        span: Span,
    },
    /// Cell assignment: A1 = expression
    Assignment(Assignment),
}

impl Statement {
    /// Creates a Let statement
    pub(crate) fn let_statement(name: String, span: Span) -> Self {
        Statement::Let { name, span }
    }

    /// Creates an Assignment statement
    pub(crate) fn assignment(assignment: Assignment) -> Self {
        Statement::Assignment(assignment)
    }
}
