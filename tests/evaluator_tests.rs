use ruskey::evaluator::eval;
use ruskey::lexer::Lexer;
use ruskey::object::{Boolean, Error, Integer, Null, Object};
use ruskey::parser::Parser;

#[test]
fn test_eval_integer_expression() {
    let tests = vec![("5", 5), ("10", 10)];
    for (input, expected) in tests {
        let evaluated = test_eval(input);
        test_integer_object(evaluated.as_ref(), expected);
    }
}

fn test_eval(input: &str) -> Box<dyn Object> {
    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    eval(&program)
}

fn test_integer_object(obj: &dyn Object, expected: i64) {
    let integer = obj.as_any().downcast_ref::<Integer>().unwrap();
    assert_eq!(integer.value, expected, "object has wrong value");
}

#[test]
fn test_eval_boolean_expression() {
    let tests = vec![("true", true), ("false", false)];

    for (input, expected) in tests {
        let evaluated = test_eval(input);
        test_boolean_object(evaluated.as_ref(), expected);
    }
}

fn test_boolean_object(obj: &dyn Object, expected: bool) {
    let boolean = obj.as_any().downcast_ref::<Boolean>().unwrap();
    assert_eq!(
        boolean.value, expected,
        "object has wrong value. got={}, want={}",
        boolean.value, expected
    );
}

#[test]
fn test_bang_operator() {
    let tests = vec![
        ("!true", false),
        ("!false", true),
        ("!5", false),
        ("!!true", true),
        ("!!false", false),
        ("!!5", true),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);
        test_boolean_object(evaluated.as_ref(), expected);
    }
}

#[test]
fn test_minus_operator() {
    let tests = vec![("-5", -5), ("-10", -10), ("--5", 5)];

    for (input, expected) in tests {
        let evaluated = test_eval(input);
        test_integer_object(evaluated.as_ref(), expected);
    }
}

#[test]
fn test_eval_integer_infix_expressions() {
    enum Expected {
        Int(i64),
        Bool(bool),
    }

    let tests = vec![
        ("5 + 5", Expected::Int(10)),
        ("5 - 5", Expected::Int(0)),
        ("5 * 5", Expected::Int(25)),
        ("5 / 5", Expected::Int(1)),
        ("5 > 5", Expected::Bool(false)),
        ("5 < 5", Expected::Bool(false)),
        ("5 == 5", Expected::Bool(true)),
        ("5 != 5", Expected::Bool(false)),
        ("5 > 4", Expected::Bool(true)),
        ("5 < 6", Expected::Bool(true)),
        ("5 == 6", Expected::Bool(false)),
        ("5 != 6", Expected::Bool(true)),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);

        match expected {
            Expected::Int(int) => test_integer_object(evaluated.as_ref(), int),
            Expected::Bool(bool_val) => test_boolean_object(evaluated.as_ref(), bool_val),
        }
    }
}

#[test]
fn test_if_else_expressions() {
    struct Test {
        input: &'static str,
        expected: Option<i64>,
    }

    let tests = vec![
        Test {
            input: "if (true) { 10 }",
            expected: Some(10),
        },
        Test {
            input: "if (false) { 10 }",
            expected: None,
        },
        Test {
            input: "if (1) { 10 }",
            expected: Some(10),
        },
        Test {
            input: "if (1 < 2) { 10 }",
            expected: Some(10),
        },
        Test {
            input: "if (1 > 2) { 10 }",
            expected: None,
        },
        Test {
            input: "if (1 > 2) { 10 } else { 20 }",
            expected: Some(20),
        },
        Test {
            input: "if (1 < 2) { 10 } else { 20 }",
            expected: Some(10),
        },
    ];

    for test in tests {
        let evaluated = test_eval(test.input);

        match test.expected {
            Some(integer) => test_integer_object(evaluated.as_ref(), integer),
            None => test_null_object(evaluated.as_ref()),
        }
    }
}

fn test_null_object(obj: &dyn Object) {
    assert!(
        obj.as_any().downcast_ref::<Null>().is_some(),
        "object is not Null, got={:?}",
        obj
    );
}

#[test]
fn test_return_statements() {
    let tests = vec![
        ("return 10;", 10),
        ("return 10; 9;", 10),
        ("return 2 * 5; 9;", 10),
        ("9; return 2 * 5; 9;", 10),
    ];

    for (input, expected) in tests {
        let evaluated = test_eval(input);
        test_integer_object(evaluated.as_ref(), expected);
    }
}

#[test]
fn test_error_handling() {
    let tests = vec![
        ("5 + true;", "type mismatch: INTEGER + BOOLEAN"),
        ("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN"),
        ("-true", "unknown operator: -BOOLEAN"),
        ("true + false;", "unknown operator: BOOLEAN + BOOLEAN"),
        ("5; true + false; 5", "unknown operator: BOOLEAN + BOOLEAN"),
        (
            "if (10 > 1) { true + false; }",
            "unknown operator: BOOLEAN + BOOLEAN",
        ),
        (
            "if (10 > 1) { if (10 > 1) { return true + false; } return 1; }",
            "unknown operator: BOOLEAN + BOOLEAN",
        ),
    ];

    for (input, expected_message) in tests {
        let evaluated = test_eval(input);

        match evaluated.as_any().downcast_ref::<Error>() {
            Some(error) => {
                assert_eq!(
                    error.message, expected_message,
                    "wrong error message. expected={}, got={}",
                    expected_message, error.message
                );
            }
            None => {
                panic!("no error object returned. got={:?}", evaluated);
            }
        }
    }
}
