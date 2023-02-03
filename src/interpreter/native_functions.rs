use super::Interpreter;
use crate::{
    error::LoxResult,
    object::{callable::LoxCallable, Object},
};
use std::time::SystemTime;

pub struct NativeClock;
impl LoxCallable for NativeClock
{
    fn call(&self, _: &Interpreter, _: Vec<Object>) -> Result<Object, LoxResult>
    {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
        {
            Ok(n) => Ok(Object::Num(n.as_secs_f64())),
            Err(e) =>
            {
                Err(LoxResult::new_system_error(&format!(
                    "Couldn't get current time: {e}"
                )))
            }
        }
    }

    fn arity(&self) -> usize { 0 }
}
