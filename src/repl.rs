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

    /// Starts the REPL session with given input and output streams
    pub fn start<R: BufRead, W: Write>(&mut self, input: &mut R, output: &mut W) -> io::Result<()> {
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

    /// Starts the REPL in parser mode, showing parsed AST
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
                return Ok(()); // EOF reached
            }

            let lexer = Lexer::new(line.clone());
            let mut parser = Parser::new(lexer);

            let program = parser.parse_program();

            // Check for parser errors
            if !parser.errors().is_empty() {
                writeln!(output, "Parser errors:")?;
                for error in parser.errors() {
                    writeln!(output, "\t{}", error)?;
                }
            } else {
                // Print the program's string representation
                writeln!(output, "{}", program)?;
            }

            line.clear(); // Reset line buffer
        }
    }
}
