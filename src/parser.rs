use crate::{token::{Token, Object}, expr::{Expr, self}, token_type::TokenType, error::Lox};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

pub struct LoxParseError;

impl<'a> Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        match self.expression() {
            Ok(expr) => Some(expr),
            Err(_) => None,
        }
    }

    fn expression(&mut self) -> Result<Expr, LoxParseError> {
        self.equality()
    }


    fn equality(&mut self) -> Result<Expr, LoxParseError> {
        let mut expr = self.comparison()?;

        while self.match_tokens(&[TokenType::BangEqual, TokenType::Equals]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(expr::BinaryExpr{ left: Box::new(expr), operator, right: Box::new(right) });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxParseError> {
        let mut expr = self.term()?;

        while self.match_tokens(&[TokenType::Less, TokenType::Greater, TokenType::LessEqual, TokenType::GreaterEqual]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(expr::BinaryExpr { left: Box::new(expr), operator, right: Box::new(right) });
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxParseError> {
        let mut expr = self.factor()?;

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(expr::BinaryExpr { left: Box::new(expr), operator, right: Box::new(right) });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxParseError> {
        let mut expr = self.power()?;

        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.power()?;
            expr = Expr::Binary(expr::BinaryExpr { left: Box::new(expr), operator, right: Box::new(right) });
        }

        Ok(expr)
    }

    fn power(&mut self) -> Result<Expr, LoxParseError> {
        let mut expr = self.unary()?;

        while self.match_tokens(&[TokenType::Exponent]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(expr::BinaryExpr{ left: Box::new(expr), operator, right: Box::new(right) });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxParseError> {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?; 
            return Ok(Expr::Unary(expr::UnaryExpr { operator, right: Box::new(right) }));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, LoxParseError> {
        if self.match_tokens(&[TokenType::False]) { 
            return Ok(Expr::Literal(expr::LiteralExpr { value: Object::False }));
        }

        if self.match_tokens(&[TokenType::True]) { 
            return Ok(Expr::Literal(expr::LiteralExpr { value: Object::True }));
        }

        if self.match_tokens(&[TokenType::Nil]) { 
            return Ok(Expr::Literal(expr::LiteralExpr { value: Object::Nil }));
        }

        if self.match_tokens(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(expr::LiteralExpr { value: self.previous().literal.clone().unwrap() }));
        }

        if self.match_tokens(&[TokenType::LeftParen]) {
            let expr = match self.expression() {
                Ok(expr) => expr,
                Err(err) => return Err(err),
            };
            self.consume(&TokenType::RightParen, String::from("Expect '(' after expression."))?;
            return Ok(Expr::Grouping(expr::GroupingExpr { expr: Box::new(expr) }));
        }
        
        Err(Parser::error(self.peek(), "Expect expression.".to_string()))
    }

    fn consume(&'a mut self, ttype: &'a TokenType, error_msg: String) -> Result<&Token, LoxParseError> {
        if self.check(ttype) { return Ok(self.advance()); }

        Err(Parser::error(self.peek(), error_msg))
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        for ty in types {
            if self.check(ty) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&mut self, ty: &TokenType) -> bool {
        if self.is_at_end() { return false }

        self.peek().ttype == *ty
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() { self.current += 1 }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().ttype == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).expect("Couldn't get current token.")
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).expect("Couldn't get previous token.")
    }

    fn error(token: &Token, message: String) -> LoxParseError {
        Lox::parse_error(token, message);
        LoxParseError
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().ttype == TokenType::SemiColon { return; }

            match self.peek().ttype {
                TokenType::If 
                | TokenType::Class 
                | TokenType::Var
                | TokenType::Fun
                | TokenType::For
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => self.advance(),
            };
        }
    }
}
