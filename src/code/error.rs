//! Error types for code block processing and execution

use thiserror::Error;

/// Errors that can occur during code block processing
#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum CodeError {
    /// Directive parsing failed
    #[error("Failed to parse md-code directive: {0}")]
    DirectiveParseError(String),

    /// Duplicate code block ID
    #[error("Duplicate code block ID '{id}' found at line {line} (previously defined at line {previous_line})")]
    DuplicateId {
        id: String,
        line: usize,
        previous_line: usize,
    },

    /// Duplicate output block ID
    #[error("Duplicate output block ID '{id}' found at line {line} (previously defined at line {previous_line})")]
    DuplicateOutputId {
        id: String,
        line: usize,
        previous_line: usize,
    },

    /// Missing required field in directive
    #[error("Code block at line {line} is missing required field: {field}")]
    MissingField { line: usize, field: String },

    /// Empty bin specification
    #[error("Empty bin specification at line {line}")]
    EmptyBin { line: usize },

    /// Code execution failed
    #[error("Code execution failed: {0}")]
    ExecutionFailed(String),

    /// Process execution error
    #[error("Process execution failed: {0}")]
    ProcessError(String),

    /// Timeout error
    #[error("Code execution timed out after {seconds} seconds")]
    Timeout { seconds: u64 },

    /// I/O error during execution
    #[error("I/O error: {0}")]
    IoError(String),
}

impl CodeError {
    /// Create a duplicate ID error
    pub fn duplicate_id(id: impl Into<String>, line: usize, previous_line: usize) -> Self {
        CodeError::DuplicateId {
            id: id.into(),
            line,
            previous_line,
        }
    }

    /// Create a duplicate output ID error
    pub fn duplicate_output_id(id: impl Into<String>, line: usize, previous_line: usize) -> Self {
        CodeError::DuplicateOutputId {
            id: id.into(),
            line,
            previous_line,
        }
    }

    /// Create a missing field error
    pub fn missing_field(line: usize, field: impl Into<String>) -> Self {
        CodeError::MissingField {
            line,
            field: field.into(),
        }
    }
}
