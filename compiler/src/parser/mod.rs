use crate::ast::*;
use crate::lexer::{Token, TokenWithPos};
use anyhow::{Context, Result};

pub struct Parser {
    tokens: Vec<TokenWithPos>,
    current: usize,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            current: 0,
        }
    }

    pub fn parse(&self, tokens: Vec<TokenWithPos>) -> Result<Program> {
        let mut parser = Self {
            tokens,
            current: 0,
        };

        let mut statements = Vec::new();

        while !parser.is_at_end() {
            parser.skip_newlines();
            if parser.is_at_end() {
                break;
            }
            statements.push(parser.parse_statement()?);
        }

        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement> {
        if self.check(&Token::Word("let".to_string())) {
            self.parse_let()
        } else if self.check(&Token::Word("func".to_string())) {
            self.parse_function()
        } else if self.check(&Token::Word("if".to_string())) {
            self.parse_if()
        } else if self.check(&Token::Word("for".to_string())) {
            self.parse_for()
        } else if self.check(&Token::Word("print".to_string())) {
            self.parse_print()
        } else if self.check(&Token::Word("return".to_string())) {
            self.parse_return()
        } else {
            self.parse_expression_statement()
        }
    }

    fn parse_let(&mut self) -> Result<Statement> {
        self.advance(); // consume 'let'
        self.skip_spaces();

        let name = self.expect_word()?;
        self.skip_spaces();

        let value = self.parse_expression()?;
        self.expect_newline_or_eof()?;

        Ok(Statement::Let { name, value })
    }

    fn parse_function(&mut self) -> Result<Statement> {
        self.advance(); // consume 'func'
        self.skip_spaces();

        let name = self.expect_word()?;
        self.skip_spaces();

        let mut params = Vec::new();
        while !self.check(&Token::Newline) && !self.check(&Token::Eof) {
            if !params.is_empty() {
                self.expect_space()?;
            }
            params.push(self.expect_word()?);
            self.skip_spaces();
        }

        self.expect_newline()?;

        let mut body = Vec::new();
        let mut indent_level = 0;

        // Check if next line starts with tab
        if self.check(&Token::Tab) {
            indent_level = 1;
            while self.check(&Token::Tab) {
                self.advance();
                let stmt = self.parse_statement()?;
                body.push(stmt);
                self.skip_newlines();
            }
        }

        Ok(Statement::Function {
            name,
            params,
            body,
        })
    }

    fn parse_if(&mut self) -> Result<Statement> {
        self.advance(); // consume 'if'
        self.skip_spaces();

        let condition = self.parse_expression()?;
        self.expect_newline()?;

        let mut then_body = Vec::new();
        if self.check(&Token::Tab) {
            while self.check(&Token::Tab) {
                self.advance();
                then_body.push(self.parse_statement()?);
                self.skip_newlines();
            }
        }

        let mut else_body = None;
        if self.check(&Token::Word("else".to_string())) {
            self.advance();
            self.expect_newline()?;
            if self.check(&Token::Tab) {
                let mut body = Vec::new();
                while self.check(&Token::Tab) {
                    self.advance();
                    body.push(self.parse_statement()?);
                    self.skip_newlines();
                }
                else_body = Some(body);
            }
        }

        Ok(Statement::If {
            condition,
            then_body,
            else_body,
        })
    }

    fn parse_for(&mut self) -> Result<Statement> {
        self.advance(); // consume 'for'
        self.skip_spaces();

        let var = self.expect_word()?;
        self.skip_spaces();

        if !self.check(&Token::Word("in".to_string())) {
            return Err(anyhow::anyhow!("Expected 'in' in for loop"));
        }
        self.advance();
        self.skip_spaces();

        let iterable = self.parse_expression()?;
        self.expect_newline()?;

        let mut body = Vec::new();
        if self.check(&Token::Tab) {
            while self.check(&Token::Tab) {
                self.advance();
                body.push(self.parse_statement()?);
                self.skip_newlines();
            }
        }

        Ok(Statement::For {
            var,
            iterable,
            body,
        })
    }

    fn parse_print(&mut self) -> Result<Statement> {
        self.advance(); // consume 'print'
        self.skip_spaces();

        let mut args = Vec::new();
        while !self.check(&Token::Newline) && !self.check(&Token::Eof) {
            if !args.is_empty() {
                self.expect_space()?;
            }
            args.push(self.parse_expression()?);
            self.skip_spaces();
        }
        self.expect_newline_or_eof()?;

        Ok(Statement::Print { args })
    }

    fn parse_return(&mut self) -> Result<Statement> {
        self.advance(); // consume 'return'
        self.skip_spaces();

        let value = if self.check(&Token::Newline) || self.check(&Token::Eof) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        self.expect_newline_or_eof()?;

        Ok(Statement::Return { value })
    }

    fn parse_expression_statement(&mut self) -> Result<Statement> {
        let expr = self.parse_expression()?;
        self.expect_newline_or_eof()?;
        Ok(Statement::Expression(expr))
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_binary(0)
    }

    fn parse_binary(&mut self, min_precedence: u8) -> Result<Expression> {
        let mut left = self.parse_unary()?;

        loop {
            let op = if self.check(&Token::Word("+".to_string())) {
                Some(BinaryOp::Add)
            } else if self.check(&Token::Word("-".to_string())) {
                Some(BinaryOp::Subtract)
            } else if self.check(&Token::Word("*".to_string())) {
                Some(BinaryOp::Multiply)
            } else if self.check(&Token::Word("/".to_string())) {
                Some(BinaryOp::Divide)
            } else if self.check(&Token::Word(">".to_string())) {
                Some(BinaryOp::Greater)
            } else if self.check(&Token::Word("<".to_string())) {
                Some(BinaryOp::Less)
            } else if self.check(&Token::Word("==".to_string())) {
                Some(BinaryOp::Equal)
            } else {
                None
            };

            if let Some(op) = op {
                let precedence = op.precedence();
                if precedence < min_precedence {
                    break;
                }
                self.advance();
                self.skip_spaces();
                let right = self.parse_binary(precedence + 1)?;
                left = Expression::Binary {
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

    fn parse_unary(&mut self) -> Result<Expression> {
        if self.check(&Token::Word("-".to_string())) {
            self.advance();
            self.skip_spaces();
            Ok(Expression::Unary {
                op: UnaryOp::Negate,
                expr: Box::new(self.parse_unary()?),
            })
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Result<Expression> {
        if self.check(&Token::Number(_)) {
            if let Token::Number(n) = self.advance().token.clone() {
                Ok(Expression::Number(n))
            } else {
                unreachable!()
            }
        } else if self.check(&Token::Float(_)) {
            if let Token::Float(n) = self.advance().token.clone() {
                Ok(Expression::Float(n))
            } else {
                unreachable!()
            }
        } else if self.check(&Token::String(_)) {
            if let Token::String(s) = self.advance().token.clone() {
                Ok(Expression::String(s))
            } else {
                unreachable!()
            }
        } else if self.check(&Token::Word(_)) {
            let name = if let Token::Word(w) = self.advance().token.clone() {
                w
            } else {
                unreachable!()
            };

            // Check if it's a function call
            if self.check(&Token::Space) {
                self.skip_spaces();
                let mut args = Vec::new();
                while !self.check(&Token::Newline)
                    && !self.check(&Token::Eof)
                    && !self.check(&Token::Tab)
                {
                    if !args.is_empty() {
                        self.expect_space()?;
                    }
                    args.push(self.parse_expression()?);
                    self.skip_spaces();
                }
                Ok(Expression::Call {
                    name,
                    args,
                })
            } else {
                Ok(Expression::Variable(name))
            }
        } else {
            Err(anyhow::anyhow!("Unexpected token in expression"))
        }
    }

    fn is_at_end(&self) -> bool {
        self.check(&Token::Eof)
    }

    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.tokens[self.current].token == token
        }
    }

    fn advance(&mut self) -> &TokenWithPos {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }

    fn skip_spaces(&mut self) {
        while self.check(&Token::Space) {
            self.advance();
        }
    }

    fn skip_newlines(&mut self) {
        while self.check(&Token::Newline) {
            self.advance();
        }
    }

    fn expect_space(&mut self) -> Result<()> {
        if self.check(&Token::Space) {
            self.advance();
            Ok(())
        } else {
            Err(anyhow::anyhow!("Expected space"))
        }
    }

    fn expect_newline(&mut self) -> Result<()> {
        if self.check(&Token::Newline) {
            self.advance();
            Ok(())
        } else {
            Err(anyhow::anyhow!("Expected newline"))
        }
    }

    fn expect_newline_or_eof(&mut self) -> Result<()> {
        if self.check(&Token::Newline) || self.check(&Token::Eof) {
            if self.check(&Token::Newline) {
                self.advance();
            }
            Ok(())
        } else {
            Err(anyhow::anyhow!("Expected newline or EOF"))
        }
    }

    fn expect_word(&mut self) -> Result<String> {
        if let Token::Word(w) = self.advance().token.clone() {
            Ok(w)
        } else {
            Err(anyhow::anyhow!("Expected word"))
        }
    }
}

