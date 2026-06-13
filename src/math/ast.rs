use anyhow::{bail, Result};

use super::token::Token;

// ============================================================================
// AST nodes
// ============================================================================
#[derive(Clone)]
pub(super) enum Expr {
    Num(f64),
    Str(String),
    Ident(String),
    Binary(Box<Expr>, char, Box<Expr>),
    Unary(Box<Expr>),
    Call(String, Vec<Expr>),
    Assign(String, Box<Expr>),
    Return(Box<Expr>),
}

// ============================================================================
// Parser
// ============================================================================
pub(super) struct Parser {
    tokens: Vec<Token>,
    positions: Vec<usize>,
    pos: usize,
}

impl Parser {
    pub(super) fn new(tokens: Vec<Token>) -> Self {
        let len = tokens.len();
        Self { tokens, positions: vec![0; len], pos: 0 }
    }

    pub(super) fn with_positions(tokens: Vec<Token>, positions: Vec<usize>) -> Self {
        Self { tokens, positions, pos: 0 }
    }

    pub(super) fn peek(&self) -> &Token { &self.tokens[self.pos] }
    pub(super) fn advance(&mut self) -> Token { let t = self.tokens[self.pos].clone(); self.pos += 1; t }
    pub(super) fn current_offset(&self) -> usize { self.positions.get(self.pos).copied().unwrap_or(0) }

    fn expect(&mut self, tok: &Token) -> Result<()> {
        let actual = self.advance();
        if actual != *tok {
            bail!("Expected {:?}, got {:?}", tok, actual);
        }
        Ok(())
    }

    pub(super) fn parse_statement(&mut self) -> Result<Expr> {
        match self.peek() {
            Token::ReturnTok => {
                self.advance();
                let expr = self.parse_expr()?;
                Ok(Expr::Return(Box::new(expr)))
            }
            Token::Ident(_) => {
                let saved = self.pos;
                let name = if let Token::Ident(s) = self.advance() { s } else { unreachable!() };
                if matches!(self.peek(), Token::Equal) {
                    self.advance();
                    let val = self.parse_expr()?;
                    Ok(Expr::Assign(name, Box::new(val)))
                } else {
                    self.pos = saved;
                    self.parse_expr()
                }
            }
            _ => self.parse_expr(),
        }
    }

    pub(super) fn parse_expr(&mut self) -> Result<Expr> {
        let mut left = self.parse_term()?;
        while matches!(self.peek(), Token::Op('+') | Token::Op('-')) {
            let op = if let Token::Op(c) = self.advance() { c } else { unreachable!() };
            let right = self.parse_term()?;
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr> {
        let mut left = self.parse_unary()?;
        while matches!(self.peek(), Token::Op('*') | Token::Op('/')) {
            let op = if let Token::Op(c) = self.advance() { c } else { unreachable!() };
            let right = self.parse_unary()?;
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr> {
        if matches!(self.peek(), Token::Op('-') | Token::Op('+')) {
            let op = if let Token::Op(c) = self.advance() { c } else { unreachable!() };
            let expr = self.parse_power()?;
            if op == '-' { return Ok(Expr::Unary(Box::new(expr))); }
            return Ok(expr);
        }
        self.parse_power()
    }

    fn parse_power(&mut self) -> Result<Expr> {
        let mut base = self.parse_call()?;
        if matches!(self.peek(), Token::Op('^')) {
            self.advance();
            let exp = self.parse_unary()?;
            base = Expr::Binary(Box::new(base), '^', Box::new(exp));
        }
        Ok(base)
    }

    fn parse_call(&mut self) -> Result<Expr> {
        let tok = self.peek().clone();
        match tok {
            Token::Ident(name) => {
                self.advance();
                match self.peek() {
                    Token::LParen => {
                        self.advance();
                        let mut args = Vec::new();
                        if !matches!(self.peek(), Token::RParen) {
                            args.push(self.parse_expr()?);
                            while matches!(self.peek(), Token::Comma) {
                                self.advance();
                                args.push(self.parse_expr()?);
                            }
                        }
                        self.expect(&Token::RParen)?;
                        Ok(Expr::Call(name, args))
                    }
                    Token::Equal => {
                        self.advance();
                        let val = self.parse_expr()?;
                        Ok(Expr::Assign(name, Box::new(val)))
                    }
                    _ => Ok(Expr::Ident(name)),
                }
            }
            Token::Num(val) => { self.advance(); Ok(Expr::Num(val)) }
            Token::Str(s) => { self.advance(); Ok(Expr::Str(s)) }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(expr)
            }
            _ => bail!("Unexpected token: {:?}", self.peek()),
        }
    }
}
