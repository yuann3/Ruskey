use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum ObjectType {
    Integer,
    Boolean,
    Null,
}

impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ObjectType::Integer => write!(f, "INTEGER"),
            ObjectType::Boolean => write!(f, "BOOLEAN"),
            ObjectType::Null => write!(f, "NULL"),
        }
    }
}

/// Trait for all object type
pub trait Object: fmt::Debug {
    /// Returns the type of the object
    fn type_(&self) -> ObjectType;

    /// Returns a string representation of the object
    fn inspect(&self) -> String;
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
}
