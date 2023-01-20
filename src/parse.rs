use crate::error::*;
use crate::expr::*;
use crate::tokens::*;

pub struct Parser
{
    /// Our array of tokens. It's a like a string and the tokens are our characters.
    tokens: Vec<Token>,

    /// Points to the next token waitig to be parsed.
    current: usize,
}

/// The parser implements funtions that match the grammar rules of lox.
/// The rules are as follows
///
/// expression  -> equality ;
/// equality    -> comparison ( ( "!=" | "==") comparison )* ;
/// comparison  -> term ( ( ">" | ">=" | "<" | "<=" ) term)* ;
/// term         -> factor ( ( "-" | "+" ) factor )* ;
/// factor       -> unary ( ( "/" | "*" ) unary )* ;
/// unary        -> ( "!" | "-" ) unary | primary ;
/// primary      -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
impl Parser
{
    pub fn new(tokens: Vec<Token>) -> Self { Self { tokens, current: 0 } }

    fn expression(&mut self) -> Result<Expr, LoxError> { self.eqaulity() }

    fn eqaulity(&mut self) -> Result<Expr, LoxError>
    {
        let mut expr = self.comparison()?;

        // Loop until we don't see any more quality operators
        while self.is_match(&[TokenType::BangEqual, TokenType::Equal])
        {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError>
    {
        let mut expr = self.term()?;

        while self.is_match(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ])
        {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxError>
    {
        let mut expr = self.factor()?;

        while self.is_match(&[TokenType::Minus, TokenType::Plus])
        {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxError>
    {
        let mut expr = self.unary()?;

        while self.is_match(&[TokenType::Slash, TokenType::Star])
        {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError>
    {
        if self.is_match(&[TokenType::Bang, TokenType::Minus])
        {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
            }));
        }
        Ok(self.primary()?)
    }

    fn primary(&mut self) -> Result<Expr, LoxError>
    {
        if self.is_match(&[TokenType::False])
        {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::False),
            }));
        }
        else if self.is_match(&[TokenType::True])
        {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::True),
            }));
        }
        else if self.is_match(&[TokenType::Nil])
        {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Nil),
            }));
        }
        else if self.is_match(&[TokenType::Number, TokenType::String])
        {
            return Ok(Expr::Literal(LiteralExpr {
                value: self.previous().literal.clone(),
            }));
        }
        else if self.is_match(&[TokenType::LeftParen])
        {
            let expr = self.expression()?;
            self.consume(
                TokenType::RightParen,
                "Expect ')' after expression".to_string(),
            )?;
            return Ok(Expr::Grouping(GroupingExpr {
                expression: Box::new(expr),
            }));
        }
        else
        {
            Err(LoxError::error(0, "Failed primary parser".to_string()))
        }

        // Err(LoxError::error(line, message))
    }

    fn consume(&mut self, ttype: TokenType, message: String) -> Result<Token, LoxError>
    {
        if self.check(ttype)
        {
            Ok(self.advance().clone())
        }
        else
        {
            let _p = self.peek();
            Err(Parser::error(self.peek(), message))
        }
    }

    fn error(token: &Token, message: String) -> LoxError { LoxError::parse_error(token, message) }

    fn synchronize(&mut self)
    {
        self.advance();

        while !self.is_at_end()
        {
            if self.previous().is(TokenType::Semicolon)
            {
                return;
            }

            if matches!(
                self.peek().token_type(),
                TokenType::Class
                    | TokenType::Var
                    | TokenType::For
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Print
                    | TokenType::Return
            )
            {
                return;
            }
            self.advance();
        }
    }

    /// Like the `.check()` method, but consumes the token if true
    fn is_match(&mut self, types: &[TokenType]) -> bool
    {
        for t in types
        {
            if self.check(*t)
            {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, ttype: TokenType) -> bool
    {
        if self.is_at_end()
        {
            false
        }
        else
        {
            self.peek().is(ttype)
        }
    }

    fn advance(&mut self) -> &Token
    {
        if !self.is_at_end()
        {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool { self.peek().is(TokenType::Eof) }

    fn peek(&self) -> &Token { self.tokens.get(self.current).unwrap() }

    fn previous(&self) -> &Token { self.tokens.get(self.current - 1).unwrap()}
}
