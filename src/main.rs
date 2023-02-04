mod error;
mod expr;
mod interpreter;
mod lexer;
mod lox;
mod lox_function;
mod object;
mod parser;
mod stmt;
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
