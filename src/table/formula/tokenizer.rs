use crate::table::formula::types::Span;

/// Represents a token with its value and position in the source expression
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Token {
    pub(crate) value: String,
    pub(crate) span: Span,
}

impl Token {
    pub(crate) fn new(value: String, span: Span) -> Self {
        Token { value, span }
    }
}

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
/// A vector of Token structs, each containing:
/// - `value`: The token string (`+`, `-`, `*`, `/`, `^`, `@`, `(`, `)`, numbers, identifiers)
/// - `span`: The position in the source expression (start and end indices)
///
/// # Tokenization Rules
///
/// **Operators** are split into individual tokens:
/// - Arithmetic: `+`, `-`, `*`, `/`, `^` (exponentiation)
/// - Matrix operations: `@` (matrix multiplication)
/// - Range: `:` (cell range operator, e.g., A1:C5)
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
/// For example, "A1 + B2 * 3" becomes tokens with values ["A1", "+", "B2", "*", "3"], and
/// "sum(A_)" becomes tokens with values ["sum", "(", "A_", ")"], each with their position spans.
pub(crate) fn tokenize_expression(expr: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut token_start = 0;
    let chars: Vec<char> = expr.chars().collect();

    let mut i = 0;
    while i < chars.len() {
        let ch = chars[i];

        match ch {
            '+' | '-' | '*' | '/' | '^' | '@' | '(' | ')' | ':' => {
                if !current_token.is_empty() {
                    let trimmed = current_token.trim();
                    if !trimmed.is_empty() {
                        tokens.push(Token::new(
                            trimmed.to_string(),
                            Span::new(token_start, i),
                        ));
                    }
                    current_token.clear();
                }
                tokens.push(Token::new(ch.to_string(), Span::new(i, i + 1)));
                token_start = i + 1;
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
                        let trimmed = current_token.trim();
                        if !trimmed.is_empty() {
                            tokens.push(Token::new(
                                trimmed.to_string(),
                                Span::new(token_start, i),
                            ));
                        }
                        current_token.clear();
                    }
                    tokens.push(Token::new(ch.to_string(), Span::new(i, i + 1)));
                    token_start = i + 1;
                    i += 1;
                }
            }
            ' ' => {
                if !current_token.is_empty() {
                    let trimmed = current_token.trim();
                    if !trimmed.is_empty() {
                        tokens.push(Token::new(
                            trimmed.to_string(),
                            Span::new(token_start, i),
                        ));
                    }
                    current_token.clear();
                }
                token_start = i + 1;
                i += 1;
            }
            _ => {
                if current_token.is_empty() {
                    token_start = i;
                }
                current_token.push(ch);
                i += 1;
            }
        }
    }

    if !current_token.is_empty() {
        let trimmed = current_token.trim();
        if !trimmed.is_empty() {
            tokens.push(Token::new(
                trimmed.to_string(),
                Span::new(token_start, chars.len()),
            ));
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_with_spans() {
        let tokens = tokenize_expression("A1 + B2");

        // Check we got the right tokens
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].value, "A1");
        assert_eq!(tokens[1].value, "+");
        assert_eq!(tokens[2].value, "B2");

        // Check spans are tracked correctly
        assert_eq!(tokens[0].span.start, 0);
        assert_eq!(tokens[0].span.end, 2);  // "A1"

        assert_eq!(tokens[1].span.start, 3);
        assert_eq!(tokens[1].span.end, 4);  // "+"

        assert_eq!(tokens[2].span.start, 5);
        assert_eq!(tokens[2].span.end, 7);  // "B2"
    }

    #[test]
    fn test_tokenize_with_multiple_operators() {
        let tokens = tokenize_expression("A1 + B2 * C3");

        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].value, "A1");
        assert_eq!(tokens[1].value, "+");
        assert_eq!(tokens[2].value, "B2");
        assert_eq!(tokens[3].value, "*");
        assert_eq!(tokens[4].value, "C3");

        // Check the multiply operator span
        assert_eq!(tokens[3].span.start, 8);
        assert_eq!(tokens[3].span.end, 9);
    }

    #[test]
    fn test_tokenize_decimal_number() {
        let tokens = tokenize_expression("3.14 + 2.5");

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].value, "3.14");
        assert_eq!(tokens[1].value, "+");
        assert_eq!(tokens[2].value, "2.5");

        // Check decimal spans
        assert_eq!(tokens[0].span.start, 0);
        assert_eq!(tokens[0].span.end, 4);  // "3.14"

        assert_eq!(tokens[2].span.start, 7);
        assert_eq!(tokens[2].span.end, 10); // "2.5"
    }
}
