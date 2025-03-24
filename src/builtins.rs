use crate::object::{Builtin, Error, Integer, Object, ObjectType, StringObj};
use std::collections::HashMap;

/// Create a new error
fn new_error(message: &str) -> Box<dyn Object> {
    Box::new(Error::new(message.to_string()))
}

/// Define the len() function
fn len_function(args: Vec<Box<dyn Object>>) -> Box<dyn Object> {
    if args.len() != 1 {
        return new_error(&format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }

    match args[0].type_() {
        ObjectType::String => {
            let string_obj = args[0].as_any().downcast_ref::<StringObj>().unwrap();
            Box::new(Integer::new(string_obj.value.len() as i64))
        }
        _ => new_error(&format!(
            "argument to `len` not supported, got {}",
            args[0].type_()
        )),
    }
}

// Map for builtin function
pub fn get_builtins() -> HashMap<String, Box<dyn Object>> {
    let mut builtins = HashMap::new();

    builtins.insert(
        "len".to_string(),
        Box::new(Builtin::new(len_function)) as Box<dyn Object>,
    );

    builtins
}
