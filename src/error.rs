#[derive(Debug)]
pub struct LoxError {
    line: usize,
    message: String,
}

impl LoxError {
    pub fn error(line: usize, message: String) -> LoxError {
        LoxError { line, message }
    }

    pub fn report(&self, loc: &str) {
        eprintln!(
            "[line {}] Error chars [{loc}] : {}",
            self.line, self.message
        );
    }
}
