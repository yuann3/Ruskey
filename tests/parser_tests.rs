use ruskey::ast::{
    Expression, ExpressionStatement, IntegerLiteral, LetStatement, Node, PrefixExpression,
    ReturnStatement, Statement,
};
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
    // We need to downcast to IntegerLiteral

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

#[test]
fn test_return_statements() {
    let input = r#"
return 5;
return 10;
return 993322;
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

    for stmt in &program.statements {
        let return_stmt = stmt
            .as_any()
            .downcast_ref::<ReturnStatement>()
            .unwrap_or_else(|| panic!("stmt not ReturnStatement. got={:?}", stmt));

        assert_eq!(
            return_stmt.token_literal(),
            "return",
            "return_stmt.token_literal not 'return'. got={}",
            return_stmt.token_literal()
        );
    }
}

#[test]
fn test_parsing_prefix_expressions() {
    struct PrefixTest {
        input: &'static str,
        operator: &'static str,
        integer_value: i64,
    }

    let prefix_tests = vec![
        PrefixTest {
            input: "!5;",
            operator: "!",
            integer_value: 5,
        },
        PrefixTest {
            input: "-15;",
            operator: "-",
            integer_value: 15,
        },
    ];

    for test in prefix_tests {
        let l = Lexer::new(test.input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);

        assert_eq!(
            program.statements.len(),
            1,
            "program.statements does not contain {} statements. got={}",
            1,
            program.statements.len()
        );

        let stmt = program.statements[0]
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .expect("statement is not ExpressionStatement");

        let exp = stmt
            .expression
            .as_any()
            .downcast_ref::<PrefixExpression>()
            .expect("expression not PrefixExpression");

        assert_eq!(
            exp.operator, test.operator,
            "exp.operator is not '{}'",
            test.operator
        );

        // Using a pattern match to safely access the integer literal
        let integer = exp
            .right
            .as_any()
            .downcast_ref::<IntegerLiteral>()
            .expect("right is not IntegerLiteral");

        assert_eq!(
            integer.value, test.integer_value,
            "integer value not {}. got={}",
            test.integer_value, integer.value
        );
    }
}

fn test_integer_literal(il: &Box<dyn Expression>, value: i64) {
    let int_lit = il
        .as_any()
        .downcast_ref::<IntegerLiteral>()
        .expect("expression not IntegerLiteral");

    assert_eq!(
        int_lit.value, value,
        "integer_literal.value not {}. got={}",
        value, int_lit.value
    );

    assert_eq!(
        int_lit.token_literal(),
        value.to_string(),
        "integer_literal.token_literal not {}. got={}",
        value,
        int_lit.token_literal()
    );
}
