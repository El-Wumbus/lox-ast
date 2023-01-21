#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum Object
{
    Num(f64),
    Str(String),
    Bool(bool),
    Nil,
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
        }
    }
}
