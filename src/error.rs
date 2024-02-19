use crate::{token::Token, token_type::TokenType};

#[derive(Debug)]
pub struct Lox {
    line: usize,
    message: String,
}

impl Lox {
    pub fn error(line: usize, message: String) -> Lox {
        Lox { line, message }
    }

    pub fn report(&self, loc: &str) {
        eprintln!(
            "[line {}] Error chars [{loc}] : {}",
            self.line, self.message
        );
    }

    pub fn parse_error(token: &Token, message: String) {
        if token.ttype == TokenType::Eof {
            Lox::report(&Lox { line: token.line, message }, "at end.");
        } else {
            Lox::report(&Lox { line: token.line, message }, &format!(" at '{}'", token.lexeme));
        }
    }

    pub fn runtime_error(token: &Token, message: &'static str) {
        eprintln!("{}\n[line {}]", message, token.line)
    }
}
