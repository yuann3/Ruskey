use crate::environment::Environment;
use crate::evaluator::eval;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::io::{self, BufRead, Write};

const PROMPT: &str = ">> ";

pub struct Repl {
    prompt: String,
}

impl Repl {
    pub fn new() -> Self {
        Repl {
            prompt: PROMPT.to_string(),
        }
    }

    pub fn start<R: BufRead, W: Write>(&mut self, input: &mut R, output: &mut W) -> io::Result<()> {
        let mut line = String::new();
        let mut env = Environment::new();

        writeln!(output, "Ruskey Console")?;
        writeln!(output, "Type command below")?;

        loop {
            output.write_all(self.prompt.as_bytes())?;
            output.flush()?;

            if input.read_line(&mut line)? == 0 {
                return Ok(());
            }

            let lexer = Lexer::new(line.clone());
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();

            if !parser.errors().is_empty() {
                writeln!(output, "Parser errors:")?;
                for error in parser.errors() {
                    writeln!(output, "\t{}", error)?;
                }
            } else {
                let evaluated = eval(&program, &mut env);

                if evaluated.type_() != crate::object::ObjectType::Function {
                    writeln!(output, "{}", evaluated.inspect())?;
                }
            }

            line.clear(); // Reset line buffer
        }
    }

    pub fn start_lexer_mode<R: BufRead, W: Write>(
        &mut self,
        input: &mut R,
        output: &mut W,
    ) -> io::Result<()> {
        let mut line = String::new();

        loop {
            output.write_all(self.prompt.as_bytes())?;
            output.flush()?;

            if input.read_line(&mut line)? == 0 {
                return Ok(()); // EOF reached
            }

            let mut lexer = Lexer::new(line.clone());

            loop {
                let tok = lexer.next_token();
                writeln!(output, "Type:{:?}, Literal:{}", tok.token_type, tok.literal)?;

                if tok.token_type == crate::token::TokenType::Eof {
                    break;
                }
            }

            line.clear(); // Reset line buffer
        }
    }

    pub fn start_parser_mode<R: BufRead, W: Write>(
        &mut self,
        input: &mut R,
        output: &mut W,
    ) -> io::Result<()> {
        let mut line = String::new();

        loop {
            output.write_all(self.prompt.as_bytes())?;
            output.flush()?;

            if input.read_line(&mut line)? == 0 {
                return Ok(());
            }

            let lexer = Lexer::new(line.clone());
            let mut parser = Parser::new(lexer);

            let program = parser.parse_program();

            if !parser.errors().is_empty() {
                writeln!(output, "Parser errors:")?;
                for error in parser.errors() {
                    writeln!(output, "\t{}", error)?;
                }
            } else {
                writeln!(output, "{}", program)?;
            }

            line.clear();
        }
    }
}
