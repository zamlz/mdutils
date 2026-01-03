use mdutils::process_code_blocks;
/// Integration tests for code execution
/// Tests use fixture files in tests/code/fixtures/ directory
use std::fs;

#[test]
fn test_basic_python_execution() {
    let input = fs::read_to_string("tests/code/fixtures/basic_python_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/code/fixtures/basic_python_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_code_blocks(&input).expect("Failed to process code blocks");
    assert_eq!(output.trim(), expected.trim());
    // Verify output block was created
    assert!(output.contains("<!-- md-code-output: id=\"hello\" -->"));
    assert!(output.contains("Hello, World!"));
}

#[test]
fn test_basic_bash_execution() {
    let input = fs::read_to_string("tests/code/fixtures/basic_bash_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/code/fixtures/basic_bash_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_code_blocks(&input).expect("Failed to process code blocks");
    assert_eq!(output.trim(), expected.trim());
    // Verify bash execution worked
    assert!(output.contains("Testing bash execution"));
    assert!(output.contains("Line 2"));
}

#[test]
fn test_multiple_code_blocks() {
    let input = fs::read_to_string("tests/code/fixtures/multiple_blocks_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/code/fixtures/multiple_blocks_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_code_blocks(&input).expect("Failed to process code blocks");
    assert_eq!(output.trim(), expected.trim());
    // Verify all three blocks executed
    assert!(output.contains("<!-- md-code-output: id=\"first\" -->"));
    assert!(output.contains("<!-- md-code-output: id=\"second\" -->"));
    assert!(output.contains("<!-- md-code-output: id=\"third\" -->"));
    assert!(output.contains("First"));
    assert!(output.contains("Second"));
    assert!(output.contains("Third"));
}

#[test]
fn test_no_execute_flag() {
    let input = fs::read_to_string("tests/code/fixtures/no_execute_flag_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/code/fixtures/no_execute_flag_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_code_blocks(&input).expect("Failed to process code blocks");
    assert_eq!(output.trim(), expected.trim());
    // Verify only the block with execute flag ran (has output block)
    assert!(!output.contains("<!-- md-code-output: id=\"no_exec\" -->"));
    assert!(output.contains("<!-- md-code-output: id=\"yes_exec\" -->"));
}

#[test]
fn test_update_existing_output() {
    let input = fs::read_to_string("tests/code/fixtures/update_output_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/code/fixtures/update_output_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_code_blocks(&input).expect("Failed to process code blocks");
    assert_eq!(output.trim(), expected.trim());
    // Verify old output was replaced with new
    assert!(!output.contains("Old output"));
    assert!(output.contains("New output"));
}

#[test]
fn test_empty_output() {
    let input = fs::read_to_string("tests/code/fixtures/empty_output_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/code/fixtures/empty_output_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_code_blocks(&input).expect("Failed to process code blocks");
    assert_eq!(output.trim(), expected.trim());
    // Verify no output block was created for empty output
    assert!(!output.contains("<!-- md-code-output: id=\"no_output\" -->"));
}

#[test]
fn test_preserve_content() {
    let input = fs::read_to_string("tests/code/fixtures/preserve_content_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/code/fixtures/preserve_content_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_code_blocks(&input).expect("Failed to process code blocks");
    assert_eq!(output.trim(), expected.trim());
    // Verify all content was preserved
    assert!(output.contains("Document Title"));
    assert!(output.contains("Some introductory text here"));
    assert!(output.contains("Text between blocks"));
    assert!(output.contains("| Table | Data |"));
    assert!(output.contains("End of document"));
}

#[test]
fn test_stderr_capture() {
    let input = fs::read_to_string("tests/code/fixtures/stderr_capture_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/code/fixtures/stderr_capture_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_code_blocks(&input).expect("Failed to process code blocks");
    assert_eq!(output.trim(), expected.trim());
    // Verify stdout was captured
    assert!(output.contains("stdout output"));
}

#[test]
fn test_skip_nested_code_blocks() {
    let input = fs::read_to_string("tests/code/fixtures/skip_nested_code_blocks_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/code/fixtures/skip_nested_code_blocks_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_code_blocks(&input).expect("Failed to process code blocks");
    assert_eq!(output.trim(), expected.trim());

    // Verify real code blocks executed
    assert!(output.contains("This should execute"));
    assert!(output.contains("Real output"));

    // Verify we have exactly 2 output blocks (for the 2 real code blocks)
    // NOT 3 - the "example" directive inside the markdown fence should be ignored
    assert_eq!(output.matches("<!-- md-code-output:").count(), 2);

    // Verify "Example code" appears in the document (as part of the markdown example)
    // but NOT as an output block (it was not executed)
    assert!(output.contains("Example code"));
    // Verify there's no output block for the "example" id
    assert!(!output.contains("<!-- md-code-output: id=\"example\" -->"));
}

#[test]
fn test_meta_programming() {
    let input = fs::read_to_string("tests/code/fixtures/meta_programming_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/code/fixtures/meta_programming_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_code_blocks(&input).expect("Failed to process code blocks");
    assert_eq!(output.trim(), expected.trim());

    // Verify md table was executed and computed the formula
    assert!(output.contains("| 5   | 10  | 15  |"));
    assert!(output.contains("| 3   | 7   | 10  |"));

    // Verify md toc was executed and generated the TOC
    assert!(output.contains("- [Section One](#section-one)"));
    assert!(output.contains("  - [Subsection A](#subsection-a)"));
    assert!(output.contains("  - [Subsection B](#subsection-b)"));
    assert!(output.contains("- [Section Two](#section-two)"));

    // Verify both output blocks were created
    assert!(output.contains("<!-- md-code-output: id=\"table_demo\" -->"));
    assert!(output.contains("<!-- md-code-output: id=\"toc_demo\" -->"));
}

#[test]
fn test_custom_fence() {
    let input = fs::read_to_string("tests/code/fixtures/custom_fence_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/code/fixtures/custom_fence_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_code_blocks(&input).expect("Failed to process code blocks");
    assert_eq!(output.trim(), expected.trim());

    // Verify default fence behavior (uses same fence as input)
    let default_section = output
        .split("## Test 1:")
        .nth(1)
        .unwrap()
        .split("## Test 2:")
        .next()
        .unwrap();
    assert!(default_section.contains("Output:\n```\n"));
    assert!(default_section.contains("\n```\n<!-- md-code-output: id=\"default_fence\""));

    // Verify custom tilde fence
    let tilde_section = output
        .split("## Test 2:")
        .nth(1)
        .unwrap()
        .split("## Test 3:")
        .next()
        .unwrap();
    assert!(tilde_section.contains("Output:\n~~~\n"));
    assert!(tilde_section.contains("\n~~~\n<!-- md-code-output: id=\"custom_tilde\""));

    // Verify custom four-backtick fence
    let four_backtick_section = output.split("## Test 3:").nth(1).unwrap();
    assert!(four_backtick_section.contains("Output:\n````\n"));
    assert!(
        four_backtick_section.contains("\n````\n<!-- md-code-output: id=\"custom_four_backticks\"")
    );
}

#[test]
fn test_custom_syntax() {
    let input = fs::read_to_string("tests/code/fixtures/custom_syntax_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/code/fixtures/custom_syntax_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_code_blocks(&input).expect("Failed to process code blocks");
    assert_eq!(output.trim(), expected.trim());

    // Verify default syntax (no language specified)
    let default_section = output
        .split("## Test 1:")
        .nth(1)
        .unwrap()
        .split("## Test 2:")
        .next()
        .unwrap();
    assert!(default_section.contains("Output:\n```\n"));
    assert!(!default_section.contains("```json"));
    assert!(!default_section.contains("```text"));

    // Verify JSON syntax
    let json_section = output
        .split("## Test 2:")
        .nth(1)
        .unwrap()
        .split("## Test 3:")
        .next()
        .unwrap();
    assert!(json_section.contains("Output:\n```json\n"));

    // Verify text syntax
    let text_section = output
        .split("## Test 3:")
        .nth(1)
        .unwrap()
        .split("## Test 4:")
        .next()
        .unwrap();
    assert!(text_section.contains("Output:\n```text\n"));

    // Verify combined fence and syntax
    let combined_section = output.split("## Test 4:").nth(1).unwrap();
    assert!(combined_section.contains("Output:\n~~~python\n"));
    assert!(combined_section.contains("\n~~~\n<!-- md-code-output: id=\"combined\""));
}
