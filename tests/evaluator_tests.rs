use ruskey::evaluator::eval;
use ruskey::lexer::Lexer;
use ruskey::object::{Integer, Object};
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
    // We need to downcast from &dyn Object to &Integer
    let integer = obj.as_any().downcast_ref::<Integer>().unwrap();
    assert_eq!(integer.value, expected, "object has wrong value");
}
