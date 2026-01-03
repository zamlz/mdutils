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

    // Idempotency check: command(expected) should equal expected
    let output2 = process_toc(&expected);
    assert_eq!(
        output2.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_nested_headers() {
    let input = fs::read_to_string("tests/toc/fixtures/nested_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/toc/fixtures/nested_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_toc(&input);
    assert_eq!(output.trim(), expected.trim());

    // Idempotency check: command(expected) should equal expected
    let output2 = process_toc(&expected);
    assert_eq!(
        output2.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_toc_update() {
    let input = fs::read_to_string("tests/toc/fixtures/update_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/toc/fixtures/update_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_toc(&input);
    assert_eq!(output.trim(), expected.trim());

    // Idempotency check: command(expected) should equal expected
    let output2 = process_toc(&expected);
    assert_eq!(
        output2.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_duplicate_headers() {
    let input = fs::read_to_string("tests/toc/fixtures/duplicates_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/toc/fixtures/duplicates_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_toc(&input);
    assert_eq!(output.trim(), expected.trim());

    // Idempotency check: command(expected) should equal expected
    let output2 = process_toc(&expected);
    assert_eq!(
        output2.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_special_characters() {
    let input = fs::read_to_string("tests/toc/fixtures/special_chars_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/toc/fixtures/special_chars_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_toc(&input);
    assert_eq!(output.trim(), expected.trim());

    // Idempotency check: command(expected) should equal expected
    let output2 = process_toc(&expected);
    assert_eq!(
        output2.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_no_toc_marker() {
    let input = fs::read_to_string("tests/toc/fixtures/no_marker_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/toc/fixtures/no_marker_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_toc(&input);
    assert_eq!(output.trim(), expected.trim());

    // Idempotency check: command(expected) should equal expected
    let output2 = process_toc(&expected);
    assert_eq!(
        output2.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_no_end_marker() {
    let input = fs::read_to_string("tests/toc/fixtures/no_end_marker_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/toc/fixtures/no_end_marker_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_toc(&input);
    assert_eq!(output.trim(), expected.trim());

    // Idempotency check: command(expected) should equal expected
    let output2 = process_toc(&expected);
    assert_eq!(
        output2.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_headers_before_marker() {
    let input = fs::read_to_string("tests/toc/fixtures/headers_before_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/toc/fixtures/headers_before_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_toc(&input);
    assert_eq!(output.trim(), expected.trim());

    // Idempotency check: command(expected) should equal expected
    let output2 = process_toc(&expected);
    assert_eq!(
        output2.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_empty_document() {
    let input = fs::read_to_string("tests/toc/fixtures/empty_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/toc/fixtures/empty_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_toc(&input);
    assert_eq!(output.trim(), expected.trim());

    // Idempotency check: command(expected) should equal expected
    let output2 = process_toc(&expected);
    assert_eq!(
        output2.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_complex_document() {
    let input = fs::read_to_string("tests/toc/fixtures/complex_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/toc/fixtures/complex_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_toc(&input);
    assert_eq!(output.trim(), expected.trim());

    // Idempotency check: command(expected) should equal expected
    let output2 = process_toc(&expected);
    assert_eq!(
        output2.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_mixed_header_levels() {
    let input = fs::read_to_string("tests/toc/fixtures/mixed_levels_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/toc/fixtures/mixed_levels_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_toc(&input);
    assert_eq!(output.trim(), expected.trim());

    // Idempotency check: command(expected) should equal expected
    let output2 = process_toc(&expected);
    assert_eq!(
        output2.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_only_h1_headers() {
    let input = fs::read_to_string("tests/toc/fixtures/only_h1_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/toc/fixtures/only_h1_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_toc(&input);
    assert_eq!(output.trim(), expected.trim());

    // Idempotency check: command(expected) should equal expected
    let output2 = process_toc(&expected);
    assert_eq!(
        output2.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_deep_nesting() {
    let input = fs::read_to_string("tests/toc/fixtures/deep_nesting_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/toc/fixtures/deep_nesting_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_toc(&input);
    assert_eq!(output.trim(), expected.trim());

    // Idempotency check: command(expected) should equal expected
    let output2 = process_toc(&expected);
    assert_eq!(
        output2.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
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

#[test]
fn test_skip_code_blocks() {
    let input = fs::read_to_string("tests/toc/fixtures/skip_code_blocks_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/toc/fixtures/skip_code_blocks_expected.md")
        .expect("Failed to read expected fixture");

    let output = process_toc(&input);
    assert_eq!(output.trim(), expected.trim());

    // Verify only real headers are in TOC
    assert!(output.contains("[Real Section 1]"));
    assert!(output.contains("[Real Section 2]"));
    assert!(output.contains("[Real Subsection]"));

    // Verify fake headers in code blocks are NOT in TOC
    assert!(!output.contains("[Fake Header in Code Block]"));
    assert!(!output.contains("[Another Fake Header]"));
    assert!(!output.contains("[Nested Fake Header]"));
    assert!(!output.contains("[This should also be ignored]"));
    assert!(!output.contains("[This TOC directive should NOT be processed]"));

    // Verify the md-toc directive inside code blocks was NOT processed
    // (it should still appear as plain text, not generate a TOC)
    // Count total "<!-- md-toc:" occurrences: 1 real start + 1 real end + 2 fake ones in code blocks = 4
    assert_eq!(output.matches("<!-- md-toc:").count(), 4);

    // Verify only ONE TOC was actually generated (the real one at top)
    // by checking there's only one end marker
    assert_eq!(output.matches("<!-- md-toc: end -->").count(), 1);

    // Verify the fake directives appear as plain text inside code blocks
    // by checking the context - they should be surrounded by markdown fence markers
    assert!(output.contains("```markdown\n# Fake Header in Code Block"));
    assert!(
        output.contains("<!-- md-toc: -->\n\n## This TOC directive should NOT be processed\n```")
    );
}
