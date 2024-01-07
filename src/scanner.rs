use crate::{
    error::LoxError,
    token::{Object, Token},
    token_type::TokenType,
};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), None, self.line));
        self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::SemiColon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => {
                let is_next = self.is_next('=');
                self.add_token(
                    if is_next {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    },
                    None,
                );
            }
            '=' => {
                let is_next = self.is_next('=');
                self.add_token(
                    if is_next {
                        TokenType::Equals
                    } else {
                        TokenType::Assign
                    },
                    None,
                );
            }
            '<' => {
                let is_next = self.is_next('=');
                self.add_token(
                    if is_next {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    },
                    None,
                );
            }
            '>' => {
                let is_next = self.is_next('=');
                self.add_token(
                    if is_next {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    },
                    None,
                );
            }
            '/' => {
                if self.is_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            n if Scanner::is_numeric(n) => self.number(),
            n if Scanner::is_alpha(n) => self.identifier(),
            _ => {
                let error = LoxError::error(self.line, String::from("Unexpected character."));
                error.report(&self.current.to_string());
            }
        }
    }

    fn is_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self
            .source
            .chars()
            .nth(self.current)
            .expect("Could not see next character.")
            != expected
        {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source
            .chars()
            .nth(self.current)
            .expect("Could not peek character.")
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source
            .chars()
            .nth(self.current + 1)
            .expect("Could not see next character.")
    }

    fn is_alpha(c: char) -> bool {
        matches!(c, 'a'..='z' | 'A'..='Z' | '_')
    }

    fn is_numeric(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_alpha_numeric(c: char) -> bool {
        Scanner::is_alpha(c) || Scanner::is_numeric(c)
    }

    fn advance(&mut self) -> char {
        let ch = self
            .source
            .chars()
            .nth(self.current)
            .expect("Error: Scanner advance: index out of bounds");
        self.current += 1;
        ch
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            let error = LoxError::error(self.line - 1, String::from("Unterminated string."));
            error.report(&self.current.to_string());
            return;
        }

        self.advance();

        let value = self.source[(self.start + 1)..(self.current - 1)].to_string();
        self.add_token(TokenType::String, Some(Object::Str(value)));
    }

    fn add_token(&mut self, ttype: TokenType, literal: Option<Object>) {
        let text = &self.source.get(self.start..self.current);
        if let Some(text) = text {
            self.tokens
                .push(Token::new(ttype, (*text).to_string(), literal, self.line));
        } else {
            self.current = self.start;
        }
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for fractional part
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the .
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let number: f64 = if let Some(text) = self.source.get(self.start..self.current) {
            text.parse().expect("Could not parse.")
        } else {
            self.current = self.start;
            return;
        };

        self.add_token(TokenType::Number, Some(Object::Num(number)));
    }

    fn identifier(&mut self) {
        while Scanner::is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = self.source[self.start..self.current].to_string();
        let ttype = if let Some(token) = Scanner::keyword(&text) {
            token
        } else {
            TokenType::Ident
        };
        self.add_token(ttype, None);
    }

    fn keyword(word: &str) -> Option<TokenType> {
        match word {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }
}
