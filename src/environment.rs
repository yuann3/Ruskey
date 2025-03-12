use crate::object::Object;
use std::collections::HashMap;

/// Stores variable bindings
pub struct Environment {
    store: HashMap<String, Box<dyn Object>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
        }
    }

    /// Gets variable from Environment
    pub fn get(&self, name: &String) -> Option<&Box<dyn Object>> {
        self.store.get(name)
    }

    /// Sets a variable in Environment
    pub fn set(&mut self, name: String, val: Box<dyn Object>) -> Box<dyn Object> {
        self.store.insert(name, val.clone());
        val
    }
}

/// Clone for Box dyn
impl Clone for Box<dyn Object> {
    fn clone(&self) -> Self {
        match self.type_() {
            crate::object::ObjectType::Integer => {
                let int = self
                    .as_any()
                    .downcast_ref::<crate::object::Integer>()
                    .unwrap();
                Box::new(crate::object::Integer::new(int.value))
            }
            crate::object::ObjectType::Boolean => {
                let boolean = self
                    .as_any()
                    .downcast_ref::<crate::object::Boolean>()
                    .unwrap();
                Box::new(crate::object::Boolean::new(boolean.value))
            }
            _ => Box::new(crate::object::Null::new()),
        }
    }
}
