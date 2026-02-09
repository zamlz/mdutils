/// Table of Contents generation for markdown documents
///
/// This module provides functionality to generate a table of contents from markdown headers.
/// The TOC is inserted at the location of the `<!-- md-toc: -->` marker and ends with
/// `<!-- md-toc: end -->`.
///
/// # Usage
///
/// The `process_toc` function reads markdown from a string, finds the TOC marker,
/// generates a table of contents from all headers in the document, and returns the
/// updated document.
///
/// # TOC Format
///
/// - Headers are converted to clickable links using GitHub-style anchor slugs
/// - Indentation reflects header level (H2 is indented more than H1, etc.)
/// - Duplicate slugs are handled by appending -1, -2, etc.
///
/// # Example
///
/// ```markdown
/// # My Document
/// <!-- md-toc: -->
/// <!-- md-toc: end -->
///
/// ## Section 1
/// ### Subsection 1.1
/// ## Section 2
/// ```
///
/// Becomes:
///
/// ```markdown
/// # My Document
/// <!-- md-toc: -->
/// - [Section 1](#section-1)
///   - [Subsection 1.1](#subsection-11)
/// - [Section 2](#section-2)
/// <!-- md-toc: end -->
///
/// ## Section 1
/// ### Subsection 1.1
/// ## Section 2
/// ```
mod parser;

use crate::common::{CodeFenceTracker, ProcessingResult};
use parser::{parse_headers, Header};

const TOC_START_MARKER: &str = "<!-- md-toc: -->";
const TOC_END_MARKER: &str = "<!-- md-toc: end -->";

/// Process a markdown document and generate/update table of contents
///
/// # Arguments
///
/// * `input` - The markdown document as a string
///
/// # Returns
///
/// A [`ProcessingResult`] containing the updated document with TOC generated or updated.
/// This operation is infallible, so the result will never contain errors.
pub fn process_toc(input: &str) -> ProcessingResult {
    let lines: Vec<&str> = input.lines().collect();

    // Find TOC marker (skip those inside code fences)
    let mut toc_start_line = None;
    let mut fence_tracker = CodeFenceTracker::new();
    for (i, line) in lines.iter().enumerate() {
        fence_tracker.process_line(line);
        if !fence_tracker.is_inside_code_block() && line.trim() == TOC_START_MARKER {
            toc_start_line = Some(i);
            break;
        }
    }

    // If no TOC marker, return input unchanged
    let Some(toc_start_line) = toc_start_line else {
        return ProcessingResult::success(input.to_string());
    };

    // Find end marker (if it exists) - also skip those inside code fences
    let mut toc_end_line = None;
    let mut fence_tracker = CodeFenceTracker::new();
    for (i, line) in lines.iter().enumerate() {
        fence_tracker.process_line(line);
        if !fence_tracker.is_inside_code_block() && line.trim() == TOC_END_MARKER {
            toc_end_line = Some(i);
            break;
        }
    }

    // Parse all headers in the document (after TOC marker to avoid self-reference)
    let headers = parse_headers(&lines, toc_start_line + 1);

    // Generate TOC content
    let toc_content = generate_toc(&headers);

    // Reconstruct document
    let mut result = Vec::new();

    // Add everything up to and including the TOC start marker
    for line in &lines[..=toc_start_line] {
        result.push(line.to_string());
    }

    // Add TOC content
    result.extend(toc_content);

    // Add end marker
    result.push(TOC_END_MARKER.to_string());

    // Add everything after the old TOC (or after start marker if no end marker)
    let skip_to = if let Some(end_line) = toc_end_line {
        end_line + 1
    } else {
        toc_start_line + 1
    };

    if skip_to < lines.len() {
        for line in &lines[skip_to..] {
            result.push(line.to_string());
        }
    }

    // Join with newlines
    ProcessingResult::success(result.join("\n") + "\n")
}

/// Generate table of contents from headers
///
/// # Arguments
///
/// * `headers` - List of parsed headers
///
/// # Returns
///
/// Vector of TOC lines (without the start/end markers)
fn generate_toc(headers: &[Header]) -> Vec<String> {
    if headers.is_empty() {
        return vec![];
    }

    // Find the minimum header level to use as base indentation
    let min_level = headers.iter().map(|h| h.level).min().unwrap_or(1);

    let mut toc_lines = Vec::new();

    for header in headers {
        // Calculate indentation (0 for min_level, 2 spaces per level after that)
        let indent_level = header.level.saturating_sub(min_level);
        let indent = "  ".repeat(indent_level);

        // Generate link: - [Text](#slug)
        let link = format!("{}- [{}](#{})", indent, header.text, header.slug);
        toc_lines.push(link);
    }

    toc_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_toc_marker() {
        let input = "# Header\n## Subheader\n";
        let result = process_toc(input);
        assert_eq!(result.output, input);
        assert!(!result.has_errors());
    }

    #[test]
    fn test_toc_generation_simple() {
        let input = r#"# Document
<!-- md-toc: -->
<!-- md-toc: end -->

## Section 1
### Subsection 1.1
## Section 2
"#;

        let result = process_toc(input);

        assert!(result.output.contains("<!-- md-toc: -->"));
        assert!(result.output.contains("<!-- md-toc: end -->"));
        assert!(result.output.contains("- [Section 1](#section-1)"));
        assert!(result.output.contains("  - [Subsection 1.1](#subsection-11)"));
        assert!(result.output.contains("- [Section 2](#section-2)"));
    }

    #[test]
    fn test_toc_update_existing() {
        let input = r#"# Document
<!-- md-toc: -->
- [Old Section](#old-section)
<!-- md-toc: end -->

## New Section
### New Subsection
"#;

        let result = process_toc(input);

        // Should update with new headers
        assert!(result.output.contains("- [New Section](#new-section)"));
        assert!(result.output.contains("  - [New Subsection](#new-subsection)"));
        // Should not contain old content
        assert!(!result.output.contains("Old Section"));
    }

    #[test]
    fn test_toc_without_end_marker() {
        let input = r#"# Document
<!-- md-toc: -->

## Section 1
"#;

        let result = process_toc(input);

        assert!(result.output.contains("<!-- md-toc: end -->"));
        assert!(result.output.contains("- [Section 1](#section-1)"));
    }
}
