use error::*;
use tokens::*;
use expr::*;

struct AstPrinter;

impl AstPrinter
{
    fn print(&self, expr: &Expr) -> Result<String, LoxError>
    {
        expr.accept(self)
 
    }

    fn parenthisize(&self, name: &String, exprs: &[Box<Expr>]) -> Result<String, LoxError>
    {
        let mut builder = format!("({name}");

        for expr in exprs
        {
            builder = format!("{builder} {}", expr.accept(self))?;
        }

        builder = format!("{builder})");
        Ok(())
    }
}

impl ExprVisitor<String> for AstPrinter
{
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<String, LoxError>
    {
        self.parenthesize(&expr.operator.lexeme,&[&expr.left, &expr.right])
    }
    fn visit_grouping_expr(&self, expr: &BinaryExpr) -> Result<String, LoxError>
    {
        self.parenthesize(&"group".to_string(), &[&expr.expression])
    }
    fn visit_literal_expr(&self, expr: &BinaryExpr) -> Result<String, LoxError>
    {
        if let Some(value) = &expr.value {
            Ok(value.to_string())
        }
        else
        {
            Ok("nil".to_string())
        }
        // self.parenthesize(&expr.operator.lexeme,&[&expr.left, &expr.right])
    }
    fn visit_unary_expr(&self, expr: &BinaryExpr) -> Result<String, LoxError>
    {
        self.parenthesize(&expr.operator.lexeme, &[expr.right])
    }
}