use crate::error::*;
use crate::expr::*;

pub struct AstPrinter;

impl AstPrinter
{
    pub fn print(&self, expr: &Expr) -> Result<String, LoxError> { expr.accept(self) }

    fn parenthesize(&self, name: &String, exprs: &[&Expr]) -> Result<String, LoxError>
    {
        let mut builder = format!("({name}");

        for expr in exprs
        {
            builder = format!("{builder} {}", expr.accept(self)?);
        }

        Ok(format!("{builder})"))
    }
}

impl ExprVisitor<String> for AstPrinter
{
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<String, LoxError>
    {
        self.parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right])
    }
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<String, LoxError>
    {
        self.parenthesize(&"group".to_string(), &[&expr.expression])
    }
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<String, LoxError>
    {
        if let Some(value) = &expr.value
        {
            Ok(value.to_string())
        }
        else
        {
            Ok("nil".to_string())
        }
        // self.parenthesize(&expr.operator.lexeme,&[&expr.left, &expr.right])
    }
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<String, LoxError>
    {
        self.parenthesize(&expr.operator.lexeme, &[&expr.right])
    }
    fn visit_variable_expr(&self, expr: &VariableExpr) -> Result<String, LoxError> {
        todo!()
    }
}
