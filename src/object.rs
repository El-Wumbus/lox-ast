use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, Clone, PartialOrd)]
#[allow(dead_code)]
pub enum Object
{
    Num(f64),
    Str(String),
    Bool(bool),
    Nil,
    ArithmeticError,
}

impl Sub for Object
{
    type Output = Self;

    fn sub(self, other: Self) -> Self
    {
        match (self, other)
        {
            (Object::Num(left), Object::Num(right)) => Self::Num(left - right),

            _ => Object::ArithmeticError,
        }
    }
}

impl Div for Object
{
    type Output = Self;

    fn div(self, other: Self) -> Self
    {
        match (self, other)
        {
            (Object::Num(left), Object::Num(right)) => Self::Num(left / right),

            _ => Object::ArithmeticError,
        }
    }
}

impl Mul for Object
{
    type Output = Self;

    fn mul(self, other: Self) -> Self
    {
        match (self, other)
        {
            (Object::Num(left), Object::Num(right)) => Self::Num(left * right),

            _ => Object::ArithmeticError,
        }
    }
}

impl Add for Object
{
    type Output = Self;

    fn add(self, other: Self) -> Self
    {
        match (self, other)
        {
            (Object::Num(left), Object::Num(right)) => Self::Num(left + right),
            (Object::Str(left), Object::Str(right)) => Self::Str(format!("{left}{right}")),
            _ => Object::ArithmeticError,
        }
    }
}

impl From<f64> for Object
{
    fn from(value: f64) -> Self { Object::Num(value) }
}

impl From<Object> for f64
{
    fn from(value: Object) -> Self
    {
        match value
        {
            Object::Num(x) => x,
            _ => 0.0,
        }
    }
}

impl std::fmt::Display for Object
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self
        {
            Self::Nil => write!(f, "nil"),
            Self::Num(x) => write!(f, "{x}"),
            Self::Bool(x) => write!(f, "{x}"),
            Self::Str(x) => write!(f, "{x}"),
            Self::ArithmeticError => panic!("Shouldn't be trying to print erronious Objects"),
        }
    }
}
