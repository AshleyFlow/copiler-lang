use crate::frontend::lexer::{Literal, Token};
use crate::util::cursor::Cursor;

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration {
        ident: Expression,
        value: Expression,
    },
    Scope(Vec<Statement>),
    FunctionCall {
        ident: Expression,
        args: Vec<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String),
    Value(Box<Expression>),
    Literal(Literal),
}

impl Expression {
    pub fn eval(&self) -> Literal {
        match self {
            Self::Literal(literal) => literal.clone(),
            Self::Value(value) => value.eval(),
            _ => panic!("{self:?} can't be evaluated into a literal"),
        }
    }
}

pub struct Parser {
    cursor: Cursor<Token>,
}

impl Parser {
    fn parse_expression(&mut self) -> Option<Expression> {
        if let Some(token) = self.cursor.eat() {
            match token {
                Token::Identifier(identifier) => Some(Expression::Identifier(identifier)),
                Token::Literal(literal) => {
                    Some(Expression::Value(Box::new(Expression::Literal(literal))))
                }
                _ => todo!(),
            }
        } else {
            None
        }
    }

    fn parse_variable(&mut self) -> Statement {
        let ident = self.parse_expression().unwrap();

        self.cursor.eat_iff(|token| {
            if let Token::Equal = token {
                true
            } else {
                panic!("Expected '=', got '{:?}'", token)
            }
        });

        let value = self.parse_expression().unwrap();

        Statement::VariableDeclaration { ident, value }
    }

    fn parse_fn_call(&mut self, _identifier: String) -> Statement {
        todo!()
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        if let Some(token) = self.cursor.eat() {
            match token {
                Token::Identifier(identifier) => match identifier.as_str() {
                    "let" => Some(self.parse_variable()),
                    _ => Some(self.parse_fn_call(identifier)),
                },
                Token::LScope => Some(self.parse_scope()),
                Token::RScope => None,
                _ => todo!(),
            }
        } else {
            None
        }
    }

    pub fn parse_scope(&mut self) -> Statement {
        let mut stack: Vec<Statement> = Vec::new();

        while let Some(expr) = self.parse_statement() {
            stack.push(expr);
        }

        Statement::Scope(stack)
    }

    pub fn load(&mut self) -> Statement {
        self.parse_scope()
    }

    pub fn new(tokens: &[Token]) -> Self {
        Self {
            cursor: Cursor::new(tokens.to_owned()),
        }
    }
}
