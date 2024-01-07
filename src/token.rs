use crate::token_type::TokenType;

#[derive(Debug)]
pub enum Object {
    Num(f64),
    Str(String),
    Boolean(bool),
    Nil,
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Object::Num(num) => num.to_string(),
            Object::Str(s) => s.clone(),
            Object::Boolean(b) => b.to_string(),
            Object::Nil => String::from("nil"),
        };
        write!(f, "{str}")
    }
}

#[derive(Debug)]
pub struct Token {
    ttype: TokenType,
    pub lexeme: String,
    pub literal: Option<Object>,
    line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Option<Object>, line: usize) -> Self {
        Self {
            ttype,
            lexeme,
            literal,
            line,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ Token: {:?}   Lexeme: {}   Literal: {} }}",
            self.ttype,
            self.lexeme,
            if let Some(literal) = &self.literal {
                literal.to_string()
            } else {
                String::from("None")
            }
        )
    }
}
