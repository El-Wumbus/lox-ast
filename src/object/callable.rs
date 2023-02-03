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
        f.debug_struct("Callable").finish()
    }
}

impl LoxCallable for Callable
{
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxResult>
    {
        self.func.call(interpreter, arguments)
    }

    fn arity(&self) -> usize { self.func.arity() }
}

impl std::fmt::Display for Callable
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Function") }
}

pub trait LoxCallable
{
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxResult>;
    fn arity(&self) -> usize;
}
