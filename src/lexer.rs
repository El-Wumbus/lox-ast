use crate::tokens::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Lexer
{
    /// An array of chars that contain the program text
    program_text: Vec<char>,

    /// The position in the program text. Used as an index
    here: usize,
    next: usize,

    ch: char,
}

impl Iterator for Lexer
{
    type Item = Token;

    fn next(&mut self) -> Option<Token>
    {
        if self.next > self.program_text.len()
        {
            return None;
        }

        self.skip_whitespace();

        let t: Token = match self.ch
        {
            '=' =>
            {
                self.advance();
                Token::new(TokenType::Assign, "=".to_string())
            }
            _ if self.ch.is_alphabetic() =>
            {
                let mut buffer = String::new();
                buffer.push(self.ch);

                self.advance();

                while self.here < self.program_text.len() && self.ch.is_alphabetic()
                {
                    buffer.push(self.ch);
                    self.advance();
                }

                let kind = match buffer.as_str()
                {
                    "let" => RESERVED_KEYWORDS.get("let").unwrap().clone(),
                    _ => TokenType::Identifier,
                };

                Token::new(kind, buffer)
            }

            _ if self.ch.is_numeric() =>
            {
                let mut buffer = String::new();
                buffer.push(self.ch);

                self.advance();

                loop
                {
                    if self.here > self.program_text.len()
                    {
                        break;
                    }

                    // Ignore number separators
                    if self.ch == '_'
                    {
                        self.advance();
                    }

                    if !self.ch.is_numeric() && self.ch != '.'
                    {
                        break;
                    }

                    buffer.push(self.ch);
                    self.advance();
                }

                Token::new(TokenType::Number, buffer)
            }

            _ =>
            {
                
                println!("{} is weird", self.ch);
                return None;
            }
        };

        Some(t)
    }
}

impl Lexer
{
    pub fn new(contents: String) -> Self
    {
        Self {
            program_text: contents.chars().collect(),
            here: 0,
            next: 1,
            ch: '\0',
        }
    }

    fn skip_whitespace(&mut self)
    {
        while self.ch.is_whitespace()
        {
            self.advance();
        }
    }

    fn advance(&mut self)
    {
        if self.next > self.program_text.len()
        {
            self.ch = '\0'
        }
        else
        {
            self.ch = self.program_text[self.next];
        }

        self.here = self.next;
        self.next += 1;
    }
}
