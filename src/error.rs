#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LoxError {
    line: usize,
    message: String,
}

impl LoxError {
    pub fn error(line: usize, message: String) -> Self {
        Self { line, message }
    }

    pub fn report(&self, loc: String) {
        eprintln!("[line {}] Error{}: {}", self.line, loc, self.message);
    }
}
