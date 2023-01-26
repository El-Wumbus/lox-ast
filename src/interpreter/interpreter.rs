use std::cell::RefCell;

use crate::{
    error::LoxError,
    expr::*,
    object::Object,
    stmt::{Stmt, StmtVisitor},
    tokens::TokenType,
};

use super::environment::Environment;

pub struct Interpreter
{
    /// Our variable environment. We use a RefCell for mutability.
    environment: RefCell<Environment>,
}

impl StmtVisitor<()> for Interpreter
{
    fn visit_expression_stmt(&self, expr: &crate::stmt::ExpressionStmt) -> Result<(), LoxError>
    {
        self.evaluate(&expr.expression)?;
        Ok(())
    }

    fn visit_print_stmt(&self, expr: &crate::stmt::PrintStmt) -> Result<(), LoxError>
    {
        let value = self.evaluate(&expr.expression)?;
        // Print the expression
        println!("{value}");
        Ok(())
    }

    fn visit_var_stmt(&self, stmt: &crate::stmt::VarStmt) -> Result<(), LoxError>
    {
        let value: Object = if let Some(expr) = &stmt.initializer
        {
            self.evaluate(expr)?
        }
        else
        {
            Object::Nil
        };

        self.environment
            .borrow_mut()
            .define(stmt.name.lexeme.clone(), value);
        Ok(())
    }
}

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
            TokenType::Greater => left.greater(right),
            TokenType::GreaterEqual => left.greater_eq(right),
            TokenType::Less => left.less(right),
            TokenType::LessEqual => left.less_eq(right),
            TokenType::BangEqual => left.neq(right),
            TokenType::Equal => left.eq(right),
            _ => todo!(),
        };

        if res == Object::ArithmeticError || res == Object::ComparisonError
        {
            Err(LoxError::runtime_error(
                &expr.operator,
                "Illegal expression",
            ))
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

    fn visit_variable_expr(&self, expr: &VariableExpr) -> Result<Object, LoxError>
    {
        self.environment.borrow().get(expr.name.clone())
    }

    fn visit_assign_expr(&self, expr: &AssignExpr) -> Result<Object, LoxError>
    {
        let value = self.evaluate(&expr.value)?;
        self.environment
            .borrow_mut()
            .assign(&expr.name, value.clone())?;
        Ok(value)
    }
}

impl Interpreter
{
    pub fn new() -> Self
    {
        Self {
            environment: RefCell::new(Environment::new()),
        }
    }
    fn evaluate(&self, expr: &Expr) -> Result<Object, LoxError> { expr.accept(self) }

    fn is_truthy(&self, object: &Object) -> bool
    {
        // `Nil` and `False` values are false, everything else is true
        !matches!(object, Object::Nil | Object::Bool(false))
    }

    pub fn interpret(&self, statements: &[Stmt]) -> bool
    {
        let mut success = true;
        for statement in statements
        {
            if let Err(e) = self.execute(statement)
            {
                e.report("");
                success = false;
                break;
            }
        }
        success
    }

    fn execute(&self, stmt: &Stmt) -> Result<(), LoxError> { stmt.accept(self) }
}


#[cfg(test)]
// TODO: Test every possible case
mod tests
{
    use super::*;
    use crate::{stmt::VarStmt, tokens::*};
    fn make_literal(o: Object) -> Box<Expr>
    {
        Box::new(Expr::Literal(LiteralExpr { value: Some(o) }))
    }

    #[test]
    /// Tests unary minus (-15) or (-value)
    fn test_unary_minus()
    {
        let i = Interpreter::new();
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
        let i = Interpreter::new();

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
        let i = Interpreter::new();

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
        let i = Interpreter::new();

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
        let i = Interpreter::new();

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
        let i = Interpreter::new();

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
        let i = Interpreter::new();

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
        let i = Interpreter::new();

        let binary_expr = BinaryExpr {
            left: make_literal(Object::Num(15.0)),
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 0),
            right: make_literal(Object::Bool(true)),
        };

        let res = i.visit_binary_expr(&binary_expr);

        assert!(res.is_err());
    }

    #[test]
    /// Test that an comparison error is thrown when trying to compare differing
    /// types
    fn test_error_cmp()
    {
        let i = Interpreter::new();

        let binary_expr = BinaryExpr {
            left: make_literal(Object::Num(15.0)),
            operator: Token::new(TokenType::Greater, ">".to_string(), None, 0),
            right: make_literal(Object::Bool(true)),
        };

        let res = i.visit_binary_expr(&binary_expr);

        assert!(res.is_err());
    }

    #[test]
    /// Test binary greater-than (15 > 10)
    fn test_binary_greater_than_true()
    {
        let i = Interpreter::new();

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
        let i = Interpreter::new();

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
        let i = Interpreter::new();

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
        let i = Interpreter::new();

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
        let i = Interpreter::new();

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
        let i = Interpreter::new();

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
        let i = Interpreter::new();

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
        let i = Interpreter::new();

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
        let i = Interpreter::new();

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
        let i = Interpreter::new();

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
        let i = Interpreter::new();

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
        let i = Interpreter::new();

        let binary_expr = BinaryExpr {
            left: make_literal(Object::Str("Hello".to_string())),
            operator: Token::new(TokenType::Equal, "==".to_string(), None, 0),
            right: make_literal(Object::Str("Hello".to_string())),
        };

        let res = i.visit_binary_expr(&binary_expr).unwrap();

        assert_eq!(res, Object::Bool(true));
    }

    #[test]
    /// Test binary doesn't equal ("Hello" != "Hello")
    fn test_binary_bang_equal_str()
    {
        let i = Interpreter::new();

        let binary_expr = BinaryExpr {
            left: make_literal(Object::Str("Hello".to_string())),
            operator: Token::new(TokenType::BangEqual, "!=".to_string(), None, 0),
            right: make_literal(Object::Str("Hello World".to_string())),
        };

        let res = i.visit_binary_expr(&binary_expr).unwrap();

        assert_eq!(res, Object::Bool(true));
    }


    #[test]
    /// Test binary equals (nil == nil)
    fn test_binary_equal_nil_eq()
    {
        let i = Interpreter::new();

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
        let i = Interpreter::new();

        let binary_expr = BinaryExpr {
            left: make_literal(Object::Nil),
            operator: Token::new(TokenType::Equal, "==".to_string(), None, 0),
            right: make_literal(Object::Num(15.0)),
        };

        let res = i.visit_binary_expr(&binary_expr).unwrap();

        assert_eq!(res, Object::Bool(false));
    }

    #[test]
    fn test_defined_var_stmt()
    {
        let i = Interpreter::new();
        let name = Token::new(TokenType::Identifier, "foo".to_string(), None, 0);
        let var_stmt = VarStmt {
            name: name.clone(),
            initializer: Some(*make_literal(Object::Num(23.0))),
        };
        i.visit_var_stmt(&var_stmt).unwrap();

        // Create a let binding so it doesn't drop
        let e = i.environment.borrow();
        assert_eq!(e.get(name).unwrap(), Object::Num(23.0))
    }

    #[test]
    fn test_undefined_var_stmt()
    {
        let i = Interpreter::new();
        let name = Token::new(TokenType::Identifier, "foo".to_string(), None, 0);
        let var_stmt = VarStmt {
            name: name.clone(),
            initializer: None,
        };
        i.visit_var_stmt(&var_stmt).unwrap();

        // Create a let binding so it doesn't drop
        let e = i.environment.borrow();
        assert_eq!(e.get(name).unwrap(), Object::Nil)
    }

    #[test]
    fn test_defined_var_expr()
    {
        let i = Interpreter::new();
        let name = Token::new(TokenType::Identifier, "foo".to_string(), None, 0);
        let var_stmt = VarStmt {
            name: name.clone(),
            initializer: Some(*make_literal(Object::Num(23.0))),
        };
        i.visit_var_stmt(&var_stmt).unwrap();

        let var_expr = VariableExpr { name };

        assert_eq!(i.visit_variable_expr(&var_expr).unwrap(), Object::Num(23.0))
    }

    #[test]
    fn test_undefined_var_expr()
    {
        let i = Interpreter::new();
        let name = Token::new(TokenType::Identifier, "foo".to_string(), None, 0);
        let var_expr = VariableExpr { name };

        assert!(i.visit_variable_expr(&var_expr).is_err())
    }
}
