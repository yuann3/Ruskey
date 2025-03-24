use ruskey::environment::Environment;
use ruskey::evaluator::eval;
use ruskey::lexer::Lexer;
use ruskey::object::{Error, Integer, Object, ObjectType};
use ruskey::parser::Parser;

#[test]
fn test_builtin_functions() {
    struct Test {
        input: &'static str,
        expected: Expected,
    }

    enum Expected {
        Int(i64),
        Error(String),
    }

    let tests = vec![
        Test {
            input: r#"len("")"#,
            expected: Expected::Int(0),
        },
        Test {
            input: r#"len("four")"#,
            expected: Expected::Int(4),
        },
        Test {
            input: r#"len("hello world")"#,
            expected: Expected::Int(11),
        },
        Test {
            input: "len(1)",
            expected: Expected::Error("argument to `len` not supported, got INTEGER".to_string()),
        },
        Test {
            input: r#"len("one", "two")"#,
            expected: Expected::Error("wrong number of arguments. got=2, want=1".to_string()),
        },
    ];

    for test in tests {
        let evaluated = test_eval(test.input);

        match test.expected {
            Expected::Int(expected) => {
                test_integer_object(&evaluated, expected);
            }
            Expected::Error(expected) => {
                let error = evaluated
                    .as_any()
                    .downcast_ref::<Error>()
                    .expect("Object is not Error");
                assert_eq!(
                    error.message, expected,
                    "wrong error message. expected={}, got={}",
                    expected, error.message
                );
            }
        }
    }
}

fn test_eval(input: &str) -> Box<dyn Object> {
    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    let mut env = Environment::new();
    eval(&program, &mut env)
}

fn test_integer_object(obj: &Box<dyn Object>, expected: i64) {
    assert_eq!(obj.type_(), ObjectType::Integer, "Object is not Integer");
    let integer = obj.as_any().downcast_ref::<Integer>().unwrap();
    assert_eq!(
        integer.value, expected,
        "object has wrong value. got={}, want={}",
        integer.value, expected
    );
}
