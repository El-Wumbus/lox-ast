use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, Clone, PartialOrd)]
/// `Object` represents an object type in lox. There are four object variants
/// (Number, String, Boolean, and Nil (NULL)) accompanied by two error types.
pub enum Object
{
    Num(f64),
    Str(String),
    Bool(bool),
    Nil,
    /// Tried to do an operation on incompatable types
    ArithmeticError,
    /// Tried to compare incomparable types
    ComparisonError,
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
            (Object::Num(left), Object::Str(right)) => Self::Str(format!("{left}{right}")),
            (Object::Str(left), Object::Num(right)) => Self::Str(format!("{left}{right}")),

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
            Self::ArithmeticError | Self::ComparisonError =>
            {
                panic!("Shouldn't be trying to print erronious Objects")
            }
        }
    }
}

impl Object
{
    /// Test if `left` is greater-than `right`
    pub fn greater(&self, right: Object) -> Self
    {
        match (self, right)
        {
            (Self::Num(left), Self::Num(right)) => Self::Bool(*left > right),
            _ => Self::ComparisonError,
        }
    }

    /// Test if `left` is greater-than, or equal-to `right`
    pub fn greater_eq(&self, right: Object) -> Self
    {
        match (self, right)
        {
            (Self::Num(left), Self::Num(right)) => Self::Bool(*left >= right),
            _ => Self::ComparisonError,
        }
    }

    /// Test if `left` is less-than `right`
    pub fn less(&self, right: Object) -> Self
    {
        match (self, right)
        {
            (Self::Num(left), Self::Num(right)) => Self::Bool(*left < right),
            _ => Self::ComparisonError,
        }
    }

    /// Test if `left` is less-than, or equal-to `right`
    pub fn less_eq(&self, right: Object) -> Self
    {
        match (self, right)
        {
            (Self::Num(left), Self::Num(right)) => Self::Bool(*left <= right),
            _ => Self::ComparisonError,
        }
    }

    /// Test if `left` and `right` are equal
    pub fn eq(&self, right: Object) -> Self
    {
        match (self, right)
        {
            (Self::Num(left), Self::Num(right)) => Self::Bool(*left == right),
            (Self::Str(left), Self::Str(right)) => Self::Bool(*left == right),
            (Self::Bool(left), Self::Bool(right)) => Self::Bool(*left == right),
            (Self::Nil, Self::Nil) => Self::Bool(true),
            (Self::Nil, _) | (_, Self::Nil) => Self::Bool(false),

            _ => Self::ComparisonError,
        }
    }

    /// Test if `left` and `right` aren't equal
    pub fn neq(&self, right: Object) -> Self
    {
        match (self, right)
        {
            (Self::Num(left), Self::Num(right)) => Self::Bool(*left != right),
            (Self::Str(left), Self::Str(right)) => Self::Bool(*left != right),
            (Self::Bool(left), Self::Bool(right)) => Self::Bool(*left != right),
            (Self::Nil, Self::Nil) => Self::Bool(false),
            (Self::Nil, _) | (_, Self::Nil) => Self::Bool(true),
            _ => Self::ComparisonError,
        }
    }
}
