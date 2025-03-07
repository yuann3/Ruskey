use crate::ast::{Expression, ExpressionStatement, IntegerLiteral, Program, Statement};
use crate::object::{Integer, Null, Object};

pub fn eval(program: &Program) -> Box<dyn Object> {
    eval_program(program)
}

fn eval_program(program: &Program) -> Box<dyn Object> {
    let mut result: Box<dyn Object> = Box::new(Null::new());
    for statement in &program.statements {
        result = eval_statement(statement.as_ref());
    }
    result
}

fn eval_statement(statement: &dyn Statement) -> Box<dyn Object> {
    match statement.as_any().downcast_ref::<ExpressionStatement>() {
        Some(expr_stmt) => eval_expression(expr_stmt.expression.as_ref()),
        None => Box::new(Null::new()),
    }
}

fn eval_expression(expression: &dyn Expression) -> Box<dyn Object> {
    match expression.as_any().downcast_ref::<IntegerLiteral>() {
        Some(int_lit) => Box::new(Integer::new(int_lit.value)),
        None => Box::new(Null::new()),
    }
}
