use crate::{
    error::*,
    interpreter::{environment::Environment, Interpreter},
    object::{callable::LoxCallable, *},
    stmt::{FunctionStmt, Stmt},
    tokens::Token,
};
use std::{cell::RefCell, rc::Rc};

pub struct LoxFunction
{
    name: Token,
    params: Rc<Vec<Token>>,
    body: Rc<Vec<Stmt>>,
    closure: Rc<RefCell<Environment>>,
}

impl LoxFunction
{
    pub fn new(declaration: &FunctionStmt, closure: &Rc<RefCell<Environment>>) -> Self
    {
        Self {
            name: declaration.name.clone(),
            body: Rc::clone(&declaration.body),
            params: Rc::clone(&declaration.params),
            closure: Rc::clone(closure),
        }
    }
}

impl LoxCallable for LoxFunction
{
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxResult>
    {
        let mut e = Environment::new_with_enclosing(Rc::clone(&self.closure));

        for (param, arg) in self.params.iter().zip(arguments.iter())
        {
            e.define(param.get_identifier(), arg.clone());
        }

        match interpreter.execute_block(&self.body, e)
        {
            Err(LoxResult::Return { value }) => Ok(value),
            Err(e) => Err(e),
            Ok(_) => Ok(Object::Nil),
        }
    }

    fn arity(&self) -> usize { self.params.len() }

    fn to_string(&self) -> String { self.name.get_identifier() }
}
