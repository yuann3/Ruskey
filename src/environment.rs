use crate::object::Object;
use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};

/// Stores variable bindings
#[derive(Debug)]
pub struct Environment {
    store: HashMap<String, Box<dyn Object>>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: Rc<RefCell<Environment>>) -> Self {
        Environment {
            store: HashMap::new(),
            outer: Some(outer),
        }
    }
    /// Gets variable from Environment
    pub fn get(&self, name: &String) -> Option<Box<dyn Object>> {
        match self.store.get(name) {
            Some(obj) => Some(obj.clone()),
            None => {
                if let Some(outer) = &self.outer {
                    outer.borrow().get(name)
                } else {
                    None
                }
            }
        }
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

/// Clone for Enviroment
impl Clone for Environment {
    fn clone(&self) -> Self {
        let mut new_env = Environment {
            store: HashMap::new(),
            outer: self.outer.clone(),
        };

        // Clone each binding
        for (key, val) in &self.store {
            new_env.store.insert(key.clone(), val.clone());
        }

        new_env
    }
}
