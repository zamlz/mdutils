use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CodeBlockDirective {
    pub id: String,
    pub execute: bool,
    pub bin: Option<String>,
    pub timeout: Option<u64>,
}

#[derive(Debug)]
pub struct CodeBlock {
    pub start_line: usize,
    pub end_line: usize,
    pub language: String,
    pub content: String,
    pub directive: Option<CodeBlockDirective>,
}

#[derive(Debug)]
pub struct OutputBlock {
    pub start_line: usize,
    pub end_line: usize,
    pub id: String,
    pub content: String,
}

/// Checks if a line is the start of a code block (opening fence)
pub fn is_code_fence(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with("```")
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
/// Format: <!-- md-code: id="foo"; execute; bin="python3"; timeout=60 -->
pub fn parse_md_code_directive(line: &str) -> Result<CodeBlockDirective, String> {
    let trimmed = line.trim();

    // Remove <!-- and -->
    let content = trimmed
        .strip_prefix("<!--")
        .ok_or("Missing opening <!--")?
        .strip_suffix("-->")
        .ok_or("Missing closing -->")?
        .trim();

    // Remove md-code: prefix
    let content = content
        .strip_prefix("md-code:")
        .ok_or("Missing md-code: prefix")?
        .trim();

    let mut id = None;
    let mut execute = false;
    let mut bin = None;
    let mut timeout = None;

    // Split by semicolons
    for part in content.split(';') {
        let part = part.trim();

        if part == "execute" {
            execute = true;
        } else if part.starts_with("id=") {
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
            timeout = Some(value.parse::<u64>()
                .map_err(|_| format!("Invalid timeout value: {}", value))?);
        }
    }

    let id = id.ok_or("Missing required id attribute")?;

    Ok(CodeBlockDirective {
        id,
        execute,
        bin,
        timeout,
    })
}

/// Parses an md-code-output directive comment to extract the id
/// Format: <!-- md-code-output: id="foo" -->
pub fn parse_md_code_output_directive(line: &str) -> Result<String, String> {
    let trimmed = line.trim();

    // Remove <!-- and -->
    let content = trimmed
        .strip_prefix("<!--")
        .ok_or("Missing opening <!--")?
        .strip_suffix("-->")
        .ok_or("Missing closing -->")?
        .trim();

    // Remove md-code-output: prefix
    let content = content
        .strip_prefix("md-code-output:")
        .ok_or("Missing md-code-output: prefix")?
        .trim();

    // Extract id value
    if content.starts_with("id=") {
        let value = content.strip_prefix("id=").unwrap().trim();
        extract_quoted_value(value)
    } else {
        Err("Missing id attribute in md-code-output".to_string())
    }
}

/// Extracts a quoted value from a string (removes surrounding quotes)
fn extract_quoted_value(s: &str) -> Result<String, String> {
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        Ok(s[1..s.len()-1].to_string())
    } else {
        Err(format!("Value must be quoted: {}", s))
    }
}

/// Parses the entire markdown document to find code blocks and output blocks
pub fn parse_document(text: &str) -> Result<(Vec<CodeBlock>, HashMap<String, OutputBlock>), String> {
    let lines: Vec<&str> = text.lines().collect();
    let mut code_blocks = Vec::new();
    let mut output_blocks = HashMap::new();
    let mut i = 0;

    while i < lines.len() {
        if is_code_fence(lines[i]) {
            let start_line = i;
            let language = extract_language(lines[i]);
            i += 1;

            // Collect code block content
            let mut content_lines = Vec::new();
            while i < lines.len() && !is_code_fence(lines[i]) {
                content_lines.push(lines[i]);
                i += 1;
            }

            if i >= lines.len() {
                return Err(format!("Unclosed code block starting at line {}", start_line + 1));
            }

            let end_line = i;
            let content = content_lines.join("\n");
            i += 1; // Move past closing fence

            // Check for md-code directive on the next line
            let directive = if i < lines.len() && is_md_code_comment(lines[i]) {
                Some(parse_md_code_directive(lines[i])?)
            } else if i < lines.len() && is_md_code_output_comment(lines[i]) {
                // This is an output block
                let id = parse_md_code_output_directive(lines[i])?;

                if output_blocks.contains_key(&id) {
                    return Err(format!("Duplicate output block id: {}", id));
                }

                output_blocks.insert(id.clone(), OutputBlock {
                    start_line,
                    end_line,
                    id,
                    content: content.clone(),
                });

                None
            } else {
                None
            };

            if let Some(ref dir) = directive {
                code_blocks.push(CodeBlock {
                    start_line,
                    end_line,
                    language,
                    content,
                    directive,
                });
            }
        } else {
            i += 1;
        }
    }

    Ok((code_blocks, output_blocks))
}

/// Validates that all code block IDs are unique
pub fn validate_unique_ids(code_blocks: &[CodeBlock]) -> Result<(), String> {
    let mut seen_ids = HashMap::new();

    for block in code_blocks {
        if let Some(ref directive) = block.directive {
            if let Some(&prev_line) = seen_ids.get(&directive.id) {
                return Err(format!(
                    "Duplicate code block id '{}' found at lines {} and {}",
                    directive.id,
                    prev_line + 1,
                    block.start_line + 1
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
        let result = parse_md_code_directive(r#"<!-- md-code: id="test"; execute; bin="python3" -->"#);
        assert!(result.is_ok());
        let directive = result.unwrap();
        assert_eq!(directive.id, "test");
        assert!(directive.execute);
        assert_eq!(directive.bin, Some("python3".to_string()));
        assert_eq!(directive.timeout, None);
    }

    #[test]
    fn test_parse_md_code_directive_with_timeout() {
        let result = parse_md_code_directive(r#"<!-- md-code: id="test"; timeout=60 -->"#);
        assert!(result.is_ok());
        let directive = result.unwrap();
        assert_eq!(directive.id, "test");
        assert!(!directive.execute);
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
        assert_eq!(extract_quoted_value(r#""python3 -u""#).unwrap(), "python3 -u");
        assert!(extract_quoted_value("hello").is_err());
    }
}
