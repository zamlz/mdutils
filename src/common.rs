//! Common utilities shared across modules

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
