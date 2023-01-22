// use crate::_ast_printer::AstPrinter;
use crate::error::*;
use crate::interpreter::*;
use crate::lexer::*;
use crate::parse::Parser;
use std::io::{self, stdout, BufRead, Write};

pub struct Lox
{
    interpreter: Interpreter,
}

impl Lox
{
    pub fn new() -> Self
    {
        Self {
            interpreter: Interpreter {},
        }
    }

    pub fn run_file(&self, path: &String) -> io::Result<()>
    {
        let buf = std::fs::read_to_string(path)?;
        if self.run(buf).is_err()
        {
            std::process::exit(65);
        }

        Ok(())
    }

    pub fn run_prompt(&self)
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
                let _ = self.run(line);
            }
            else
            {
                break;
            }

            print!("> ");
            stdout().flush().unwrap();
        }
    }

    fn run(&self, source: String) -> Result<(), LoxError>
    {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;
        let mut parser = Parser::new(tokens);


        match parser.parse()
        {
            None => return Ok(()),
            Some(expr) =>
            {
                self.interpreter.interpret(&expr);
                // let printer = AstPrinter {};
                // println!("AST Printer:\n{}", printer.print(&expr)?)
            }
        }

        Ok(())
    }
}