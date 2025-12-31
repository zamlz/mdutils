//! Error types for table processing and formula evaluation

use thiserror::Error;

/// Errors that can occur during table formula parsing and evaluation
#[derive(Debug, Error, Clone, PartialEq)]
#[allow(dead_code)]
pub enum FormulaError {
    /// Formula parsing failed
    #[error("Failed to parse formula: {0}")]
    ParseError(String),

    /// Invalid formula syntax
    #[error("Invalid formula syntax: {0}")]
    InvalidSyntax(String),

    /// Empty expression
    #[error("Empty expression")]
    EmptyExpression,

    /// Invalid token in expression
    #[error("Invalid token: '{token}' {context}")]
    InvalidToken { token: String, context: String },

    /// Cell reference out of bounds
    #[error("cell {cell} is out of bounds: {reason}")]
    CellOutOfBounds { cell: String, reason: String },

    /// Column reference out of bounds
    #[error("column vector {column}_ is out of bounds: {reason}")]
    ColumnOutOfBounds { column: String, reason: String },

    /// Row reference out of bounds
    #[error("row vector _{row} is out of bounds: {reason}")]
    RowOutOfBounds { row: usize, reason: String },

    /// Matrix dimension mismatch
    #[error("matrix operation failed: {operation} requires {expected}, but got {actual}")]
    DimensionMismatch {
        operation: String,
        expected: String,
        actual: String,
    },

    /// Type mismatch in operation
    #[error("type error: {operation} cannot be applied to {operand_type}")]
    TypeError {
        operation: String,
        operand_type: String,
    },

    /// Division by zero
    #[error("division by zero")]
    DivisionByZero,

    /// Unknown function
    #[error("unknown function: '{name}' (supported functions: {supported})")]
    UnknownFunction { name: String, supported: String },

    /// Function argument error
    #[error("Function '{function}' error: {reason}")]
    FunctionError { function: String, reason: String },

    /// Evaluation failed
    #[error("Failed to evaluate expression '{expression}': {reason}")]
    EvalError { expression: String, reason: String },

    /// Unexpected token during parsing
    #[error("unexpected token: '{token}' at position {position}")]
    UnexpectedToken { token: String, position: usize },

    /// Unmatched parenthesis
    #[error("unmatched {paren_type} parenthesis '{paren}'")]
    UnmatchedParenthesis {
        paren: char,
        paren_type: String, // "opening" or "closing"
    },

    /// Assignment target error
    #[error("Invalid assignment target: {reason}")]
    InvalidAssignment { reason: String },

    /// Transpose operation error
    #[error("cannot transpose {value_type}")]
    TransposeError { value_type: String },

    /// Matrix multiplication error
    #[error("Matrix multiplication failed: {reason}")]
    MatrixMultiplicationError { reason: String },

    /// Runtime evaluation error (generic message without prefix)
    #[error("{0}")]
    RuntimeError(String),
}

impl FormulaError {
    /// Create a cell out of bounds error
    pub fn cell_out_of_bounds(cell: impl Into<String>, reason: impl Into<String>) -> Self {
        FormulaError::CellOutOfBounds {
            cell: cell.into(),
            reason: reason.into(),
        }
    }

    /// Create a column out of bounds error
    pub fn column_out_of_bounds(column: impl Into<String>, reason: impl Into<String>) -> Self {
        FormulaError::ColumnOutOfBounds {
            column: column.into(),
            reason: reason.into(),
        }
    }

    /// Create a row out of bounds error
    pub fn row_out_of_bounds(row: usize, reason: impl Into<String>) -> Self {
        FormulaError::RowOutOfBounds {
            row,
            reason: reason.into(),
        }
    }

    /// Format error with visual context showing position in expression
    pub fn with_context(&self, expression: &str, span: crate::table::formula::Span) -> String {
        let mut result = format!("{}\n", self);

        // Add visual indicator
        result.push_str(&format!("{}\n", expression));

        // Add pointer to error location
        let pointer = " ".repeat(span.start) + &"^".repeat((span.end - span.start).max(1));
        result.push_str(&pointer);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_with_context_single_char() {
        let error = FormulaError::RuntimeError("invalid operator".to_string());
        let expression = "A1 + B2 @ C3";
        let span = crate::table::formula::Span::new(8, 9); // Points to '@'

        let result = error.with_context(expression, span);

        // Should show the expression, then a pointer under the error location
        assert!(result.contains("A1 + B2 @ C3"));
        assert!(result.contains("        ^")); // 8 spaces then caret
    }

    #[test]
    fn test_error_with_context_multi_char() {
        let error = FormulaError::RuntimeError("invalid cell reference".to_string());
        let expression = "A1 + ZZ99 * C3";
        let span = crate::table::formula::Span::new(5, 9); // Points to "ZZ99"

        let result = error.with_context(expression, span);

        // Should show multiple carets for multi-character span
        assert!(result.contains("A1 + ZZ99 * C3"));
        assert!(result.contains("     ^^^^")); // 5 spaces then 4 carets
    }

    #[test]
    fn test_error_with_context_at_start() {
        let error = FormulaError::RuntimeError("invalid token".to_string());
        let expression = "@ + B2";
        let span = crate::table::formula::Span::new(0, 1); // Points to first char

        let result = error.with_context(expression, span);

        assert!(result.contains("@ + B2"));
        assert!(result.contains("^")); // No leading spaces, caret at position 0
    }
}
