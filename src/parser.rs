//! Parser module for the Ruskey programming language
//!
//! Implements a recursive descent parser with Pratt parsing for expressions.
//! The parser converts tokens into an Abstract Syntax Tree (AST).

use crate::ast::{
    BlockStatement, Boolean, CallExpression, DummyExpression, Expression, ExpressionStatement,
    FunctionLiteral, Identifier, IfExpression, InfixExpression, IntegerLiteral, LetStatement,
    PrefixExpression, Program, ReturnStatement, Statement,
};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use std::collections::HashMap;

/// Function type for parsing prefix expressions (e.g., -5, !true)
type PrefixParseFn = fn(&mut Parser) -> Option<Box<dyn Expression>>;
/// Function type for parsing infix expressions (e.g., 5 + 3, x * y)
type InfixParseFn = fn(&mut Parser, Box<dyn Expression>) -> Option<Box<dyn Expression>>;

/// Parser state and parsing functions
pub struct Parser {
    /// Lexical analyzer that provides tokens
    lexer: Lexer,
    /// Current token being examined
    cur_token: Token,
    /// Next token in the input
    peek_token: Token,
    /// Collection of parsing errors
    errors: Vec<String>,

    /// Registry of functions for parsing prefix expressions
    prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
    /// Registry of functions for parsing infix expressions
    infix_parse_fns: HashMap<TokenType, InfixParseFn>,
}

/// Operator precedence levels for expression parsing
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(X)
}

impl Precedence {
    /// Maps token types to their precedence levels
    fn from_token_type(token_type: &TokenType) -> Self {
        match token_type {
            TokenType::Eq | TokenType::NotEq => Precedence::Equals,
            TokenType::Lt | TokenType::Gt => Precedence::LessGreater,
            TokenType::Plus | TokenType::Minus => Precedence::Sum,
            TokenType::Slash | TokenType::Asterisk => Precedence::Product,
            TokenType::Lparen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }
}

impl Parser {
    /// Creates a new Parser with registered parsing functions
    pub fn new(mut lexer: Lexer) -> Self {
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();

        let mut p = Parser {
            lexer,
            cur_token,
            peek_token,
            errors: Vec::new(),
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        // Register prefix parse functions
        p.register_prefix(TokenType::Int, Parser::parse_integer_literal);
        p.register_prefix(TokenType::Bang, Parser::parse_prefix_expression);
        p.register_prefix(TokenType::Minus, Parser::parse_prefix_expression);
        p.register_prefix(TokenType::True, Parser::parse_boolean);
        p.register_prefix(TokenType::False, Parser::parse_boolean);
        p.register_prefix(TokenType::Lparen, Parser::parse_grouped_expression);
        p.register_prefix(TokenType::If, Parser::parse_if_expression);
        p.register_prefix(TokenType::Ident, Parser::parse_identifier);
        p.register_prefix(TokenType::Function, Parser::parse_function_literal);

        // Register infix parse functions
        p.register_infix(TokenType::Plus, Parser::parse_infix_expression);
        p.register_infix(TokenType::Minus, Parser::parse_infix_expression);
        p.register_infix(TokenType::Slash, Parser::parse_infix_expression);
        p.register_infix(TokenType::Asterisk, Parser::parse_infix_expression);
        p.register_infix(TokenType::Eq, Parser::parse_infix_expression);
        p.register_infix(TokenType::NotEq, Parser::parse_infix_expression);
        p.register_infix(TokenType::Lt, Parser::parse_infix_expression);
        p.register_infix(TokenType::Gt, Parser::parse_infix_expression);
        p.register_infix(TokenType::Lparen, Parser::parse_call_expression);

        p
    }

    /// Parses the complete program into an AST
    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.cur_token.token_type != TokenType::Eof {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }

        program
    }

    /// Returns any errors encountered during parsing
    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    /// Parses a single statement based on the current token
    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    /// Parses an expression with the given precedence level
    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<dyn Expression>> {
        let prefix = self
            .prefix_parse_fns
            .get(&self.cur_token.token_type)
            .copied();

        if prefix.is_none() {
            let token_type = self.cur_token.token_type.clone();
            self.no_prefix_parse_fn_error(&token_type);
            return None;
        }

        let mut left_exp = prefix.unwrap()(self);

        while !self.peek_token_is(&TokenType::Semicolon) && precedence < self.peek_precedence() {
            let infix = self
                .infix_parse_fns
                .get(&self.peek_token.token_type)
                .copied();

            if infix.is_none() {
                return left_exp;
            }

            self.next_token();

            left_exp = match left_exp {
                Some(le) => infix.unwrap()(self, le),
                None => return None,
            };
        }

        left_exp
    }

    fn parse_expression_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.parse_expression(Precedence::Lowest) {
            Some(expression) => {
                let stmt = ExpressionStatement {
                    token: self.cur_token.clone(),
                    expression,
                };

                if self.peek_token_is(&TokenType::Semicolon) {
                    self.next_token();
                }

                Some(Box::new(stmt))
            }
            None => None,
        }
    }

    fn parse_grouped_expression(&mut self) -> Option<Box<dyn Expression>> {
        self.next_token();

        let exp = self.parse_expression(Precedence::Lowest);

        if !self.expect_peek(TokenType::Rparen) {
            return None;
        }

        exp
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        self.next_token();

        let value = self.parse_expression(Precedence::Lowest);

        if self.peek_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        let stmt = LetStatement { token, name, value };

        Some(Box::new(stmt))
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token = self.cur_token.clone();

        self.next_token();

        let return_value = match self.parse_expression(Precedence::Lowest) {
            Some(expr) => Some(expr),
            None => None,
        };

        if self.peek_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        let stmt = ReturnStatement {
            token,
            return_value,
        };

        Some(Box::new(stmt))
    }

    fn parse_identifier(&mut self) -> Option<Box<dyn Expression>> {
        Some(Box::new(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        }))
    }

    fn parse_integer_literal(&mut self) -> Option<Box<dyn Expression>> {
        let mut lit = IntegerLiteral {
            token: self.cur_token.clone(),
            value: 0,
        };

        match self.cur_token.literal.parse::<i64>() {
            Ok(value) => {
                lit.value = value;
                Some(Box::new(lit))
            }
            Err(_) => {
                let msg = format!("could not parse {} as integer", self.cur_token.literal);
                self.errors.push(msg);
                None
            }
        }
    }

    fn parse_prefix_expression(&mut self) -> Option<Box<dyn Expression>> {
        let mut expression = PrefixExpression {
            token: self.cur_token.clone(),
            operator: self.cur_token.literal.clone(),
            right: Box::new(DummyExpression),
        };

        self.next_token();

        match self.parse_expression(Precedence::Prefix) {
            Some(right) => {
                expression.right = right;
                Some(Box::new(expression))
            }
            None => None,
        }
    }

    fn parse_infix_expression(&mut self, left: Box<dyn Expression>) -> Option<Box<dyn Expression>> {
        let mut expression = InfixExpression {
            token: self.cur_token.clone(),
            operator: self.cur_token.literal.clone(),
            left,
            right: Box::new(DummyExpression),
        };

        let precedence = self.cur_precedence();
        self.next_token();

        match self.parse_expression(precedence) {
            Some(right) => {
                expression.right = right;
                Some(Box::new(expression))
            }
            None => None,
        }
    }

    fn parse_if_expression(&mut self) -> Option<Box<dyn Expression>> {
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::Lparen) {
            return None;
        }

        // Parse the condition
        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek(TokenType::Rparen) {
            return None;
        }

        if !self.expect_peek(TokenType::Lbrace) {
            return None;
        }

        let consequence = self.parse_block_statement();

        // Check if there's an 'else'
        let alternative = if self.peek_token_is(&TokenType::Else) {
            self.next_token(); // Now current token is 'else'

            if !self.expect_peek(TokenType::Lbrace) {
                return None;
            }

            Some(self.parse_block_statement())
        } else {
            None
        };

        Some(Box::new(IfExpression {
            token,
            condition,
            consequence,
            alternative,
        }))
    }

    fn parse_block_statement(&mut self) -> BlockStatement {
        let token = self.cur_token.clone();
        let mut statements = Vec::new();

        self.next_token();

        while !self.cur_token_is(TokenType::Rbrace) && !self.cur_token_is(TokenType::Eof) {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }

        BlockStatement { token, statements }
    }

    fn parse_function_literal(&mut self) -> Option<Box<dyn Expression>> {
        let token = self.cur_token.clone();

        if !self.expect_peek(TokenType::Lparen) {
            return None;
        }

        let parameters = self.parse_function_parameters()?;

        if !self.expect_peek(TokenType::Lbrace) {
            return None;
        }

        let body = self.parse_block_statement();

        Some(Box::new(FunctionLiteral {
            token,
            parameters,
            body,
        }))
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<Identifier>> {
        let mut identifiers = Vec::new();

        if self.peek_token_is(&TokenType::Rparen) {
            self.next_token();
            return Some(identifiers);
        }

        // parse first parameter
        self.next_token();

        let ident = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };
        identifiers.push(ident);

        // parse subsequent parameters
        while self.peek_token_is(&TokenType::Comma) {
            self.next_token();
            self.next_token();

            let ident = Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            };
            identifiers.push(ident);
        }

        if !self.expect_peek(TokenType::Rparen) {
            return None;
        }

        Some(identifiers)
    }

    fn parse_call_expression(
        &mut self,
        function: Box<dyn Expression>,
    ) -> Option<Box<dyn Expression>> {
        let exp = CallExpression {
            token: self.cur_token.clone(),
            function,
            arguments: self.parse_call_arguments(),
        };

        Some(Box::new(exp))
    }

    fn parse_call_arguments(&mut self) -> Vec<Box<dyn Expression>> {
        let mut args = Vec::new();

        if self.peek_token_is(&TokenType::Rparen) {
            self.next_token();
            return args;
        }

        // parse first argument
        self.next_token();
        if let Some(arg) = self.parse_expression(Precedence::Lowest) {
            args.push(arg);
        }

        while self.peek_token_is(&TokenType::Comma) {
            self.next_token(); // consume comma
            self.next_token();

            if let Some(arg) = self.parse_expression(Precedence::Lowest) {
                args.push(arg);
            }
        }

        if !self.expect_peek(TokenType::Rparen) {
            return Vec::new();
        }

        args
    }

    fn register_prefix(&mut self, token_type: TokenType, function: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_type, function);
    }

    fn register_infix(&mut self, token_type: TokenType, function: InfixParseFn) {
        self.infix_parse_fns.insert(token_type, function);
    }

    fn peek_precedence(&self) -> Precedence {
        Precedence::from_token_type(&self.peek_token.token_type)
    }

    fn cur_precedence(&self) -> Precedence {
        Precedence::from_token_type(&self.cur_token.token_type)
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn peek_token_is(&self, token_type: &TokenType) -> bool {
        &self.peek_token.token_type == token_type
    }

    fn parse_boolean(&mut self) -> Option<Box<dyn Expression>> {
        Some(Box::new(Boolean {
            token: self.cur_token.clone(),
            value: self.cur_token_is(TokenType::True),
        }))
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        self.cur_token.token_type == t
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token.token_type == t {
            self.next_token();
            true
        } else {
            self.peek_error(t);
            false
        }
    }

    fn no_prefix_parse_fn_error(&mut self, token_type: &TokenType) {
        let msg = format!("no prefix parse function for {:?} found", token_type);
        self.errors.push(msg);
    }

    fn peek_error(&mut self, t: TokenType) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            t, self.peek_token.token_type
        );
        self.errors.push(msg);
    }
}
