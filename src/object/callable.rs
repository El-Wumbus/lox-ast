use super::*;
use crate::{error::LoxResult, interpreter::Interpreter};
use std::rc::Rc;

#[derive(Clone)]
pub struct Callable
{
    pub func: Rc<dyn LoxCallable>,
}

impl PartialEq for Callable
{
    fn eq(&self, other: &Self) -> bool
    {
        std::ptr::eq(&self.func, &other.func) && self.arity() == other.arity()
    }
}


impl std::fmt::Debug for Callable
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}", LoxCallable::to_string(self))
    }
}

impl LoxCallable for Callable
{
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxResult>
    {
        self.func.call(interpreter, arguments)
    }

    fn arity(&self) -> usize { self.func.arity() }

    fn to_string(&self) -> String { self.func.to_string() }
}

impl std::fmt::Display for Callable
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "<fn {}>", self.func.to_string())
    }
}

pub trait LoxCallable
{
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxResult>;
    fn arity(&self) -> usize;
    fn to_string(&self) -> String;
}
