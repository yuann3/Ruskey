use ruskey::ast::{LetStatement, Node, Statement};
use ruskey::lexer::Lexer;
use ruskey::parser::Parser;

#[test]
fn test_let_statements() {
    let input = r#"
let x = 5;
let y = 10; 
let foobar = 838383;
"#;
    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);
    let program = p.parse_program();
    check_parser_errors(&p);

    assert_eq!(
        program.statements.len(),
        3,
        "program.statements does not contain 3 statements. got={}",
        program.statements.len()
    );

    let tests = vec!["x", "y", "foobar"];

    for (i, expected_identifier) in tests.iter().enumerate() {
        test_let_statement(&*program.statements[i], expected_identifier);
    }
}

fn check_parser_errors(parser: &Parser) {
    let errors = parser.errors();
    if errors.is_empty() {
        return;
    }
    println!("parser has {} errors", errors.len());
    for error in errors {
        println!("parser error: {}", error);
    }
    panic!("parser had errors");
}

fn test_let_statement(stmt: &dyn Statement, name: &str) {
    let let_stmt = stmt
        .as_any()
        .downcast_ref::<LetStatement>()
        .unwrap_or_else(|| panic!("stmt is not LetStatement. got={:?}", stmt));

    assert_eq!(
        stmt.token_literal(),
        "let",
        "stmt.token_literal not 'let'. got={}",
        stmt.token_literal()
    );

    assert_eq!(
        let_stmt.name.value, name,
        "let_stmt.name.value not '{}'. got={}",
        name, let_stmt.name.value
    );

    assert_eq!(
        let_stmt.name.token_literal(),
        name,
        "let_stmt.name.token_literal() not '{}'. got={}",
        name,
        let_stmt.name.token_literal()
    );
}
