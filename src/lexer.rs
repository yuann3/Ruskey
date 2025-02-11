use crate::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,      // current position
            read_position: 0, // next position of current position
            ch: 0,
        }
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        let tok = match self.ch {
            b'=' => Token::new(TokenType::Assign, String::from("=")),
            b'+' => Token::new(TokenType::Plus, String::from("+")),
            b'(' => Token::new(TokenType::Lparen, String::from("(")),
            b')' => Token::new(TokenType::Rparen, String::from(")")),
            b'{' => Token::new(TokenType::Lbrace, String::from("{")),
            b'}' => Token::new(TokenType::Rbrace, String::from("}")),
            b',' => Token::new(TokenType::Comma, String::from(",")),
            b';' => Token::new(TokenType::Semicolon, String::from(";")),
            0 => Token::new(TokenType::Eof, String::from("")),
            _ => Token::new(TokenType::Illegal, String::from("")),
        }

        self.read_char();
        tok
    }
}
