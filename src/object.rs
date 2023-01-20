#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum Object
{
    Num(f64),
    Str(String),
    Nil,
    True,
    False,
}

impl std::fmt::Display for Object
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self
        {
            Self::Num(x) => write!(f, "{x}"),
            Self::Str(x) => write!(f, "{x}"),
            Self::Nil => write!(f, "nil"),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
        }
    }
}
