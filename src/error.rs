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
    pub fn error(line: usize, message: String) -> Self
    {
        Self {
            line,
            message,
            token: None,
        }
    }

    pub fn parse_error(token: &Token, message: String) -> LoxError
    {
        let err = LoxError {
            token: Some(token.clone()),
            line: token.line,
            message,
        };
        err.report("".to_string());
        err
    }

    pub fn report(&self, loc: String)
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
