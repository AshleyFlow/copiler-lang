use crate::frontend::lexer::{Literal, Token};
use crate::util::cursor::Cursor;

#[derive(Debug, Clone)]
pub enum Statement {
    VariableAssignment {
        ident: Expression,
        value: Expression,
    },
    VariableDeclaration {
        ident: Expression,
        value: Expression,
    },
    ClassConstructor {
        ident: Expression,
        body: Expression,
    },
    Return(Expression),
    If {
        expr: Expression,
        body: Box<Statement>,
    },
    Scope(Vec<Statement>),
    Luau(String),
}

#[derive(Debug, Clone)]
pub enum Expression {
    MethodCall {
        ident: Box<Expression>,
        args: Vec<Expression>,
    },
    FunctionCall {
        ident: Box<Expression>,
        args: Vec<Expression>,
    },
    Function {
        params: Vec<Expression>,
        stmt: Box<Statement>,
    },
    Parameter {
        ident: Box<Expression>,
        expected_type: Box<Option<Expression>>,
    },
    ClassBody {
        properties: Vec<Statement>,
        methods: Vec<Statement>,
    },
    Identifier(String),
    Indexing(Box<Expression>, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    String(String),
    Bool(bool),
    Char(char),
    Number(f32),
}

pub struct Parser {
    cursor: Cursor<Token>,
}

impl Parser {
    fn parse_anon_fn(&mut self) -> Option<Expression> {
        self.cursor
            .eat_iff(|token| matches!(token, Token::LParen))
            .unwrap();

        let mut args = vec![];

        while self
            .cursor
            .peek_iff(None, |token| !matches!(token, Token::RParen))
            .is_some()
        {
            args.push(self.parse_parameter().unwrap());

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

        self.cursor
            .eat_iff(|token| matches!(token, Token::RScope))
            .unwrap();

        self.cursor
            .eat_iff(|token| matches!(token, Token::RParen))
            .unwrap();

        Some(Expression::Function {
            params: args,
            stmt: Box::new(scope),
        })
    }

    fn parse_single_expression(&mut self) -> Option<Expression> {
        if let Some(token) = self.cursor.peek(None) {
            let token = match token {
                Token::LParen => return self.parse_anon_fn(),
                Token::Identifier(identifier) => {
                    let identifier = Expression::Identifier(identifier);

                    let dot = matches!(self.cursor.peek(Some(2)), Some(Token::Dot));
                    let colon = matches!(self.cursor.peek(Some(2)), Some(Token::Colon));

                    if dot || colon {
                        self.cursor.eat();
                        self.cursor.eat();

                        let indexed = self.parse_expression().unwrap();

                        match indexed.clone() {
                            Expression::FunctionCall { ident, args, .. } => {
                                if colon {
                                    return Some(Expression::MethodCall {
                                        ident: Box::new(Expression::Indexing(
                                            Box::new(identifier),
                                            ident,
                                        )),
                                        args,
                                    });
                                } else {
                                    return Some(Expression::FunctionCall {
                                        ident: Box::new(Expression::Indexing(
                                            Box::new(identifier),
                                            ident,
                                        )),
                                        args,
                                    });
                                }
                            }
                            _ => {
                                return Some(Expression::Indexing(
                                    Box::new(identifier),
                                    Box::new(indexed),
                                ));
                            }
                        }
                    } else if matches!(self.cursor.peek(Some(2)), Some(Token::LParen)) {
                        self.cursor.eat();
                        self.cursor.eat();

                        let mut args = vec![];

                        while self
                            .cursor
                            .peek_iff(None, |token| !matches!(token, Token::RParen))
                            .is_some()
                        {
                            args.push(self.parse_expression().unwrap());

                            if matches!(self.cursor.peek(None), Some(Token::Comma)) {
                                self.cursor.eat();
                            } else {
                                break;
                            }
                        }

                        Some(Expression::FunctionCall {
                            ident: Box::new(identifier),
                            args,
                        })
                    } else {
                        Some(identifier)
                    }
                }
                Token::Literal(literal) => match literal {
                    Literal::Char(char) => Some(Expression::Char(char)),
                    Literal::Identifier(ident) => Some(Expression::Identifier(ident)),
                    Literal::Number(number) => Some(Expression::Number(number)),
                    Literal::String(string) => Some(Expression::String(string)),
                    Literal::Bool(bool) => Some(Expression::Bool(bool)),
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

    fn parse_expression(&mut self) -> Option<Expression> {
        let single_expr = self.parse_single_expression();

        if let Some(ref l) = single_expr {
            if self
                .cursor
                .eat_iff(|token| matches!(token, Token::And))
                .is_some()
            {
                let r = self.parse_expression().unwrap();
                Some(Expression::And(Box::new(l.clone()), Box::new(r)))
            } else if self
                .cursor
                .eat_iff(|token| matches!(token, Token::Or))
                .is_some()
            {
                let r = self.parse_expression().unwrap();
                Some(Expression::Or(Box::new(l.clone()), Box::new(r)))
            } else {
                single_expr
            }
        } else {
            single_expr
        }
    }

    fn parse_parameter(&mut self) -> Option<Expression> {
        let ident = self
            .cursor
            .eat_iff(|token| matches!(token, Token::Identifier(_)));

        let ident = ident.as_ref()?;
        let ident = match ident {
            Token::Identifier(ident) => Expression::Identifier(ident.to_string()),
            _ => panic!(),
        };

        let expected_type = if self
            .cursor
            .eat_iff(|token| matches!(token, Token::Colon))
            .is_some()
        {
            self.parse_single_expression()
        } else {
            None
        };

        Some(Expression::Parameter {
            ident: Box::new(ident),
            expected_type: Box::new(expected_type),
        })
    }

    fn parse_class_body(&mut self) -> Expression {
        let mut properties = vec![];
        let mut methods = vec![];

        self.cursor
            .eat_iff(|token| matches!(token, Token::LScope))
            .unwrap();

        while self
            .cursor
            .eat_iff(|token| !matches!(token, Token::RScope))
            .is_some()
        {
            let property = self.parse_variable();

            if let Some(property) = property {
                if let Statement::VariableDeclaration { ident: _, value } = &property {
                    if matches!(value, Expression::Function { .. }) {
                        methods.push(property);
                    } else {
                        properties.push(property);
                    }
                }
            }
        }

        self.cursor.eat();

        Expression::ClassBody {
            properties,
            methods,
        }
    }

    fn parse_if_statement(&mut self) -> Statement {
        let expr = self.parse_expression().unwrap();

        self.cursor
            .eat_iff(|token| matches!(token, Token::LScope))
            .unwrap();

        let body = self.parse_scope();

        self.cursor
            .eat_iff(|token| matches!(token, Token::RScope))
            .unwrap();

        Statement::If {
            expr,
            body: Box::new(body),
        }
    }

    fn parse_class(&mut self) -> Statement {
        let ident = self.parse_expression();

        Statement::ClassConstructor {
            ident: ident.expect("Expected class identifier"),
            body: self.parse_class_body(),
        }
    }

    fn parse_variable(&mut self) -> Option<Statement> {
        let ident = self.parse_expression();
        let ident = ident.as_ref()?.clone();

        self.cursor.eat_iff(|token| matches!(token, Token::Equal))?;

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

            self.cursor
                .eat_iff(|token| matches!(token, Token::RScope))
                .unwrap();

            Some(Statement::VariableDeclaration {
                ident,
                value: Expression::Function {
                    params,
                    stmt: Box::new(scope),
                },
            })
        } else {
            let value = self.parse_expression();
            let value = value.as_ref()?.clone();

            Some(Statement::VariableDeclaration { ident, value })
        }
    }

    fn parse_variable_assignment(&mut self, ident: Option<Expression>) -> Statement {
        let ident = ident.unwrap_or_else(|| self.parse_expression().unwrap());

        self.cursor
            .eat_iff(|token| matches!(token, Token::Equal))
            .unwrap();

        let value = self.parse_expression().unwrap();

        Statement::VariableAssignment { ident, value }
    }

    fn parse_fn_call(&mut self, ident: Option<Expression>) -> Statement {
        let ident = ident.unwrap_or_else(|| self.parse_expression().unwrap());

        Statement::VariableDeclaration {
            ident: Expression::Identifier(String::from("_")),
            value: ident,
        }
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        if let Some(token) = self.cursor.peek(None) {
            match token.clone() {
                Token::Identifier(identifier) => match identifier.as_str() {
                    "let" => {
                        self.cursor.eat();
                        Some(self.parse_variable().unwrap())
                    }
                    "class" => {
                        self.cursor.eat();
                        Some(self.parse_class())
                    }
                    "if" => {
                        self.cursor.eat();
                        Some(self.parse_if_statement())
                    }
                    "return" => {
                        self.cursor.eat();
                        Some(Statement::Return(
                            self.parse_expression()
                                .unwrap_or(Expression::Identifier("nil".into())),
                        ))
                    }
                    _ => {
                        let index = self.parse_expression();

                        if matches!(self.cursor.peek(None), Some(Token::Equal)) {
                            Some(self.parse_variable_assignment(index))
                        } else {
                            Some(self.parse_fn_call(index))
                        }
                    }
                },
                Token::LScope => {
                    self.cursor.eat();

                    let scope = self.parse_scope();

                    self.cursor
                        .eat_iff(|token| matches!(token, Token::RScope))
                        .unwrap();

                    Some(scope)
                }
                Token::RScope => None,
                Token::Luau(code) => {
                    self.cursor.eat();

                    Some(Statement::Luau(code))
                }
                _ => todo!("{token:?}"),
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
