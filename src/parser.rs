use crate::ast::{
    DummyExpression, Expression, ExpressionStatement, Identifier, IntegerLiteral, LetStatement,
    PrefixExpression, Program, ReturnStatement, Statement,
};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use std::collections::HashMap;

type PrefixParseFn = fn(&mut Parser) -> Option<Box<dyn Expression>>;
type InfixParseFn = fn(&mut Parser, Box<dyn Expression>) -> Option<Box<dyn Expression>>;

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,

    prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
    infix_parse_fns: HashMap<TokenType, InfixParseFn>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

impl Precedence {
    fn from_token_type(token_type: &TokenType) -> Self {
        match token_type {
            TokenType::Eq | TokenType::NotEq => Precedence::Equals,
            TokenType::Lt | TokenType::Gt => Precedence::LessGreater,
            TokenType::Plus | TokenType::Minus => Precedence::Sum,
            TokenType::Slash | TokenType::Asterisk => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }
}

impl Parser {
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

        p.register_prefix(TokenType::Int, Parser::parse_integer_literal);
        p.register_prefix(TokenType::Bang, Parser::parse_prefix_expression);
        p.register_prefix(TokenType::Minus, Parser::parse_prefix_expression);

        p
    }

    fn register_prefix(&mut self, token_type: TokenType, function: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_type, function);
    }

    fn register_infix(&mut self, token_type: TokenType, function: InfixParseFn) {
        self.infix_parse_fns.insert(token_type, function);
    }

    fn parse_expression(&mut self, _precedence: Precedence) -> Option<Box<dyn Expression>> {
        let prefix = self
            .prefix_parse_fns
            .get(&self.cur_token.token_type)
            .copied();

        if prefix.is_none() {
            let token_type = self.cur_token.token_type.clone();
            self.no_prefix_parse_fn_error(&token_type);
            return None;
        }

        let left_exp = prefix.unwrap()(self);

        left_exp
    }

    fn no_prefix_parse_fn_error(&mut self, token_type: &TokenType) {
        let msg = format!("no prefix parse function for {:?} found", token_type);
        self.errors.push(msg);
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

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

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

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
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

    fn peek_token_is(&self, token_type: &TokenType) -> bool {
        &self.peek_token.token_type == token_type
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

        while self.cur_token.token_type != TokenType::Semicolon {
            self.next_token();
        }

        let stmt = LetStatement {
            token,
            name,
            value: None,
        };

        Some(Box::new(stmt))
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token = self.cur_token.clone();

        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        let stmt = ReturnStatement {
            token,
            return_value: None,
        };

        Some(Box::new(stmt))
    }

    fn cur_token_is(&self, t: TokenType) -> bool {
        self.cur_token.token_type == t
    }

    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    fn peek_error(&mut self, t: TokenType) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            t, self.peek_token.token_type
        );
        self.errors.push(msg);
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
}
