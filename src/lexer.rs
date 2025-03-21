use crate::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    /// Creates a new Lexer instance
    ///
    /// # Arguments
    /// * `input` - Source code string to tokenize
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        lexer.read_char();
        lexer
    }

    /// Reads the next character in the input and advances the position
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    /// Peek next char without advancing the position
    fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        }
    }

    /// Returns the next token in the input
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            b'"' => {
                let literal = self.read_string();
                Token::new(TokenType::String, literal)
            }
            b'=' => {
                if self.peek_char() == b'=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", ch as char, self.ch as char);
                    Token::new(TokenType::Eq, literal)
                } else {
                    Token::new(TokenType::Assign, String::from("="))
                }
            }
            b'!' => {
                if self.peek_char() == b'=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", ch as char, self.ch as char);
                    Token::new(TokenType::NotEq, literal)
                } else {
                    Token::new(TokenType::Bang, String::from("!"))
                }
            }
            b'+' => Token::new(TokenType::Plus, String::from("+")),
            b'-' => Token::new(TokenType::Minus, String::from("-")),
            b'/' => Token::new(TokenType::Slash, String::from("/")),
            b'*' => Token::new(TokenType::Asterisk, String::from("*")),
            b'<' => Token::new(TokenType::Lt, String::from("<")),
            b'>' => Token::new(TokenType::Gt, String::from(">")),
            b'(' => Token::new(TokenType::Lparen, String::from("(")),
            b')' => Token::new(TokenType::Rparen, String::from(")")),
            b'{' => Token::new(TokenType::Lbrace, String::from("{")),
            b'}' => Token::new(TokenType::Rbrace, String::from("}")),
            b',' => Token::new(TokenType::Comma, String::from(",")),
            b';' => Token::new(TokenType::Semicolon, String::from(";")),
            0 => Token::new(TokenType::Eof, String::from("")),
            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let token_type = Token::lookup_ident(&literal);
                    return Token::new(token_type, literal);
                } else if is_digit(self.ch) {
                    let literal = self.read_numbers();
                    return Token::new(TokenType::Int, literal);
                } else {
                    Token::new(TokenType::Illegal, String::from(""))
                }
            }
        };

        self.read_char();
        tok
    }

    /// Reads an identifier from the input
    ///
    /// Continues reading until it encounters a non-letter character
    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.position < self.input.len() && is_letter(self.ch) {
            self.read_char()
        }
        self.input[position..self.position].to_string()
    }

    /// Reads a number from the input
    ///
    /// Continues reading until it encounters a non-digit character
    fn read_numbers(&mut self) -> String {
        let position = self.position;
        while self.position < self.input.len() && is_digit(self.ch) {
            self.read_char()
        }
        self.input[position..self.position].to_string()
    }

    /// Skips whitespace characters in the input
    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    /// Reads String from the input
    fn read_string(&mut self) -> String {
        let position = self.position + 1;

        loop {
            self.read_char();
            if self.ch == b'"' || self.ch == 0 {
                break;
            }
        }

        self.input[position..self.position].to_string()
    }
}

fn is_digit(ch: u8) -> bool {
    ch.is_ascii_digit() || ch == b'_'
}

fn is_letter(ch: u8) -> bool {
    ch.is_ascii_alphabetic() || ch == b'_'
}
