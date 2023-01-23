// use crate::_ast_printer::AstPrinter;
use crate::error::*;
use crate::interpreter::interpreter::*;
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
            interpreter: Interpreter::new(),
        }
    }

    /// Open a file and interpret its contents.
    pub fn run_file(&self, path: &String) -> io::Result<()>
    {
        let buf = std::fs::read_to_string(path)?;
        if self.run(buf).is_err()
        {
            std::process::exit(65);
        }

        Ok(())
    }

    /// Open a REPL (Read-Eval-Print loop) interactive programming environment.
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
        let statements = parser.parse()?;

        if self.interpreter.interpret(&statements)
        {
            Ok(())
        }
        else
        {
            Err(LoxError::error(0, ""))
        }
    }
}
