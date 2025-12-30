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

#[test]
fn test_formula_parse_error() {
    let input = fs::read_to_string("tests/table/fixtures/formula_parse_error_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/formula_parse_error_expected.md")
        .expect("Failed to read expected fixture");

    let output = format_tables(&input);
    assert_eq!(output.trim(), expected.trim());
    // Verify error comment is present
    assert!(output.contains("md-error:"));
    assert!(output.contains("Failed to parse formula"));
}

#[test]
fn test_formula_eval_error() {
    let input = fs::read_to_string("tests/table/fixtures/formula_eval_error_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/formula_eval_error_expected.md")
        .expect("Failed to read expected fixture");

    let output = format_tables(&input);
    assert_eq!(output.trim(), expected.trim());
    // Verify error comment is present
    assert!(output.contains("md-error:"));
    // First formula should succeed
    assert!(output.contains("| 1   | 2   | 3   |"));
}

#[test]
fn test_error_cell_out_of_bounds() {
    let input = fs::read_to_string("tests/table/fixtures/error_cell_out_of_bounds_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/error_cell_out_of_bounds_expected.md")
        .expect("Failed to read expected fixture");

    let output = format_tables(&input);
    assert_eq!(output.trim(), expected.trim());
    assert!(output.contains("md-error:"));
    assert!(output.contains("out of bounds"));
}

#[test]
fn test_error_column_out_of_bounds() {
    let input = fs::read_to_string("tests/table/fixtures/error_column_out_of_bounds_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/error_column_out_of_bounds_expected.md")
        .expect("Failed to read expected fixture");

    let output = format_tables(&input);
    assert_eq!(output.trim(), expected.trim());
    assert!(output.contains("md-error:"));
    assert!(output.contains("column index out of bounds"));
}

#[test]
fn test_error_row_out_of_bounds() {
    let input = fs::read_to_string("tests/table/fixtures/error_row_out_of_bounds_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/error_row_out_of_bounds_expected.md")
        .expect("Failed to read expected fixture");

    let output = format_tables(&input);
    assert_eq!(output.trim(), expected.trim());
    assert!(output.contains("md-error:"));
    assert!(output.contains("row vector _5 is out of bounds"));
}

#[test]
fn test_error_matrix_mult_dimension() {
    let input = fs::read_to_string("tests/table/fixtures/error_matrix_mult_dimension_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/error_matrix_mult_dimension_expected.md")
        .expect("Failed to read expected fixture");

    let output = format_tables(&input);
    assert_eq!(output.trim(), expected.trim());
    assert!(output.contains("md-error:"));
    assert!(output.contains("matrix multiplication dimension mismatch"));
    assert!(output.contains("inner dimensions"));
}

#[test]
fn test_error_transpose_scalar() {
    let input = fs::read_to_string("tests/table/fixtures/error_transpose_scalar_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/error_transpose_scalar_expected.md")
        .expect("Failed to read expected fixture");

    let output = format_tables(&input);
    assert_eq!(output.trim(), expected.trim());
    assert!(output.contains("md-error:"));
    assert!(output.contains("cannot transpose a scalar"));
}

#[test]
fn test_error_elementwise_dimension() {
    let input = fs::read_to_string("tests/table/fixtures/error_elementwise_dimension_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/error_elementwise_dimension_expected.md")
        .expect("Failed to read expected fixture");

    let output = format_tables(&input);
    assert_eq!(output.trim(), expected.trim());
    assert!(output.contains("md-error:"));
    assert!(output.contains("element-wise operation"));
    assert!(output.contains("matching dimensions"));
}

#[test]
fn test_error_unmatched_paren() {
    let input = fs::read_to_string("tests/table/fixtures/error_unmatched_paren_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/error_unmatched_paren_expected.md")
        .expect("Failed to read expected fixture");

    let output = format_tables(&input);
    assert_eq!(output.trim(), expected.trim());
    assert!(output.contains("md-error:"));
    assert!(output.contains("unmatched"));
}

#[test]
fn test_error_invalid_token() {
    let input = fs::read_to_string("tests/table/fixtures/error_invalid_token_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/error_invalid_token_expected.md")
        .expect("Failed to read expected fixture");

    let output = format_tables(&input);
    assert_eq!(output.trim(), expected.trim());
    assert!(output.contains("md-error:"));
    assert!(output.contains("invalid token"));
}

#[test]
fn test_error_unknown_function() {
    let input = fs::read_to_string("tests/table/fixtures/error_unknown_function_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/error_unknown_function_expected.md")
        .expect("Failed to read expected fixture");

    let output = format_tables(&input);
    assert_eq!(output.trim(), expected.trim());
    assert!(output.contains("md-error:"));
    assert!(output.contains("unknown function"));
}

#[test]
fn test_error_division_by_zero() {
    let input = fs::read_to_string("tests/table/fixtures/error_division_by_zero_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/error_division_by_zero_expected.md")
        .expect("Failed to read expected fixture");

    let output = format_tables(&input);
    assert_eq!(output.trim(), expected.trim());
    assert!(output.contains("md-error:"));
    assert!(output.contains("division by zero"));
}

#[test]
fn test_error_matmul_scalar() {
    let input = fs::read_to_string("tests/table/fixtures/error_matmul_scalar_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/error_matmul_scalar_expected.md")
        .expect("Failed to read expected fixture");

    let output = format_tables(&input);
    assert_eq!(output.trim(), expected.trim());
    assert!(output.contains("md-error:"));
    assert!(output.contains("cannot use matrix multiplication (@)"));
    assert!(output.contains("scalar"));
}

#[test]
fn test_complex_nested_expression() {
    let input = fs::read_to_string("tests/table/fixtures/error_matrix_in_expression_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/error_matrix_in_expression_expected.md")
        .expect("Failed to read expected fixture");

    let output = format_tables(&input);
    assert_eq!(output.trim(), expected.trim());
    // This complex expression now works with AST parser!
    assert!(output.contains("| 1   | 2   | 2   | 283      |"));
    assert!(!output.contains("md-error:")); // No error - it works!
}

#[test]
fn test_complex_expression_outer_transpose() {
    let input = fs::read_to_string("tests/table/fixtures/complex_expression_outer_transpose_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/complex_expression_outer_transpose_expected.md")
        .expect("Failed to read expected fixture");

    let output = format_tables(&input);
    assert_eq!(output.trim(), expected.trim());
    // Verify the complex expression with transpose evaluates correctly
    assert!(output.contains("| 1   | 2   | 2   | 283      |"));
    assert!(!output.contains("md-error:"));
}

#[test]
fn test_all_formula_functions() {
    let input = fs::read_to_string("tests/table/fixtures/all_functions_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/all_functions_expected.md")
        .expect("Failed to read expected fixture");

    let output = format_tables(&input);
    assert_eq!(output.trim(), expected.trim());
    // Verify all functions work correctly: sum, avg, min, max, count, prod
    assert!(output.contains("| 10     | 100 | 25   | 10  | 40  | 4     | 240000 |"));
    assert!(!output.contains("md-error:"));
}
