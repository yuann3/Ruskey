use ruskey::lexer::Lexer;
use ruskey::token::{Token, TokenType};

#[test]
fn test_next_token() {
    let input = "=+(){},;";

    let tests = vec![
        (TokenType::Assign, "="),
        (TokenType::Plus, "+"),
        (TokenType::Lparen, "("),
        (TokenType::Rparen, ")"),
        (TokenType::Lbrace, "{"),
        (TokenType::Rbrace, "}"),
        (TokenType::Comma, ","),
        (TokenType::Semicolon, ";"),
        (TokenType::Eof, ""),
    ];

    let mut lexer = Lexer::new(input.to_string());

    for (expected_type, expected_literal) in tests {
        let tok = lexer.next_token();
        assert_eq!(
            tok.token_type, expected_type,
            "wrong token type. expected={:?}, got={:?}",
            expected_type, tok.token_type
        );
        assert_eq!(
            tok.literal, expected_literal,
            "wrong literal. expected={}, got={}",
            expected_literal, tok.literal
        );
    }
}
