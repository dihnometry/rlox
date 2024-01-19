use crate::{
    error::Lox,
    token::{Object, Token},
    token_type::TokenType,
};

pub struct Scanner<'a> {
    source: &'a [u8],
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a [u8]) -> Self {
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
            b'(' => self.add_token(TokenType::LeftParen, None),
            b')' => self.add_token(TokenType::RightParen, None),
            b'{' => self.add_token(TokenType::LeftBrace, None),
            b'}' => self.add_token(TokenType::RightBrace, None),
            b',' => self.add_token(TokenType::Comma, None),
            b'.' => self.add_token(TokenType::Dot, None),
            b'-' => self.add_token(TokenType::Minus, None),
            b'+' => self.add_token(TokenType::Plus, None),
            b';' => self.add_token(TokenType::SemiColon, None),
            b'*' => self.add_token(TokenType::Star, None),
            b'^' => self.add_token(TokenType::Exponent, None),
            b'!' => {
                let is_next = self.is_next(b'=');
                self.add_token(
                    if is_next {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    },
                    None,
                );
            }
            b'=' => {
                let is_next = self.is_next(b'=');
                self.add_token(
                    if is_next {
                        TokenType::Equals
                    } else {
                        TokenType::Assign
                    },
                    None,
                );
            }
            b'<' => {
                let is_next = self.is_next(b'=');
                self.add_token(
                    if is_next {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    },
                    None,
                );
            }
            b'>' => {
                let is_next = self.is_next(b'=');
                self.add_token(
                    if is_next {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    },
                    None,
                );
            }
            b'/' => {
                if self.is_next(b'/') {
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.is_next(b'*') {
                    loop {
                        if self.peek() == b'*' && self.peek_next() == b'/' {
                            self.advance();
                            self.advance();
                            break;
                        }
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            b' ' | b'\r' | b'\t' => {}
            b'\n' => self.line += 1,
            b'"' => self.string(),
            n if Scanner::is_numeric(n) => self.number(),
            n if Scanner::is_alpha(n) => self.identifier(),
            _ => {
                let error = Lox::error(self.line, String::from("Unexpected character."));
                error.report(&self.current.to_string());
            }
        }
    }

    fn is_next(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            return false;
        }
        if *self
            .source
            .get(self.current)
            .expect("Could not see next character.")
            != expected
        {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            return b'\0';
        }
        *self.source
            .get(self.current)
            .expect("Could not peek character.")
    }

    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() {
            return b'\0';
        }
        *self.source
            .get(self.current + 1)
            .expect("Could not see next character.")
    }

    fn is_alpha(c: u8) -> bool {
        matches!(c, b'a'..=b'z' | b'A'..=b'Z' | b'_')
    }

    fn is_numeric(c: u8) -> bool {
        c.is_ascii_digit()
    }

    fn is_alpha_numeric(c: u8) -> bool {
        Scanner::is_alpha(c) || Scanner::is_numeric(c)
    }

    fn advance(&mut self) -> u8 {
        let ch = self
            .source
            .get(self.current)
            .expect("Error: Scanner advance: index out of bounds");
        self.current += 1;
        *ch
    }

    fn string(&mut self) {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            let error = Lox::error(self.line - 1, String::from("Unterminated string."));
            error.report(&self.current.to_string());
            return;
        }

        self.advance();

        let byte_slice = &self.source[(self.start + 1)..(self.current - 1)];
        let value_str = std::str::from_utf8(byte_slice).expect("Couldn't read string.");
        self.add_token(TokenType::String, Some(Object::Str(value_str.to_string())));
    }

    fn add_token(&mut self, ttype: TokenType, literal: Option<Object>) {
        let text = self.source.get(self.start..self.current);
        if let Some(text) = text {
            let text_str = std::str::from_utf8(text).unwrap();
            self.tokens
                .push(Token::new(ttype, text_str.to_string(), literal, self.line));
        } else {
            self.current = self.start;
        }
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for fractional part
        if self.peek() == b'.' && self.peek_next().is_ascii_digit() {
            // Consume the .
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let number: f64 = if let Some(text) = self.source.get(self.start..self.current) {
            let text_str = std::str::from_utf8(text).expect("Could not build number from source. [Will be fixed]");
            text_str.parse().expect("Could not parse.")
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

        let text: &str = if let Some(text) = self.source.get(self.start..self.current) {
            std::str::from_utf8(text).expect("Could not build identifier from source.")
        } else {
            panic!("Hola :D")
        };
        
        let ttype = if let Some(token) = Scanner::keyword(text) {
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
