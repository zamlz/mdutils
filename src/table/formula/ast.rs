use crate::table::error::FormulaError;
use crate::table::formula::types::CellReference;
use crate::table::formula::reference::parse_cell_reference;
use rust_decimal::Decimal;
use std::str::FromStr;

/// Abstract Syntax Tree node representing an expression
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Expr {
    /// A literal decimal number
    Literal(Decimal),

    /// A cell reference (scalar, column vector, or row vector)
    CellRef(CellReference),

    /// Binary operation (e.g., A + B, A * B, A @ B)
    BinaryOp {
        left: Box<Expr>,
        op: BinaryOperator,
        right: Box<Expr>,
    },

    /// Unary transpose operation (e.g., A_.T)
    Transpose(Box<Expr>),

    /// Function call (e.g., sum(A_))
    FunctionCall {
        name: String,
        arg: Box<Expr>,
    },
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
    tokens: Vec<String>,
    pos: usize,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<String>) -> Self {
        Parser { tokens, pos: 0 }
    }

    /// Parse the entire expression
    pub(crate) fn parse(&mut self) -> Result<Expr, FormulaError> {
        let expr = self.parse_expression()?;
        if self.pos < self.tokens.len() {
            return Err(FormulaError::UnexpectedToken {
                token: self.tokens[self.pos].clone(),
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
            if token == "+" || token == "-" {
                let op = BinaryOperator::from_token(token).unwrap();
                self.pos += 1;
                let right = self.parse_term()?;
                left = Expr::BinaryOp {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
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
            if token == "*" || token == "/" || token == "@" {
                let op = BinaryOperator::from_token(token).unwrap();
                self.pos += 1;
                let right = self.parse_factor()?;
                left = Expr::BinaryOp {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
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

        if self.pos < self.tokens.len() && self.tokens[self.pos] == "^" {
            self.pos += 1;
            let right = self.parse_factor()?; // Right-associative recursion
            left = Expr::BinaryOp {
                left: Box::new(left),
                op: BinaryOperator::Pow,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse unary: primary ('.T')?
    fn parse_unary(&mut self) -> Result<Expr, FormulaError> {
        let mut expr = self.parse_primary()?;

        // Check for transpose operator .T
        if self.pos + 1 < self.tokens.len()
            && self.tokens[self.pos] == "."
            && self.tokens[self.pos + 1] == "T" {
            self.pos += 2;
            expr = Expr::Transpose(Box::new(expr));
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
        if token == "(" {
            self.pos += 1;
            let expr = self.parse_expression()?;
            if self.pos >= self.tokens.len() || self.tokens[self.pos] != ")" {
                return Err(FormulaError::RuntimeError(
                    "unmatched opening parenthesis '(' - missing closing ')'".to_string()
                ));
            }
            self.pos += 1;
            return Ok(expr);
        }

        // Check for function call
        if self.pos + 1 < self.tokens.len() && self.tokens[self.pos + 1] == "(" {
            let func_name = token.clone();
            self.pos += 2; // Skip function name and '('
            let arg = self.parse_expression()?;
            if self.pos >= self.tokens.len() || self.tokens[self.pos] != ")" {
                return Err(FormulaError::RuntimeError(
                    format!("unmatched '(' in function call '{}'", func_name)
                ));
            }
            self.pos += 1; // Skip ')'
            return Ok(Expr::FunctionCall {
                name: func_name,
                arg: Box::new(arg),
            });
        }

        // Check for cell reference
        if let Some(cell_ref) = parse_cell_reference(&token) {
            self.pos += 1;
            return Ok(Expr::CellRef(cell_ref));
        }

        // Check for number literal
        if let Ok(decimal) = Decimal::from_str(&token) {
            self.pos += 1;
            return Ok(Expr::Literal(decimal));
        }

        Err(FormulaError::RuntimeError(
            format!("invalid token: '{}' is not a valid number or cell reference", token)
        ))
    }
}
