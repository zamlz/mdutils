mod error;
mod executor;
mod parser;

pub use error::CodeError;

use executor::execute_code;
use parser::{
    is_code_fence, is_md_code_comment, is_md_code_output_comment, parse_document,
    parse_md_code_output_directive, validate_unique_ids, CodeBlock, OutputBlock,
};
use std::collections::HashMap;

/// Processes markdown code blocks with md-code directives
pub fn process_code_blocks(text: &str) -> Result<String, CodeError> {
    // Parse the document to find all code blocks and output blocks
    let (code_blocks, mut output_blocks) = parse_document(text)?;

    // Validate that all code block IDs are unique
    validate_unique_ids(&code_blocks)?;

    // Execute code blocks and collect results
    let mut execution_results = HashMap::new();

    for block in &code_blocks {
        if let Some(ref directive) = block.directive {
            if directive.execute {
                // Validate that bin is specified
                let bin = directive
                    .bin
                    .as_ref()
                    .ok_or_else(|| CodeError::missing_field(block.start_line + 1, "bin"))?;

                // Execute the code
                let result = execute_code(&block.content, bin, directive.timeout)?;

                // Only store non-empty outputs
                if !result.output.trim().is_empty() {
                    execution_results.insert(directive.id.clone(), result.output);
                }
            }
        }
    }

    // Reconstruct the document
    reconstruct_document(text, &code_blocks, &mut output_blocks, &execution_results)
}

/// Reconstructs the document with updated/new output blocks
fn reconstruct_document(
    text: &str,
    code_blocks: &[CodeBlock],
    output_blocks: &mut HashMap<String, OutputBlock>,
    execution_results: &HashMap<String, String>,
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
                        // Determine which fence to use: directive override or code block's fence
                        let output_fence = directive.fence.as_ref().unwrap_or(&block.fence);

                        // Check if output block already exists
                        if output_blocks.contains_key(&directive.id) {
                            // Mark it as updated (we'll replace it when we encounter it)
                            updated_output_blocks.insert(
                                directive.id.clone(),
                                (output.clone(), output_fence.clone()),
                            );
                        } else {
                            // Create new output block immediately after code block
                            output_lines.push(String::new());
                            output_lines.push("Output:".to_string());
                            output_lines.push(output_fence.clone());
                            output_lines.push(output.clone());
                            output_lines.push(output_fence.clone());
                            output_lines
                                .push(format!(r#"<!-- md-code-output: id="{}" -->"#, directive.id));
                        }
                    }
                }
            }
        } else if is_code_fence(lines[i]) {
            // This might be an output block or a regular code fence
            output_lines.push(lines[i].to_string());
            i += 1;

            // Collect content
            let mut content_lines = Vec::new();
            while i < lines.len() && !is_code_fence(lines[i]) {
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
                    if let Some((new_output, new_fence)) = updated_output_blocks.get(&id) {
                        // Replace the content with new output
                        output_lines.pop(); // Remove closing fence we just added
                        for _ in content_lines.iter() {
                            output_lines.pop(); // Remove old content
                        }
                        output_lines.pop(); // Remove opening fence

                        // Add new output block with the correct fence
                        output_lines.push(new_fence.clone());
                        output_lines.push(new_output.clone());
                        output_lines.push(new_fence.clone());
                    }

                    // Output the directive comment
                    output_lines.push(lines[i].to_string());
                    i += 1;
                }
                // If not an output block, content is already output, just continue
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
    fn test_process_code_blocks_no_execute() {
        let input = r#"# Test

```python
print("hello")
```
<!-- md-code: id="test" -->

More text."#;

        let result = process_code_blocks(input);
        assert!(result.is_ok());
        let output = result.unwrap();
        // Should be unchanged since execute is not set
        assert_eq!(output, input);
    }

    #[test]
    fn test_process_code_blocks_with_execute() {
        let input = r#"```python
print("hello world")
```
<!-- md-code: id="test"; execute; bin="python3" -->"#;

        let result = process_code_blocks(input);

        // This test requires python3 to be installed
        if let Ok(output) = result {
            // Should contain output block
            assert!(output.contains("Output:"));
            assert!(output.contains("md-code-output:"));
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
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Duplicate code block ID"));
    }

    #[test]
    fn test_execute_without_bin_error() {
        let input = r#"```python
print("hello")
```
<!-- md-code: id="test"; execute -->"#;

        let result = process_code_blocks(input);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("missing required field: bin"));
    }

    #[test]
    fn test_update_existing_output_block() {
        let input = r#"```python
print("new output")
```
<!-- md-code: id="test"; execute; bin="python3" -->

Some text.

Output:
```
old output
```
<!-- md-code-output: id="test" -->

More text."#;

        let result = process_code_blocks(input);
        if let Ok(output) = result {
            // Old output should be replaced
            assert!(!output.contains("old output"));
            // New output should be present
            assert!(output.contains("new output"));
            // Text should be preserved
            assert!(output.contains("Some text."));
            assert!(output.contains("More text."));
        }
    }

    #[test]
    fn test_empty_output_no_block_created() {
        let input = r#"```python
x = 1 + 1
```
<!-- md-code: id="test"; execute; bin="python3" -->

End."#;

        let result = process_code_blocks(input);
        if let Ok(output) = result {
            // No output block should be created
            assert!(!output.contains("Output:"));
            assert!(!output.contains("md-code-output:"));
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
<!-- md-code: id="test"; execute; bin="python3" -->"#;

        let result = process_code_blocks(input);
        // Should succeed (not error out), but capture stderr in output
        if let Ok(output) = result {
            // Output block should be created (stderr is not empty)
            assert!(output.contains("md-code-output:"));
            assert!(output.contains("Error message"));

            // Extract just the output block to verify stdout was not captured
            let output_block_start = output.find("Output:\n```").unwrap();
            let output_block_end = output.find("<!-- md-code-output:").unwrap();
            let output_block = &output[output_block_start..output_block_end];

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
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Duplicate output block ID"));
    }

    #[test]
    fn test_bin_with_arguments() {
        let input = r#"```python
print("test")
```
<!-- md-code: id="test"; execute; bin="python3 -u" -->"#;

        let result = process_code_blocks(input);
        if let Ok(output) = result {
            assert!(output.contains("test"));
        }
    }

    #[test]
    fn test_custom_timeout() {
        let input = r#"```python
import time
time.sleep(0.1)
print("done")
```
<!-- md-code: id="test"; execute; bin="python3"; timeout=5 -->"#;

        let result = process_code_blocks(input);
        if let Ok(output) = result {
            assert!(output.contains("done"));
        }
    }
}
