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
