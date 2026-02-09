use mdutils::done::process_done_with_timestamp;
/// Integration tests for done command
/// Tests use fixture files in tests/done/fixtures/ directory
use std::fs;

const TEST_TIMESTAMP: &str = "2024-01-15 14:30:00";

#[test]
fn test_basic_done() {
    let input = fs::read_to_string("tests/done/fixtures/basic_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/done/fixtures/basic_expected.md")
        .expect("Failed to read expected fixture");

    let result = process_done_with_timestamp(&input, TEST_TIMESTAMP);
    assert_eq!(result.output.trim(), expected.trim());

    // Idempotency check: command(expected) should equal expected
    let result2 = process_done_with_timestamp(&expected, TEST_TIMESTAMP);
    assert_eq!(
        result2.output.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_already_checked_items() {
    let input = fs::read_to_string("tests/done/fixtures/already_checked_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/done/fixtures/already_checked_expected.md")
        .expect("Failed to read expected fixture");

    let result = process_done_with_timestamp(&input, TEST_TIMESTAMP);
    assert_eq!(result.output.trim(), expected.trim());

    // Idempotency check
    let result2 = process_done_with_timestamp(&expected, TEST_TIMESTAMP);
    assert_eq!(
        result2.output.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_already_done_strikethrough() {
    let input = fs::read_to_string("tests/done/fixtures/already_done_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/done/fixtures/already_done_expected.md")
        .expect("Failed to read expected fixture");

    let result = process_done_with_timestamp(&input, TEST_TIMESTAMP);
    assert_eq!(result.output.trim(), expected.trim());

    // Idempotency check
    let result2 = process_done_with_timestamp(&expected, TEST_TIMESTAMP);
    assert_eq!(
        result2.output.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_nested_items() {
    let input = fs::read_to_string("tests/done/fixtures/nested_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/done/fixtures/nested_expected.md")
        .expect("Failed to read expected fixture");

    let result = process_done_with_timestamp(&input, TEST_TIMESTAMP);
    assert_eq!(result.output.trim(), expected.trim());

    // Idempotency check
    let result2 = process_done_with_timestamp(&expected, TEST_TIMESTAMP);
    assert_eq!(
        result2.output.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_mixed_content() {
    let input = fs::read_to_string("tests/done/fixtures/mixed_content_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/done/fixtures/mixed_content_expected.md")
        .expect("Failed to read expected fixture");

    let result = process_done_with_timestamp(&input, TEST_TIMESTAMP);
    assert_eq!(result.output.trim(), expected.trim());

    // Idempotency check
    let result2 = process_done_with_timestamp(&expected, TEST_TIMESTAMP);
    assert_eq!(
        result2.output.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_passthrough_non_checklist() {
    let input = fs::read_to_string("tests/done/fixtures/passthrough_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/done/fixtures/passthrough_expected.md")
        .expect("Failed to read expected fixture");

    let result = process_done_with_timestamp(&input, TEST_TIMESTAMP);
    assert_eq!(result.output.trim(), expected.trim());

    // Idempotency check
    let result2 = process_done_with_timestamp(&expected, TEST_TIMESTAMP);
    assert_eq!(
        result2.output.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}
