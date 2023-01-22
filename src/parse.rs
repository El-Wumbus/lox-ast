use crate::error::*;
use crate::expr::*;
use crate::lox::Lox;
use crate::object::*;
use crate::stmt::*;
use crate::tokens::*;

pub struct Parser<'a>
{
    /// Our array of tokens. It's a like a string and the tokens are our
    /// characters.
    tokens: &'a Vec<Token>,

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
/// primary      -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression
/// ")" ;
impl<'a> Parser<'a>
{
    /// Create a new parser
    pub fn new(tokens: &'a Vec<Token>) -> Self { Self { tokens, current: 0 } }

    /// Parses a single expression and returns it.
    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxError>
    {
        let mut statements = Vec::new();

        while !self.is_at_end()
        {
            statements.push(self.declaration()?)
        }
        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expr, LoxError> { self.eqaulity() }

    fn declaration(&mut self) -> Result<Stmt, LoxError>
    {
        let res = if self.is_match(&[TokenType::Var])
        {
            self.var_declaration()
        }
        else
        {
            self.statement()
        };

        if res.is_err()
        {
            self.synchronize();
        }
        res
    }


    fn statement(&mut self) -> Result<Stmt, LoxError>
    {
        if self.is_match(&[TokenType::Print])
        {
            return self.print_statement();
        }

        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxError>
    {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(PrintStmt { expression: value }))
    }

    fn var_declaration(&mut self) -> Result<Stmt, LoxError>
    {
        let name = self.consume(TokenType::Identifier, "Expected variable name")?;

        let initializer = if self.is_match(&[TokenType::Assign])
        {
            Some(self.expression()?)
        }
        else
        {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expected ';' after variable decalaration.",
        );

        Ok(Stmt::Var(VarStmt { name, initializer }))
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxError>
    {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Expression(ExpressionStmt { expression: expr }))
    }

    /// The equality rule.
    /// It matches an equality operator or anything of higher precedece.
    /// Equality operators have the lowest precedence.
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

    /// The comparison rule.
    /// It matches a comparison operator or anything of higer precedence
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

    /// It matches an additon or subtraction operator, or anything of higer
    /// precedence.
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

    /// It matches an multiplication or division operator, or anything of higer
    /// precedence.
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

    /// It matches a unary operator or anything of higher precedence.
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
        self.primary()
    }

    /// It matches a primary, the highest level of precedence.
    /// Primaries are as follows:
    ///     NUMBERS
    ///     STRINGS
    ///     True
    ///     False
    ///     Nil
    ///     (...)
    fn primary(&mut self) -> Result<Expr, LoxError>
    {
        if self.is_match(&[TokenType::False])
        {
            Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(false)),
            }))
        }
        else if self.is_match(&[TokenType::True])
        {
            Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(true)),
            }))
        }
        else if self.is_match(&[TokenType::Nil])
        {
            Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Nil),
            }))
        }
        else if self.is_match(&[TokenType::Number, TokenType::String])
        {
            Ok(Expr::Literal(LiteralExpr {
                value: self.previous().literal.clone(),
            }))
        }
        else if self.is_match(&[TokenType::LeftParen])
        {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression")?;
            Ok(Expr::Grouping(GroupingExpr {
                expression: Box::new(expr),
            }))
        }
        else if self.is_match(&[TokenType::Identifier])
        {
            Ok(Expr::Variable(VariableExpr {
                name: self.previous().clone(),
            }))
        }
        else
        {
            Err(LoxError::error(0, "Expect expression"))
        }

        // Err(LoxError::error(line, message))
    }

    fn consume(&mut self, ttype: TokenType, message: &str) -> Result<Token, LoxError>
    {
        if self.check(ttype)
        {
            Ok(self.advance().clone())
        }
        else
        {
            let _p = self.peek();
            Err(Parser::error(self.peek(), message.to_string()))
        }
    }

    fn error(token: &Token, message: String) -> LoxError { LoxError::parse_error(token, &message) }

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

    fn previous(&self) -> &Token { self.tokens.get(self.current - 1).unwrap() }
}
