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
            TokenType::Minus => Ok(left - right),
            TokenType::Slash => Ok(left / right),
            TokenType::Star => Ok(left * right),
            _ => todo!(),
        };

        if res == Ok(Object::ArithmeticError)
        {
            Err(LoxError::error(expr.operator.line, "Illegal expression"))
        }
        else
        {
            res
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

    fn make_literal_num(n :f64) -> Box<Expr>
    {
        Box::new(Expr::Literal(LiteralExpr {
            value: Some(Object::Num(n)),
        }))
    }

    #[test]
    /// Tests unary minus (-15) or (-value)
    fn test_unary_minus()
    {
        let i = Interpreter {};
        let unary_expr = UnaryExpr {
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
            right: make_literal_num(123.5),
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
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(false)),
            })),
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
            left: make_literal_num(15.0),
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 0),
            right: make_literal_num(7.0),
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
            left: make_literal_num(21.0),
            operator: Token::new(TokenType::Slash, "/".to_string(), None, 0),
            right: make_literal_num(7.0),
        };

        let res = i.visit_binary_expr(&binary_expr).unwrap();

        assert_eq!(res, Object::Num(3.0));
    }
}
