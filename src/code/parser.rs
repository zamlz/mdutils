use crate::code::error::CodeError;
use crate::common::validate_id;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CodeBlockDirective {
    pub id: String,
    pub bin: Option<String>,
    pub timeout: Option<u64>,
    pub fence: Option<String>, // Optional fence override for output block (e.g., "```", "~~~", "````")
    pub syntax: Option<String>, // Optional syntax language for output block (e.g., "json", "text")
}

#[derive(Debug)]
pub struct CodeBlock {
    pub start_line: usize,
    pub end_line: usize,
    #[allow(dead_code)]
    pub language: String,
    pub content: String,
    pub directive: Option<CodeBlockDirective>,
    pub fence: String, // The fence used for this code block (e.g., "```", "~~~")
}

#[derive(Debug)]
pub struct OutputBlock {
    #[allow(dead_code)]
    pub start_line: usize,
    #[allow(dead_code)]
    pub end_line: usize,
    #[allow(dead_code)]
    pub id: String,
    #[allow(dead_code)]
    pub content: String,
}

/// Checks if a line is the start of a code block (opening fence)
pub fn is_code_fence(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with("```") || trimmed.starts_with("~~~")
}

/// Gets the fence type (backtick or tilde) from a fence line
fn get_fence_type(line: &str) -> Option<char> {
    let trimmed = line.trim();
    if trimmed.starts_with("```") {
        Some('`')
    } else if trimmed.starts_with("~~~") {
        Some('~')
    } else {
        None
    }
}

/// Extracts the fence string from a fence line (e.g., "```", "~~~", "````")
fn extract_fence(line: &str) -> String {
    let trimmed = line.trim();
    let fence_char = if trimmed.starts_with('`') { '`' } else { '~' };
    trimmed.chars().take_while(|&c| c == fence_char).collect()
}

/// Extracts the language from a code fence line
pub fn extract_language(fence_line: &str) -> String {
    let trimmed = fence_line.trim();
    if let Some(lang) = trimmed.strip_prefix("```") {
        lang.trim().to_string()
    } else {
        String::new()
    }
}

/// Checks if a line is an md-code directive comment
pub fn is_md_code_comment(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with("<!--") && trimmed.contains("md-code:")
}

/// Checks if a line is an md-code-output directive comment
pub fn is_md_code_output_comment(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with("<!--") && trimmed.contains("md-code-output:")
}

/// Parses an md-code directive comment into a CodeBlockDirective
/// Format: <!-- md-code: id="foo"; bin="python3"; timeout=60 -->
pub fn parse_md_code_directive(line: &str) -> Result<CodeBlockDirective, CodeError> {
    let trimmed = line.trim();

    // Remove <!-- and -->
    let content = trimmed
        .strip_prefix("<!--")
        .ok_or_else(|| CodeError::DirectiveParseError("Missing opening <!--".to_string()))?
        .strip_suffix("-->")
        .ok_or_else(|| CodeError::DirectiveParseError("Missing closing -->".to_string()))?
        .trim();

    // Remove md-code: prefix
    let content = content
        .strip_prefix("md-code:")
        .ok_or_else(|| CodeError::DirectiveParseError("Missing md-code: prefix".to_string()))?
        .trim();

    let mut id = None;
    let mut bin = None;
    let mut timeout = None;
    let mut fence = None;
    let mut syntax = None;

    // Split by semicolons
    for part in content.split(';') {
        let part = part.trim();

        if part.starts_with("id=") {
            // Extract id value from quotes
            let value = part.strip_prefix("id=").unwrap().trim();
            id = Some(extract_quoted_value(value)?);
        } else if part.starts_with("bin=") {
            // Extract bin value from quotes
            let value = part.strip_prefix("bin=").unwrap().trim();
            bin = Some(extract_quoted_value(value)?);
        } else if part.starts_with("timeout=") {
            // Extract timeout value (no quotes)
            let value = part.strip_prefix("timeout=").unwrap().trim();
            timeout = Some(value.parse::<u64>().map_err(|_| {
                CodeError::DirectiveParseError(format!("Invalid timeout value: {}", value))
            })?);
        } else if part.starts_with("fence=") {
            // Extract fence value from quotes
            let value = part.strip_prefix("fence=").unwrap().trim();
            fence = Some(extract_quoted_value(value)?);
        } else if part.starts_with("syntax=") {
            // Extract syntax value from quotes
            let value = part.strip_prefix("syntax=").unwrap().trim();
            syntax = Some(extract_quoted_value(value)?);
        }
    }

    let id = id.ok_or_else(|| {
        CodeError::DirectiveParseError("Missing required id attribute".to_string())
    })?;

    // Validate ID format
    validate_id(&id).map_err(|e| CodeError::DirectiveParseError(format!("Invalid ID: {}", e)))?;

    // Validate fence if specified (must be ``` or ~~~ with optional repetitions)
    if let Some(ref f) = fence {
        if !f.chars().all(|c| c == '`' || c == '~') || f.len() < 3 {
            return Err(CodeError::DirectiveParseError(format!(
                "Invalid fence: '{}'. Must be at least 3 backticks (`) or tildes (~)",
                f
            )));
        }
    }

    Ok(CodeBlockDirective {
        id,
        bin,
        timeout,
        fence,
        syntax,
    })
}

/// Parses an md-code-output directive comment to extract the id
/// Format: <!-- md-code-output: id="foo" -->
pub fn parse_md_code_output_directive(line: &str) -> Result<String, CodeError> {
    let trimmed = line.trim();

    // Remove <!-- and -->
    let content = trimmed
        .strip_prefix("<!--")
        .ok_or_else(|| CodeError::DirectiveParseError("Missing opening <!--".to_string()))?
        .strip_suffix("-->")
        .ok_or_else(|| CodeError::DirectiveParseError("Missing closing -->".to_string()))?
        .trim();

    // Remove md-code-output: prefix
    let content = content
        .strip_prefix("md-code-output:")
        .ok_or_else(|| {
            CodeError::DirectiveParseError("Missing md-code-output: prefix".to_string())
        })?
        .trim();

    // Extract id value
    if content.starts_with("id=") {
        let value = content.strip_prefix("id=").unwrap().trim();
        let id = extract_quoted_value(value)?;

        // Validate ID format
        validate_id(&id)
            .map_err(|e| CodeError::DirectiveParseError(format!("Invalid ID: {}", e)))?;

        Ok(id)
    } else {
        Err(CodeError::DirectiveParseError(
            "Missing id attribute in md-code-output".to_string(),
        ))
    }
}

/// Extracts a quoted value from a string (removes surrounding quotes)
fn extract_quoted_value(s: &str) -> Result<String, CodeError> {
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        Ok(s[1..s.len() - 1].to_string())
    } else {
        Err(CodeError::DirectiveParseError(format!(
            "Value must be quoted: {}",
            s
        )))
    }
}

/// Parses the entire markdown document to find code blocks and output blocks
/// Skips code blocks that are nested inside other code fences
pub fn parse_document(
    text: &str,
) -> Result<(Vec<CodeBlock>, HashMap<String, OutputBlock>), CodeError> {
    let lines: Vec<&str> = text.lines().collect();
    let mut code_blocks = Vec::new();
    let mut output_blocks = HashMap::new();
    let mut output_block_lines = HashMap::new(); // Track line numbers for duplicate detection
    let mut i = 0;
    let mut active_fence_type: Option<char> = None; // Track fence type (` or ~)

    while i < lines.len() {
        if is_code_fence(lines[i]) {
            let fence_type = get_fence_type(lines[i]);

            // Only process as a real code block if we're not already inside another fence
            if active_fence_type.is_none() {
                active_fence_type = fence_type;
                let start_line = i;
                let language = extract_language(lines[i]);
                let fence = extract_fence(lines[start_line]);
                i += 1;

                // Collect code block content until we find a matching closing fence
                let mut content_lines = Vec::new();
                while i < lines.len() {
                    // Check if this line closes the fence (must be same type)
                    if is_code_fence(lines[i]) && get_fence_type(lines[i]) == active_fence_type {
                        break;
                    }
                    content_lines.push(lines[i]);
                    i += 1;
                }

                if i >= lines.len() {
                    return Err(CodeError::DirectiveParseError(format!(
                        "Unclosed code block starting at line {}",
                        start_line + 1
                    )));
                }

                let end_line = i;
                let content = content_lines.join("\n");
                active_fence_type = None;
                i += 1; // Move past closing fence

                // Check for md-code directive on the next line
                let directive = if i < lines.len() && is_md_code_comment(lines[i]) {
                    Some(parse_md_code_directive(lines[i])?)
                } else if i < lines.len() && is_md_code_output_comment(lines[i]) {
                    // This is an output block
                    let id = parse_md_code_output_directive(lines[i])?;

                    if let Some(&prev_line) = output_block_lines.get(&id) {
                        return Err(CodeError::duplicate_output_id(
                            &id,
                            start_line + 1,
                            prev_line + 1,
                        ));
                    }

                    output_block_lines.insert(id.clone(), start_line);
                    output_blocks.insert(
                        id.clone(),
                        OutputBlock {
                            start_line,
                            end_line,
                            id,
                            content: content.clone(),
                        },
                    );

                    None
                } else {
                    None
                };

                if directive.is_some() {
                    code_blocks.push(CodeBlock {
                        start_line,
                        end_line,
                        language,
                        content,
                        directive,
                        fence,
                    });
                }
            } else {
                // We're inside a fence, just skip this line
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    Ok((code_blocks, output_blocks))
}

/// Validates that all code block IDs are unique
pub fn validate_unique_ids(code_blocks: &[CodeBlock]) -> Result<(), CodeError> {
    let mut seen_ids = HashMap::new();

    for block in code_blocks {
        if let Some(ref directive) = block.directive {
            if let Some(&prev_line) = seen_ids.get(&directive.id) {
                return Err(CodeError::duplicate_id(
                    &directive.id,
                    block.start_line + 1,
                    prev_line + 1,
                ));
            }
            seen_ids.insert(directive.id.clone(), block.start_line);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_code_fence() {
        assert!(is_code_fence("```"));
        assert!(is_code_fence("```python"));
        assert!(is_code_fence("  ```rust  "));
        assert!(!is_code_fence("code"));
        assert!(!is_code_fence("<!--"));
    }

    #[test]
    fn test_extract_language() {
        assert_eq!(extract_language("```python"), "python");
        assert_eq!(extract_language("```rust"), "rust");
        assert_eq!(extract_language("```"), "");
        assert_eq!(extract_language("  ```  javascript  "), "javascript");
    }

    #[test]
    fn test_parse_md_code_directive() {
        let result = parse_md_code_directive(r#"<!-- md-code: id="test"; bin="python3" -->"#);
        assert!(result.is_ok());
        let directive = result.unwrap();
        assert_eq!(directive.id, "test");
        assert_eq!(directive.bin, Some("python3".to_string()));
        assert_eq!(directive.timeout, None);
    }

    #[test]
    fn test_parse_md_code_directive_with_timeout() {
        let result = parse_md_code_directive(r#"<!-- md-code: id="test"; timeout=60 -->"#);
        assert!(result.is_ok());
        let directive = result.unwrap();
        assert_eq!(directive.id, "test");
        assert_eq!(directive.timeout, Some(60));
    }

    #[test]
    fn test_parse_md_code_output_directive() {
        let result = parse_md_code_output_directive(r#"<!-- md-code-output: id="test" -->"#);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test");
    }

    #[test]
    fn test_extract_quoted_value() {
        assert_eq!(extract_quoted_value(r#""hello""#).unwrap(), "hello");
        assert_eq!(
            extract_quoted_value(r#""python3 -u""#).unwrap(),
            "python3 -u"
        );
        assert!(extract_quoted_value("hello").is_err());
    }
}
