use crate::token::Token;
use std::any::Any;
use std::fmt::Debug;

pub trait Node: Any {
    fn token_literal(&self) -> String;
}

pub trait Statement: Node + Debug {
    fn statement_node(&self);
    fn as_any(&self) -> &dyn Any;
}

pub trait Expression: Node + Debug {
    fn expression_node(&self);
}

#[derive(Debug)]
pub struct DummyExpression;

impl Node for DummyExpression {
    fn token_literal(&self) -> String {
        String::new()
    }
}

impl Expression for DummyExpression {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Box<DummyExpression>>,
}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Box<DummyExpression>>,
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if let Some(first) = self.statements.first() {
            first.token_literal()
        } else {
            String::new()
        }
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}
