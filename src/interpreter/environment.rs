use crate::{error::LoxResult, object::Object, tokens::Token};
use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
    rc::Rc,
};

/// An `Environment` contains variable's identifiers and their associated
/// values.
#[derive(Debug, Clone)]
pub struct Environment
{
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Object>,
}

impl Environment
{
    /// Create a new `Environment`
    pub fn new() -> Self
    {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Environment
    {
        Environment {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }
    }

    /// Define a new variable in the envrionment
    pub fn define(&mut self, name: String, value: Object) { self.values.insert(name, value); }

    /// Get a variable's value from the environment
    pub fn get(&self, name: Token) -> Result<Object, LoxResult>
    {
        if let Some(o) = self.values.get(&name.lexeme)
        {
            Ok(o.clone())
        }
        else if let Some(enclosing) = &self.enclosing
        {
            // Check the enclosing scope for the variable
            enclosing.borrow().get(name)
        }
        else
        {
            Err(LoxResult::new_runtime_error(
                name.clone(),
                format!("Undefined variable '{}'.", name.lexeme),
            ))
        }
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), LoxResult>
    {
        if let Entry::Occupied(mut object) = self.values.entry(name.lexeme.clone())
        {
            object.insert(value);
            Ok(())
        }
        else if let Some(enclosing) = &self.enclosing
        {
            // Check the enclosing scope for the variable
            enclosing.borrow_mut().assign(name, value)
        }
        else
        {
            Err(LoxResult::new_runtime_error(
                name.clone(),
                format!("Undefined variable '{}'.", name.lexeme),
            ))
        }
    }
}

#[cfg(test)]
mod tests
{
    use crate::tokens::TokenType;

    use super::*;

    #[test]
    fn test_variable_definition()
    {
        let mut e = Environment::new();

        e.define("One".to_string(), Object::Bool(true));

        assert!(e.values.contains_key("One"));
        assert_eq!(*e.values.get("One").unwrap(), Object::Bool(true));
    }

    #[test]
    fn test_variable_redefinition()
    {
        let mut e = Environment::new();

        e.define("Cool".to_string(), Object::Num(7.8));

        assert!(e.values.contains_key("Cool"));
        assert_eq!(*e.values.get("Cool").unwrap(), Object::Num(7.8));

        e.define("Cool".to_string(), Object::Num(88.8));
        assert_eq!(*e.values.get("Cool").unwrap(), Object::Num(88.8));
    }

    #[test]
    fn test_variable_lookup()
    {
        let mut e = Environment::new();

        e.define(
            "cool".to_string(),
            Object::Str("FooBar is cool".to_string()),
        );

        assert_eq!(
            e.get(Token::new(
                TokenType::Identifier,
                "cool".to_string(),
                None,
                0
            ))
            .unwrap(),
            Object::Str("FooBar is cool".to_string())
        )
    }

    #[test]
    fn test_undefined_variable()
    {
        let e = Environment::new();

        let res = e.get(Token::new(
            TokenType::Identifier,
            "cool".to_string(),
            None,
            0,
        ));

        assert!(res.is_err());
    }

    #[test]
    fn test_assign_defined_variable()
    {
        let mut e = Environment::new();
        let tok = Token::new(TokenType::Identifier, "cool".to_string(), None, 0);

        // Define the variable
        e.define(
            "cool".to_string(),
            Object::Str("FooBar is cool".to_string()),
        );

        // Check that the variable's value is defined
        assert_eq!(
            e.get(tok.clone()).unwrap(),
            Object::Str("FooBar is cool".to_string())
        );

        // Assign a new value to the variable
        e.assign(&tok, Object::Bool(true)).unwrap();

        // Check that the new value has been assigned to the variable
        assert_eq!(e.get(tok.clone()).unwrap(), Object::Bool(true));
    }

    #[test]
    fn test_assign_undefined_variable()
    {
        let mut e = Environment::new();

        let tok = Token::new(TokenType::Identifier, "cool".to_string(), None, 0);

        assert!(e.assign(&tok, Object::Nil).is_err());
    }


    #[test]
    fn test_enclose_environment()
    {
        let e = Rc::new(RefCell::new(Environment::new()));
        let f = Environment::new_with_enclosing(Rc::clone(&e));

        assert_eq!(f.enclosing.unwrap().borrow().values, e.borrow().values);
    }

    #[test]
    fn test_read_enclosed_environment()
    {
        let e = Rc::new(RefCell::new(Environment::new()));
        e.borrow_mut()
            .define("variable".to_string(), Object::Num(77.8));
        let f = Environment::new_with_enclosing(Rc::clone(&e));
        assert_eq!(
            f.get(Token::new(
                TokenType::Identifier,
                "variable".to_string(),
                None,
                0
            ))
            .unwrap(),
            Object::Num(77.8)
        );
    }

    #[test]
    fn test_assign_to_enclosed_environment()
    {
        let e = Rc::new(RefCell::new(Environment::new()));

        // Define
        e.borrow_mut().define("cool".to_string(), Object::Num(77.8));
        let mut f = Environment::new_with_enclosing(Rc::clone(&e));

        // Assert the variable was defined
        assert_eq!(
            f.get(Token::new(
                TokenType::Identifier,
                "cool".to_string(),
                None,
                0
            ))
            .unwrap(),
            Object::Num(77.8)
        );

        let tok = Token::new(TokenType::Identifier, "cool".to_string(), None, 0);

        // Assign a new value to the variable
        f.assign(&tok, Object::Bool(true)).unwrap();

        // Check that the new value has been assigned to the variable
        assert_eq!(f.get(tok.clone()).unwrap(), Object::Bool(true));
    }
}
