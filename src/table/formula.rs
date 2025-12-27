/// Applies formulas to table cells
pub fn apply_formulas(rows: &mut Vec<Vec<String>>, formulas: &[String]) {
    for formula in formulas {
        if let Some((cell_ref, expr)) = parse_formula(formula) {
            if let Some((row_idx, col_idx)) = cell_ref_to_index(&cell_ref) {
                // Evaluate the expression
                if let Some(value) = evaluate_expression(&expr, rows) {
                    // Update the cell value
                    if row_idx < rows.len() && col_idx < rows[row_idx].len() {
                        rows[row_idx][col_idx] = value;
                    }
                }
            }
        }
    }
}

/// Parses a formula like "A1 = B1 + C1" into (cell_ref, expression)
fn parse_formula(formula: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = formula.split('=').collect();
    if parts.len() == 2 {
        Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
    } else {
        None
    }
}

/// Converts cell reference (like "A1", "B2") to (row_index, col_index)
/// Note: Row 1 is the first data row (headers are not addressable)
/// The separator row (|---|---|) is automatically skipped
fn cell_ref_to_index(cell_ref: &str) -> Option<(usize, usize)> {
    let cell_ref = cell_ref.trim().to_uppercase();
    let mut chars = cell_ref.chars();

    // Get column letter (A, B, C, etc.)
    let col_char = chars.next()?;
    if !col_char.is_ascii_alphabetic() {
        return None;
    }

    // Convert A=0, B=1, C=2, etc.
    let col_idx = (col_char as u32 - 'A' as u32) as usize;

    // Get row number (1, 2, 3, etc.)
    let row_str: String = chars.collect();
    let row_num: usize = row_str.parse().ok()?;

    // Convert to 0-based index
    // Row 1 = index 2 (first data row, skipping header at 0 and separator at 1)
    // Row 2 = index 3 (second data row)
    // Row 3 = index 4, etc.
    if row_num == 0 {
        return None;
    }

    let row_idx = row_num + 1; // Add 1 to skip header and separator

    Some((row_idx, col_idx))
}

/// Evaluates a mathematical expression with cell references
fn evaluate_expression(expr: &str, rows: &Vec<Vec<String>>) -> Option<String> {
    // Tokenize the expression
    let tokens = tokenize_expression(expr);

    // Replace cell references with their values
    let mut resolved_tokens = Vec::new();
    for token in tokens {
        if is_cell_reference(&token) {
            // Get cell value
            if let Some((row_idx, col_idx)) = cell_ref_to_index(&token) {
                if row_idx < rows.len() && col_idx < rows[row_idx].len() {
                    let cell_value = &rows[row_idx][col_idx];
                    // Parse as number
                    if let Ok(num) = cell_value.parse::<f64>() {
                        resolved_tokens.push(num.to_string());
                    } else {
                        // Can't evaluate non-numeric cell
                        return None;
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            }
        } else {
            resolved_tokens.push(token);
        }
    }

    // Evaluate the mathematical expression
    evaluate_math_expression(&resolved_tokens.join(" "))
}

/// Tokenizes an expression into parts (numbers, operators, cell references)
fn tokenize_expression(expr: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();

    for ch in expr.chars() {
        match ch {
            '+' | '-' | '*' | '/' | '(' | ')' => {
                if !current_token.is_empty() {
                    tokens.push(current_token.trim().to_string());
                    current_token.clear();
                }
                tokens.push(ch.to_string());
            }
            ' ' => {
                if !current_token.is_empty() {
                    tokens.push(current_token.trim().to_string());
                    current_token.clear();
                }
            }
            _ => {
                current_token.push(ch);
            }
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token.trim().to_string());
    }

    tokens
}

/// Checks if a token is a cell reference (like A1, B2, etc.)
fn is_cell_reference(token: &str) -> bool {
    let token = token.trim().to_uppercase();
    if token.is_empty() {
        return false;
    }

    let mut chars = token.chars();
    let first = chars.next().unwrap();

    // Must start with a letter
    if !first.is_ascii_alphabetic() {
        return false;
    }

    // Rest must be digits
    for ch in chars {
        if !ch.is_ascii_digit() {
            return false;
        }
    }

    true
}

/// Evaluates a simple mathematical expression (numbers and +, -, *, / operators)
fn evaluate_math_expression(expr: &str) -> Option<String> {
    // Simple recursive descent parser for basic arithmetic
    let tokens: Vec<&str> = expr.split_whitespace().collect();
    if tokens.is_empty() {
        return None;
    }

    // For now, use a simple left-to-right evaluation with proper operator precedence
    let result = eval_tokens(&tokens)?;

    // Format the result (remove unnecessary decimals)
    if result.fract() == 0.0 {
        Some(format!("{:.0}", result))
    } else {
        Some(format!("{}", result))
    }
}

/// Evaluates tokens with proper operator precedence and parentheses support
fn eval_tokens(tokens: &[&str]) -> Option<f64> {
    if tokens.is_empty() {
        return None;
    }

    // Handle single number
    if tokens.len() == 1 {
        return tokens[0].parse::<f64>().ok();
    }

    // First, handle parentheses by finding and evaluating them
    // Look for the innermost parentheses and evaluate them recursively
    if let Some(processed) = process_parentheses(tokens) {
        return eval_tokens(&processed.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    }

    // Find lowest precedence operator (+ or -) at the top level
    let mut depth = 0;
    for (i, &token) in tokens.iter().enumerate().rev() {
        if token == ")" {
            depth += 1;
        } else if token == "(" {
            depth -= 1;
        } else if depth == 0 && (token == "+" || token == "-") && i > 0 {
            let left = eval_tokens(&tokens[..i])?;
            let right = eval_tokens(&tokens[i + 1..])?;
            return Some(if token == "+" { left + right } else { left - right });
        }
    }

    // Find next precedence operator (* or /) at the top level
    depth = 0;
    for (i, &token) in tokens.iter().enumerate().rev() {
        if token == ")" {
            depth += 1;
        } else if token == "(" {
            depth -= 1;
        } else if depth == 0 && (token == "*" || token == "/") && i > 0 {
            let left = eval_tokens(&tokens[..i])?;
            let right = eval_tokens(&tokens[i + 1..])?;
            return Some(if token == "*" { left * right } else { left / right });
        }
    }

    None
}

/// Processes parentheses by finding the first pair and evaluating the content
/// Returns None if no parentheses found, otherwise returns tokens with parentheses replaced by result
fn process_parentheses(tokens: &[&str]) -> Option<Vec<String>> {
    // Find the first opening parenthesis
    let open_idx = tokens.iter().position(|&t| t == "(")?;

    // Find the matching closing parenthesis
    let mut depth = 1;
    let mut close_idx = None;
    for (i, &token) in tokens.iter().enumerate().skip(open_idx + 1) {
        if token == "(" {
            depth += 1;
        } else if token == ")" {
            depth -= 1;
            if depth == 0 {
                close_idx = Some(i);
                break;
            }
        }
    }

    let close_idx = close_idx?;

    // Evaluate the content inside the parentheses
    let inner_tokens = &tokens[open_idx + 1..close_idx];
    let result = eval_tokens(inner_tokens)?;

    // Build new token list with the parentheses replaced by the result
    let mut new_tokens = Vec::new();
    new_tokens.extend(tokens[..open_idx].iter().map(|s| s.to_string()));
    new_tokens.push(result.to_string());
    new_tokens.extend(tokens[close_idx + 1..].iter().map(|s| s.to_string()));

    Some(new_tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_ref_to_index() {
        assert_eq!(cell_ref_to_index("A1"), Some((2, 0))); // First data row (skips header at 0 and separator at 1)
        assert_eq!(cell_ref_to_index("B1"), Some((2, 1))); // First data row, column B
        assert_eq!(cell_ref_to_index("A2"), Some((3, 0))); // Second data row
        assert_eq!(cell_ref_to_index("D3"), Some((4, 3))); // Third data row, column D
        assert_eq!(cell_ref_to_index("Z10"), Some((11, 25))); // 10th data row, column Z
        assert_eq!(cell_ref_to_index("a1"), Some((2, 0))); // lowercase
        assert_eq!(cell_ref_to_index("A0"), None); // row 0 invalid
        assert_eq!(cell_ref_to_index("1A"), None); // wrong format
    }

    #[test]
    fn test_is_cell_reference() {
        assert!(is_cell_reference("A1"));
        assert!(is_cell_reference("B2"));
        assert!(is_cell_reference("Z99"));
        assert!(is_cell_reference("a1")); // lowercase should work
        assert!(!is_cell_reference("1A")); // number first
        assert!(!is_cell_reference("AB")); // no number
        assert!(!is_cell_reference("123")); // only number
        assert!(!is_cell_reference("")); // empty
    }

    #[test]
    fn test_parse_formula() {
        assert_eq!(
            parse_formula("A1 = B1 + C1"),
            Some(("A1".to_string(), "B1 + C1".to_string()))
        );
        assert_eq!(
            parse_formula("D2 = B2 * C2"),
            Some(("D2".to_string(), "B2 * C2".to_string()))
        );
        assert_eq!(parse_formula("invalid"), None);
    }

    #[test]
    fn test_parentheses_basic() {
        // Test basic parentheses: (2 + 3) * 4 = 20
        let tokens = vec!["(", "2", "+", "3", ")", "*", "4"];
        assert_eq!(eval_tokens(&tokens), Some(20.0));
    }

    #[test]
    fn test_parentheses_nested() {
        // Test nested parentheses: ((2 + 3) * 4) + 1 = 21
        let tokens = vec!["(", "(", "2", "+", "3", ")", "*", "4", ")", "+", "1"];
        assert_eq!(eval_tokens(&tokens), Some(21.0));
    }

    #[test]
    fn test_parentheses_precedence() {
        // Test that parentheses override precedence: 10 - (2 * 3) = 4
        let tokens = vec!["10", "-", "(", "2", "*", "3", ")"];
        assert_eq!(eval_tokens(&tokens), Some(4.0));

        // Without parentheses: 10 - 2 * 3 = 4 (same due to precedence)
        let tokens2 = vec!["10", "-", "2", "*", "3"];
        assert_eq!(eval_tokens(&tokens2), Some(4.0));

        // But (10 - 2) * 3 = 24
        let tokens3 = vec!["(", "10", "-", "2", ")", "*", "3"];
        assert_eq!(eval_tokens(&tokens3), Some(24.0));
    }

    #[test]
    fn test_parentheses_complex() {
        // Test complex expression: (5 + 3) * (10 - 2) / 4 = 16
        let tokens = vec!["(", "5", "+", "3", ")", "*", "(", "10", "-", "2", ")", "/", "4"];
        assert_eq!(eval_tokens(&tokens), Some(16.0));
    }
}
