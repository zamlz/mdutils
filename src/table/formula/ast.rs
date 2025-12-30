use crate::table::error::FormulaError;
use crate::table::formula::types::{CellReference, Span};
use crate::table::formula::reference::parse_cell_reference;
use rust_decimal::Decimal;
use std::str::FromStr;

/// Abstract Syntax Tree node representing an expression
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Expr {
    /// A literal decimal number
    Literal(Decimal, Span),

    /// A string literal (e.g., "table_name")
    String(String, Span),

    /// A cell reference (scalar, column vector, or row vector)
    CellRef(CellReference, Span),

    /// Binary operation (e.g., A + B, A * B, A @ B)
    BinaryOp {
        left: Box<Expr>,
        op: BinaryOperator,
        right: Box<Expr>,
        span: Span,
    },

    /// Unary transpose operation (e.g., A_.T)
    Transpose(Box<Expr>, Span),

    /// Function call (e.g., sum(A_), from("sales", A1:C3))
    FunctionCall {
        name: String,
        args: Vec<Expr>,
        span: Span,
    },
}

impl Expr {
    /// Get the span of this expression
    pub(crate) fn span(&self) -> Span {
        match self {
            Expr::Literal(_, s) => *s,
            Expr::String(_, s) => *s,
            Expr::CellRef(_, s) => *s,
            Expr::BinaryOp { span, .. } => *span,
            Expr::Transpose(_, s) => *s,
            Expr::FunctionCall { span, .. } => *span,
        }
    }
}

/// Binary operators supported in expressions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BinaryOperator {
    Add,     // +
    Sub,     // -
    Mul,     // *
    Div,     // /
    Pow,     // ^
    MatMul,  // @
}

impl BinaryOperator {
    /// Parse operator from token string
    pub(crate) fn from_token(token: &str) -> Option<Self> {
        match token {
            "+" => Some(BinaryOperator::Add),
            "-" => Some(BinaryOperator::Sub),
            "*" => Some(BinaryOperator::Mul),
            "/" => Some(BinaryOperator::Div),
            "^" => Some(BinaryOperator::Pow),
            "@" => Some(BinaryOperator::MatMul),
            _ => None,
        }
    }
}

/// Recursive descent parser for converting tokens to AST
pub(crate) struct Parser {
    tokens: Vec<crate::table::formula::tokenizer::Token>,
    pos: usize,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<crate::table::formula::tokenizer::Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    /// Parse the entire expression
    pub(crate) fn parse(&mut self) -> Result<Expr, FormulaError> {
        let expr = self.parse_expression()?;
        if self.pos < self.tokens.len() {
            return Err(FormulaError::UnexpectedToken {
                token: self.tokens[self.pos].value.clone(),
                position: self.pos,
            });
        }
        Ok(expr)
    }

    /// Parse expression: term (('+' | '-') term)*
    fn parse_expression(&mut self) -> Result<Expr, FormulaError> {
        let mut left = self.parse_term()?;

        while self.pos < self.tokens.len() {
            let token = &self.tokens[self.pos];
            if token.value == "+" || token.value == "-" {
                let op = BinaryOperator::from_token(&token.value).unwrap();
                self.pos += 1;
                let right = self.parse_term()?;
                let span = left.span().merge(&right.span());
                left = Expr::BinaryOp {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                    span,
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Parse term: factor (('*' | '/' | '@') factor)*
    fn parse_term(&mut self) -> Result<Expr, FormulaError> {
        let mut left = self.parse_factor()?;

        while self.pos < self.tokens.len() {
            let token = &self.tokens[self.pos];
            if token.value == "*" || token.value == "/" || token.value == "@" {
                let op = BinaryOperator::from_token(&token.value).unwrap();
                self.pos += 1;
                let right = self.parse_factor()?;
                let span = left.span().merge(&right.span());
                left = Expr::BinaryOp {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                    span,
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Parse factor: unary ('^' unary)*
    /// Right-associative for exponentiation
    fn parse_factor(&mut self) -> Result<Expr, FormulaError> {
        let mut left = self.parse_unary()?;

        if self.pos < self.tokens.len() && self.tokens[self.pos].value == "^" {
            self.pos += 1;
            let right = self.parse_factor()?; // Right-associative recursion
            let span = left.span().merge(&right.span());
            left = Expr::BinaryOp {
                left: Box::new(left),
                op: BinaryOperator::Pow,
                right: Box::new(right),
                span,
            };
        }

        Ok(left)
    }

    /// Parse unary: primary ('.T')?
    fn parse_unary(&mut self) -> Result<Expr, FormulaError> {
        let mut expr = self.parse_primary()?;

        // Check for transpose operator .T
        if self.pos + 1 < self.tokens.len()
            && self.tokens[self.pos].value == "."
            && self.tokens[self.pos + 1].value == "T" {
            let t_span = self.tokens[self.pos + 1].span;
            self.pos += 2;
            let span = expr.span().merge(&t_span);
            expr = Expr::Transpose(Box::new(expr), span);
        }

        Ok(expr)
    }

    /// Parse primary: NUMBER | CELLREF | FUNCTION '(' expression ')' | '(' expression ')'
    fn parse_primary(&mut self) -> Result<Expr, FormulaError> {
        if self.pos >= self.tokens.len() {
            return Err(FormulaError::EmptyExpression);
        }

        let token = self.tokens[self.pos].clone();

        // Check for parentheses
        if token.value == "(" {
            self.pos += 1;
            let expr = self.parse_expression()?;
            if self.pos >= self.tokens.len() || self.tokens[self.pos].value != ")" {
                return Err(FormulaError::RuntimeError(
                    "unmatched opening parenthesis '(' - missing closing ')'".to_string()
                ));
            }
            self.pos += 1;
            return Ok(expr);
        }

        // Check for function call
        if self.pos + 1 < self.tokens.len() && self.tokens[self.pos + 1].value == "(" {
            let func_name = token.value.clone();
            let func_span = token.span;
            self.pos += 2; // Skip function name and '('

            // Parse function arguments (comma-separated)
            let mut args = Vec::new();

            // Check for empty argument list
            if self.pos < self.tokens.len() && self.tokens[self.pos].value != ")" {
                args.push(self.parse_expression()?);

                // Parse additional arguments separated by commas
                while self.pos < self.tokens.len() && self.tokens[self.pos].value == "," {
                    self.pos += 1; // Skip ','
                    args.push(self.parse_expression()?);
                }
            }

            if self.pos >= self.tokens.len() || self.tokens[self.pos].value != ")" {
                return Err(FormulaError::RuntimeError(
                    format!("unmatched '(' in function call '{}'", func_name)
                ));
            }
            let close_span = self.tokens[self.pos].span;
            self.pos += 1; // Skip ')'
            let span = func_span.merge(&close_span);
            return Ok(Expr::FunctionCall {
                name: func_name,
                args,
                span,
            });
        }

        // Check for cell reference (potentially a range)
        if let Some(cell_ref) = parse_cell_reference(&token.value) {
            let mut span = token.span;
            self.pos += 1;

            // Check if this is a range (A1:C5, A_:C_, _1:_5)
            if self.pos < self.tokens.len() && self.tokens[self.pos].value == ":" {
                self.pos += 1; // Skip ':'

                // Parse the end of the range
                if self.pos >= self.tokens.len() {
                    return Err(FormulaError::RuntimeError(
                        "expected cell reference after ':' in range".to_string()
                    ));
                }

                let end_token = self.tokens[self.pos].clone();
                if let Some(end_ref) = parse_cell_reference(&end_token.value) {
                    span = span.merge(&end_token.span);
                    self.pos += 1;

                    // Handle different range types based on start and end reference types
                    match (&cell_ref, &end_ref) {
                        // Scalar range: A1:C5
                        (CellReference::Scalar { row: start_row, col: start_col },
                         CellReference::Scalar { row: end_row, col: end_col }) => {
                            // Validate that start is before or equal to end
                            if start_row > end_row || start_col > end_col {
                                return Err(FormulaError::RuntimeError(
                                    "invalid range: start cell must be before or equal to end cell".to_string()
                                ));
                            }

                            let range_ref = CellReference::Range {
                                start_row: *start_row,
                                start_col: *start_col,
                                end_row: *end_row,
                                end_col: *end_col
                            };
                            return Ok(Expr::CellRef(range_ref, span));
                        }

                        // Column range: A_:C_
                        (CellReference::ColumnVector { col: start_col },
                         CellReference::ColumnVector { col: end_col }) => {
                            if start_col > end_col {
                                return Err(FormulaError::RuntimeError(
                                    "invalid column range: start column must be before or equal to end column".to_string()
                                ));
                            }

                            let range_ref = CellReference::ColumnRange {
                                start_col: *start_col,
                                end_col: *end_col
                            };
                            return Ok(Expr::CellRef(range_ref, span));
                        }

                        // Row range: _1:_5
                        (CellReference::RowVector { row: start_row },
                         CellReference::RowVector { row: end_row }) => {
                            if start_row > end_row {
                                return Err(FormulaError::RuntimeError(
                                    "invalid row range: start row must be before or equal to end row".to_string()
                                ));
                            }

                            let range_ref = CellReference::RowRange {
                                start_row: *start_row,
                                end_row: *end_row
                            };
                            return Ok(Expr::CellRef(range_ref, span));
                        }

                        // Mixed types are invalid
                        _ => {
                            return Err(FormulaError::RuntimeError(
                                "invalid range: cannot mix different reference types (e.g., A_:_5 or A1:B_ are not allowed)".to_string()
                            ));
                        }
                    }
                } else {
                    return Err(FormulaError::RuntimeError(
                        format!("invalid range end: '{}' is not a valid cell reference", end_token.value)
                    ));
                }
            }

            return Ok(Expr::CellRef(cell_ref, span));
        }

        // Check for string literal
        if token.value.starts_with('"') && token.value.ends_with('"') {
            let span = token.span;
            // Remove surrounding quotes
            let string_content = token.value[1..token.value.len()-1].to_string();
            self.pos += 1;
            return Ok(Expr::String(string_content, span));
        }

        // Check for number literal
        if let Ok(decimal) = Decimal::from_str(&token.value) {
            let span = token.span;
            self.pos += 1;
            return Ok(Expr::Literal(decimal, span));
        }

        Err(FormulaError::RuntimeError(
            format!("invalid token: '{}' is not a valid number, string, or cell reference", token.value)
        ))
    }
}
