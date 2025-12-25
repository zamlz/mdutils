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
/// Note: Row 1 is the header row, Row 2 is the first data row
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
    // Row 1 = index 0 (header)
    // Row 2 = index 2 (first data row, skipping separator at index 1)
    // Row 3 = index 3, etc.
    if row_num == 0 {
        return None;
    }

    let row_idx = if row_num == 1 {
        0 // Header row
    } else {
        row_num // Skip separator row at index 1
    };

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

/// Evaluates tokens with proper operator precedence
fn eval_tokens(tokens: &[&str]) -> Option<f64> {
    if tokens.is_empty() {
        return None;
    }

    // Handle single number
    if tokens.len() == 1 {
        return tokens[0].parse::<f64>().ok();
    }

    // Find lowest precedence operator (+ or -)
    for (i, &token) in tokens.iter().enumerate().skip(1) {
        if token == "+" || token == "-" {
            let left = eval_tokens(&tokens[..i])?;
            let right = eval_tokens(&tokens[i + 1..])?;
            return Some(if token == "+" { left + right } else { left - right });
        }
    }

    // Find next precedence operator (* or /)
    for (i, &token) in tokens.iter().enumerate().skip(1) {
        if token == "*" || token == "/" {
            let left = eval_tokens(&tokens[..i])?;
            let right = eval_tokens(&tokens[i + 1..])?;
            return Some(if token == "*" { left * right } else { left / right });
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_ref_to_index() {
        assert_eq!(cell_ref_to_index("A1"), Some((0, 0))); // Header row
        assert_eq!(cell_ref_to_index("B1"), Some((0, 1))); // Header row
        assert_eq!(cell_ref_to_index("A2"), Some((2, 0))); // First data row (skips separator at index 1)
        assert_eq!(cell_ref_to_index("D4"), Some((4, 3))); // Third data row
        assert_eq!(cell_ref_to_index("Z10"), Some((10, 25)));
        assert_eq!(cell_ref_to_index("a1"), Some((0, 0))); // lowercase
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
}
