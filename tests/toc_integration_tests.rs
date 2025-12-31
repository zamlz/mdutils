use mdutils::process_toc;
/// Integration tests for table of contents generation
/// Tests use fixture files in tests/toc/fixtures/ directory
use std::fs;

#[test]
fn test_basic_toc_generation() {
    let input = fs::read_to_string("tests/toc/fixtures/basic_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/toc/fixtures/basic_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_toc(&input);
    assert_eq!(output.trim(), expected.trim());
}

#[test]
fn test_nested_headers() {
    let input = fs::read_to_string("tests/toc/fixtures/nested_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/toc/fixtures/nested_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_toc(&input);
    assert_eq!(output.trim(), expected.trim());
}

#[test]
fn test_toc_update() {
    let input = fs::read_to_string("tests/toc/fixtures/update_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/toc/fixtures/update_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_toc(&input);
    assert_eq!(output.trim(), expected.trim());
}

#[test]
fn test_duplicate_headers() {
    let input = fs::read_to_string("tests/toc/fixtures/duplicates_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/toc/fixtures/duplicates_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_toc(&input);
    assert_eq!(output.trim(), expected.trim());
}

#[test]
fn test_special_characters() {
    let input = fs::read_to_string("tests/toc/fixtures/special_chars_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/toc/fixtures/special_chars_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_toc(&input);
    assert_eq!(output.trim(), expected.trim());
}

#[test]
fn test_no_toc_marker() {
    let input = fs::read_to_string("tests/toc/fixtures/no_marker_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/toc/fixtures/no_marker_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_toc(&input);
    assert_eq!(output.trim(), expected.trim());
}

#[test]
fn test_no_end_marker() {
    let input = fs::read_to_string("tests/toc/fixtures/no_end_marker_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/toc/fixtures/no_end_marker_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_toc(&input);
    assert_eq!(output.trim(), expected.trim());
}

#[test]
fn test_headers_before_marker() {
    let input = fs::read_to_string("tests/toc/fixtures/headers_before_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/toc/fixtures/headers_before_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_toc(&input);
    assert_eq!(output.trim(), expected.trim());
}

#[test]
fn test_empty_document() {
    let input = fs::read_to_string("tests/toc/fixtures/empty_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/toc/fixtures/empty_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_toc(&input);
    assert_eq!(output.trim(), expected.trim());
}

#[test]
fn test_complex_document() {
    let input = fs::read_to_string("tests/toc/fixtures/complex_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/toc/fixtures/complex_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_toc(&input);
    assert_eq!(output.trim(), expected.trim());
}

#[test]
fn test_mixed_header_levels() {
    let input = fs::read_to_string("tests/toc/fixtures/mixed_levels_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/toc/fixtures/mixed_levels_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_toc(&input);
    assert_eq!(output.trim(), expected.trim());
}

#[test]
fn test_only_h1_headers() {
    let input = fs::read_to_string("tests/toc/fixtures/only_h1_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/toc/fixtures/only_h1_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_toc(&input);
    assert_eq!(output.trim(), expected.trim());
}

#[test]
fn test_deep_nesting() {
    let input = fs::read_to_string("tests/toc/fixtures/deep_nesting_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/toc/fixtures/deep_nesting_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_toc(&input);
    assert_eq!(output.trim(), expected.trim());
}

#[test]
fn test_code_blocks() {
    let input = fs::read_to_string("tests/toc/fixtures/code_blocks_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/toc/fixtures/code_blocks_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_toc(&input);
    assert_eq!(output.trim(), expected.trim());

    // Verify that headers in code blocks are NOT in the TOC
    assert!(!output.contains("[Example Header]"));
    assert!(!output.contains("[Should not appear in TOC]"));
    assert!(!output.contains("[Neither should this]"));
}

#[test]
fn test_multiple_code_blocks() {
    let input = fs::read_to_string("tests/toc/fixtures/multiple_code_blocks_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/toc/fixtures/multiple_code_blocks_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_toc(&input);
    assert_eq!(output.trim(), expected.trim());

    // Verify only real sections are in TOC
    assert!(output.contains("[Section 1]"));
    assert!(output.contains("[Section 2]"));
    assert!(output.contains("[Section 3]"));
    assert!(output.contains("[Section 4]"));

    // Verify fake headers are NOT in TOC
    assert!(!output.contains("[Fake header 1]"));
    assert!(!output.contains("[Another fake header]"));
    assert!(!output.contains("[Not real]"));
    assert!(!output.contains("[Also fake]"));
}
