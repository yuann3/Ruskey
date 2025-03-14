use crate::ast::{BlockStatement, Identifier};
use crate::environment::Environment;
use std::any::Any;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum ObjectType {
    Integer,
    Boolean,
    Null,
    ReturnValue,
    Function,
    Error,
}

impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ObjectType::Integer => write!(f, "INTEGER"),
            ObjectType::Boolean => write!(f, "BOOLEAN"),
            ObjectType::Null => write!(f, "NULL"),
            ObjectType::Function => write!(f, "FUNCTION"),
            ObjectType::ReturnValue => write!(f, "RETURN_VALUE"),
            ObjectType::Error => write!(f, "ERROR"),
        }
    }
}

/// Trait for all object type
pub trait Object: fmt::Debug {
    /// Returns the type of the object
    fn type_(&self) -> ObjectType;

    /// Returns a string representation of the object
    fn inspect(&self) -> String;

    /// Returns self as Any for downcasting
    fn as_any(&self) -> &dyn Any;
}

/// Integer object
#[derive(Debug, Clone, PartialEq)]
pub struct Integer {
    pub value: i64,
}

impl Integer {
    pub fn new(value: i64) -> Self {
        Integer { value }
    }
}

impl Object for Integer {
    fn type_(&self) -> ObjectType {
        ObjectType::Integer
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Boolean object
#[derive(Debug, Clone, PartialEq)]
pub struct Boolean {
    pub value: bool,
}

impl Boolean {
    pub fn new(value: bool) -> Self {
        Boolean { value }
    }
}

impl Object for Boolean {
    fn type_(&self) -> ObjectType {
        ObjectType::Boolean
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Null object
#[derive(Debug, Clone, PartialEq)]
pub struct Null;

impl Null {
    pub fn new() -> Self {
        Null
    }
}

impl Object for Null {
    fn type_(&self) -> ObjectType {
        ObjectType::Null
    }

    fn inspect(&self) -> String {
        "null".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// ReturnValue struct
#[derive(Debug)]
pub struct ReturnValue {
    pub value: Box<dyn Object>,
}

impl ReturnValue {
    pub fn new(value: Box<dyn Object>) -> Self {
        ReturnValue { value }
    }
}

impl Object for ReturnValue {
    fn type_(&self) -> ObjectType {
        ObjectType::ReturnValue
    }

    fn inspect(&self) -> String {
        self.value.inspect()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Function
#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
    pub env: Rc<RefCell<Environment>>,
}

impl Function {
    pub fn new(
        parameters: Vec<Identifier>,
        body: BlockStatement,
        env: Rc<RefCell<Environment>>,
    ) -> Self {
        Function {
            parameters,
            body,
            env,
        }
    }
}

impl Object for Function {
    fn type_(&self) -> ObjectType {
        ObjectType::Function
    }

    fn inspect(&self) -> String {
        let mut out = String::new();

        let params: Vec<String> = self.parameters.iter().map(|p| p.value.clone()).collect();

        out.push_str("fn(");
        out.push_str(&params.join(", "));
        out.push_str(") {\n");
        out.push_str(&format!("{}", self.body));
        out.push_str("\n}");

        out
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Error Handling
#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new(message: String) -> Self {
        Error { message }
    }
}

impl Object for Error {
    fn type_(&self) -> ObjectType {
        ObjectType::Error
    }

    fn inspect(&self) -> String {
        format!("ERROR: {}", self.message)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
