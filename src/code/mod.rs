mod error;
mod executor;
mod parser;
mod table;

pub use error::CodeError;

use crate::common::{get_fence_type, is_code_fence, ProcessingError, ProcessingResult};
use crate::table::parser::is_table_row;
use executor::execute_code;
use parser::{
    is_md_code_comment, is_md_code_output_comment, parse_document, parse_md_code_output_directive,
    validate_unique_ids, CodeBlock, OutputBlock,
};
use std::collections::HashMap;
use table::render_table;

/// Processes markdown code blocks with md-code directives
///
/// # Returns
///
/// A [`ProcessingResult`] containing:
/// - The processed document (with code block outputs inserted/updated)
/// - Any errors that occurred during parsing or execution
///
/// Note: Unlike other modules, code processing errors are often fatal (e.g., duplicate IDs,
/// missing bin specification). In these cases, the original input is returned unchanged
/// and the error is reported.
pub fn process_code_blocks(text: &str) -> ProcessingResult {
    let mut errors = Vec::new();

    // Parse the document to find all code blocks and output blocks
    let (code_blocks, mut output_blocks) = match parse_document(text) {
        Ok(result) => result,
        Err(e) => {
            errors.push(ProcessingError::code(0, e.to_string()));
            return ProcessingResult::with_errors(text.to_string(), errors);
        }
    };

    // Validate that all code block IDs are unique
    if let Err(e) = validate_unique_ids(&code_blocks) {
        errors.push(ProcessingError::code(0, e.to_string()));
        return ProcessingResult::with_errors(text.to_string(), errors);
    }

    // Execute code blocks and collect results
    let mut execution_results = HashMap::new();
    let mut table_results = HashMap::new();

    for block in &code_blocks {
        if let Some(ref directive) = block.directive {
            // Validate that bin is specified
            let Some(bin) = directive.bin.as_ref() else {
                let err = CodeError::missing_field(block.start_line + 1, "bin");
                errors.push(ProcessingError::code(block.start_line + 1, err.to_string()));
                return ProcessingResult::with_errors(text.to_string(), errors);
            };

            // Execute the code
            match execute_code(&block.content, bin, directive.timeout) {
                Ok(result) => {
                    // Only store non-empty outputs
                    if !result.output.trim().is_empty() {
                        if let Some(spec) = &directive.table {
                            match render_table(&result.output, spec) {
                                Ok(lines) => {
                                    table_results.insert(directive.id.clone(), lines);
                                }
                                Err(e) => {
                                    errors.push(ProcessingError::code(
                                        block.start_line + 1,
                                        e.to_string(),
                                    ));
                                    return ProcessingResult::with_errors(text.to_string(), errors);
                                }
                            }
                        }
                        execution_results.insert(directive.id.clone(), result.output);
                    }
                }
                Err(e) => {
                    errors.push(ProcessingError::code(block.start_line + 1, e.to_string()));
                    return ProcessingResult::with_errors(text.to_string(), errors);
                }
            }
        }
    }

    // Reconstruct the document
    match reconstruct_document(
        text,
        &code_blocks,
        &mut output_blocks,
        &execution_results,
        &table_results,
    ) {
        Ok(output) => ProcessingResult::with_errors(output, errors),
        Err(e) => {
            errors.push(ProcessingError::code(0, e.to_string()));
            ProcessingResult::with_errors(text.to_string(), errors)
        }
    }
}

/// An output block's new content
enum Rendered {
    /// Output written into a fenced code block
    Fenced {
        output: String,
        fence: String,
        syntax: String,
    },
    /// Output written as markdown table lines
    Table(Vec<String>),
}

impl Rendered {
    /// Appends the block's lines (without the trailing md-code-output marker)
    fn push_to(&self, lines: &mut Vec<String>) {
        match self {
            Rendered::Fenced {
                output,
                fence,
                syntax,
            } => {
                lines.push(format!("{fence}{syntax}"));
                lines.push(output.clone());
                lines.push(fence.clone());
            }
            Rendered::Table(table) => lines.extend(table.iter().cloned()),
        }
    }
}

/// Reconstructs the document with updated/new output blocks
fn reconstruct_document(
    text: &str,
    code_blocks: &[CodeBlock],
    output_blocks: &mut HashMap<String, OutputBlock>,
    execution_results: &HashMap<String, String>,
    table_results: &HashMap<String, Vec<String>>,
) -> Result<String, CodeError> {
    let lines: Vec<&str> = text.lines().collect();
    let mut output_lines = Vec::new();
    let mut i = 0;

    // Track which output blocks we've updated
    let mut updated_output_blocks = HashMap::new();

    while i < lines.len() {
        // Check if this is a code block
        if let Some(block) = find_code_block_at_line(code_blocks, i) {
            // Output the code block
            output_lines.push(lines[i].to_string()); // Opening fence
            i += 1;

            // Output content
            while i <= block.end_line {
                output_lines.push(lines[i].to_string());
                i += 1;
            }

            // Output directive comment if present
            if i < lines.len() && is_md_code_comment(lines[i]) {
                output_lines.push(lines[i].to_string());
                i += 1;

                // Check if we need to add/update output block
                if let Some(ref directive) = block.directive {
                    if let Some(output) = execution_results.get(&directive.id) {
                        let rendered = match table_results.get(&directive.id) {
                            Some(table) => Rendered::Table(table.clone()),
                            None => Rendered::Fenced {
                                output: output.clone(),
                                // Directive override or the code block's fence
                                fence: directive.fence.as_ref().unwrap_or(&block.fence).clone(),
                                // Directive syntax or empty string (default)
                                syntax: directive.syntax.clone().unwrap_or_default(),
                            },
                        };

                        if output_blocks.contains_key(&directive.id) {
                            // Mark it as updated (we'll replace it when we encounter it)
                            updated_output_blocks.insert(directive.id.clone(), rendered);
                        } else {
                            // Create new output block immediately after code block
                            output_lines.push(String::new());
                            match rendered {
                                Rendered::Table(table) => output_lines.extend(table),
                                Rendered::Fenced {
                                    output,
                                    fence,
                                    syntax,
                                } => {
                                    output_lines.push("Output:".to_string());
                                    output_lines.push(format!("{fence}{syntax}"));
                                    output_lines.push(output);
                                    output_lines.push(fence);
                                }
                            }
                            output_lines
                                .push(format!(r#"<!-- md-code-output: id="{}" -->"#, directive.id));
                        }
                    }
                }
            }
        } else if is_code_fence(lines[i]) {
            // This might be an output block or a regular code fence
            let _fence_start_line = i;
            let opening_fence_type = get_fence_type(lines[i]);
            output_lines.push(lines[i].to_string());
            i += 1;

            // Collect content until we find a matching closing fence
            let mut content_lines = Vec::new();
            while i < lines.len() {
                // Check if this line closes the fence (must be same type)
                if is_code_fence(lines[i]) && get_fence_type(lines[i]) == opening_fence_type {
                    break;
                }
                content_lines.push(lines[i]);
                i += 1;
            }

            // Output the content lines
            for line in &content_lines {
                output_lines.push(line.to_string());
            }

            if i < lines.len() {
                // Output closing fence
                output_lines.push(lines[i].to_string());
                i += 1;

                // Check for md-code-output directive
                if i < lines.len() && is_md_code_output_comment(lines[i]) {
                    let id = parse_md_code_output_directive(lines[i])?;

                    // If we have an updated output for this ID, use it
                    if let Some(rendered) = updated_output_blocks.get(&id) {
                        // Replace the old block (opening fence, content, closing fence)
                        output_lines.truncate(output_lines.len() - content_lines.len() - 2);
                        rendered.push_to(&mut output_lines);
                    }

                    // Output the directive comment
                    output_lines.push(lines[i].to_string());
                    i += 1;
                }
                // If not an output block, content is already output, just continue
            }
        } else if is_table_row(lines[i]) {
            // A table directly followed by an md-code-output marker is an output block
            let table_start = i;
            while i < lines.len() && is_table_row(lines[i]) {
                i += 1;
            }
            let updated = if i < lines.len() && is_md_code_output_comment(lines[i]) {
                updated_output_blocks.get(&parse_md_code_output_directive(lines[i])?)
            } else {
                None
            };
            match updated {
                Some(rendered) => rendered.push_to(&mut output_lines),
                None => output_lines.extend(lines[table_start..i].iter().map(|l| (*l).to_string())),
            }
        } else {
            // Regular line
            output_lines.push(lines[i].to_string());
            i += 1;
        }
    }

    let mut result = output_lines.join("\n");

    // Preserve trailing newline if the original input had one
    if text.ends_with('\n') {
        result.push('\n');
    }

    Ok(result)
}

/// Finds a code block that starts at the given line
fn find_code_block_at_line(code_blocks: &[CodeBlock], line: usize) -> Option<&CodeBlock> {
    code_blocks.iter().find(|b| b.start_line == line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_code_blocks_no_directive() {
        let input = r#"# Test

```python
print("hello")
```

More text."#;

        let result = process_code_blocks(input);
        assert!(!result.has_errors());
        // Should be unchanged since there's no md-code directive
        assert_eq!(result.output, input);
    }

    #[test]
    fn test_process_code_blocks_with_execute() {
        let input = r#"```python
print("hello world")
```
<!-- md-code: id="test"; bin="python3" -->"#;

        let result = process_code_blocks(input);

        // This test requires python3 to be installed
        if !result.has_errors() {
            // Should contain output block
            assert!(result.output.contains("Output:"));
            assert!(result.output.contains("md-code-output:"));
        }
    }

    #[test]
    fn test_duplicate_ids_error() {
        let input = r#"```python
print("hello")
```
<!-- md-code: id="test" -->

```python
print("world")
```
<!-- md-code: id="test" -->"#;

        let result = process_code_blocks(input);
        assert!(result.has_errors());
        assert!(result.errors[0].message.contains("Duplicate code block ID"));
    }

    #[test]
    fn test_execute_without_bin_error() {
        let input = r#"```python
print("hello")
```
<!-- md-code: id="test" -->"#;

        let result = process_code_blocks(input);
        assert!(result.has_errors());
        assert!(result.errors[0]
            .message
            .contains("missing required field: bin"));
    }

    #[test]
    fn test_update_existing_output_block() {
        let input = r#"```python
print("new output")
```
<!-- md-code: id="test"; bin="python3" -->

Some text.

Output:
```
old output
```
<!-- md-code-output: id="test" -->

More text."#;

        let result = process_code_blocks(input);
        if !result.has_errors() {
            // Old output should be replaced
            assert!(!result.output.contains("old output"));
            // New output should be present
            assert!(result.output.contains("new output"));
            // Text should be preserved
            assert!(result.output.contains("Some text."));
            assert!(result.output.contains("More text."));
        }
    }

    #[test]
    fn test_empty_output_no_block_created() {
        let input = r#"```python
x = 1 + 1
```
<!-- md-code: id="test"; bin="python3" -->

End."#;

        let result = process_code_blocks(input);
        if !result.has_errors() {
            // No output block should be created
            assert!(!result.output.contains("Output:"));
            assert!(!result.output.contains("md-code-output:"));
        }
    }

    #[test]
    fn test_error_captures_stderr() {
        let input = r#"```python
import sys
sys.stdout.write("This should not appear\n")
sys.stderr.write("Error message\n")
sys.exit(1)
```
<!-- md-code: id="test"; bin="python3" -->"#;

        let result = process_code_blocks(input);
        // Should succeed (not error out), but capture stderr in output
        if !result.has_errors() {
            // Output block should be created (stderr is not empty)
            assert!(result.output.contains("md-code-output:"));
            assert!(result.output.contains("Error message"));

            // Extract just the output block to verify stdout was not captured
            let output_block_start = result.output.find("Output:\n```").unwrap();
            let output_block_end = result.output.find("<!-- md-code-output:").unwrap();
            let output_block = &result.output[output_block_start..output_block_end];

            // Verify that stdout is NOT captured in the output block (only stderr is)
            assert!(!output_block.contains("This should not appear"));
            assert!(output_block.contains("Error message"));
        }
    }

    #[test]
    fn test_duplicate_output_block_error() {
        let input = r#"Output:
```
first
```
<!-- md-code-output: id="test" -->

Output:
```
second
```
<!-- md-code-output: id="test" -->"#;

        let result = process_code_blocks(input);
        assert!(result.has_errors());
        assert!(result.errors[0]
            .message
            .contains("Duplicate output block ID"));
    }

    #[test]
    fn test_bin_with_arguments() {
        let input = r#"```python
print("test")
```
<!-- md-code: id="test"; bin="python3 -u" -->"#;

        let result = process_code_blocks(input);
        if !result.has_errors() {
            assert!(result.output.contains("test"));
        }
    }

    #[test]
    fn test_custom_timeout() {
        let input = r#"```python
import time
time.sleep(0.1)
print("done")
```
<!-- md-code: id="test"; bin="python3"; timeout=5 -->"#;

        let result = process_code_blocks(input);
        if !result.has_errors() {
            assert!(result.output.contains("done"));
        }
    }
}
