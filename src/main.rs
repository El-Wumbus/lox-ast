mod _ast_printer;
mod error;
mod expr;
mod interpreter;
mod lexer;
mod object;
mod parse;
mod tokens;

use _ast_printer::AstPrinter;
use error::*;
use lexer::scanner::*;
use parse::Parser;
use std::env::args;
use std::io::{self, stdout, BufRead, Write};

pub fn main()
{
    let args: Vec<String> = args().collect();
    match args.len()
    {
        1 => run_prompt(),
        2 => run_file(&args[1]).expect("Couldn't run file"),
        _ =>
        {
            println!("Usage: lox-ast [script]");
            std::process::exit(64);
        }
    }
}

fn run_file(path: &String) -> io::Result<()>
{
    let buf = std::fs::read_to_string(path)?;
    if run(buf).is_err()
    {
        std::process::exit(65);
    }

    Ok(())
}

fn run_prompt()
{
    let stdin = io::stdin();
    print!("> ");
    stdout().flush().unwrap();
    for line in stdin.lock().lines()
    {
        if let Ok(line) = line
        {
            if line.is_empty()
            {
                break;
            }
            let _ = run(line);
        }
        else
        {
            break;
        }

        print!("> ");
        stdout().flush().unwrap();
    }
}

fn run(source: String) -> Result<(), LoxError>
{
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    let mut parser = Parser::new(tokens);

    match parser.parse()
    {
        None => return Ok(()),
        Some(expr) =>
        {
            let printer = AstPrinter {};
            println!("AST Printer:\n{}", printer.print(&expr)?)
        }
    }

    Ok(())
}
