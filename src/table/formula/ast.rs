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

    /// Function call (e.g., sum(A_))
    FunctionCall {
        name: String,
        arg: Box<Expr>,
        span: Span,
    },
}

impl Expr {
    /// Get the span of this expression
    pub(crate) fn span(&self) -> Span {
        match self {
            Expr::Literal(_, s) => *s,
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
            let arg = self.parse_expression()?;
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
                arg: Box::new(arg),
                span,
            });
        }

        // Check for cell reference
        if let Some(cell_ref) = parse_cell_reference(&token.value) {
            let span = token.span;
            self.pos += 1;
            return Ok(Expr::CellRef(cell_ref, span));
        }

        // Check for number literal
        if let Ok(decimal) = Decimal::from_str(&token.value) {
            let span = token.span;
            self.pos += 1;
            return Ok(Expr::Literal(decimal, span));
        }

        Err(FormulaError::RuntimeError(
            format!("invalid token: '{}' is not a valid number or cell reference", token.value)
        ))
    }
}
