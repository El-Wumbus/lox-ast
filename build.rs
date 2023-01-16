mod tools;
use tools::{generate_ast::generate};
use std::io;

fn main() -> io::Result<()>
{
    generate("src".to_string())?;
    Ok(())
}

