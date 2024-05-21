use crate::frontend::lexer::{Literal, Token};
use crate::util::cursor::Cursor;

#[derive(Debug, Clone)]
pub enum Expression {
    Variable(String, Literal),
    Value(Literal),
    Scope(Vec<Expression>),
}

pub struct Parser {
    cursor: Cursor<Token>,
}

impl Parser {
    fn variable(&mut self) -> Expression {
        let identifier = if let Token::Identifier(identifier) = self.cursor.eat().unwrap() {
            identifier
        } else {
            panic!()
        };

        self.cursor.eat_iff(|token| {
            if let Token::Equal = token {
                true
            } else {
                panic!("Expected '=', got '{:?}'", token)
            }
        });

        let value = if let Token::Literal(literal) = self.cursor.eat().unwrap() {
            literal
        } else {
            panic!()
        };

        Expression::Variable(identifier, value)
    }

    pub fn parse_expression(&mut self) -> Option<Expression> {
        if let Some(token) = self.cursor.eat() {
            match token {
                Token::Literal(literal) => {
                    let expr = Expression::Value(literal);
                    Some(expr)
                }
                Token::Identifier(identifier) => {
                    if identifier == "let" {
                        Some(self.variable())
                    } else {
                        panic!("Unexpected identifier '{}'", identifier)
                    }
                }
                Token::LScope => Some(self.parse_scope()),
                Token::RScope => None,
                _ => todo!(),
            }
        } else {
            None
        }
    }

    pub fn parse_scope(&mut self) -> Expression {
        let mut stack: Vec<Expression> = Vec::new();

        while let Some(expr) = self.parse_expression() {
            stack.push(expr);
        }

        Expression::Scope(stack)
    }

    pub fn load(&mut self) -> Expression {
        self.parse_scope()
    }

    pub fn new(tokens: &[Token]) -> Self {
        Self {
            cursor: Cursor::new(tokens.to_owned()),
        }
    }
}
