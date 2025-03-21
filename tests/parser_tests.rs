use ruskey::ast::{
    Boolean, CallExpression, Expression, ExpressionStatement, FunctionLiteral, Identifier,
    IfExpression, InfixExpression, IntegerLiteral, LetStatement, Node, PrefixExpression,
    ReturnStatement, Statement, StringLiteral,
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

#[test]
fn test_parsing_infix_expressions() {
    struct InfixTest {
        input: &'static str,
        left_value: i64,
        operator: &'static str,
        right_value: i64,
    }

    let infix_tests = vec![
        InfixTest {
            input: "5 + 5;",
            left_value: 5,
            operator: "+",
            right_value: 5,
        },
        InfixTest {
            input: "5 - 5;",
            left_value: 5,
            operator: "-",
            right_value: 5,
        },
        InfixTest {
            input: "5 * 5;",
            left_value: 5,
            operator: "*",
            right_value: 5,
        },
        InfixTest {
            input: "5 / 5;",
            left_value: 5,
            operator: "/",
            right_value: 5,
        },
        InfixTest {
            input: "5 > 5;",
            left_value: 5,
            operator: ">",
            right_value: 5,
        },
        InfixTest {
            input: "5 < 5;",
            left_value: 5,
            operator: "<",
            right_value: 5,
        },
        InfixTest {
            input: "5 == 5;",
            left_value: 5,
            operator: "==",
            right_value: 5,
        },
        InfixTest {
            input: "5 != 5;",
            left_value: 5,
            operator: "!=",
            right_value: 5,
        },
    ];

    for test in infix_tests {
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

        // We'll need to implement test_infix_expression after we create the InfixExpression struct
        test_infix_expression(
            &stmt.expression,
            test.left_value,
            test.operator,
            test.right_value,
        );
    }
}

#[test]
fn test_operator_precedence_parsing() {
    let tests = vec![
        ("-a * b", "((-a) * b)"),
        ("!-a", "(!(-a))"),
        ("a + b + c", "((a + b) + c)"),
        ("a + b - c", "((a + b) - c)"),
        ("a * b * c", "((a * b) * c)"),
        ("a * b / c", "((a * b) / c)"),
        ("a + b / c", "(a + (b / c))"),
        ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
        ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
        ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
        ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
        ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
        ("(5 + 5) * 2", "((5 + 5) * 2)"),
        ("2 / (5 + 5)", "(2 / (5 + 5))"),
        ("-(5 + 5)", "(-(5 + 5))"),
        ("!(true == true)", "(!(true == true))"),
        ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
        (
            "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
            "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
        ),
        (
            "add(a + b + c * d / f + g)",
            "add((((a + b) + ((c * d) / f)) + g))",
        ),
    ];

    for (input, expected) in tests {
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);

        let actual = program.to_string();
        assert_eq!(actual, expected, "expected={}, got={}", expected, actual);
    }
}

#[test]
fn test_boolean_expression() {
    let input = "true;";

    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);
    let program = p.parse_program();
    check_parser_errors(&p);

    assert_eq!(
        program.statements.len(),
        1,
        "program statements does not contain 1 statement. got={}",
        program.statements.len()
    );

    let stmt = program.statements[0]
        .as_any()
        .downcast_ref::<ExpressionStatement>()
        .expect("statement is not ExpressionStatement");

    let boolean = stmt
        .expression
        .as_any()
        .downcast_ref::<Boolean>()
        .expect("expression not Boolean");

    assert_eq!(
        boolean.value, true,
        "boolean.value not {}. got={}",
        true, boolean.value
    );
}

#[test]
fn test_if_expression() {
    let input = "if (x < y) { x }";

    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);
    let program = p.parse_program();
    check_parser_errors(&p);

    assert_eq!(
        program.statements.len(),
        1,
        "program.statements does not contain 1 statement. got={}",
        program.statements.len()
    );

    let stmt = program.statements[0]
        .as_any()
        .downcast_ref::<ExpressionStatement>()
        .expect("statement is not ExpressionStatement");

    let if_exp = stmt
        .expression
        .as_any()
        .downcast_ref::<IfExpression>()
        .expect("expression not IfExpression");

    // Test the condition (x < y)
    test_infix_expression(&if_exp.condition, "x", "<", "y");

    // Test the consequence
    let consequence = &if_exp.consequence;
    assert_eq!(
        consequence.statements.len(),
        1,
        "consequence is not 1 statement. got={}",
        consequence.statements.len()
    );

    let consequence_stmt = consequence.statements[0]
        .as_any()
        .downcast_ref::<ExpressionStatement>()
        .expect("consequence statements[0] is not ExpressionStatement");

    test_identifier(&consequence_stmt.expression, "x");

    // Test that alternative is None
    assert!(
        if_exp.alternative.is_none(),
        "if_exp.alternative was not None. got={:?}",
        if_exp.alternative
    );
}

// Add test for if-else expression
#[test]
fn test_if_else_expression() {
    let input = "if (x < y) { x } else { y }";

    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);
    let program = p.parse_program();
    check_parser_errors(&p);

    let stmt = program.statements[0]
        .as_any()
        .downcast_ref::<ExpressionStatement>()
        .expect("statement is not ExpressionStatement");

    let if_exp = stmt
        .expression
        .as_any()
        .downcast_ref::<IfExpression>()
        .expect("expression not IfExpression");

    // Test the condition
    test_infix_expression(&if_exp.condition, "x", "<", "y");

    // Test the consequence
    let consequence = &if_exp.consequence;
    assert_eq!(
        consequence.statements.len(),
        1,
        "consequence is not 1 statement. got={}",
        consequence.statements.len()
    );

    let consequence_stmt = consequence.statements[0]
        .as_any()
        .downcast_ref::<ExpressionStatement>()
        .expect("consequence statements[0] is not ExpressionStatement");

    test_identifier(&consequence_stmt.expression, "x");

    // Test the alternative
    let alternative = if_exp
        .alternative
        .as_ref()
        .expect("if_exp.alternative was None");

    assert_eq!(
        alternative.statements.len(),
        1,
        "alternative is not 1 statement. got={}",
        alternative.statements.len()
    );

    let alternative_stmt = alternative.statements[0]
        .as_any()
        .downcast_ref::<ExpressionStatement>()
        .expect("alternative statements[0] is not ExpressionStatement");

    test_identifier(&alternative_stmt.expression, "y");
}

// Helper function to test identifiers
fn test_identifier(exp: &Box<dyn Expression>, value: &str) {
    let ident = exp
        .as_any()
        .downcast_ref::<Identifier>()
        .expect("expression not Identifier");

    assert_eq!(
        ident.value, value,
        "ident.value not {}. got={}",
        value, ident.value
    );

    assert_eq!(
        ident.token_literal(),
        value,
        "ident.token_literal not {}. got={}",
        value,
        ident.token_literal()
    );
}

fn test_infix_expression(
    exp: &Box<dyn Expression>,
    left: impl Into<Value>,
    operator: &str,
    right: impl Into<Value>,
) {
    let op_exp = exp
        .as_any()
        .downcast_ref::<InfixExpression>()
        .expect("expression is not InfixExpression");

    test_literal_expression(&op_exp.left, left);

    assert_eq!(
        op_exp.operator, operator,
        "exp.operator is not '{}'. got={}",
        operator, op_exp.operator
    );

    test_literal_expression(&op_exp.right, right);
}

// A helper enum to handle different types of values
enum Value {
    Int(i64),
    String(String),
    Bool(bool),
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::Int(i)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

fn test_literal_expression(exp: &Box<dyn Expression>, expected: impl Into<Value>) {
    match expected.into() {
        Value::Int(int) => test_integer_literal(exp, int),
        Value::String(string) => test_identifier(exp, &string),
        Value::Bool(boolean) => test_boolean_literal(exp, boolean),
    }
}

fn test_boolean_literal(exp: &Box<dyn Expression>, value: bool) {
    let bo = exp
        .as_any()
        .downcast_ref::<Boolean>()
        .expect("expression not Boolean");

    assert_eq!(
        bo.value, value,
        "boolean.value not {}. got={}",
        value, bo.value
    );

    assert_eq!(
        bo.token_literal(),
        value.to_string(),
        "boolean.token_literal not {}. got={}",
        value,
        bo.token_literal()
    );
}

#[test]
fn test_function_literal_parsing() {
    let input = "fn(x, y) { x + y; }";

    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);
    let program = p.parse_program();
    check_parser_errors(&p);

    // Check if program has one statement
    assert_eq!(
        program.statements.len(),
        1,
        "program.statements does not contain 1 statement. got={}",
        program.statements.len()
    );

    // Get the expression statement
    let stmt = program.statements[0]
        .as_any()
        .downcast_ref::<ExpressionStatement>()
        .expect("statement is not ExpressionStatement");

    // Check if the expression is a function literal
    let function = stmt
        .expression
        .as_any()
        .downcast_ref::<FunctionLiteral>()
        .expect("exp not FunctionLiteral");

    // Check if the function has the correct number of parameters
    assert_eq!(
        function.parameters.len(),
        2,
        "function literal parameters wrong. want 2, got={}",
        function.parameters.len()
    );

    // Check the parameter names directly
    assert_eq!(function.parameters[0].value, "x");
    assert_eq!(function.parameters[1].value, "y");

    // Check the body
    assert_eq!(
        function.body.statements.len(),
        1,
        "function.body.statements has not 1 statement. got={}",
        function.body.statements.len()
    );

    let body_stmt = function.body.statements[0]
        .as_any()
        .downcast_ref::<ExpressionStatement>()
        .expect("function body stmt is not ast.ExpressionStatement");

    test_infix_expression(&body_stmt.expression, "x", "+", "y");
}

#[test]
fn test_function_parameter_parsing() {
    // Test cases with different parameter counts
    let tests = vec![
        ("fn() {};", vec![]),
        ("fn(x) {};", vec!["x"]),
        ("fn(x, y, z) {};", vec!["x", "y", "z"]),
    ];

    for (input, expected_params) in tests {
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program();
        check_parser_errors(&p);

        let stmt = program.statements[0]
            .as_any()
            .downcast_ref::<ExpressionStatement>()
            .expect("statement is not ExpressionStatement");

        let function = stmt
            .expression
            .as_any()
            .downcast_ref::<FunctionLiteral>()
            .expect("exp not FunctionLiteral");

        assert_eq!(
            function.parameters.len(),
            expected_params.len(),
            "length parameters wrong. want {}, got={}",
            expected_params.len(),
            function.parameters.len()
        );

        // Test parameter values directly
        for (i, ident) in expected_params.iter().enumerate() {
            assert_eq!(
                function.parameters[i].value, *ident,
                "parameter {} value wrong: expected {}, got {}",
                i, ident, function.parameters[i].value
            );
        }
    }
}

#[test]
fn test_call_expression_parsing() {
    let input = "add(1, 2 * 3, 4 + 5);";

    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);
    let program = p.parse_program();
    check_parser_errors(&p);

    assert_eq!(
        program.statements.len(),
        1,
        "program.statements does not contain 1 statement. got={}",
        program.statements.len()
    );

    let stmt = program.statements[0]
        .as_any()
        .downcast_ref::<ExpressionStatement>()
        .expect("statement is not ExpressionStatement");

    // Check if it's a call expression
    let exp = stmt
        .expression
        .as_any()
        .downcast_ref::<CallExpression>()
        .expect("expression is not CallExpression");

    // Test the function part (should be an identifier "add")
    test_identifier(&exp.function, "add");

    // Test that we have the right number of arguments
    assert_eq!(
        exp.arguments.len(),
        3,
        "wrong length of arguments. got={}",
        exp.arguments.len()
    );

    // Test each argument
    test_literal_expression(&exp.arguments[0], 1);
    test_infix_expression(&exp.arguments[1], 2, "*", 3);
    test_infix_expression(&exp.arguments[2], 4, "+", 5);
}

#[test]
fn test_string_literal_expression() {
    let input = r#""hello world";"#;

    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    check_parser_errors(&parser);

    let stmt = program.statements[0]
        .as_any()
        .downcast_ref::<ExpressionStatement>()
        .expect("Expected ExpressionStatement");

    let literal = stmt
        .expression
        .as_any()
        .downcast_ref::<StringLiteral>()
        .expect("Expected StringLiteral");

    assert_eq!(
        literal.value, "hello world",
        "literal.value not correct. got={}",
        literal.value
    );
}
