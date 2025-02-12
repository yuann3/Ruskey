use crate::lexer::Lexer;
use std::io::{self, BufRead, Write};

pub struct Repl {
    prompt: String,
}

impl Repl {
    pub fn new() -> Self {
        Repl {
            prompt: ">> ".to_string(),
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
}
