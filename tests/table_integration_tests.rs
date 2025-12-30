/// Integration tests for table formatting and formula evaluation
/// Tests use fixture files in tests/table/fixtures/ directory
use std::fs;
use mdutils::format_tables;

#[test]
fn test_vector_addition() {
    let input = fs::read_to_string("tests/table/fixtures/vector_addition_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/vector_addition_expected.md")
        .expect("Failed to read expected fixture");

    let output = format_tables(&input);
    assert_eq!(output.trim(), expected.trim());
}

#[test]
fn test_vector_scalar_multiplication() {
    let input = fs::read_to_string("tests/table/fixtures/vector_scalar_mult_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/vector_scalar_mult_expected.md")
        .expect("Failed to read expected fixture");

    let output = format_tables(&input);
    assert_eq!(output.trim(), expected.trim());
}

#[test]
fn test_matrix_multiplication() {
    let input = fs::read_to_string("tests/table/fixtures/matrix_mult_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/matrix_mult_expected.md")
        .expect("Failed to read expected fixture");

    let output = format_tables(&input);
    assert_eq!(output.trim(), expected.trim());
}

#[test]
fn test_transpose_operator() {
    let input = fs::read_to_string("tests/table/fixtures/transpose_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/transpose_expected.md")
        .expect("Failed to read expected fixture");

    let output = format_tables(&input);
    assert_eq!(output.trim(), expected.trim());
}

#[test]
fn test_multiple_formulas() {
    let input = fs::read_to_string("tests/table/fixtures/multiple_formulas_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/multiple_formulas_expected.md")
        .expect("Failed to read expected fixture");

    let output = format_tables(&input);
    assert_eq!(output.trim(), expected.trim());
}

#[test]
fn test_sum_function() {
    let input = fs::read_to_string("tests/table/fixtures/sum_function_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/sum_function_expected.md")
        .expect("Failed to read expected fixture");

    let output = format_tables(&input);
    assert_eq!(output.trim(), expected.trim());
}

#[test]
fn test_complex_expression() {
    let input = fs::read_to_string("tests/table/fixtures/complex_expression_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/complex_expression_expected.md")
        .expect("Failed to read expected fixture");

    let output = format_tables(&input);
    assert_eq!(output.trim(), expected.trim());
}

#[test]
fn test_real_world_tax_calculation() {
    let input = fs::read_to_string("tests/table/fixtures/real_world_tax_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/real_world_tax_expected.md")
        .expect("Failed to read expected fixture");

    let output = format_tables(&input);
    assert_eq!(output.trim(), expected.trim());
}
