use mdutils::format_tables;
/// Integration tests for table formatting and formula evaluation
/// Tests use fixture files in tests/table/fixtures/ directory
use std::fs;

#[test]
fn test_vector_addition() {
    let input = fs::read_to_string("tests/table/fixtures/vector_addition_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/vector_addition_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());

    // Idempotency check: command(expected) should equal expected
    let result2 = format_tables(&expected);
    assert_eq!(
        result2.output.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_vector_scalar_multiplication() {
    let input = fs::read_to_string("tests/table/fixtures/vector_scalar_mult_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/vector_scalar_mult_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());

    // Idempotency check: command(expected) should equal expected
    let result2 = format_tables(&expected);
    assert_eq!(
        result2.output.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_matrix_multiplication() {
    let input = fs::read_to_string("tests/table/fixtures/matrix_mult_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/matrix_mult_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());

    // Idempotency check: command(expected) should equal expected
    let result2 = format_tables(&expected);
    assert_eq!(
        result2.output.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_transpose_operator() {
    let input = fs::read_to_string("tests/table/fixtures/transpose_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/transpose_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());

    // Idempotency check: command(expected) should equal expected
    let result2 = format_tables(&expected);
    assert_eq!(
        result2.output.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_multiple_formulas() {
    let input = fs::read_to_string("tests/table/fixtures/multiple_formulas_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/multiple_formulas_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());

    // Idempotency check: command(expected) should equal expected
    let result2 = format_tables(&expected);
    assert_eq!(
        result2.output.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_sum_function() {
    let input = fs::read_to_string("tests/table/fixtures/sum_function_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/sum_function_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());

    // Idempotency check: command(expected) should equal expected
    let result2 = format_tables(&expected);
    assert_eq!(
        result2.output.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_complex_expression() {
    let input = fs::read_to_string("tests/table/fixtures/complex_expression_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/complex_expression_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());

    // Idempotency check: command(expected) should equal expected
    let result2 = format_tables(&expected);
    assert_eq!(
        result2.output.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_real_world_tax_calculation() {
    let input = fs::read_to_string("tests/table/fixtures/real_world_tax_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/real_world_tax_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());

    // Idempotency check: command(expected) should equal expected
    let result2 = format_tables(&expected);
    assert_eq!(
        result2.output.trim(),
        expected.trim(),
        "Not idempotent: running on expected output produced different result"
    );
}

#[test]
fn test_formula_parse_error() {
    let input = fs::read_to_string("tests/table/fixtures/formula_parse_error_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/formula_parse_error_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    // Verify error comment is present
    assert!(result.output.contains("md-error:"));
    assert!(result.output.contains("Failed to parse statement"));
}

#[test]
fn test_formula_eval_error() {
    let input = fs::read_to_string("tests/table/fixtures/formula_eval_error_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/formula_eval_error_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    // Verify error comment is present
    assert!(result.output.contains("md-error:"));
    // First formula should succeed
    assert!(result.output.contains("| 1   | 2   | 3   |"));
}

#[test]
fn test_error_cell_out_of_bounds() {
    let input = fs::read_to_string("tests/table/fixtures/error_cell_out_of_bounds_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/error_cell_out_of_bounds_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    assert!(result.output.contains("md-error:"));
    assert!(result.output.contains("out of bounds"));
}

#[test]
fn test_error_column_out_of_bounds() {
    let input = fs::read_to_string("tests/table/fixtures/error_column_out_of_bounds_input.md")
        .expect("Failed to read input fixture");
    let expected =
        fs::read_to_string("tests/table/fixtures/error_column_out_of_bounds_expected.md")
            .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    assert!(result.output.contains("md-error:"));
    assert!(result.output.contains("column index out of bounds"));
}

#[test]
fn test_error_row_out_of_bounds() {
    let input = fs::read_to_string("tests/table/fixtures/error_row_out_of_bounds_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/error_row_out_of_bounds_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    assert!(result.output.contains("md-error:"));
    assert!(result.output.contains("row vector _5 is out of bounds"));
}

#[test]
fn test_error_matrix_mult_dimension() {
    let input = fs::read_to_string("tests/table/fixtures/error_matrix_mult_dimension_input.md")
        .expect("Failed to read input fixture");
    let expected =
        fs::read_to_string("tests/table/fixtures/error_matrix_mult_dimension_expected.md")
            .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    assert!(result.output.contains("md-error:"));
    assert!(result
        .output
        .contains("matrix multiplication dimension mismatch"));
    assert!(result.output.contains("inner dimensions"));
}

#[test]
fn test_error_transpose_scalar() {
    let input = fs::read_to_string("tests/table/fixtures/error_transpose_scalar_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/error_transpose_scalar_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    assert!(result.output.contains("md-error:"));
    assert!(result.output.contains("cannot transpose a scalar"));
}

#[test]
fn test_error_elementwise_dimension() {
    let input = fs::read_to_string("tests/table/fixtures/error_elementwise_dimension_input.md")
        .expect("Failed to read input fixture");
    let expected =
        fs::read_to_string("tests/table/fixtures/error_elementwise_dimension_expected.md")
            .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    assert!(result.output.contains("md-error:"));
    assert!(result.output.contains("element-wise operation"));
    assert!(result.output.contains("matching dimensions"));
}

#[test]
fn test_error_unmatched_paren() {
    let input = fs::read_to_string("tests/table/fixtures/error_unmatched_paren_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/error_unmatched_paren_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    assert!(result.output.contains("md-error:"));
    assert!(result.output.contains("unmatched"));
}

#[test]
fn test_error_invalid_token() {
    let input = fs::read_to_string("tests/table/fixtures/error_invalid_token_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/error_invalid_token_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    assert!(result.output.contains("md-error:"));
    assert!(result.output.contains("undefined variable"));
}

#[test]
fn test_error_unknown_function() {
    let input = fs::read_to_string("tests/table/fixtures/error_unknown_function_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/error_unknown_function_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    assert!(result.output.contains("md-error:"));
    assert!(result.output.contains("unknown function"));
}

#[test]
fn test_error_division_by_zero() {
    let input = fs::read_to_string("tests/table/fixtures/error_division_by_zero_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/error_division_by_zero_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    assert!(result.output.contains("md-error:"));
    assert!(result.output.contains("division by zero"));
}

#[test]
fn test_error_matmul_scalar() {
    let input = fs::read_to_string("tests/table/fixtures/error_matmul_scalar_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/error_matmul_scalar_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    assert!(result.output.contains("md-error:"));
    assert!(result
        .output
        .contains("cannot use matrix multiplication (@)"));
    assert!(result.output.contains("scalar"));
}

#[test]
fn test_complex_nested_expression() {
    let input = fs::read_to_string("tests/table/fixtures/error_matrix_in_expression_input.md")
        .expect("Failed to read input fixture");
    let expected =
        fs::read_to_string("tests/table/fixtures/error_matrix_in_expression_expected.md")
            .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    // This complex expression now works with AST parser!
    assert!(result.output.contains("| 1   | 2   | 2   | 283      |"));
    assert!(!result.output.contains("md-error:")); // No error - it works!
}

#[test]
fn test_complex_expression_outer_transpose() {
    let input =
        fs::read_to_string("tests/table/fixtures/complex_expression_outer_transpose_input.md")
            .expect("Failed to read input fixture");
    let expected =
        fs::read_to_string("tests/table/fixtures/complex_expression_outer_transpose_expected.md")
            .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    // Verify the complex expression with transpose evaluates correctly
    assert!(result.output.contains("| 1   | 2   | 2   | 283      |"));
    assert!(!result.output.contains("md-error:"));
}

#[test]
fn test_all_formula_functions() {
    let input = fs::read_to_string("tests/table/fixtures/all_functions_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/all_functions_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    // Verify all functions work correctly: sum, avg, min, max, count, prod
    assert!(result
        .output
        .contains("| 10     | 100 | 25   | 10  | 40  | 4     | 240000 |"));
    assert!(!result.output.contains("md-error:"));
}

#[test]
fn test_cross_table_reference() {
    let input = fs::read_to_string("tests/table/fixtures/cross_table_reference_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/cross_table_reference_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    // Verify from() function works with column reference
    assert!(result.output.contains("| 60   | 20   |"));
    assert!(!result.output.contains("md-error:"));
}

#[test]
fn test_cross_table_full_matrix() {
    let input = fs::read_to_string("tests/table/fixtures/cross_table_full_matrix_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/cross_table_full_matrix_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    // Verify from() function works with entire table
    assert!(result.output.contains("| 10    |"));
    assert!(!result.output.contains("md-error:"));
}

#[test]
fn test_cross_table_missing_id() {
    let input = fs::read_to_string("tests/table/fixtures/cross_table_missing_id_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/cross_table_missing_id_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    // Verify error when table ID doesn't exist
    assert!(result.output.contains("md-error:"));
    assert!(result.output.contains("table 'nonexistent' not found"));
}

#[test]
fn test_assignment_row_vector() {
    let input = fs::read_to_string("tests/table/fixtures/assignment_row_vector_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/assignment_row_vector_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    // Verify row vector assignment works
    assert!(result.output.contains("| 2   | 4   | 6   |"));
    assert!(!result.output.contains("md-error:"));
}

#[test]
fn test_assignment_range() {
    let input = fs::read_to_string("tests/table/fixtures/assignment_range_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/assignment_range_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    // Verify range assignment works
    assert!(result.output.contains("| 1   | 2   | 11  | 12  |"));
    assert!(result.output.contains("| 3   | 4   | 13  | 14  |"));
    assert!(!result.output.contains("md-error:"));
}

#[test]
fn test_assignment_column_range() {
    let input = fs::read_to_string("tests/table/fixtures/assignment_column_range_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/assignment_column_range_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    // Verify column range assignment works
    assert!(result.output.contains("| 1   | 2   | 10  | 20  |"));
    assert!(result.output.contains("| 3   | 4   | 30  | 40  |"));
    assert!(result.output.contains("| 5   | 6   | 50  | 60  |"));
    assert!(!result.output.contains("md-error:"));
}

#[test]
fn test_assignment_row_range() {
    let input = fs::read_to_string("tests/table/fixtures/assignment_row_range_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/assignment_row_range_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    // Verify row range assignment works
    assert!(result.output.contains("| 10  | 20  | 30  |"));
    assert!(result.output.contains("| 40  | 50  | 60  |"));
    assert!(!result.output.contains("md-error:"));
}

// Tests for variables (let statements)

#[test]
fn test_variables_basic() {
    let input = fs::read_to_string("tests/table/fixtures/variables_basic_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/variables_basic_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    // Verify variable assignment worked
    assert!(result.output.contains("| 5   | 10  | 15  |"));
    assert!(!result.output.contains("md-error:"));
}

#[test]
fn test_variables_vector() {
    let input = fs::read_to_string("tests/table/fixtures/variables_vector_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/variables_vector_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    // Verify vector variable worked
    assert!(result.output.contains("| 1   | 2   | 3   |"));
    assert!(result.output.contains("| 3   | 4   | 7   |"));
    assert!(result.output.contains("| 5   | 6   | 11  |"));
    assert!(!result.output.contains("md-error:"));
}

#[test]
fn test_variables_multiple() {
    let input = fs::read_to_string("tests/table/fixtures/variables_multiple_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/variables_multiple_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    // Verify multiple variables and expressions worked
    assert!(result.output.contains("| 5   | 10  | 50  | 15  |"));
    assert!(!result.output.contains("md-error:"));
}

#[test]
fn test_variables_complex_expressions() {
    let input = fs::read_to_string("tests/table/fixtures/variables_complex_expr_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/variables_complex_expr_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    // Verify complex expression with parentheses worked
    assert!(result.output.contains("| 2   | 3   | 10  |"));
    assert!(result.output.contains("| 4   | 5   | 18  |"));
    assert!(!result.output.contains("md-error:"));
}

#[test]
fn test_variables_with_functions() {
    let input = fs::read_to_string("tests/table/fixtures/variables_with_functions_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/variables_with_functions_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    // Verify variables with sum() function worked
    assert!(result.output.contains("| 1   | 10  | 6   | 60  |"));
    assert!(!result.output.contains("md-error:"));
}

#[test]
fn test_variables_transpose() {
    let input = fs::read_to_string("tests/table/fixtures/variables_transpose_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/variables_transpose_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    // Verify transpose operation on variable worked
    assert!(result.output.contains("| 1   | 2   |"));
    assert!(!result.output.contains("md-error:"));
}

#[test]
fn test_variables_error_undefined() {
    let input = fs::read_to_string("tests/table/fixtures/variables_error_undefined_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/variables_error_undefined_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    // Verify error for undefined variable
    assert!(result.output.contains("md-error:"));
    assert!(result
        .output
        .contains("undefined variable: 'undefined_var'"));
}

#[test]
fn test_variables_error_invalid_name() {
    let input = fs::read_to_string("tests/table/fixtures/variables_error_invalid_name_input.md")
        .expect("Failed to read input fixture");
    let expected =
        fs::read_to_string("tests/table/fixtures/variables_error_invalid_name_expected.md")
            .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    // Verify error for cell-reference-like variable name
    assert!(result.output.contains("md-error:"));
    assert!(result.output.contains("Failed to parse statement"));
}

#[test]
fn test_skip_code_blocks() {
    let input = fs::read_to_string("tests/table/fixtures/skip_code_blocks_input.md")
        .expect("Failed to read input fixture");
    let expected = fs::read_to_string("tests/table/fixtures/skip_code_blocks_expected.md")
        .expect("Failed to read expected fixture");

    let result = format_tables(&input);
    assert_eq!(result.output.trim(), expected.trim());
    // Verify real tables outside code blocks were formatted with formulas applied
    assert!(result.output.contains("| 5   | 10  | 15  |"));
    assert!(result.output.contains("| 1   | 2   | 2   |"));
    // Verify table inside code block was NOT processed (still has 0 in column C)
    assert!(result.output.contains("| 3   | 2   | 0   |"));
    // No errors should be present
    assert!(!result.output.contains("md-error:"));
}
