#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    // Single char tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two char tokens
    Bang,
    BangEqual,
    Assign,      // Assignment
    Equal, // Equality
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum Object {
    Num(f64),
    Str(String),
    Nil,
    True,
    False,
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Num(x) => write!(f, "{x}"),
            Self::Str(x) => write!(f, "{x}"),
            Self::Nil => write!(f, "nil"),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
    literal: Option<Object>,
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

    pub fn eof(line: usize) -> Token {
        Token {
            ttype: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:?} {} {}",
            self.ttype,
            self.lexeme,
            if let Some(literal) = &self.literal {
                literal.to_string()
            } else {
                "None".to_string()
            }
        )
    }
}
