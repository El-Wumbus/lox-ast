mod tools;
use std::io;
use tools::generate_ast::generate;

fn main() -> io::Result<()>
{
    generate("src".to_string())?;
    Ok(())
}
