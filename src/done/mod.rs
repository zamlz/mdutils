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
/// let output = process_done_with_timestamp(input, "2024-01-15 14:30:00");
/// assert!(output.contains("- [x] ~~Buy groceries~~ `COMPLETED: 2024-01-15 14:30:00`"));
/// assert!(output.contains("- [x] Walk the dog")); // unchanged, already checked
/// ```
use chrono::Local;

/// Process markdown and mark checklist items as done
///
/// # Arguments
///
/// * `input` - The markdown document as a string
///
/// # Returns
///
/// The updated document with checklist items marked as done
pub fn process_done(input: &str) -> String {
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
/// The updated document with checklist items marked as done
pub fn process_done_with_timestamp(input: &str, timestamp: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut result: Vec<String> = Vec::new();
    let mut code_fence_marker: Option<&str> = None; // Track which fence type started the block

    for line in lines {
        // Check if this line is a code fence
        if let Some(fence_type) = get_fence_type(line) {
            match code_fence_marker {
                None => {
                    // Not in a code block, this fence starts one
                    code_fence_marker = Some(fence_type);
                    result.push(line.to_string());
                }
                Some(marker) if marker == fence_type => {
                    // In a code block and found matching fence type, close it
                    code_fence_marker = None;
                    result.push(line.to_string());
                }
                Some(_) => {
                    // In a code block but fence type doesn't match, treat as content
                    result.push(line.to_string());
                }
            }
        } else if code_fence_marker.is_some() {
            // Pass through lines inside code blocks unchanged
            result.push(line.to_string());
        } else {
            result.push(process_line(line, timestamp));
        }
    }

    // Preserve trailing newline if input had one
    if input.ends_with('\n') {
        result.join("\n") + "\n"
    } else {
        result.join("\n")
    }
}

/// Returns the fence type if the line is a code fence, None otherwise.
/// Returns "```" for backtick fences and "~~~" for tilde fences.
fn get_fence_type(line: &str) -> Option<&'static str> {
    let trimmed = line.trim();
    if trimmed.starts_with("```") {
        Some("```")
    } else if trimmed.starts_with("~~~") {
        Some("~~~")
    } else {
        None
    }
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
        let output = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert_eq!(
            output,
            "- [x] ~~Buy groceries~~ `COMPLETED: 2024-01-15 14:30:00`"
        );
    }

    #[test]
    fn test_checked_item_unchanged() {
        // Already checked items should pass through unchanged
        let input = "- [x] Walk the dog";
        let output = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert_eq!(output, input);
    }

    #[test]
    fn test_uppercase_x_unchanged() {
        // Already checked items (uppercase X) should pass through unchanged
        let input = "- [X] Task with uppercase";
        let output = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert_eq!(output, input);
    }

    #[test]
    fn test_already_strikethrough() {
        let input = "- [x] ~~Already done~~ `COMPLETED: 2024-01-01 10:00:00`";
        let output = process_done_with_timestamp(input, TEST_TIMESTAMP);
        // Should be unchanged
        assert_eq!(output, input);
    }

    #[test]
    fn test_indented_item() {
        let input = "  - [ ] Nested task";
        let output = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert_eq!(
            output,
            "  - [x] ~~Nested task~~ `COMPLETED: 2024-01-15 14:30:00`"
        );
    }

    #[test]
    fn test_deeply_nested() {
        let input = "      - [ ] Very nested";
        let output = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert_eq!(
            output,
            "      - [x] ~~Very nested~~ `COMPLETED: 2024-01-15 14:30:00`"
        );
    }

    #[test]
    fn test_non_checklist_line() {
        let input = "This is regular text";
        let output = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert_eq!(output, input);
    }

    #[test]
    fn test_bullet_without_checkbox() {
        let input = "- Regular bullet point";
        let output = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert_eq!(output, input);
    }

    #[test]
    fn test_multiple_lines() {
        let input = "- [ ] Task 1\n- [x] Task 2\n- [ ] Task 3\n";
        let output = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert!(output.contains("- [x] ~~Task 1~~ `COMPLETED: 2024-01-15 14:30:00`"));
        assert!(output.contains("- [x] Task 2")); // unchanged, already checked
        assert!(output.contains("- [x] ~~Task 3~~ `COMPLETED: 2024-01-15 14:30:00`"));
    }

    #[test]
    fn test_mixed_content() {
        let input = "# Header\n- [ ] Task\n\nSome text\n- Regular bullet\n";
        let output = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert!(output.contains("# Header"));
        assert!(output.contains("- [x] ~~Task~~ `COMPLETED: 2024-01-15 14:30:00`"));
        assert!(output.contains("Some text"));
        assert!(output.contains("- Regular bullet"));
    }

    #[test]
    fn test_empty_checklist_item() {
        let input = "- [ ]";
        let output = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert_eq!(output, "- [x] ~~~~ `COMPLETED: 2024-01-15 14:30:00`");
    }

    #[test]
    fn test_preserves_trailing_newline() {
        let input = "- [ ] Task\n";
        let output = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert!(output.ends_with('\n'));
    }

    #[test]
    fn test_no_trailing_newline() {
        let input = "- [ ] Task";
        let output = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert!(!output.ends_with('\n'));
    }

    #[test]
    fn test_idempotency() {
        let input = "- [ ] Task\n";
        let output1 = process_done_with_timestamp(input, TEST_TIMESTAMP);
        let output2 = process_done_with_timestamp(&output1, TEST_TIMESTAMP);
        assert_eq!(output1, output2, "process_done should be idempotent");
    }

    #[test]
    fn test_tab_indentation() {
        let input = "\t- [ ] Tab indented";
        let output = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert_eq!(
            output,
            "\t- [x] ~~Tab indented~~ `COMPLETED: 2024-01-15 14:30:00`"
        );
    }

    #[test]
    fn test_code_block_passthrough() {
        let input = "- [ ] Real task\n```\n- [ ] Fake task in code\n```\n- [ ] Another real task\n";
        let output = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert!(output.contains("- [x] ~~Real task~~ `COMPLETED: 2024-01-15 14:30:00`"));
        assert!(output.contains("- [ ] Fake task in code")); // unchanged in code block
        assert!(output.contains("- [x] ~~Another real task~~ `COMPLETED: 2024-01-15 14:30:00`"));
    }

    #[test]
    fn test_tilde_code_block_passthrough() {
        let input = "- [ ] Real task\n~~~\n- [ ] Fake task\n~~~\n";
        let output = process_done_with_timestamp(input, TEST_TIMESTAMP);
        assert!(output.contains("- [x] ~~Real task~~ `COMPLETED: 2024-01-15 14:30:00`"));
        assert!(output.contains("- [ ] Fake task")); // unchanged in code block
    }
}
