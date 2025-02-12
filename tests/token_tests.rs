use ruskey::token::{Token, TokenType};

#[test]
fn test_basic_token_functionality() {
    let let_token = Token::new(TokenType::Let, String::from("let"));
    assert_eq!(let_token.token_type, TokenType::Let);
    assert_eq!(let_token.literal, "let");

    let fn_token = Token::new(TokenType::Function, String::from("fn"));
    assert_eq!(fn_token.token_type, TokenType::Function);
    assert_eq!(fn_token.literal, "fn");
}

#[test]
fn test_all_token_types() {
    // Test creation of all token types
    let tokens = vec![
        (TokenType::Illegal, "ILLEGAL"),
        (TokenType::Eof, "EOF"),
        (TokenType::Ident, "x"),
        (TokenType::Int, "5"),
        (TokenType::Assign, "="),
        (TokenType::Minus, "-"),
        (TokenType::Bang, "!"),
        (TokenType::Asterisk, "*"),
        (TokenType::Slash, "/"),
        (TokenType::Lt, "<"),
        (TokenType::Gt, ">"),
        (TokenType::Eq, "=="),
        (TokenType::NotEq, "!="),
        (TokenType::Plus, "+"),
        (TokenType::Comma, ","),
        (TokenType::Semicolon, ";"),
        (TokenType::Lparen, "("),
        (TokenType::Rparen, ")"),
        (TokenType::Lbrace, "{"),
        (TokenType::Rbrace, "}"),
        (TokenType::Function, "fn"),
        (TokenType::Let, "let"),
        (TokenType::True, "true"),
        (TokenType::False, "false"),
        (TokenType::If, "if"),
        (TokenType::Else, "else"),
        (TokenType::Return, "return"),
    ];

    for (token_type, literal) in tokens {
        let token = Token::new(token_type.clone(), String::from(literal));
        assert_eq!(token.literal, literal);
        assert_eq!(token.token_type, token_type);
    }
}
