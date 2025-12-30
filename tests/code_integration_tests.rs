/// Integration tests for code execution
/// Tests use fixture files in tests/code/fixtures/ directory
use std::fs;
use mdutils::process_code_blocks;

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
