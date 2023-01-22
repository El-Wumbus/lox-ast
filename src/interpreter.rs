use crate::{error::LoxError, expr::*, object::Object, tokens::TokenType};

pub struct Interpreter {}

impl ExprVisitor<Object> for Interpreter
{
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Object, LoxError>
    {
        Ok(expr.value.to_owned().unwrap())
    }

    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Object, LoxError>
    {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        let res = match expr.operator.token_type()
        {
            TokenType::Minus => left - right,
            TokenType::Slash => left / right,
            TokenType::Star => left * right,
            TokenType::Plus => left + right,
            TokenType::Greater => Object::Bool(left > right),
            TokenType::GreaterEqual => Object::Bool(left >= right),
            TokenType::Less => Object::Bool(left < right),
            TokenType::LessEqual => Object::Bool(left <= right),
            TokenType::BangEqual => Object::Bool(left != right),
            TokenType::Equal => Object::Bool(left == right),
            _ => todo!(),
        };

        if res == Object::ArithmeticError
        {
            Err(LoxError::error(expr.operator.line, "Illegal expression"))
        }
        else
        {
            Ok(res)
        }
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Object, LoxError>
    {
        self.evaluate(&expr.expression)
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Object, LoxError>
    {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type()
        {
            TokenType::Minus =>
            {
                match right
                {
                    Object::Num(n) => Ok(Object::Num(-n)),
                    _ => Ok(Object::Nil),
                }
            }
            TokenType::Bang => Ok(Object::Bool(!self.is_truthy(&right))),

            _ => Err(LoxError::error(expr.operator.line, "Unreachable error")),
        }
    }
}

impl Interpreter
{
    fn evaluate(&self, expr: &Expr) -> Result<Object, LoxError> { expr.accept(self) }

    fn is_truthy(&self, object: &Object) -> bool
    {
        // `Nil` and `False` values are false, everything else is true
        !matches!(object, Object::Nil | Object::Bool(false))
    }
}


#[cfg(test)]
mod tests
{
    use super::*;
    use crate::tokens::*;
    fn make_literal(o: Object) -> Box<Expr>
    {
        Box::new(Expr::Literal(LiteralExpr { value: Some(o) }))
    }

    #[test]
    /// Tests unary minus (-15) or (-value)
    fn test_unary_minus()
    {
        let i = Interpreter {};
        let unary_expr = UnaryExpr {
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
            right: make_literal(Object::Num(123.5)),
        };

        let res = i.visit_unary_expr(&unary_expr).unwrap();

        assert_eq!(res, Object::Num(-123.5));
    }

    #[test]
    /// Tests unary not (!true)
    fn test_unary_bang()
    {
        let i = Interpreter {};

        let unary_expr = UnaryExpr {
            operator: Token::new(TokenType::Bang, "!".to_string(), None, 1),
            right: make_literal(Object::Bool(false)),
        };

        let res = i.visit_unary_expr(&unary_expr).unwrap();

        assert_eq!(res, Object::Bool(true));
    }

    #[test]
    /// Tests binary subtraction (15 - 7)
    fn test_binary_minus()
    {
        let i = Interpreter {};

        let binary_expr = BinaryExpr {
            left: make_literal(Object::Num(15.0)),
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 0),
            right: make_literal(Object::Num(7.0)),
        };

        let res = i.visit_binary_expr(&binary_expr).unwrap();

        assert_eq!(res, Object::Num(8.0));
    }

    #[test]
    /// Test binary division (21 / 7)
    fn test_binary_slash()
    {
        let i = Interpreter {};

        let binary_expr = BinaryExpr {
            left: make_literal(Object::Num(21.0)),
            operator: Token::new(TokenType::Slash, "/".to_string(), None, 0),
            right: make_literal(Object::Num(7.0)),
        };

        let res = i.visit_binary_expr(&binary_expr).unwrap();

        assert_eq!(res, Object::Num(3.0));
    }

    #[test]
    /// Test binary multiplication (15 * 7)
    fn test_binary_star()
    {
        let i = Interpreter {};

        let binary_expr = BinaryExpr {
            left: make_literal(Object::Num(15.0)),
            operator: Token::new(TokenType::Star, "*".to_string(), None, 0),
            right: make_literal(Object::Num(7.0)),
        };

        let res = i.visit_binary_expr(&binary_expr).unwrap();

        assert_eq!(res, Object::Num(105.0));
    }

    #[test]
    /// Test binary additon (21 + 7)
    fn test_binary_plus_num()
    {
        let i = Interpreter {};

        let binary_expr = BinaryExpr {
            left: make_literal(Object::Num(21.0)),
            operator: Token::new(TokenType::Plus, "+".to_string(), None, 0),
            right: make_literal(Object::Num(7.0)),
        };

        let res = i.visit_binary_expr(&binary_expr).unwrap();

        assert_eq!(res, Object::Num(28.0));
    }

    #[test]
    /// Test binary string concatenation ("Hello, " + "World!")
    fn test_binary_plus_str()
    {
        let i = Interpreter {};

        let binary_expr = BinaryExpr {
            left: make_literal(Object::Str("Hello, ".to_string())),
            operator: Token::new(TokenType::Plus, "+".to_string(), None, 0),
            right: make_literal(Object::Str("World!".to_string())),
        };

        let res = i.visit_binary_expr(&binary_expr).unwrap();

        assert_eq!(res, Object::Str("Hello, World!".to_string()));
    }

    #[test]
    /// Test that an arithmetic error is thrown when trying to do operations on
    /// differing types
    fn test_arithmetic_error_minus()
    {
        let i = Interpreter {};

        let binary_expr = BinaryExpr {
            left: make_literal(Object::Num(15.0)),
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 0),
            right: make_literal(Object::Bool(true)),
        };

        let res = i.visit_binary_expr(&binary_expr);

        assert!(res.is_err());
    }

    #[test]
    /// Test binary greater-than (15 > 10)
    fn test_binary_greater_than_true()
    {
        let i = Interpreter {};

        let binary_expr = BinaryExpr {
            left: make_literal(Object::Num(15.0)),
            operator: Token::new(TokenType::Greater, ">".to_string(), None, 0),
            right: make_literal(Object::Num(10.0)),
        };

        let res = i.visit_binary_expr(&binary_expr).unwrap();
        assert_eq!(res, Object::Bool(true));
    }

    #[test]
    /// Test binary greater-than or equal-to (15 >= 15)
    fn test_binary_greater_than_equal_eq()
    {
        let i = Interpreter {};

        let binary_expr = BinaryExpr {
            left: make_literal(Object::Num(15.0)),
            operator: Token::new(TokenType::GreaterEqual, ">=".to_string(), None, 0),
            right: make_literal(Object::Num(15.0)),
        };

        let res = i.visit_binary_expr(&binary_expr).unwrap();
        assert_eq!(res, Object::Bool(true));
    }

    #[test]
    /// Test binary greater-than or equal-to (15 >= 7)
    fn test_binary_greater_than_equal_neq()
    {
        let i = Interpreter {};

        let binary_expr = BinaryExpr {
            left: make_literal(Object::Num(15.0)),
            operator: Token::new(TokenType::GreaterEqual, ">=".to_string(), None, 0),
            right: make_literal(Object::Num(7.0)),
        };

        let res = i.visit_binary_expr(&binary_expr).unwrap();

        assert_eq!(res, Object::Bool(true));
    }

    #[test]
    /// Test binary less-than (5 < 7)
    fn test_binary_less_than()
    {
        let i = Interpreter {};

        let binary_expr = BinaryExpr {
            left: make_literal(Object::Num(5.0)),
            operator: Token::new(TokenType::Less, "<".to_string(), None, 0),
            right: make_literal(Object::Num(7.0)),
        };

        let res = i.visit_binary_expr(&binary_expr).unwrap();

        assert_eq!(res, Object::Bool(true));
    }

    #[test]
    /// Test binary less-than (15 < 15)
    fn test_binary_less_than_equal_eq()
    {
        let i = Interpreter {};

        let binary_expr = BinaryExpr {
            left: make_literal(Object::Num(15.0)),
            operator: Token::new(TokenType::LessEqual, "<=".to_string(), None, 0),
            right: make_literal(Object::Num(15.0)),
        };

        let res = i.visit_binary_expr(&binary_expr).unwrap();
        assert_eq!(res, Object::Bool(true));
    }

    #[test]
    /// Test binary less-than or equal-to (20 <= 20.8)
    fn test_binary_less_than_equal_neq()
    {
        let i = Interpreter {};

        let binary_expr = BinaryExpr {
            left: make_literal(Object::Num(20.0)),
            operator: Token::new(TokenType::LessEqual, "<=".to_string(), None, 0),
            right: make_literal(Object::Num(20.8)),
        };

        let res = i.visit_binary_expr(&binary_expr).unwrap();

        assert_eq!(res, Object::Bool(true));
    }

    #[test]
    /// Test binary greater-than (10 > 15)
    fn test_binary_greater_than_false()
    {
        let i = Interpreter {};

        let binary_expr = BinaryExpr {
            left: make_literal(Object::Num(10.0)),
            operator: Token::new(TokenType::Greater, ">".to_string(), None, 0),
            right: make_literal(Object::Num(15.0)),
        };

        let res = i.visit_binary_expr(&binary_expr).unwrap();
        assert_eq!(res, Object::Bool(false));
    }

    #[test]
    /// Test binary equals (7 == 7)
    fn test_binary_equal_eq()
    {
        let i = Interpreter {};

        let binary_expr = BinaryExpr {
            left: make_literal(Object::Num(7.0)),
            operator: Token::new(TokenType::Equal, "==".to_string(), None, 0),
            right: make_literal(Object::Num(7.0)),
        };

        let res = i.visit_binary_expr(&binary_expr).unwrap();

        assert_eq!(res, Object::Bool(true));
    }

    #[test]
    /// Test binary equals (7.23 == 7.0)
    fn test_binary_equal_neq()
    {
        let i = Interpreter {};

        let binary_expr = BinaryExpr {
            left: make_literal(Object::Num(7.23)),
            operator: Token::new(TokenType::Equal, "==".to_string(), None, 0),
            right: make_literal(Object::Num(7.0)),
        };

        let res = i.visit_binary_expr(&binary_expr).unwrap();

        assert_eq!(res, Object::Bool(false));
    }

    #[test]
    /// Test binary not-equals (7.23 != 7)
    fn test_binary_bang_equal_neq()
    {
        let i = Interpreter {};

        let binary_expr = BinaryExpr {
            left: make_literal(Object::Num(7.23)),
            operator: Token::new(TokenType::BangEqual, "!=".to_string(), None, 0),
            right: make_literal(Object::Num(7.0)),
        };

        let res = i.visit_binary_expr(&binary_expr).unwrap();

        assert_eq!(res, Object::Bool(true));
    }

    #[test]
    /// Test binary not-equals (7 != 7)
    fn test_binary_bang_equal_eq()
    {
        let i = Interpreter {};

        let binary_expr = BinaryExpr {
            left: make_literal(Object::Num(7.0)),
            operator: Token::new(TokenType::BangEqual, "!=".to_string(), None, 0),
            right: make_literal(Object::Num(7.0)),
        };

        let res = i.visit_binary_expr(&binary_expr).unwrap();

        assert_eq!(res, Object::Bool(false));
    }

    #[test]
    /// Test binary equals ("Hello" == "Hello")
    fn test_binary_equal_str()
    {
        let i = Interpreter {};

        let binary_expr = BinaryExpr {
            left: make_literal(Object::Str("Hello".to_string())),
            operator: Token::new(TokenType::Equal, "==".to_string(), None, 0),
            right: make_literal(Object::Str("Hello".to_string())),
        };

        let res = i.visit_binary_expr(&binary_expr).unwrap();

        assert_eq!(res, Object::Bool(true));
    }

    #[test]
    /// Test binary equals (nil == nil)
    fn test_binary_equal_nil_eq()
    {
        let i = Interpreter {};

        let binary_expr = BinaryExpr {
            left: make_literal(Object::Nil),
            operator: Token::new(TokenType::Equal, "==".to_string(), None, 0),
            right: make_literal(Object::Nil),
        };

        let res = i.visit_binary_expr(&binary_expr).unwrap();

        assert_eq!(res, Object::Bool(true));
    }

    #[test]
    /// Test binary equals (nil == true)
    fn test_binary_equal_nil_neq()
    {
        let i = Interpreter {};

        let binary_expr = BinaryExpr {
            left: make_literal(Object::Nil),
            operator: Token::new(TokenType::Equal, "==".to_string(), None, 0),
            right: make_literal(Object::Num(15.0)),
        };

        let res = i.visit_binary_expr(&binary_expr).unwrap();

        assert_eq!(res, Object::Bool(false));
    }
}
