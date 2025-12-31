/// Parser for markdown headers
///
/// This module provides functionality to parse markdown headers and generate
/// GitHub-style anchor slugs for table of contents links.
use std::collections::HashMap;

/// Represents a markdown header
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Header {
    pub level: usize,
    pub text: String,
    pub slug: String,
    pub line_number: usize,
}

/// Parse all headers from markdown lines
///
/// # Arguments
///
/// * `lines` - All lines in the document
/// * `start_from` - Line number to start parsing from (to skip TOC marker itself)
///
/// # Returns
///
/// Vector of parsed headers with generated slugs
///
/// # Note
///
/// Headers inside markdown code blocks (delimited by ``` or ~~~) are ignored.
pub(crate) fn parse_headers(lines: &[&str], start_from: usize) -> Vec<Header> {
    let mut headers = Vec::new();
    let mut slug_counts: HashMap<String, usize> = HashMap::new();
    let mut in_code_block = false;

    for (line_num, line) in lines.iter().enumerate().skip(start_from) {
        // Check for code block delimiters (``` or ~~~)
        let trimmed = line.trim();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_code_block = !in_code_block;
            continue;
        }

        // Skip header parsing if we're inside a code block
        if in_code_block {
            continue;
        }

        if let Some(header) = parse_header_line(line, line_num) {
            // Generate unique slug
            let base_slug = generate_slug(&header.text);
            let unique_slug = make_unique_slug(&base_slug, &mut slug_counts);

            headers.push(Header {
                level: header.level,
                text: header.text,
                slug: unique_slug,
                line_number: header.line_number,
            });
        }
    }

    headers
}

/// Parse a single line to extract header information
///
/// # Arguments
///
/// * `line` - The line to parse
/// * `line_number` - The line number in the document
///
/// # Returns
///
/// Some(Header) if the line is a header, None otherwise
fn parse_header_line(line: &str, line_number: usize) -> Option<Header> {
    let trimmed = line.trim_start();

    // Check if line starts with '#'
    if !trimmed.starts_with('#') {
        return None;
    }

    // Count the number of '#' characters
    let level = trimmed.chars().take_while(|&c| c == '#').count();

    // ATX headers (# Header) have a max level of 6
    if level > 6 {
        return None;
    }

    // Extract the text after the '#' characters
    let text = trimmed[level..].trim();

    // Empty headers are not valid
    if text.is_empty() {
        return None;
    }

    // Remove trailing '#' characters if present (alternate ATX syntax: # Header #)
    let text = text.trim_end_matches('#').trim_end();

    Some(Header {
        level,
        text: text.to_string(),
        slug: String::new(), // Will be filled in by parse_headers
        line_number,
    })
}

/// Generate a GitHub-style slug from header text
///
/// Rules:
/// - Convert to lowercase
/// - Replace spaces with hyphens
/// - Remove most special characters
/// - Keep alphanumeric, hyphens, and underscores
/// - Collapse consecutive hyphens to single hyphen
/// - Remove leading/trailing hyphens
///
/// # Arguments
///
/// * `text` - The header text
///
/// # Returns
///
/// The slug string
fn generate_slug(text: &str) -> String {
    let mut result = text
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else if c.is_whitespace() {
                '-'
            } else {
                // Remove special characters
                '\0'
            }
        })
        .filter(|&c| c != '\0')
        .collect::<String>();

    // Collapse consecutive hyphens
    while result.contains("--") {
        result = result.replace("--", "-");
    }

    result.trim_matches('-').to_string()
}

/// Make a slug unique by appending a number if necessary
///
/// # Arguments
///
/// * `slug` - The base slug
/// * `slug_counts` - HashMap tracking how many times each slug has been used
///
/// # Returns
///
/// A unique slug (may have -1, -2, etc. appended)
fn make_unique_slug(slug: &str, slug_counts: &mut HashMap<String, usize>) -> String {
    let count = slug_counts.entry(slug.to_string()).or_insert(0);
    *count += 1;

    if *count == 1 {
        slug.to_string()
    } else {
        format!("{}-{}", slug, *count - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_header_line() {
        let header = parse_header_line("# Hello World", 0).unwrap();
        assert_eq!(header.level, 1);
        assert_eq!(header.text, "Hello World");

        let header = parse_header_line("## Sub Section", 1).unwrap();
        assert_eq!(header.level, 2);
        assert_eq!(header.text, "Sub Section");

        let header = parse_header_line("### Level 3", 2).unwrap();
        assert_eq!(header.level, 3);
        assert_eq!(header.text, "Level 3");
    }

    #[test]
    fn test_parse_header_with_trailing_hashes() {
        let header = parse_header_line("# Hello World #", 0).unwrap();
        assert_eq!(header.text, "Hello World");

        let header = parse_header_line("## Sub Section ##", 1).unwrap();
        assert_eq!(header.text, "Sub Section");
    }

    #[test]
    fn test_not_a_header() {
        assert!(parse_header_line("Not a header", 0).is_none());
        assert!(parse_header_line("", 0).is_none());
        assert!(parse_header_line("#", 0).is_none()); // Empty header
        assert!(parse_header_line("####### Too many", 0).is_none());
    }

    #[test]
    fn test_generate_slug() {
        assert_eq!(generate_slug("Hello World"), "hello-world");
        assert_eq!(generate_slug("Test Section 1"), "test-section-1");
        assert_eq!(generate_slug("Special!@# Characters"), "special-characters");
        assert_eq!(generate_slug("Multiple   Spaces"), "multiple-spaces");
        assert_eq!(generate_slug("CamelCase"), "camelcase");
        assert_eq!(generate_slug("with_underscores"), "with_underscores");
    }

    #[test]
    fn test_make_unique_slug() {
        let mut counts = HashMap::new();

        assert_eq!(make_unique_slug("test", &mut counts), "test");
        assert_eq!(make_unique_slug("test", &mut counts), "test-1");
        assert_eq!(make_unique_slug("test", &mut counts), "test-2");
        assert_eq!(make_unique_slug("other", &mut counts), "other");
        assert_eq!(make_unique_slug("test", &mut counts), "test-3");
    }

    #[test]
    fn test_parse_headers() {
        let lines = vec![
            "# Main Header",
            "Some text",
            "## Section 1",
            "More text",
            "## Section 1", // Duplicate
            "### Subsection",
        ];

        let headers = parse_headers(&lines, 0);

        assert_eq!(headers.len(), 4);
        assert_eq!(headers[0].text, "Main Header");
        assert_eq!(headers[0].slug, "main-header");
        assert_eq!(headers[1].text, "Section 1");
        assert_eq!(headers[1].slug, "section-1");
        assert_eq!(headers[2].text, "Section 1");
        assert_eq!(headers[2].slug, "section-1-1"); // Duplicate slug
        assert_eq!(headers[3].text, "Subsection");
        assert_eq!(headers[3].slug, "subsection");
    }

    #[test]
    fn test_parse_headers_with_start_from() {
        let lines = vec![
            "# TOC Header", // Should be skipped
            "<!-- md-toc: -->",
            "## Actual Content",
            "### Subsection",
        ];

        let headers = parse_headers(&lines, 2); // Start from line 2

        assert_eq!(headers.len(), 2);
        assert_eq!(headers[0].text, "Actual Content");
        assert_eq!(headers[1].text, "Subsection");
    }

    #[test]
    fn test_skip_headers_in_code_blocks() {
        let lines = vec![
            "# Real Header",
            "```markdown",
            "# Fake Header in Code",
            "## Another Fake",
            "```",
            "## Real Header 2",
        ];

        let headers = parse_headers(&lines, 0);

        assert_eq!(headers.len(), 2);
        assert_eq!(headers[0].text, "Real Header");
        assert_eq!(headers[1].text, "Real Header 2");
    }

    #[test]
    fn test_skip_headers_in_tilde_code_blocks() {
        let lines = vec![
            "## Header 1",
            "~~~",
            "# Should be ignored",
            "~~~",
            "## Header 2",
        ];

        let headers = parse_headers(&lines, 0);

        assert_eq!(headers.len(), 2);
        assert_eq!(headers[0].text, "Header 1");
        assert_eq!(headers[1].text, "Header 2");
    }

    #[test]
    fn test_multiple_code_blocks() {
        let lines = vec![
            "# Header 1",
            "```",
            "# Ignore this",
            "```",
            "## Header 2",
            "```rust",
            "## Also ignore",
            "```",
            "### Header 3",
        ];

        let headers = parse_headers(&lines, 0);

        assert_eq!(headers.len(), 3);
        assert_eq!(headers[0].text, "Header 1");
        assert_eq!(headers[1].text, "Header 2");
        assert_eq!(headers[2].text, "Header 3");
    }

    #[test]
    fn test_nested_code_block_markers() {
        let lines = vec![
            "## Header 1",
            "```",
            "Some code with ``` inside",
            "# Not a header",
            "```",
            "## Header 2",
        ];

        let headers = parse_headers(&lines, 0);

        // The ``` inside the code block doesn't close it
        // Only the final ``` closes it
        assert_eq!(headers.len(), 2);
        assert_eq!(headers[0].text, "Header 1");
        assert_eq!(headers[1].text, "Header 2");
    }
}
