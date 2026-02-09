//! Common utilities shared across modules

use thiserror::Error;

// ============================================================================
// Exit Codes (BSD sysexits.h)
// ============================================================================

/// Exit codes following BSD sysexits.h conventions.
///
/// These codes provide meaningful exit status for different error conditions,
/// allowing callers to distinguish between different failure modes.
///
/// Reference: <https://man.freebsd.org/cgi/man.cgi?query=sysexits>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCode {
    /// Successful termination (0)
    Success,
    /// Command line usage error - invalid arguments or options (64)
    Usage,
    /// Data format error - input data was incorrect in some way (65)
    DataErr,
    /// Input/output error - error occurred during I/O operations (74)
    IoErr,
}

impl ExitCode {
    /// Returns the numeric exit code value
    pub fn code(self) -> i32 {
        match self {
            ExitCode::Success => 0,
            ExitCode::Usage => 64,
            ExitCode::DataErr => 65,
            ExitCode::IoErr => 74,
        }
    }
}

impl From<ExitCode> for i32 {
    fn from(exit_code: ExitCode) -> Self {
        exit_code.code()
    }
}

impl std::process::Termination for ExitCode {
    fn report(self) -> std::process::ExitCode {
        std::process::ExitCode::from(self.code() as u8)
    }
}

/// The type of code fence used in markdown
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FenceType {
    /// Backtick fence (```)
    Backtick,
    /// Tilde fence (~~~)
    Tilde,
}

/// Detects the fence type from a line, if it is a code fence.
///
/// A code fence must start with at least 3 backticks or 3 tildes.
///
/// # Examples
///
/// ```
/// use mdutils::common::get_fence_type;
/// use mdutils::common::FenceType;
///
/// assert_eq!(get_fence_type("```python"), Some(FenceType::Backtick));
/// assert_eq!(get_fence_type("~~~"), Some(FenceType::Tilde));
/// assert_eq!(get_fence_type("``"), None); // Not enough backticks
/// assert_eq!(get_fence_type("regular text"), None);
/// ```
pub fn get_fence_type(line: &str) -> Option<FenceType> {
    let trimmed = line.trim();
    if trimmed.starts_with("```") {
        Some(FenceType::Backtick)
    } else if trimmed.starts_with("~~~") {
        Some(FenceType::Tilde)
    } else {
        None
    }
}

/// Checks if a line is a code fence (``` or ~~~)
///
/// # Examples
///
/// ```
/// use mdutils::common::is_code_fence;
///
/// assert!(is_code_fence("```"));
/// assert!(is_code_fence("```python"));
/// assert!(is_code_fence("~~~markdown"));
/// assert!(!is_code_fence("regular text"));
/// ```
pub fn is_code_fence(line: &str) -> bool {
    get_fence_type(line).is_some()
}

/// Tracks code fence state to correctly handle nested code blocks.
///
/// This tracker ensures that when inside a backtick fence, encountering a tilde
/// fence doesn't incorrectly toggle the state (and vice versa). Only the same
/// type of fence that opened a code block can close it.
///
/// # Examples
///
/// ```
/// use mdutils::common::CodeFenceTracker;
///
/// let mut tracker = CodeFenceTracker::new();
///
/// assert!(!tracker.is_inside_code_block());
///
/// tracker.process_line("```python");
/// assert!(tracker.is_inside_code_block());
///
/// // Tilde fence inside backtick block doesn't close it
/// tracker.process_line("~~~");
/// assert!(tracker.is_inside_code_block());
///
/// // Matching backtick fence closes it
/// tracker.process_line("```");
/// assert!(!tracker.is_inside_code_block());
/// ```
#[derive(Debug, Clone, Default)]
pub struct CodeFenceTracker {
    active_fence: Option<FenceType>,
}

impl CodeFenceTracker {
    /// Creates a new tracker not inside any code block
    pub fn new() -> Self {
        Self { active_fence: None }
    }

    /// Returns true if currently inside a code block
    pub fn is_inside_code_block(&self) -> bool {
        self.active_fence.is_some()
    }

    /// Process a line and update the fence state.
    ///
    /// Returns true if the line is a fence that changed state (opened or closed a block).
    pub fn process_line(&mut self, line: &str) -> bool {
        if let Some(fence_type) = get_fence_type(line) {
            match self.active_fence {
                None => {
                    // Not in a code block, this fence starts one
                    self.active_fence = Some(fence_type);
                    true
                }
                Some(active) if active == fence_type => {
                    // In a code block and found matching fence type, close it
                    self.active_fence = None;
                    true
                }
                Some(_) => {
                    // In a code block but fence type doesn't match, treat as content
                    false
                }
            }
        } else {
            false
        }
    }
}

// ============================================================================
// Processing Result Types
// ============================================================================

/// The origin/category of a processing error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorOrigin {
    /// Error from table/formula processing
    Table,
    /// Error from code block execution
    Code,
    /// Error from TOC generation
    Toc,
    /// Error from done/checklist processing
    Done,
}

impl std::fmt::Display for ErrorOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorOrigin::Table => write!(f, "table"),
            ErrorOrigin::Code => write!(f, "code"),
            ErrorOrigin::Toc => write!(f, "toc"),
            ErrorOrigin::Done => write!(f, "done"),
        }
    }
}

/// A processing error with location and context information
#[derive(Debug, Clone, Error)]
#[error("[{origin}] line {line}: {message}")]
pub struct ProcessingError {
    /// The origin module that generated this error
    pub origin: ErrorOrigin,
    /// The line number where the error occurred (1-indexed for user display)
    pub line: usize,
    /// The error message
    pub message: String,
}

impl ProcessingError {
    /// Create a new processing error
    pub fn new(origin: ErrorOrigin, line: usize, message: impl Into<String>) -> Self {
        Self {
            origin,
            line,
            message: message.into(),
        }
    }

    /// Create a table error
    pub fn table(line: usize, message: impl Into<String>) -> Self {
        Self::new(ErrorOrigin::Table, line, message)
    }

    /// Create a code error
    pub fn code(line: usize, message: impl Into<String>) -> Self {
        Self::new(ErrorOrigin::Code, line, message)
    }

    /// Create a TOC error
    pub fn toc(line: usize, message: impl Into<String>) -> Self {
        Self::new(ErrorOrigin::Toc, line, message)
    }

    /// Create a done error
    pub fn done(line: usize, message: impl Into<String>) -> Self {
        Self::new(ErrorOrigin::Done, line, message)
    }
}

/// Result of processing a markdown document
///
/// This type allows modules to always produce output (even when errors occur)
/// while still collecting and reporting errors. This enables:
/// - Inline error markers in the output (like `<!-- md-error: ... -->`)
/// - Proper exit codes in the CLI
/// - Error reporting to stderr
///
/// # Examples
///
/// ```
/// use mdutils::common::ProcessingResult;
///
/// // Successful processing
/// let result = ProcessingResult::success("processed output".to_string());
/// assert!(!result.has_errors());
///
/// // Processing with errors
/// let result = ProcessingResult::success("partial output".to_string())
///     .with_error(mdutils::common::ProcessingError::table(5, "division by zero"));
/// assert!(result.has_errors());
/// ```
#[derive(Debug, Clone)]
pub struct ProcessingResult {
    /// The processed output (always produced, even with errors)
    pub output: String,
    /// Errors encountered during processing
    pub errors: Vec<ProcessingError>,
}

impl ProcessingResult {
    /// Create a successful result with no errors
    pub fn success(output: String) -> Self {
        Self {
            output,
            errors: vec![],
        }
    }

    /// Create a result with output and errors
    pub fn with_errors(output: String, errors: Vec<ProcessingError>) -> Self {
        Self { output, errors }
    }

    /// Add an error to this result (builder pattern)
    pub fn with_error(mut self, error: ProcessingError) -> Self {
        self.errors.push(error);
        self
    }

    /// Add multiple errors to this result (builder pattern)
    pub fn with_errors_added(mut self, errors: impl IntoIterator<Item = ProcessingError>) -> Self {
        self.errors.extend(errors);
        self
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Get the number of errors
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }
}

impl Default for ProcessingResult {
    fn default() -> Self {
        Self::success(String::new())
    }
}

/// Validates that an ID is not empty.
///
/// Valid IDs must:
/// - Not be empty
///
/// Any non-empty string is a valid ID.
///
/// # Examples
///
/// ```
/// use mdutils::common::validate_id;
///
/// assert!(validate_id("my_table_123").is_ok());
/// assert!(validate_id("MyTable").is_ok());
/// assert!(validate_id("my-table").is_ok());
/// assert!(validate_id("my table").is_ok());
/// assert!(validate_id("table!").is_ok());
/// assert!(validate_id("2024-sales").is_ok());
///
/// assert!(validate_id("").is_err());
/// ```
pub fn validate_id(id: &str) -> Result<(), String> {
    if id.is_empty() {
        return Err("ID cannot be empty".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fence_type() {
        assert_eq!(get_fence_type("```"), Some(FenceType::Backtick));
        assert_eq!(get_fence_type("```python"), Some(FenceType::Backtick));
        assert_eq!(get_fence_type("````"), Some(FenceType::Backtick));
        assert_eq!(get_fence_type("  ```rust  "), Some(FenceType::Backtick));

        assert_eq!(get_fence_type("~~~"), Some(FenceType::Tilde));
        assert_eq!(get_fence_type("~~~markdown"), Some(FenceType::Tilde));
        assert_eq!(get_fence_type("~~~~"), Some(FenceType::Tilde));
        assert_eq!(get_fence_type("  ~~~  "), Some(FenceType::Tilde));

        assert_eq!(get_fence_type("``"), None); // Not enough
        assert_eq!(get_fence_type("~~"), None); // Not enough
        assert_eq!(get_fence_type("regular text"), None);
        assert_eq!(get_fence_type(""), None);
    }

    #[test]
    fn test_is_code_fence() {
        assert!(is_code_fence("```"));
        assert!(is_code_fence("~~~"));
        assert!(!is_code_fence("text"));
    }

    #[test]
    fn test_code_fence_tracker_basic() {
        let mut tracker = CodeFenceTracker::new();
        assert!(!tracker.is_inside_code_block());

        // Enter backtick fence
        assert!(tracker.process_line("```python"));
        assert!(tracker.is_inside_code_block());

        // Regular line doesn't change state
        assert!(!tracker.process_line("some code"));
        assert!(tracker.is_inside_code_block());

        // Exit backtick fence
        assert!(tracker.process_line("```"));
        assert!(!tracker.is_inside_code_block());
    }

    #[test]
    fn test_code_fence_tracker_tilde() {
        let mut tracker = CodeFenceTracker::new();

        assert!(tracker.process_line("~~~"));
        assert!(tracker.is_inside_code_block());

        assert!(tracker.process_line("~~~"));
        assert!(!tracker.is_inside_code_block());
    }

    #[test]
    fn test_code_fence_tracker_nested_different_types() {
        let mut tracker = CodeFenceTracker::new();

        // Enter tilde fence
        assert!(tracker.process_line("~~~markdown"));
        assert!(tracker.is_inside_code_block());

        // Backtick fence inside tilde block should NOT toggle state
        assert!(!tracker.process_line("```python"));
        assert!(tracker.is_inside_code_block());

        // Another backtick fence still doesn't toggle
        assert!(!tracker.process_line("```"));
        assert!(tracker.is_inside_code_block());

        // Only tilde fence closes it
        assert!(tracker.process_line("~~~"));
        assert!(!tracker.is_inside_code_block());
    }

    #[test]
    fn test_code_fence_tracker_nested_same_type() {
        let mut tracker = CodeFenceTracker::new();

        // Enter backtick fence
        tracker.process_line("```");
        assert!(tracker.is_inside_code_block());

        // Text that looks like a fence marker but is content
        // (this is the edge case - if someone writes ``` in their code)
        // The tracker will close the block, which matches markdown behavior
        tracker.process_line("```");
        assert!(!tracker.is_inside_code_block());
    }

    #[test]
    fn test_processing_result_success() {
        let result = ProcessingResult::success("output".to_string());
        assert_eq!(result.output, "output");
        assert!(!result.has_errors());
        assert_eq!(result.error_count(), 0);
    }

    #[test]
    fn test_processing_result_with_errors() {
        let result = ProcessingResult::success("output".to_string())
            .with_error(ProcessingError::table(5, "error 1"))
            .with_error(ProcessingError::code(10, "error 2"));

        assert_eq!(result.output, "output");
        assert!(result.has_errors());
        assert_eq!(result.error_count(), 2);
        assert_eq!(result.errors[0].line, 5);
        assert_eq!(result.errors[0].origin, ErrorOrigin::Table);
        assert_eq!(result.errors[1].line, 10);
        assert_eq!(result.errors[1].origin, ErrorOrigin::Code);
    }

    #[test]
    fn test_processing_error_display() {
        let error = ProcessingError::table(42, "division by zero");
        let display = format!("{}", error);
        assert_eq!(display, "[table] line 42: division by zero");
    }

    #[test]
    fn test_valid_ids() {
        // Alphanumeric and underscores
        assert!(validate_id("table1").is_ok());
        assert!(validate_id("my_table").is_ok());
        assert!(validate_id("MyTable123").is_ok());
        assert!(validate_id("_table").is_ok());
        assert!(validate_id("table_").is_ok());
        assert!(validate_id("TABLE").is_ok());
        assert!(validate_id("t").is_ok());
        assert!(validate_id("_").is_ok());
        assert!(validate_id("___").is_ok());
        assert!(validate_id("a1b2c3").is_ok());

        // Whitespace is now allowed
        assert!(validate_id("my table").is_ok());
        assert!(validate_id("table ").is_ok());
        assert!(validate_id(" table").is_ok());
        assert!(validate_id("my\ttable").is_ok());

        // Special characters are now allowed
        assert!(validate_id("my-table").is_ok());
        assert!(validate_id("my.table").is_ok());
        assert!(validate_id("my@table").is_ok());
        assert!(validate_id("my!table").is_ok());
        assert!(validate_id("my#table").is_ok());
        assert!(validate_id("my$table").is_ok());
        assert!(validate_id("my%table").is_ok());
        assert!(validate_id("2024-sales-data").is_ok());
        assert!(validate_id("user@domain").is_ok());
        assert!(validate_id("v1.0.0").is_ok());
    }

    #[test]
    fn test_invalid_ids_empty() {
        assert!(validate_id("").is_err());

        let err = validate_id("").unwrap_err();
        assert_eq!(err, "ID cannot be empty");
    }
}
