mod _ast_printer;
mod error;
mod expr;
mod interpreter;
mod lexer;
mod lox;
mod object;
mod parse;
mod tokens;
use lox::Lox;
use std::env::args;


pub fn main()
{
    let args: Vec<String> = args().collect();
    let lox = Lox::new();
    match args.len()
    {
        1 => lox.run_prompt(),
        2 => lox.run_file(&args[1]).expect("Couldn't run file"),
        _ =>
        {
            println!("Usage: lox-ast [script]");
            std::process::exit(64);
        }
    }
}
