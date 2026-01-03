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
