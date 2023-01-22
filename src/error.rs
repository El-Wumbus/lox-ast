use crate::tokens::*;

#[derive(Debug, PartialEq, Clone)]
pub struct LoxError
{
    token: Option<Token>,
    line: usize,
    message: String,
}

impl LoxError
{
    pub fn error(line: usize, message: &str) -> Self
    {
        Self {
            line,
            message: message.to_string(),
            token: None,
        }
    }

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
