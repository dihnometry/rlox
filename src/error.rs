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
}
