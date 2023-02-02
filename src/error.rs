use crate::tokens::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoxResult
{
    #[error("[line {}] ParseError: at '{}' {message}", token.line, token.lexeme)]
    ParseError
    {
        token: Token, message: String
    },

    #[error("[line {}] RuntimeError at '{}': {message}", token.line, token.lexeme)]
    RuntimeError
    {
        token: Token, message: String
    },

    #[error("[line {line}] Error: {message}")]
    LoxError
    {
        line: usize, message: String
    }, // Break

    #[error("")]
    Break,
}

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

impl LoxResult
{
    pub fn report(&self)
    {
        eprintln!("{self}");
    }

    /// Create a `LoxError`
    pub fn error(line: usize, message: &str) -> Self
    {
        let err = Self::LoxError {
            line,
            message: message.to_string(),
        };
        eprintln!("{err}");
        err
    }

    /// Create a `LoxError` at parsing time
    pub fn parse_error(token: &Token, message: &str) -> Self
    {
        let err = Self::ParseError {
            token: token.clone(),
            message: message.to_string(),
        };
        eprintln!("{err}");
        err
    }

    /// Create a `LoxError` at runtime
    pub fn new_runtime_error(token: Token, message: String) -> Self
    {
        let err = Self::RuntimeError {
            token,
            message,
        };
        eprintln!("{err}");
        err
    }
}
