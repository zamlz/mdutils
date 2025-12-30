/// Tokenizes a mathematical expression string into individual components.
///
/// Splits the expression into tokens while preserving operators, parentheses,
/// numbers, and identifiers (cell references, function names). Whitespace is
/// ignored except as a separator.
///
/// # Arguments
///
/// * `expr` - The expression string to tokenize
///
/// # Returns
///
/// A vector of token strings. Each token is one of:
/// - An operator: `+`, `-`, `*`, `/`, `^`, `@`
/// - A parenthesis: `(`, `)`
/// - A number: `42`, `3.14`
/// - An identifier: `A1`, `B_`, `sum`
///
/// # Tokenization Rules
///
/// **Operators** are split into individual tokens:
/// - Arithmetic: `+`, `-`, `*`, `/`, `^` (exponentiation)
/// - Matrix operations: `@` (matrix multiplication)
/// - Each operator becomes a single-character token
///
/// **Parentheses** are split into individual tokens:
/// - Opening `(` and closing `)` parentheses
/// - Used for grouping expressions and function arguments
///
/// **The dot operator** for transpose (`.T`) is handled specially:
/// - The dot `.` and `T` are kept as separate tokens
/// - Allows the parser to recognize the `.T` transpose operator
///
/// **Numbers** are kept together as single tokens:
/// - Integer literals: "42", "100"
/// - Decimal literals: "3.14", "0.5"
/// - Scientific notation is not currently supported
///
/// **Identifiers** (cell references and function names) are kept together:
/// - Cell references: "A1", "B2", "A_", "_1"
/// - Function names: "sum", "avg"
/// - Alphanumeric characters and underscores
///
/// **Whitespace** is ignored except as a token separator. Multiple spaces are
/// treated the same as a single space.
///
/// For example, "A1 + B2 * 3" becomes ["A1", "+", "B2", "*", "3"], and
/// "sum(A_)" becomes ["sum", "(", "A_", ")"].
pub(crate) fn tokenize_expression(expr: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let chars: Vec<char> = expr.chars().collect();

    let mut i = 0;
    while i < chars.len() {
        let ch = chars[i];

        match ch {
            '+' | '-' | '*' | '/' | '^' | '@' | '(' | ')' => {
                if !current_token.is_empty() {
                    tokens.push(current_token.trim().to_string());
                    current_token.clear();
                }
                tokens.push(ch.to_string());
                i += 1;
            }
            '.' => {
                // Check if this is a decimal point in a number
                // It's a decimal point if:
                // 1. We're building a numeric token AND
                // 2. The next character is a digit
                let is_decimal_point = !current_token.is_empty()
                    && current_token.chars().all(|c| c.is_ascii_digit())
                    && i + 1 < chars.len()
                    && chars[i + 1].is_ascii_digit();

                if is_decimal_point {
                    // Include the decimal point in the current number token
                    current_token.push(ch);
                    i += 1;
                } else {
                    // It's the transpose operator - treat as separate token
                    if !current_token.is_empty() {
                        tokens.push(current_token.trim().to_string());
                        current_token.clear();
                    }
                    tokens.push(ch.to_string());
                    i += 1;
                }
            }
            ' ' => {
                if !current_token.is_empty() {
                    tokens.push(current_token.trim().to_string());
                    current_token.clear();
                }
                i += 1;
            }
            _ => {
                current_token.push(ch);
                i += 1;
            }
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token.trim().to_string());
    }

    tokens
}
