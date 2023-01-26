use crate::tokens::*;

#[derive(Debug, PartialEq, Clone)]
/// Contains information used for error reporting
pub struct LoxError
{
    /// The token that relates to the error
    token: Option<Token>,

    /// The line the error occurred on
    line: usize,

    /// The error message
    message: String,
}

impl LoxError
{
    /// Create a `LoxError`
    pub fn error(line: usize, message: &str) -> Self
    {
        let e = Self {
            line,
            message: message.to_string(),
            token: None,
        };
        e.report("");
        e
    }

    /// Create a `LoxError` at parsing time
    pub fn parse_error(token: &Token, message: &str) -> LoxError
    {
        let err = LoxError {
            token: Some(token.clone()),
            line: token.line,
            message: message.to_string(),
        };
        err.report("");
        err
    }

    /// Create a `LoxError` at runtime
    pub fn runtime_error(token: &Token, message: &str) -> LoxError
    {
        let err = LoxError {
            token: Some(token.clone()),
            line: token.line,
            message: message.to_string(),
        };
        err.report("");
        err
    }

    /// Print the error
    pub fn report(&self, loc: &str)
    {
        if let Some(token) = self.token.clone()
        {
            if token.is(TokenType::Eof)
            {
                eprintln!("Error: {} at end {}", token.line, self.message);
            }
            else
            {
                eprintln!(
                    "Error: {} at '{}' {}",
                    token.line, token.lexeme, self.message
                )
            }
        }
        else
        {
            eprintln!("[line {}] Error{}: {}", self.line, loc, self.message);
        }
    }
}
