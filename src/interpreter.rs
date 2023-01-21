use crate::{error::LoxError, expr::*, object::Object, tokens::TokenType};

pub struct Interpreter {}

impl ExprVisitor<Object> for Interpreter
{
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Object, LoxError>
    {
        Ok(expr.value.to_owned().unwrap())
    }

    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Object, LoxError> { Ok(Object::Nil) }

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
            TokenType::Bang => Ok(Object::Bool(self.is_truthy(&right))),

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
