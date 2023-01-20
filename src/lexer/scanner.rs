use crate::object::Object;
use crate::{error::LoxError, tokens::*};
use std::collections::HashMap;
pub struct Scanner
{
    source: Vec<char>,
    tokens: Vec<Token>,

    // Offsets that index into the string
    start: usize,
    current: usize,

    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl Scanner
{
    pub fn new(source: String) -> Self
    {
        let keywords = HashMap::from([
            ("and".to_string(), TokenType::And),
            ("class".to_string(), TokenType::Class),
            ("else".to_string(), TokenType::Else),
            ("false".to_string(), TokenType::False),
            ("for".to_string(), TokenType::For),
            ("fun".to_string(), TokenType::Fun),
            ("if".to_string(), TokenType::If),
            ("nil".to_string(), TokenType::Nil),
            ("or".to_string(), TokenType::Or),
            ("print".to_string(), TokenType::Print),
            ("return".to_string(), TokenType::Return),
            ("super".to_string(), TokenType::Super),
            ("this".to_string(), TokenType::This),
            ("true".to_string(), TokenType::True),
            ("var".to_string(), TokenType::Var),
            ("while".to_string(), TokenType::While),
        ]);
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError>
    {
        let mut had_error: Option<LoxError> = None;

        while !self.is_at_end()
        {
            self.start = self.current;
            match self.scan_token()
            {
                Ok(_) =>
                {}
                Err(x) =>
                {
                    x.report("".to_string());
                    had_error = Some(x);
                }
            };
        }

        self.tokens.push(Token::eof(self.line));

        // If an error occurs, return the error
        if let Some(e) = had_error
        {
            Err(e)
        }
        else
        {
            Ok(&self.tokens)
        }
    }

    fn scan_token(&mut self) -> Result<(), LoxError>
    {
        let c = self.advance();

        match c
        {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' =>
            {
                let tok = if self.is_match('=')
                {
                    TokenType::BangEqual
                }
                else
                {
                    TokenType::Bang
                };
                self.add_token(tok);
            }
            '=' =>
            {
                let tok = if self.is_match('=')
                {
                    TokenType::Equal
                }
                else
                {
                    TokenType::Assign
                };
                self.add_token(tok);
            }
            '<' =>
            {
                let tok = if self.is_match('=')
                {
                    TokenType::LessEqual
                }
                else
                {
                    TokenType::Less
                };
                self.add_token(tok);
            }
            '>' =>
            {
                let tok = if self.is_match('=')
                {
                    TokenType::GreaterEqual
                }
                else
                {
                    TokenType::Greater
                };
                self.add_token(tok);
            }
            '/' =>
            {
                if self.is_match('/')
                {
                    // A comment goes until the end of the line. So we ignore until then
                    while let Some(ch) = self.peek()
                    {
                        if ch != '\n'
                        {
                            self.advance();
                        }
                        else
                        {
                            break;
                        }
                    }
                }
                else if self.is_match('*')
                {
                    // Block comment start
                    self.scan_comment()?;
                }
                else
                {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' =>
            {
                // Ignore whitespace
            }
            '\n' => self.line += 1,
            '"' =>
            {
                self.string()?;
            }
            '0'..='9' =>
            {
                self.number();
            }

            _ if c.is_ascii_alphabetic() || c == '_' => self.identifier(),
            _ =>
            {
                return Err(LoxError::error(
                    self.line,
                    format!("Unexpected character '{c}'"),
                ))
            }
        }
        Ok(())
    }

    /// A recursive function that scans for block comments and supports nesting
    fn scan_comment(&mut self) -> Result<(), LoxError>
    {
        loop
        {
            match self.peek()
            {
                // The end of the comment
                Some('*') =>
                {
                    self.advance();
                    if self.is_match('/')
                    {
                        return Ok(());
                    }
                }

                // The beginning of a new, nested comment.
                Some('/') =>
                {
                    self.advance();
                    // We just opened another comment, so we call .scan_comment on that block
                    // This allows for nested block comments
                    if self.is_match('*')
                    {
                        self.scan_comment()?;
                    }
                }

                // Handle our newlines
                Some('\n') =>
                {
                    self.advance();
                    self.line += 1;
                }

                None =>
                {
                    return Err(LoxError::error(
                        self.line,
                        "Unterminated block comment.".to_string(),
                    ))
                }
                _ =>
                {
                    self.advance();
                }
            };
        }
    }

    fn identifier(&mut self)
    {
        while Scanner::is_alpha_numeric(self.peek())
        {
            self.advance();
        }
        let text: String = self.source[self.start..self.current].iter().collect();
        let ttype;

        // If
        if let Some(type_) = self.keywords.get(&text)
        {
            ttype = *type_;
        }
        else
        {
            ttype = TokenType::Identifier;
        }

        self.add_token(ttype);
    }

    fn is_alpha_numeric(ch: Option<char>) -> bool
    {
        if let Some(ch) = ch
        {
            ch.is_ascii_alphanumeric() || ch == '_'
        }
        else
        {
            false
        }
    }
    fn number(&mut self)
    {
        // Consume all integer digits
        while Scanner::is_digit(self.peek())
        {
            self.advance();
        }

        // If a fractional part is present, and the next char is a digit, consume them.
        if self.peek() == Some('.') && Scanner::is_digit(self.peek_next())
        {
            self.advance();

            while Scanner::is_digit(self.peek())
            {
                self.advance();
            }
        }

        // Collect the number as a string, then parse it.
        let value: String = self.source[self.start..self.current].iter().collect();
        self.add_token_object(TokenType::Number, Some(Object::Num(value.parse().unwrap())));
    }

    /// Peaks two chars ahead.
    fn peek_next(&self) -> Option<char> { self.source.get(self.current + 1).copied() }

    fn is_digit(ch: Option<char>) -> bool
    {
        if let Some(ch) = ch
        {
            ch.is_ascii_digit()
        }
        else
        {
            false
        }
    }

    fn string(&mut self) -> Result<(), LoxError>
    {
        // Consume chars until we find the ending quote
        while let Some(ch) = self.peek()
        {
            match ch
            {
                '"' => break,
                '\n' => self.line += 1,
                _ => (),
            };
            self.advance();
        }

        // If there is no ending quote then we complain
        if self.is_at_end()
        {
            return Err(LoxError::error(
                self.line,
                "Unterminated String".to_string(),
            ));
        }
        // Consume closing quote
        self.advance();

        // TODO: Handle escape sequences here //
        // Get the value excluding the quotes
        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token_object(TokenType::String, Some(Object::Str(value)));
        Ok(())
    }
    /// Like the `advance()` method, but doesn't consume chars.
    fn peek(&self) -> Option<char> { self.source.get(self.current).copied() }

    /// Consumes the next char in the source file, then returns it.
    fn advance(&mut self) -> char
    {
        let result = self.source.get(self.current).unwrap();
        self.current += 1;
        *result
    }

    /// Appends a token with no literal.
    fn add_token(&mut self, ttype: TokenType) { self.add_token_object(ttype, None); }

    /// Appends a token.
    fn add_token_object(&mut self, ttype: TokenType, literal: Option<Object>)
    {
        // Get a char slice from the source, then turn it to an iterator. After this, collect
        // into a string
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::new(ttype, lexeme, literal, self.line))
    }

    /// Returns true if we're at the end of the string.
    fn is_at_end(&self) -> bool { self.current >= self.source.len() }

    /// If the expected char matches the actual char it will return true. The char
    /// is consumed and we advance.
    fn is_match(&mut self, expected: char) -> bool
    {
        match self.source.get(self.current)
        {
            Some(ch) if *ch == expected =>
            {
                self.current += 1; // Increment
                true
            }
            _ => false,
        }
    }
}
