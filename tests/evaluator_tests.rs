use ruskey::evaluator::eval;
use ruskey::lexer::Lexer;
use ruskey::object::{Boolean, Integer, Object};
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
