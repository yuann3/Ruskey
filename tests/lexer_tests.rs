use ruskey::lexer::Lexer;
use ruskey::token::{Token, TokenType};

#[test]
fn test_next_token_complex() {
    let input = r#"let five = 5;
let ten = 10;
let add = fn(x, y) {
    x + y;
};
let result = add(five, ten);"#;

    let tests = vec![
        (TokenType::Let, "let"),
        (TokenType::Ident, "five"),
        (TokenType::Assign, "="),
        (TokenType::Int, "5"),
        (TokenType::Semicolon, ";"),
        (TokenType::Let, "let"),
        (TokenType::Ident, "ten"),
        (TokenType::Assign, "="),
        (TokenType::Int, "10"),
        (TokenType::Semicolon, ";"),
        (TokenType::Let, "let"),
        (TokenType::Ident, "add"),
        (TokenType::Assign, "="),
        (TokenType::Function, "fn"),
        (TokenType::Lparen, "("),
        (TokenType::Ident, "x"),
        (TokenType::Comma, ","),
        (TokenType::Ident, "y"),
        (TokenType::Rparen, ")"),
        (TokenType::Lbrace, "{"),
        (TokenType::Ident, "x"),
        (TokenType::Plus, "+"),
        (TokenType::Ident, "y"),
        (TokenType::Semicolon, ";"),
        (TokenType::Rbrace, "}"),
        (TokenType::Semicolon, ";"),
        (TokenType::Let, "let"),
        (TokenType::Ident, "result"),
        (TokenType::Assign, "="),
        (TokenType::Ident, "add"),
        (TokenType::Lparen, "("),
        (TokenType::Ident, "five"),
        (TokenType::Comma, ","),
        (TokenType::Ident, "ten"),
        (TokenType::Rparen, ")"),
        (TokenType::Semicolon, ";"),
        (TokenType::Eof, ""),
    ];

    let mut lexer = Lexer::new(input.to_string());

    for (i, (expected_type, expected_literal)) in tests.iter().enumerate() {
        let tok = lexer.next_token();
        assert_eq!(
            tok.token_type, *expected_type,
            "tests[{}] - wrong token type. expected={:?}, got={:?}",
            i, expected_type, tok.token_type
        );
        assert_eq!(
            tok.literal, *expected_literal,
            "tests[{}] - wrong literal. expected={}, got={}",
            i, expected_literal, tok.literal
        );
    }
}

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
