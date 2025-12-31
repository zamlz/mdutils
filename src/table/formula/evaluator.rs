use crate::table::error::FormulaError;
use crate::table::formula::ast::{BinaryOperator, Expr};
use crate::table::formula::reference::{self, resolve_reference};
use crate::table::formula::types::Value;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;

/// Evaluates an AST expression node to a Value with access to other tables and variables
pub(crate) fn eval_ast_with_tables(
    expr: &Expr,
    rows: &Vec<Vec<String>>,
    table_map: &std::collections::HashMap<String, Vec<Vec<String>>>,
    variable_map: &std::collections::HashMap<String, Value>,
) -> Result<Value, FormulaError> {
    match expr {
        Expr::Literal(d, _span) => Ok(Value::Scalar(*d)),

        Expr::String(_s, _span) => Err(FormulaError::RuntimeError(
            "string literals can only be used as arguments to functions like from()".to_string(),
        )),

        Expr::Variable(name, _span) => variable_map
            .get(name)
            .cloned()
            .ok_or_else(|| FormulaError::RuntimeError(format!("undefined variable: '{}'", name))),

        Expr::CellRef(cell_ref, _span) => resolve_reference(cell_ref, rows),

        Expr::BinaryOp {
            left,
            op,
            right,
            span: _span,
        } => {
            let left_val = eval_ast_with_tables(left, rows, table_map, variable_map)?;
            let right_val = eval_ast_with_tables(right, rows, table_map, variable_map)?;
            eval_binary_op(*op, left_val, right_val)
        }

        Expr::Transpose(inner, _span) => {
            let val = eval_ast_with_tables(inner, rows, table_map, variable_map)?;
            match val {
                Value::Scalar(_) => Err(FormulaError::RuntimeError(
                    "cannot transpose a scalar value - only matrices can be transposed".to_string(),
                )),
                Value::Matrix { .. } => val.transpose().ok_or_else(|| {
                    FormulaError::RuntimeError("transpose operation failed".to_string())
                }),
            }
        }

        Expr::FunctionCall {
            name,
            args,
            span: _span,
        } => eval_function_call_with_tables(name, args, rows, table_map, variable_map),
    }
}

/// Evaluates an AST expression node - test helper (no cross-table refs or variables)
#[cfg(test)]
pub(crate) fn eval_ast(expr: &Expr, rows: &Vec<Vec<String>>) -> Result<Value, FormulaError> {
    use std::collections::HashMap;
    eval_ast_with_tables(expr, rows, &HashMap::new(), &HashMap::new())
}

/// Evaluate a binary operation
pub(crate) fn eval_binary_op(
    op: BinaryOperator,
    left: Value,
    right: Value,
) -> Result<Value, FormulaError> {
    match op {
        BinaryOperator::Add => evaluate_operation('+', left, right),
        BinaryOperator::Sub => evaluate_operation('-', left, right),
        BinaryOperator::Mul => evaluate_operation('*', left, right),
        BinaryOperator::Div => evaluate_operation('/', left, right),
        BinaryOperator::Pow => evaluate_operation('^', left, right),
        BinaryOperator::MatMul => evaluate_operation('@', left, right),
    }
}

/// Evaluate a function call from AST with table map support
fn eval_function_call_with_tables(
    name: &str,
    args: &[Expr],
    rows: &Vec<Vec<String>>,
    table_map: &std::collections::HashMap<String, Vec<Vec<String>>>,
    variable_map: &std::collections::HashMap<String, Value>,
) -> Result<Value, FormulaError> {
    match name.to_lowercase().as_str() {
        "from" => {
            // from() requires 1 or 2 arguments
            // from("table_id") - returns entire table
            // from("table_id", range) - returns specific range from table
            // from(variable) - returns variable value (must be matrix)
            // from(variable, range) - returns specific range from variable matrix
            if args.is_empty() || args.len() > 2 {
                return Err(FormulaError::RuntimeError(format!(
                    "function 'from' expects 1 or 2 arguments, got {}",
                    args.len()
                )));
            }

            // First argument can be a string literal (table ID) or variable reference
            match &args[0] {
                Expr::String(table_id, _) => {
                    // Look up the table
                    let target_rows = table_map.get(table_id).ok_or_else(|| {
                        FormulaError::RuntimeError(format!(
                            "table '{}' not found (tables must have an id attribute)",
                            table_id
                        ))
                    })?;

                    // If only one argument, return entire table as matrix
                    if args.len() == 1 {
                        return reference::table_to_matrix(target_rows);
                    }

                    // If two arguments, second must be a cell reference or range
                    let range_value = match &args[1] {
                        Expr::CellRef(cell_ref, _) => {
                            // Resolve the reference from the target table
                            resolve_reference(cell_ref, target_rows)?
                        }
                        _ => {
                            return Err(FormulaError::RuntimeError(
                                "from() second argument must be a cell reference or range"
                                    .to_string(),
                            ));
                        }
                    };

                    Ok(range_value)
                }
                Expr::Variable(var_name, _) => {
                    // Look up the variable
                    let var_value = variable_map.get(var_name).ok_or_else(|| {
                        FormulaError::RuntimeError(format!("undefined variable: '{}'", var_name))
                    })?;

                    // Variables used in from() must be matrices
                    if let Value::Scalar(_) = var_value {
                        return Err(FormulaError::RuntimeError(format!(
                            "cannot use from() with scalar variable '{}' - expected matrix",
                            var_name
                        )));
                    }

                    // If only one argument, return the variable value
                    if args.len() == 1 {
                        return Ok(var_value.clone());
                    }

                    // If two arguments with a variable, we need to handle range selection from the matrix
                    // For now, return an error as this is complex to implement
                    Err(FormulaError::RuntimeError(
                        "from(variable, range) is not yet supported - use from(variable) to get the entire matrix".to_string()
                    ))
                }
                _ => {
                    Err(FormulaError::RuntimeError(
                        "from() first argument must be a string literal (table ID) or variable reference".to_string()
                    ))
                }
            }
        }
        // All other functions expect exactly one argument
        "sum" | "avg" | "min" | "max" | "count" | "prod" => {
            if args.len() != 1 {
                return Err(FormulaError::RuntimeError(format!(
                    "function '{}' expects exactly 1 argument, got {}",
                    name,
                    args.len()
                )));
            }

            let arg = eval_ast_with_tables(&args[0], rows, table_map, variable_map)?;
            eval_function(name, arg)
        }
        _ => Err(FormulaError::RuntimeError(format!(
            "unknown function: '{}' (supported functions: sum, avg, min, max, count, prod, from)",
            name
        ))),
    }
}

/// Evaluate a function with a Value argument (for single-arg functions)
pub(crate) fn eval_function(name: &str, arg: Value) -> Result<Value, FormulaError> {
    match name.to_lowercase().as_str() {
        "sum" => match arg {
            Value::Scalar(s) => Ok(Value::Scalar(s)),
            Value::Matrix { data, .. } => {
                let sum = data.iter().fold(Decimal::ZERO, |acc, &x| acc + x);
                Ok(Value::Scalar(sum))
            }
        },
        "avg" => match arg {
            Value::Scalar(s) => Ok(Value::Scalar(s)),
            Value::Matrix { data, .. } => {
                if data.is_empty() {
                    return Ok(Value::Scalar(Decimal::ZERO));
                }
                let sum = data.iter().fold(Decimal::ZERO, |acc, &x| acc + x);
                let count = Decimal::from(data.len());
                Ok(Value::Scalar(sum / count))
            }
        },
        "min" => match arg {
            Value::Scalar(s) => Ok(Value::Scalar(s)),
            Value::Matrix { data, .. } => {
                if data.is_empty() {
                    return Ok(Value::Scalar(Decimal::ZERO));
                }
                let min = data
                    .iter()
                    .fold(data[0], |acc, &x| if x < acc { x } else { acc });
                Ok(Value::Scalar(min))
            }
        },
        "max" => match arg {
            Value::Scalar(s) => Ok(Value::Scalar(s)),
            Value::Matrix { data, .. } => {
                if data.is_empty() {
                    return Ok(Value::Scalar(Decimal::ZERO));
                }
                let max = data
                    .iter()
                    .fold(data[0], |acc, &x| if x > acc { x } else { acc });
                Ok(Value::Scalar(max))
            }
        },
        "count" => match arg {
            Value::Scalar(_) => Ok(Value::Scalar(Decimal::ONE)),
            Value::Matrix { data, .. } => {
                let count = Decimal::from(data.len());
                Ok(Value::Scalar(count))
            }
        },
        "prod" => match arg {
            Value::Scalar(s) => Ok(Value::Scalar(s)),
            Value::Matrix { data, .. } => {
                let product = data.iter().fold(Decimal::ONE, |acc, &x| acc * x);
                Ok(Value::Scalar(product))
            }
        },
        _ => Err(FormulaError::RuntimeError(format!(
            "unknown function: '{}'",
            name
        ))),
    }
}

/// Evaluates a binary operation with automatic broadcasting support.
///
/// Supports all four combinations of scalar and vector operands:
/// - **Scalar ○ Scalar**: Standard arithmetic
/// - **Vector ○ Vector**: Element-wise operation (uses minimum length if sizes differ)
/// - **Vector ○ Scalar**: Broadcasts scalar to each vector element
/// - **Scalar ○ Vector**: Broadcasts scalar to each vector element
///
/// # Broadcasting Rules
///
/// When combining a scalar with a vector, the scalar is automatically applied
/// to every element of the vector. For example:
/// - `[1, 2, 3] + 10` → `[11, 12, 13]`
/// - `5 * [2, 4, 6]` → `[10, 20, 30]`
///
/// # Supported Operators
///
/// * `+` - Addition
/// * `-` - Subtraction
/// * `*` - Multiplication
/// * `/` - Division (returns `None` on division by zero)
/// * `^` - Exponentiation
/// * `@` - Matrix multiplication (dot product for vectors)
///
/// # Arguments
///
/// * `op` - The operator character
/// * `left` - Left operand (Scalar or Vector)
/// * `right` - Right operand (Scalar or Vector)
///
/// # Returns
///
/// * `Ok(Value)` if the operation succeeds
/// * `Err(String)` with a specific error message if the operation fails
pub(crate) fn evaluate_operation(
    op: char,
    left: Value,
    right: Value,
) -> Result<Value, FormulaError> {
    // Handle matrix multiplication (@) - uses proper matrix multiplication rules
    if op == '@' {
        return match (&left, &right) {
            (Value::Matrix { rows: m, cols: n, data: left_data },
             Value::Matrix { rows: n2, cols: p, data: right_data }) => {
                // Check dimension compatibility: (m×n) @ (n2×p) requires n == n2
                if n != n2 {
                    return Err(FormulaError::RuntimeError(
                        format!("matrix multiplication dimension mismatch: cannot multiply ({}×{}) @ ({}×{}) - inner dimensions {} and {} must match", m, n, n2, p, n, n2)
                    ));
                }

                // Perform matrix multiplication
                let mut result = Vec::with_capacity(m * p);
                for i in 0..(*m) {
                    for j in 0..(*p) {
                        let mut sum = Decimal::ZERO;
                        for k in 0..(*n) {
                            sum += left_data[i * n + k] * right_data[k * p + j];
                        }
                        result.push(sum);
                    }
                }

                // Return result as (m×p) matrix
                Ok(Value::Matrix {
                    rows: *m,
                    cols: *p,
                    data: result,
                })
            }
            (Value::Scalar(_), Value::Scalar(_)) => {
                Err(FormulaError::RuntimeError(
                    "cannot use matrix multiplication (@) with two scalar values - use * for scalar multiplication".to_string()
                ))
            }
            (Value::Scalar(_), Value::Matrix { rows, cols, .. }) => {
                Err(FormulaError::RuntimeError(
                    format!("cannot use matrix multiplication (@) with scalar on left side and ({}×{}) matrix on right side", rows, cols)
                ))
            }
            (Value::Matrix { rows, cols, .. }, Value::Scalar(_)) => {
                Err(FormulaError::RuntimeError(
                    format!("cannot use matrix multiplication (@) with ({}×{}) matrix on left side and scalar on right side", rows, cols)
                ))
            }
        };
    }

    // Handle other operators (+, -, *, /, ^)
    match (left, right) {
        // Scalar op Scalar
        (Value::Scalar(l), Value::Scalar(r)) => {
            let result = apply_scalar_op(op, l, r).ok_or_else(|| {
                FormulaError::RuntimeError(format!(
                    "division by zero in scalar operation: {} {} {}",
                    l, op, r
                ))
            })?;
            Ok(Value::Scalar(result))
        }

        // Matrix op Matrix (element-wise) - must have same dimensions
        (
            Value::Matrix {
                rows: m1,
                cols: n1,
                data: data1,
            },
            Value::Matrix {
                rows: m2,
                cols: n2,
                data: data2,
            },
        ) => {
            // For element-wise operations, dimensions must match
            if m1 != m2 || n1 != n2 {
                return Err(FormulaError::RuntimeError(
                    format!("element-wise operation '{}' requires matching dimensions: got ({}×{}) and ({}×{})", op, m1, n1, m2, n2)
                ));
            }

            let mut result = Vec::with_capacity(data1.len());
            for (i, (&a, &b)) in data1.iter().zip(data2.iter()).enumerate() {
                let value = apply_scalar_op(op, a, b).ok_or_else(|| {
                    FormulaError::RuntimeError(format!(
                        "division by zero in element-wise operation at position {}",
                        i
                    ))
                })?;
                result.push(value);
            }

            Ok(Value::Matrix {
                rows: m1,
                cols: n1,
                data: result,
            })
        }

        // Matrix op Scalar (broadcast scalar to all elements)
        (Value::Matrix { rows, cols, data }, Value::Scalar(scalar)) => {
            let mut result = Vec::with_capacity(data.len());
            for (i, &v) in data.iter().enumerate() {
                let value = apply_scalar_op(op, v, scalar).ok_or_else(|| {
                    FormulaError::RuntimeError(format!(
                        "division by zero when broadcasting scalar to matrix at position {}",
                        i
                    ))
                })?;
                result.push(value);
            }

            Ok(Value::Matrix {
                rows,
                cols,
                data: result,
            })
        }

        // Scalar op Matrix (broadcast scalar to all elements)
        (Value::Scalar(scalar), Value::Matrix { rows, cols, data }) => {
            let mut result = Vec::with_capacity(data.len());
            for (i, &v) in data.iter().enumerate() {
                let value = apply_scalar_op(op, scalar, v).ok_or_else(|| {
                    FormulaError::RuntimeError(format!(
                        "division by zero when broadcasting scalar to matrix at position {}",
                        i
                    ))
                })?;
                result.push(value);
            }

            Ok(Value::Matrix {
                rows,
                cols,
                data: result,
            })
        }
    }
}

/// Helper function to apply a scalar operation to two Decimal values
pub(crate) fn apply_scalar_op(op: char, left: Decimal, right: Decimal) -> Option<Decimal> {
    match op {
        '+' => Some(left + right),
        '-' => Some(left - right),
        '*' => Some(left * right),
        '/' => {
            if right == Decimal::ZERO {
                None
            } else {
                Some(left / right)
            }
        }
        '^' => decimal_pow(left, right),
        _ => None,
    }
}

/// Helper function to compute decimal power for integer exponents
pub(crate) fn decimal_pow(base: Decimal, exp: Decimal) -> Option<Decimal> {
    // Try to convert exponent to i64 for integer power
    if let Some(exp_i64) = exp.to_i64() {
        if exp_i64 >= 0 {
            // Positive integer exponent: compute by repeated multiplication
            let mut result = Decimal::ONE;
            for _ in 0..exp_i64 {
                result *= base;
            }
            return Some(result);
        } else {
            // Negative exponent: base^(-n) = 1 / base^n
            let positive_exp = exp_i64.unsigned_abs();
            let mut result = Decimal::ONE;
            for _ in 0..positive_exp {
                result *= base;
            }
            return Some(Decimal::ONE / result);
        }
    }

    // For non-integer exponents, we'd need to convert to f64, compute, and convert back
    // This loses precision but is necessary for fractional powers
    if let Some(base_f64) = base.to_f64() {
        if let Some(exp_f64) = exp.to_f64() {
            let result = base_f64.powf(exp_f64);
            return Decimal::from_f64(result);
        }
    }

    None
}
