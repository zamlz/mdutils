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

    let result = process_code_blocks(&input);
    assert!(
        !result.has_errors(),
        "Processing failed: {:?}",
        result.errors
    );
    assert_eq!(result.output.trim(), expected.trim());
    // Verify output block was created
    assert!(result
        .output
        .contains("<!-- md-code-output: id=\"hello\" -->"));
    assert!(result.output.contains("Hello, World!"));

    // Idempotency check: command(expected) should equal expected
    let result2 = process_code_blocks(&expected);
    assert!(
        !result2.has_errors(),
        "Processing expected failed: {:?}",
        result2.errors
    );
    assert_eq!(
        result2.output.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_basic_bash_execution() {
    let input = fs::read_to_string("tests/code/fixtures/basic_bash_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/code/fixtures/basic_bash_expected.md")
        .expect("Failed to read expected fixture");

    let result = process_code_blocks(&input);
    assert!(
        !result.has_errors(),
        "Processing failed: {:?}",
        result.errors
    );
    assert_eq!(result.output.trim(), expected.trim());
    // Verify bash execution worked
    assert!(result.output.contains("Testing bash execution"));
    assert!(result.output.contains("Line 2"));

    // Idempotency check: command(expected) should equal expected
    let result2 = process_code_blocks(&expected);
    assert!(
        !result2.has_errors(),
        "Processing expected failed: {:?}",
        result2.errors
    );
    assert_eq!(
        result2.output.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_multiple_code_blocks() {
    let input = fs::read_to_string("tests/code/fixtures/multiple_blocks_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/code/fixtures/multiple_blocks_expected.md")
        .expect("Failed to read expected fixture");

    let result = process_code_blocks(&input);
    assert!(
        !result.has_errors(),
        "Processing failed: {:?}",
        result.errors
    );
    assert_eq!(result.output.trim(), expected.trim());
    // Verify all three blocks executed
    assert!(result
        .output
        .contains("<!-- md-code-output: id=\"first\" -->"));
    assert!(result
        .output
        .contains("<!-- md-code-output: id=\"second\" -->"));
    assert!(result
        .output
        .contains("<!-- md-code-output: id=\"third\" -->"));
    assert!(result.output.contains("First"));
    assert!(result.output.contains("Second"));
    assert!(result.output.contains("Third"));

    // Idempotency check: command(expected) should equal expected
    let result2 = process_code_blocks(&expected);
    assert!(
        !result2.has_errors(),
        "Processing expected failed: {:?}",
        result2.errors
    );
    assert_eq!(
        result2.output.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_no_execute_flag() {
    let input = fs::read_to_string("tests/code/fixtures/no_execute_flag_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/code/fixtures/no_execute_flag_expected.md")
        .expect("Failed to read expected fixture");

    let result = process_code_blocks(&input);
    assert!(
        !result.has_errors(),
        "Processing failed: {:?}",
        result.errors
    );
    assert_eq!(result.output.trim(), expected.trim());
    // Verify both blocks executed (both have md-code directives)
    assert!(result
        .output
        .contains("<!-- md-code-output: id=\"no_exec\" -->"));
    assert!(result
        .output
        .contains("<!-- md-code-output: id=\"yes_exec\" -->"));

    // Idempotency check: command(expected) should equal expected
    let result2 = process_code_blocks(&expected);
    assert!(
        !result2.has_errors(),
        "Processing expected failed: {:?}",
        result2.errors
    );
    assert_eq!(
        result2.output.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_update_existing_output() {
    let input = fs::read_to_string("tests/code/fixtures/update_output_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/code/fixtures/update_output_expected.md")
        .expect("Failed to read expected fixture");

    let result = process_code_blocks(&input);
    assert!(
        !result.has_errors(),
        "Processing failed: {:?}",
        result.errors
    );
    assert_eq!(result.output.trim(), expected.trim());
    // Verify old output was replaced with new
    assert!(!result.output.contains("Old output"));
    assert!(result.output.contains("New output"));

    // Idempotency check: command(expected) should equal expected
    let result2 = process_code_blocks(&expected);
    assert!(
        !result2.has_errors(),
        "Processing expected failed: {:?}",
        result2.errors
    );
    assert_eq!(
        result2.output.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_empty_output() {
    let input = fs::read_to_string("tests/code/fixtures/empty_output_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/code/fixtures/empty_output_expected.md")
        .expect("Failed to read expected fixture");

    let result = process_code_blocks(&input);
    assert!(
        !result.has_errors(),
        "Processing failed: {:?}",
        result.errors
    );
    assert_eq!(result.output.trim(), expected.trim());
    // Verify no output block was created for empty output
    assert!(!result
        .output
        .contains("<!-- md-code-output: id=\"no_output\" -->"));

    // Idempotency check: command(expected) should equal expected
    let result2 = process_code_blocks(&expected);
    assert!(
        !result2.has_errors(),
        "Processing expected failed: {:?}",
        result2.errors
    );
    assert_eq!(
        result2.output.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_preserve_content() {
    let input = fs::read_to_string("tests/code/fixtures/preserve_content_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/code/fixtures/preserve_content_expected.md")
        .expect("Failed to read expected fixture");

    let result = process_code_blocks(&input);
    assert!(
        !result.has_errors(),
        "Processing failed: {:?}",
        result.errors
    );
    assert_eq!(result.output.trim(), expected.trim());
    // Verify all content was preserved
    assert!(result.output.contains("Document Title"));
    assert!(result.output.contains("Some introductory text here"));
    assert!(result.output.contains("Text between blocks"));
    assert!(result.output.contains("| Table | Data |"));
    assert!(result.output.contains("End of document"));

    // Idempotency check: command(expected) should equal expected
    let result2 = process_code_blocks(&expected);
    assert!(
        !result2.has_errors(),
        "Processing expected failed: {:?}",
        result2.errors
    );
    assert_eq!(
        result2.output.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_stderr_capture() {
    let input = fs::read_to_string("tests/code/fixtures/stderr_capture_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/code/fixtures/stderr_capture_expected.md")
        .expect("Failed to read expected fixture");

    let result = process_code_blocks(&input);
    assert!(
        !result.has_errors(),
        "Processing failed: {:?}",
        result.errors
    );
    assert_eq!(result.output.trim(), expected.trim());
    // Verify stdout was captured
    assert!(result.output.contains("stdout output"));

    // Idempotency check: command(expected) should equal expected
    let result2 = process_code_blocks(&expected);
    assert!(
        !result2.has_errors(),
        "Processing expected failed: {:?}",
        result2.errors
    );
    assert_eq!(
        result2.output.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_skip_nested_code_blocks() {
    let input = fs::read_to_string("tests/code/fixtures/skip_nested_code_blocks_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/code/fixtures/skip_nested_code_blocks_expected.md")
        .expect("Failed to read expected fixture");

    let result = process_code_blocks(&input);
    assert!(
        !result.has_errors(),
        "Processing failed: {:?}",
        result.errors
    );
    assert_eq!(result.output.trim(), expected.trim());

    // Verify real code blocks executed
    assert!(result.output.contains("This should execute"));
    assert!(result.output.contains("Real output"));

    // Verify we have exactly 2 output blocks (for the 2 real code blocks)
    // NOT 3 - the "example" directive inside the markdown fence should be ignored
    assert_eq!(result.output.matches("<!-- md-code-output:").count(), 2);

    // Verify "Example code" appears in the document (as part of the markdown example)
    // but NOT as an output block (it was not executed)
    assert!(result.output.contains("Example code"));
    // Verify there's no output block for the "example" id
    assert!(!result
        .output
        .contains("<!-- md-code-output: id=\"example\" -->"));

    // Idempotency check: command(expected) should equal expected
    let result2 = process_code_blocks(&expected);
    assert!(
        !result2.has_errors(),
        "Processing expected failed: {:?}",
        result2.errors
    );
    assert_eq!(
        result2.output.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_meta_programming() {
    let input = fs::read_to_string("tests/code/fixtures/meta_programming_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/code/fixtures/meta_programming_expected.md")
        .expect("Failed to read expected fixture");

    let result = process_code_blocks(&input);
    assert!(
        !result.has_errors(),
        "Processing failed: {:?}",
        result.errors
    );
    assert_eq!(result.output.trim(), expected.trim());

    // Verify md table was executed and computed the formula
    assert!(result.output.contains("| 5   | 10  | 15  |"));
    assert!(result.output.contains("| 3   | 7   | 10  |"));

    // Verify md toc was executed and generated the TOC
    assert!(result.output.contains("- [Section One](#section-one)"));
    assert!(result.output.contains("  - [Subsection A](#subsection-a)"));
    assert!(result.output.contains("  - [Subsection B](#subsection-b)"));
    assert!(result.output.contains("- [Section Two](#section-two)"));

    // Verify both output blocks were created
    assert!(result
        .output
        .contains("<!-- md-code-output: id=\"table_demo\" -->"));
    assert!(result
        .output
        .contains("<!-- md-code-output: id=\"toc_demo\" -->"));

    // Idempotency check: command(expected) should equal expected
    let result2 = process_code_blocks(&expected);
    assert!(
        !result2.has_errors(),
        "Processing expected failed: {:?}",
        result2.errors
    );
    assert_eq!(
        result2.output.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_custom_fence() {
    let input = fs::read_to_string("tests/code/fixtures/custom_fence_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/code/fixtures/custom_fence_expected.md")
        .expect("Failed to read expected fixture");

    let result = process_code_blocks(&input);
    assert!(
        !result.has_errors(),
        "Processing failed: {:?}",
        result.errors
    );
    assert_eq!(result.output.trim(), expected.trim());

    // Verify default fence behavior (uses same fence as input)
    let default_section = result
        .output
        .split("## Test 1:")
        .nth(1)
        .unwrap()
        .split("## Test 2:")
        .next()
        .unwrap();
    assert!(default_section.contains("Output:\n```\n"));
    assert!(default_section.contains("\n```\n<!-- md-code-output: id=\"default_fence\""));

    // Verify custom tilde fence
    let tilde_section = result
        .output
        .split("## Test 2:")
        .nth(1)
        .unwrap()
        .split("## Test 3:")
        .next()
        .unwrap();
    assert!(tilde_section.contains("Output:\n~~~\n"));
    assert!(tilde_section.contains("\n~~~\n<!-- md-code-output: id=\"custom_tilde\""));

    // Verify custom four-backtick fence
    let four_backtick_section = result.output.split("## Test 3:").nth(1).unwrap();
    assert!(four_backtick_section.contains("Output:\n````\n"));
    assert!(
        four_backtick_section.contains("\n````\n<!-- md-code-output: id=\"custom_four_backticks\"")
    );

    // Idempotency check: command(expected) should equal expected
    let result2 = process_code_blocks(&expected);
    assert!(
        !result2.has_errors(),
        "Processing expected failed: {:?}",
        result2.errors
    );
    assert_eq!(
        result2.output.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_custom_syntax() {
    let input = fs::read_to_string("tests/code/fixtures/custom_syntax_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/code/fixtures/custom_syntax_expected.md")
        .expect("Failed to read expected fixture");

    let result = process_code_blocks(&input);
    assert!(
        !result.has_errors(),
        "Processing failed: {:?}",
        result.errors
    );
    assert_eq!(result.output.trim(), expected.trim());

    // Verify default syntax (no language specified)
    let default_section = result
        .output
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
    let json_section = result
        .output
        .split("## Test 2:")
        .nth(1)
        .unwrap()
        .split("## Test 3:")
        .next()
        .unwrap();
    assert!(json_section.contains("Output:\n```json\n"));

    // Verify text syntax
    let text_section = result
        .output
        .split("## Test 3:")
        .nth(1)
        .unwrap()
        .split("## Test 4:")
        .next()
        .unwrap();
    assert!(text_section.contains("Output:\n```text\n"));

    // Verify combined fence and syntax
    let combined_section = result.output.split("## Test 4:").nth(1).unwrap();
    assert!(combined_section.contains("Output:\n~~~python\n"));
    assert!(combined_section.contains("\n~~~\n<!-- md-code-output: id=\"combined\""));

    // Idempotency check: command(expected) should equal expected
    let result2 = process_code_blocks(&expected);
    assert!(
        !result2.has_errors(),
        "Processing expected failed: {:?}",
        result2.errors
    );
    assert_eq!(
        result2.output.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_table_output_tsv() {
    let input = fs::read_to_string("tests/code/fixtures/table_tsv_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/code/fixtures/table_tsv_expected.md")
        .expect("Failed to read expected fixture");

    let result = process_code_blocks(&input);
    assert!(
        !result.has_errors(),
        "Processing failed: {:?}",
        result.errors
    );
    assert_eq!(result.output.trim(), expected.trim());

    // Idempotency check: command(expected) should equal expected
    let result2 = process_code_blocks(&expected);
    assert!(!result2.has_errors());
    assert_eq!(result2.output.trim(), expected.trim());
}

#[test]
fn test_table_output_replaces_stale_table() {
    let input = r#"```bash
printf 'a,b\n1,2\n'
```
<!-- md-code: id="t"; bin="bash"; table="csv" -->

| old | data |
| --- | ---- |
| x   | y    |
<!-- md-code-output: id="t" -->

Trailing text.
"#;
    let result = process_code_blocks(input);
    assert!(!result.has_errors(), "{:?}", result.errors);
    assert!(result.output.contains("| 1   | 2   |"));
    assert!(!result.output.contains("old"));
    assert_eq!(result.output.matches("md-code-output").count(), 1);
    assert!(result.output.ends_with("Trailing text.\n"));
}

#[test]
fn test_table_output_converts_existing_fenced_output() {
    let input = r#"```bash
printf 'a\tb\n1\t2\n'
```
<!-- md-code: id="t"; bin="bash"; table="tsv" -->

Output:
```
stale
```
<!-- md-code-output: id="t" -->
"#;
    let result = process_code_blocks(input);
    assert!(!result.has_errors(), "{:?}", result.errors);
    assert!(result.output.contains("| 1   | 2   |"));
    assert!(!result.output.contains("stale"));
    assert_eq!(result.output.matches("md-code-output").count(), 1);
}

#[test]
fn test_table_output_converts_back_to_fenced() {
    let input = r#"```bash
echo hello
```
<!-- md-code: id="t"; bin="bash" -->

| old | data |
| --- | ---- |
<!-- md-code-output: id="t" -->
"#;
    let result = process_code_blocks(input);
    assert!(!result.has_errors(), "{:?}", result.errors);
    assert!(result.output.contains("hello"));
    assert!(!result.output.contains("old"));
    assert_eq!(result.output.matches("md-code-output").count(), 1);
}

#[test]
fn test_table_output_columns_and_delimiter() {
    let input = r#"```bash
printf '1::2\n3::4\n'
```
<!-- md-code: id="t"; bin="bash"; delimiter="::"; columns="A, B" -->
"#;
    let result = process_code_blocks(input);
    assert!(!result.has_errors(), "{:?}", result.errors);
    assert!(result.output.contains("| A   | B   |"));
    assert!(result.output.contains("| 1   | 2   |"));
    assert!(result.output.contains("| 3   | 4   |"));
}

#[test]
fn test_table_output_ragged_rows_error() {
    let input = r#"```bash
printf 'a\tb\n1\t2\t3\n'
```
<!-- md-code: id="t"; bin="bash"; table="tsv" -->
"#;
    let result = process_code_blocks(input);
    assert!(result.has_errors());
    assert!(result.errors[0].message.contains("3 columns, expected 2"));
    // Input is returned unchanged on error
    assert_eq!(result.output, input);
}

#[test]
fn test_table_directive_errors() {
    for directive in [
        r#"id="t"; bin="bash"; table="xml""#,
        r#"id="t"; bin="bash"; columns="A,B""#,
        r#"id="t"; bin="bash"; table="tsv"; header=maybe"#,
    ] {
        let input = format!("```bash\necho hi\n```\n<!-- md-code: {directive} -->\n");
        let result = process_code_blocks(&input);
        assert!(result.has_errors(), "expected error for: {directive}");
    }
}
