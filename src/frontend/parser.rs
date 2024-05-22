use crate::frontend::lexer::{Literal, Token};
use crate::util::cursor::Cursor;

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration {
        ident: Expression,
        value: Expression,
    },
    ClassConstructor {
        ident: Expression,
        body: Expression,
    },
    Scope(Vec<Statement>),
    FunctionCall {
        ident: Expression,
        args: Vec<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Function {
        params: Vec<Expression>,
        stmt: Box<Statement>,
    },
    Parameter {
        ident: Box<Expression>,
        expected_type: Box<Option<Expression>>,
    },
    ClassBody,
    Identifier(String),
    String(String),
    Char(char),
    Number(f32),
}

pub struct Parser {
    cursor: Cursor<Token>,
}

impl Parser {
    fn parse_expression(&mut self) -> Option<Expression> {
        if let Some(token) = self.cursor.peek(None) {
            let token = match token {
                Token::Identifier(identifier) => Some(Expression::Identifier(identifier)),
                Token::Literal(literal) => match literal {
                    Literal::Char(char) => Some(Expression::Char(char)),
                    Literal::Identifier(ident) => Some(Expression::Identifier(ident)),
                    Literal::Number(number) => Some(Expression::Number(number)),
                    Literal::String(string) => Some(Expression::String(string)),
                },
                _ => None,
            };

            if token.is_some() {
                self.cursor.eat();

                token
            } else {
                None
            }
        } else {
            None
        }
    }

    fn parse_parameter(&mut self) -> Option<Expression> {
        let ident = self.parse_expression();

        ident.as_ref()?;

        let expected_type = if self
            .cursor
            .eat_iff(|token| matches!(token, Token::Colon))
            .is_some()
        {
            self.parse_expression()
        } else {
            None
        };

        Some(Expression::Parameter {
            ident: Box::new(ident.unwrap()),
            expected_type: Box::new(expected_type),
        })
    }

    fn parse_class(&mut self) -> Statement {
        // Statement::ClassConstructor { ident: (), body: () }
        todo!()
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

        if matches!(self.cursor.peek(None), Some(Token::LParen)) {
            let mut params: Vec<Expression> = vec![];
            self.cursor.eat(); // (

            while let Some(expr) = self.parse_parameter() {
                params.push(expr);

                if matches!(self.cursor.peek(None), Some(Token::Comma)) {
                    self.cursor.eat();
                } else {
                    break;
                }
            }

            self.cursor
                .eat_iff(|token| matches!(token, Token::RParen))
                .unwrap();

            self.cursor
                .eat_iff(|token| matches!(token, Token::LScope))
                .unwrap();

            let scope = self.parse_scope();

            Statement::VariableDeclaration {
                ident,
                value: Expression::Function {
                    params,
                    stmt: Box::new(scope),
                },
            }
        } else {
            let value = self.parse_expression().unwrap();

            Statement::VariableDeclaration { ident, value }
        }
    }

    fn parse_fn_call(&mut self, identifier: String) -> Statement {
        let mut args = vec![];
        let ident = Expression::Identifier(identifier);

        self.cursor
            .eat_iff(|token| matches!(token, Token::LParen))
            .unwrap();

        while let Some(expr) = self.parse_expression() {
            args.push(expr);

            if matches!(self.cursor.peek(None), Some(Token::Comma)) {
                self.cursor.eat();
            } else {
                break;
            }
        }

        self.cursor
            .eat_iff(|token| matches!(token, Token::RParen))
            .unwrap();

        Statement::FunctionCall { ident, args }
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        if let Some(token) = self.cursor.eat() {
            match token {
                Token::Identifier(identifier) => match identifier.as_str() {
                    "let" => Some(self.parse_variable()),
                    "class" => Some(self.parse_class()),
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
