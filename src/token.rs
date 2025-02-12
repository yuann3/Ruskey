#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    /// Represents an invalid or unknown token
    Illegal,
    Eof,

    // Identifiers + Literals
    Ident,
    Int,

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    // Keywords
    Function,
    Let,
}

/// Represents a token in the Monkey programming language
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    /// Creates a new token with the specified type and literal value
    ///
    /// # Arguments
    /// * `token_type` - The type of token to create
    /// * `literal` - The literal string value of the token
    pub fn new(token_type: TokenType, literal: String) -> Token {
        Token {
            token_type,
            literal,
        }
    }

    /// Determines if an identifier is a keyword and returns the appropriate TokenType
    ///
    /// # Arguments
    /// * `ident` - The identifier string to look up
    ///
    pub fn lookup_ident(ident: &str) -> TokenType {
        match ident {
            "fn" => TokenType::Function,
            "let" => TokenType::Let,
            _ => TokenType::Ident,
        }
    }
}
