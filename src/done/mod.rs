/// Mark markdown checklist items as done
///
/// This module provides functionality to mark open checklist items as completed.
/// Open items (`- [ ]`) are marked with `[x]`, text is struck through, and a
/// completion timestamp is added.
///
/// # Usage
///
/// The `process_done` function reads markdown from a string and transforms open checklist items.
///
/// # Transformation
///
/// - `- [ ] task` becomes `- [x] ~~task~~ \`COMPLETED: 2024-01-15 14:30:00\``
/// - `- [x] task` is left unchanged (already completed)
/// - Already strikethrough items are left unchanged (idempotent)
/// - Non-checklist lines pass through unchanged
/// - Indentation is preserved for nested items
///
/// # Example
///
/// ```
/// use mdutils::done::process_done_with_timestamp;
///
/// let input = "- [ ] Buy groceries\n- [x] Walk the dog\n";
/// let result = process_done_with_timestamp(input, "2024-01-15 14:30:00");
/// assert!(result.output.contains("- [x] ~~Buy groceries~~ `COMPLETED: 2024-01-15 14:30:00`"));
/// assert!(result.output.contains("- [x] Walk the dog")); // unchanged, already checked
/// assert!(!result.has_errors());
/// ```
use crate::common::{CodeFenceTracker, ProcessingResult};
use chrono::Local;

/// Process markdown and mark checklist items as done
///
/// # Arguments
///
/// * `input` - The markdown document as a string
///
/// # Returns
///
/// A [`ProcessingResult`] containing the updated document with checklist items marked as done.
/// This operation is infallible, so the result will never contain errors.
pub fn process_done(input: &str) -> ProcessingResult {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    process_done_with_timestamp(input, &timestamp)
}

/// Process markdown and mark checklist items as done with a specific timestamp
///
/// This is useful for testing to ensure deterministic output.
///
/// # Arguments
///
/// * `input` - The markdown document as a string
/// * `timestamp` - The timestamp to use for completion markers
///
/// # Returns
///
/// A [`ProcessingResult`] containing the updated document with checklist items marked as done.
/// This operation is infallible, so the result will never contain errors.
pub fn process_done_with_timestamp(input: &str, timestamp: &str) -> ProcessingResult {
    let lines: Vec<&str> = input.lines().collect();
    let mut result: Vec<String> = Vec::new();
    let mut fence_tracker = CodeFenceTracker::new();

    for line in lines {
        // Process line through fence tracker (updates state if it's a fence)
        fence_tracker.process_line(line);

        if fence_tracker.is_inside_code_block() {
            // Pass through lines inside code blocks unchanged (including the opening fence)
            result.push(line.to_string());
        } else {
            // Check if this was a closing fence (we just exited the code block)
            if crate::common::is_code_fence(line) {
                result.push(line.to_string());
            } else {
                result.push(process_line(line, timestamp));
            }
        }
    }

    // Preserve trailing newline if input had one
    let output = if input.ends_with('\n') {
        result.join("\n") + "\n"
    } else {
        result.join("\n")
    };

    ProcessingResult::success(output)
}

/// Process a single line
fn process_line(line: &str, timestamp: &str) -> String {
    // Check if line is already strikethrough (contains ~~)
    if line.contains("~~") {
        return line.to_string();
    }

    // Try to parse as a checklist item
    if let Some(parsed) = parse_checklist_item(line) {
        format!(
            "{}- [x] ~~{}~~ `COMPLETED: {}`",
            parsed.indent, parsed.text, timestamp
        )
    } else {
        line.to_string()
    }
}

/// Parsed checklist item
struct ChecklistItem<'a> {
    indent: &'a str,
    text: &'a str,
}

/// Parse a line as an open (unchecked) checklist item
///
/// Returns None if the line is not an open checklist item.
/// Already checked items (`- [x]`) are not matched.
fn parse_checklist_item(line: &str) -> Option<ChecklistItem<'_>> {
    // Find leading whitespace
    let trimmed = line.trim_start();
    let indent_len = line.len() - trimmed.len();
    let indent = &line[..indent_len];

    // Only match unchecked items: - [ ]
    if let Some(text) = trimmed.strip_prefix("- [ ] ") {
        Some(ChecklistItem { indent, text })
    } else if trimmed == "- [ ]" {
        // Empty unchecked checklist item
        Some(ChecklistItem { indent, text: "" })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_TIMESTAMP: &str = "2024-01-15 14:30:00";

    #[test]
    fn test_unchecked_item() {
        let input = "- [ ] Buy groceries";
        let result = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert_eq!(
            result.output,
            "- [x] ~~Buy groceries~~ `COMPLETED: 2024-01-15 14:30:00`"
        );
        assert!(!result.has_errors());
    }

    #[test]
    fn test_checked_item_unchanged() {
        // Already checked items should pass through unchanged
        let input = "- [x] Walk the dog";
        let result = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert_eq!(result.output, input);
    }

    #[test]
    fn test_uppercase_x_unchanged() {
        // Already checked items (uppercase X) should pass through unchanged
        let input = "- [X] Task with uppercase";
        let result = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert_eq!(result.output, input);
    }

    #[test]
    fn test_already_strikethrough() {
        let input = "- [x] ~~Already done~~ `COMPLETED: 2024-01-01 10:00:00`";
        let result = process_done_with_timestamp(input, TEST_TIMESTAMP);
        // Should be unchanged
        assert_eq!(result.output, input);
    }

    #[test]
    fn test_indented_item() {
        let input = "  - [ ] Nested task";
        let result = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert_eq!(
            result.output,
            "  - [x] ~~Nested task~~ `COMPLETED: 2024-01-15 14:30:00`"
        );
    }

    #[test]
    fn test_deeply_nested() {
        let input = "      - [ ] Very nested";
        let result = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert_eq!(
            result.output,
            "      - [x] ~~Very nested~~ `COMPLETED: 2024-01-15 14:30:00`"
        );
    }

    #[test]
    fn test_non_checklist_line() {
        let input = "This is regular text";
        let result = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert_eq!(result.output, input);
    }

    #[test]
    fn test_bullet_without_checkbox() {
        let input = "- Regular bullet point";
        let result = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert_eq!(result.output, input);
    }

    #[test]
    fn test_multiple_lines() {
        let input = "- [ ] Task 1\n- [x] Task 2\n- [ ] Task 3\n";
        let result = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert!(result.output.contains("- [x] ~~Task 1~~ `COMPLETED: 2024-01-15 14:30:00`"));
        assert!(result.output.contains("- [x] Task 2")); // unchanged, already checked
        assert!(result.output.contains("- [x] ~~Task 3~~ `COMPLETED: 2024-01-15 14:30:00`"));
    }

    #[test]
    fn test_mixed_content() {
        let input = "# Header\n- [ ] Task\n\nSome text\n- Regular bullet\n";
        let result = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert!(result.output.contains("# Header"));
        assert!(result.output.contains("- [x] ~~Task~~ `COMPLETED: 2024-01-15 14:30:00`"));
        assert!(result.output.contains("Some text"));
        assert!(result.output.contains("- Regular bullet"));
    }

    #[test]
    fn test_empty_checklist_item() {
        let input = "- [ ]";
        let result = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert_eq!(result.output, "- [x] ~~~~ `COMPLETED: 2024-01-15 14:30:00`");
    }

    #[test]
    fn test_preserves_trailing_newline() {
        let input = "- [ ] Task\n";
        let result = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert!(result.output.ends_with('\n'));
    }

    #[test]
    fn test_no_trailing_newline() {
        let input = "- [ ] Task";
        let result = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert!(!result.output.ends_with('\n'));
    }

    #[test]
    fn test_idempotency() {
        let input = "- [ ] Task\n";
        let result1 = process_done_with_timestamp(input, TEST_TIMESTAMP);
        let result2 = process_done_with_timestamp(&result1.output, TEST_TIMESTAMP);
        assert_eq!(result1.output, result2.output, "process_done should be idempotent");
    }

    #[test]
    fn test_tab_indentation() {
        let input = "\t- [ ] Tab indented";
        let result = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert_eq!(
            result.output,
            "\t- [x] ~~Tab indented~~ `COMPLETED: 2024-01-15 14:30:00`"
        );
    }

    #[test]
    fn test_code_block_passthrough() {
        let input = "- [ ] Real task\n```\n- [ ] Fake task in code\n```\n- [ ] Another real task\n";
        let result = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert!(result.output.contains("- [x] ~~Real task~~ `COMPLETED: 2024-01-15 14:30:00`"));
        assert!(result.output.contains("- [ ] Fake task in code")); // unchanged in code block
        assert!(result.output.contains("- [x] ~~Another real task~~ `COMPLETED: 2024-01-15 14:30:00`"));
    }

    #[test]
    fn test_tilde_code_block_passthrough() {
        let input = "- [ ] Real task\n~~~\n- [ ] Fake task\n~~~\n";
        let result = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert!(result.output.contains("- [x] ~~Real task~~ `COMPLETED: 2024-01-15 14:30:00`"));
        assert!(result.output.contains("- [ ] Fake task")); // unchanged in code block
    }
}
