//! Abstract Syntax Tree (AST) module
use crate::token::Token;
use std::any::Any;
use std::fmt;
use std::fmt::Debug;

/// Base trait for all AST nodes
///
/// Every node in our AST must implement this trait to provide
/// a consistent interface for token information.
pub trait Node: Any {
    /// Returns the literal value of the token associated with this node
    fn token_literal(&self) -> String;
}

/// Represents a statement in the language
///
/// Statements are language constructs that perform actions but don't
/// produce values (e.g., let statements, return statements)
pub trait Statement: Node + Debug {
    /// Marker method to identify statement nodes
    fn statement_node(&self);
    /// Enables downcasting to concrete statement types
    fn as_any(&self) -> &dyn Any;
}

/// Represents an expression in the language
///
/// Expressions are language constructs that produce values
/// (e.g., identifiers, literals, operators)
pub trait Expression: Node + Debug {
    /// Marker method to identify expression nodes
    fn expression_node(&self);
    /// Enables downcasting to concrete expression types
    fn as_any(&self) -> &dyn Any;
}

/// A return statement (e.g., "return 5;")
#[derive(Debug)]
pub struct ReturnStatement {
    /// The 'return' token
    pub token: Token,
    /// The value being returned (optional)
    pub return_value: Option<Box<dyn Expression>>,
}

/// An identifier (e.g., variable names)
#[derive(Debug, Clone)]
pub struct Identifier {
    /// The identifier's token
    pub token: Token,
    /// The identifier's string value
    pub value: String,
}

/// A let statement (e.g., "let x = 5;")
#[derive(Debug)]
pub struct LetStatement {
    /// The 'let' token
    pub token: Token,
    /// The identifier being bound
    pub name: Identifier,
    /// The value being assigned (optional)
    pub value: Option<Box<dyn Expression>>,
}

/// The root node of our AST
#[derive(Debug)]
pub struct Program {
    /// Collection of statements that make up the program
    pub statements: Vec<Box<dyn Statement>>,
}

/// A placeholder for expressions that haven't been implemented yet
#[derive(Debug)]
pub struct DummyExpression;

/// A statement that consists of a single expression
#[derive(Debug)]
pub struct ExpressionStatement {
    /// The first token of the expression
    pub token: Token,
    /// The actual expression
    pub expression: Box<dyn Expression>,
}

/// An infix expression (e.g., "a + b", "x * y")
#[derive(Debug)]
pub struct InfixExpression {
    /// The operator token
    pub token: Token,
    /// Left-hand expression
    pub left: Box<dyn Expression>,
    /// The operator string
    pub operator: String,
    /// Right-hand expression
    pub right: Box<dyn Expression>,
}

/// A prefix expression (e.g., "!true", "-5")
#[derive(Debug)]
pub struct PrefixExpression {
    /// The operator token
    pub token: Token,
    /// The operator string
    pub operator: String,
    /// The expression being operated on
    pub right: Box<dyn Expression>,
}

/// An integer literal (e.g., "5", "10")
#[derive(Debug)]
pub struct IntegerLiteral {
    /// The integer token
    pub token: Token,
    /// The parsed integer value
    pub value: i64,
}

/// boolean literal (true or false)
#[derive(Debug)]
pub struct Boolean {
    /// boolean token
    pub token: Token,
    /// boolean value
    pub value: bool,
}

// block statement, a collection of statments
#[derive(Debug)]
pub struct BlockStatement {
    /// opening '{' token
    pub token: Token,
    /// statements in the block
    pub statements: Vec<Box<dyn Statement>>,
}

/// if expression (if (condition) { consequence } else { alternative })
#[derive(Debug)]
pub struct IfExpression {
    /// 'if' token
    pub token: Token,
    /// condition expression
    pub condition: Box<dyn Expression>,
    /// consequence block
    pub consequence: BlockStatement,
    /// optional alternative block
    pub alternative: Option<BlockStatement>,
}

/// function literal (eg. "fn(x, y) { x + y; }"
#[derive(Debug)]
pub struct FunctionLiteral {
    /// 'fn' token
    pub token: Token,
    /// function parameters
    pub parameters: Vec<Identifier>,
    /// function body
    pub body: BlockStatement,
}

/// call expression (eg. "add(1, 2)", "fn(x, y) { x + y; }(1, 2)")
#[derive(Debug)]
pub struct CallExpression {
    /// '(' token
    pub token: Token,
    /// the expression that produces the function
    pub function: Box<dyn Expression>,
    /// argument expression
    pub arguments: Vec<Box<dyn Expression>>,
}

impl Node for CallExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Node for FunctionLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Node for IfExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Node for Boolean {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Node for DummyExpression {
    fn token_literal(&self) -> String {
        String::new()
    }
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
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

impl Statement for ExpressionStatement {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for BlockStatement {
    fn statement_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
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

impl Expression for DummyExpression {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for PrefixExpression {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for InfixExpression {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for Boolean {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for IfExpression {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for FunctionLiteral {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for CallExpression {
    fn expression_node(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for s in &self.statements {
            write!(f, "{}", s)?;
        }
        Ok(())
    }
}

impl fmt::Display for dyn Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(stmt) = self.as_any().downcast_ref::<ExpressionStatement>() {
            return write!(f, "{}", stmt);
        }
        if let Some(stmt) = self.as_any().downcast_ref::<LetStatement>() {
            return write!(f, "{}", stmt);
        }
        if let Some(stmt) = self.as_any().downcast_ref::<ReturnStatement>() {
            return write!(f, "{}", stmt);
        }
        if let Some(stmt) = self.as_any().downcast_ref::<BlockStatement>() {
            return write!(f, "{}", stmt);
        }
        if let Some(expr) = self.as_any().downcast_ref::<IfExpression>() {
            return write!(f, "{}", expr);
        }
        write!(f, "{}", self.token_literal())
    }
}

impl fmt::Display for LetStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} = ",
            self.token_literal(),
            self.name.token_literal()
        )?;
        if let Some(value) = &self.value {
            write!(f, "{}", value)?;
        }
        write!(f, ";")
    }
}

impl fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.token_literal())?;
        if let Some(value) = &self.return_value {
            write!(f, "{}", value)?;
        }
        write!(f, ";")
    }
}

impl fmt::Display for DummyExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl fmt::Display for ExpressionStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expression)
    }
}

impl fmt::Display for dyn Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(expr) = self.as_any().downcast_ref::<PrefixExpression>() {
            return write!(f, "{}", expr);
        }
        if let Some(expr) = self.as_any().downcast_ref::<InfixExpression>() {
            return write!(f, "{}", expr);
        }
        if let Some(expr) = self.as_any().downcast_ref::<IntegerLiteral>() {
            return write!(f, "{}", expr);
        }
        if let Some(expr) = self.as_any().downcast_ref::<Identifier>() {
            return write!(f, "{}", expr);
        }
        if let Some(expr) = self.as_any().downcast_ref::<Boolean>() {
            return write!(f, "{}", expr);
        }
        if let Some(expr) = self.as_any().downcast_ref::<IfExpression>() {
            return write!(f, "{}", expr);
        }
        if let Some(expr) = self.as_any().downcast_ref::<FunctionLiteral>() {
            return write!(f, "{}", expr);
        }
        if let Some(expr) = self.as_any().downcast_ref::<CallExpression>() {
            return write!(f, "{}", expr);
        }
        write!(f, "{}", self.token_literal())
    }
}

impl fmt::Display for PrefixExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}{})", self.operator, self.right)
    }
}

impl fmt::Display for InfixExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
}

impl fmt::Display for IntegerLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Display for BlockStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ")?; // Add opening brace
        for stmt in &self.statements {
            write!(f, "{}", stmt)?;
        }
        write!(f, " }}")
    }
}

impl fmt::Display for IfExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "if{} {}", self.condition, self.consequence)?;

        if let Some(alt) = &self.alternative {
            write!(f, "else {}", alt)?;
        }

        Ok(())
    }
}

impl fmt::Display for FunctionLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params: Vec<String> = self.parameters.iter().map(|p| p.to_string()).collect();

        write!(
            f,
            "{}({}) {}",
            self.token_literal(),
            params.join(", "),
            self.body
        )
    }
}

impl fmt::Display for CallExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args: Vec<String> = self.arguments.iter().map(|a| a.to_string()).collect();

        write!(f, "{}({})", self.function, args.join(", "))
    }
}

impl Clone for BlockStatement {
    fn clone(&self) -> Self {
        BlockStatement {
            token: self.token.clone(),
            statements: Vec::new(),
        }
    }
}

impl Clone for FunctionLiteral {
    fn clone(&self) -> Self {
        FunctionLiteral {
            token: self.token.clone(),
            parameters: self.parameters.clone(),
            body: BlockStatement {
                token: self.body.token.clone(),
                statements: Vec::new(),
            },
        }
    }
}
