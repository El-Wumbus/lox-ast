use crate::{error::LoxError, object::Object, tokens::Token};
use std::collections::HashMap;

pub struct Environment
{
    values: HashMap<String, Object>,
}

impl Environment
{
    pub fn new() -> Self
    {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Object) { self.values.insert(name, value); }

    pub fn get(&self, name: Token) -> Result<Object, LoxError>
    {
        if let Some(o) = self.values.get(&name.lexeme)
        {
            Ok(o.clone())
        }
        else
        {
            Err(LoxError::error(
                name.line,
                &format!("Undefined variable '{}'.", name.lexeme),
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
}
