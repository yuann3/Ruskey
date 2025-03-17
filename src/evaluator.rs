use crate::ast::{
    self, BlockStatement, Expression, ExpressionStatement, InfixExpression, IntegerLiteral,
    LetStatement, PrefixExpression, Program, ReturnStatement, Statement,
};
use crate::environment::Environment;
use crate::object::{Boolean, Error, Function, Integer, Null, Object, ObjectType, ReturnValue};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::OnceLock;

/// Static TRUE and FALSE objects
fn true_obj() -> &'static Boolean {
    static TRUE: OnceLock<Boolean> = OnceLock::new();
    TRUE.get_or_init(|| Boolean::new(true))
}

fn false_obj() -> &'static Boolean {
    static FALSE: OnceLock<Boolean> = OnceLock::new();
    FALSE.get_or_init(|| Boolean::new(false))
}

/// Null obj
fn null_obj() -> &'static Null {
    static NULL: OnceLock<Null> = OnceLock::new();
    NULL.get_or_init(Null::new)
}

/// Convert Rust bool to monkey Boolean Obj
fn native_bool_to_boolean_object(input: bool) -> Box<dyn Object> {
    if input {
        Box::new(true_obj().clone())
    } else {
        Box::new(false_obj().clone())
    }
}

pub fn eval(program: &Program, env: &mut Environment) -> Box<dyn Object> {
    eval_program(program, env)
}

/// Create new error object
fn new_error(message: &str) -> Box<dyn Object> {
    Box::new(Error::new(message.to_string()))
}

fn is_error(obj: &dyn Object) -> bool {
    obj.type_() == ObjectType::Error
}

fn eval_program(program: &Program, env: &mut Environment) -> Box<dyn Object> {
    let mut result: Box<dyn Object> = Box::new(null_obj().clone());

    for statement in &program.statements {
        result = eval_statement(statement.as_ref(), env);

        if is_error(&*result) {
            return result;
        }

        // handle return value
        if let Some(return_value) = result.as_any().downcast_ref::<ReturnValue>() {
            // ectract the acatual value
            match return_value.value.type_() {
                ObjectType::Integer => {
                    if let Some(int) = return_value.value.as_any().downcast_ref::<Integer>() {
                        return Box::new(Integer::new(int.value));
                    }
                }
                ObjectType::Boolean => {
                    if let Some(boolean) = return_value.value.as_any().downcast_ref::<Boolean>() {
                        return native_bool_to_boolean_object(boolean.value);
                    }
                }
                ObjectType::Null => {
                    return Box::new(null_obj().clone());
                }
                _ => {}
            }
            return Box::new(null_obj().clone());
        }
    }

    result
}

fn eval_statement(statement: &dyn Statement, env: &mut Environment) -> Box<dyn Object> {
    match statement.as_any().downcast_ref::<ExpressionStatement>() {
        Some(expr_stmt) => {
            let result = eval_expression(expr_stmt.expression.as_ref(), env);
            if is_error(&*result) {
                return result;
            }
            result
        }
        None => {
            if let Some(return_stmt) = statement.as_any().downcast_ref::<ReturnStatement>() {
                if let Some(return_val) = &return_stmt.return_value {
                    let val = eval_expression(return_val.as_ref(), env);
                    if is_error(&*val) {
                        return val;
                    }
                    return Box::new(ReturnValue::new(val));
                }
                return Box::new(null_obj().clone());
            }

            // Handle let statements
            if let Some(let_stmt) = statement.as_any().downcast_ref::<LetStatement>() {
                if let Some(val_expr) = &let_stmt.value {
                    let val = eval_expression(val_expr.as_ref(), env);
                    if is_error(&*val) {
                        return val;
                    }
                    return env.set(let_stmt.name.value.clone(), val);
                }
                return Box::new(null_obj().clone());
            }

            Box::new(null_obj().clone())
        }
    }
}

fn eval_expression(expression: &dyn Expression, env: &mut Environment) -> Box<dyn Object> {
    if let Some(int_lit) = expression.as_any().downcast_ref::<IntegerLiteral>() {
        return Box::new(Integer::new(int_lit.value));
    }

    if let Some(bool_lit) = expression.as_any().downcast_ref::<ast::Boolean>() {
        return native_bool_to_boolean_object(bool_lit.value);
    }

    // Handle identifiers
    if let Some(ident) = expression.as_any().downcast_ref::<ast::Identifier>() {
        return eval_identifier(ident, env);
    }

    if let Some(prefix) = expression.as_any().downcast_ref::<PrefixExpression>() {
        let right = eval_expression(prefix.right.as_ref(), env);

        // Check for errors in the right expression
        if is_error(&*right) {
            return right;
        }

        return eval_prefix_expression(&prefix.operator, right);
    }

    if let Some(infix) = expression.as_any().downcast_ref::<InfixExpression>() {
        let left = eval_expression(infix.left.as_ref(), env);

        // Check for errors in left expression
        if is_error(&*left) {
            return left;
        }

        let right = eval_expression(infix.right.as_ref(), env);

        // Check for errors in right expression
        if is_error(&*right) {
            return right;
        }

        return eval_infix_expression(&infix.operator, left, right);
    }

    if let Some(if_expr) = expression.as_any().downcast_ref::<ast::IfExpression>() {
        return eval_if_expression(if_expr, env);
    }

    if let Some(fn_lit) = expression.as_any().downcast_ref::<ast::FunctionLiteral>() {
        let env_rc = Rc::new(RefCell::new(env.clone()));

        let fn_rc = Rc::new(fn_lit.clone());

        return Box::new(Function::new(fn_rc, env_rc));
    }

    if let Some(call) = expression.as_any().downcast_ref::<ast::CallExpression>() {
        let function = eval_expression(call.function.as_ref(), env);
        if is_error(&*function) {
            return function;
        }

        let args = eval_expressions(&call.arguments, env);
        if !args.is_empty() && is_error(&*args[0]) {
            return args[0].clone();
        }

        return apply_function(function, args);
    }

    Box::new(null_obj().clone())
}

fn eval_expressions(exps: &[Box<dyn Expression>], env: &mut Environment) -> Vec<Box<dyn Object>> {
    let mut result = Vec::new();

    for exp in exps {
        let evaluated = eval_expression(exp.as_ref(), env);
        if is_error(&*evaluated) {
            return vec![evaluated];
        }
        result.push(evaluated);
    }

    result
}

fn apply_function(func: Box<dyn Object>, args: Vec<Box<dyn Object>>) -> Box<dyn Object> {
    let function = match func.as_any().downcast_ref::<Function>() {
        Some(f) => f,
        None => return new_error(&format!("not a function: {}", func.type_())),
    };

    let mut extended_env = extend_function_env(function, &args);

    // Evaluate the function body with the extended environment
    let evaluated = eval_block_statement(&function.body_node.body, &mut extended_env);

    // Unwrap any return value objects
    unwrap_return_value(evaluated)
}

fn extend_function_env(function: &Function, args: &[Box<dyn Object>]) -> Environment {
    let mut extended_env = Environment::new_enclosed(Rc::clone(&function.env));

    // Bind each parameter to the corresponding argument
    for (param_idx, param) in function.parameters.iter().enumerate() {
        if param_idx < args.len() {
            extended_env.set(param.value.clone(), args[param_idx].clone());
        }
    }

    extended_env
}

fn unwrap_return_value(obj: Box<dyn Object>) -> Box<dyn Object> {
    if let Some(return_value) = obj.as_any().downcast_ref::<ReturnValue>() {
        return return_value.value.clone();
    }
    obj
}

fn eval_identifier(node: &ast::Identifier, env: &Environment) -> Box<dyn Object> {
    match env.get(&node.value) {
        Some(val) => val.clone(),
        None => new_error(&format!("identifier not found: {}", node.value)),
    }
}

fn eval_if_expression(if_expression: &ast::IfExpression, env: &mut Environment) -> Box<dyn Object> {
    let condition = eval_expression(if_expression.condition.as_ref(), env);

    if is_error(&*condition) {
        return condition;
    }

    if is_truthy(condition) {
        eval_block_statement(&if_expression.consequence, env)
    } else if let Some(alt) = &if_expression.alternative {
        eval_block_statement(alt, env)
    } else {
        Box::new(null_obj().clone())
    }
}

fn eval_block_statement(block: &BlockStatement, env: &mut Environment) -> Box<dyn Object> {
    let mut result: Box<dyn Object> = Box::new(null_obj().clone());

    for statement in &block.statements {
        result = eval_statement(statement.as_ref(), env);

        match result.type_() {
            ObjectType::ReturnValue | ObjectType::Error => return result,
            _ => {}
        }
    }

    result
}

fn is_truthy(obj: Box<dyn Object>) -> bool {
    match obj.type_() {
        ObjectType::Null => false,
        ObjectType::Boolean => {
            if let Some(boolean) = obj.as_any().downcast_ref::<Boolean>() {
                boolean.value
            } else {
                false
            }
        }
        _ => true,
    }
}

fn eval_infix_expression(
    operator: &str,
    left: Box<dyn Object>,
    right: Box<dyn Object>,
) -> Box<dyn Object> {
    if left.type_() == ObjectType::Integer && right.type_() == ObjectType::Integer {
        return eval_integer_infix_expression(operator, left, right);
    }

    if left.type_() != right.type_() {
        return new_error(&format!(
            "type mismatch: {} {} {}",
            left.type_(),
            operator,
            right.type_()
        ));
    }

    if operator == "==" {
        let left_is_true = left
            .as_any()
            .downcast_ref::<Boolean>()
            .map_or(false, |b| b.value);
        let right_is_true = right
            .as_any()
            .downcast_ref::<Boolean>()
            .map_or(false, |b| b.value);
        return native_bool_to_boolean_object(left_is_true == right_is_true);
    }

    if operator == "!=" {
        let left_is_true = left
            .as_any()
            .downcast_ref::<Boolean>()
            .map_or(false, |b| b.value);
        let right_is_true = right
            .as_any()
            .downcast_ref::<Boolean>()
            .map_or(false, |b| b.value);
        return native_bool_to_boolean_object(left_is_true != right_is_true);
    }

    new_error(&format!(
        "unknown operator: {} {} {}",
        left.type_(),
        operator,
        right.type_()
    ))
}

fn eval_integer_infix_expression(
    operator: &str,
    left: Box<dyn Object>,
    right: Box<dyn Object>,
) -> Box<dyn Object> {
    let left_val = left.as_any().downcast_ref::<Integer>().unwrap().value;
    let right_val = right.as_any().downcast_ref::<Integer>().unwrap().value;

    match operator {
        "+" => Box::new(Integer::new(left_val + right_val)),
        "-" => Box::new(Integer::new(left_val - right_val)),
        "*" => Box::new(Integer::new(left_val * right_val)),
        "/" => Box::new(Integer::new(left_val / right_val)),
        "<" => native_bool_to_boolean_object(left_val < right_val),
        ">" => native_bool_to_boolean_object(left_val > right_val),
        "==" => native_bool_to_boolean_object(left_val == right_val),
        "!=" => native_bool_to_boolean_object(left_val != right_val),
        _ => Box::new(null_obj().clone()),
    }
}

fn eval_prefix_expression(operator: &str, right: Box<dyn Object>) -> Box<dyn Object> {
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        _ => new_error(&format!("unknown operator: {}{}", operator, right.type_())),
    }
}

fn eval_bang_operator_expression(right: Box<dyn Object>) -> Box<dyn Object> {
    if let Some(boolean) = right.as_any().downcast_ref::<Boolean>() {
        return native_bool_to_boolean_object(!boolean.value);
    }

    if right.as_any().downcast_ref::<Null>().is_some() {
        return native_bool_to_boolean_object(true);
    }

    native_bool_to_boolean_object(false)
}

fn eval_minus_prefix_operator_expression(right: Box<dyn Object>) -> Box<dyn Object> {
    if right.type_() != ObjectType::Integer {
        return new_error(&format!("unknown operator: -{}", right.type_()));
    }

    if let Some(integer) = right.as_any().downcast_ref::<Integer>() {
        return Box::new(Integer::new(-integer.value));
    }

    Box::new(null_obj().clone())
}
