use crate::cursor::{Cursor, ItemKind};

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Identifier(String),
    Literal(Literal),

    ParenOpen,
    ParenClose,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Literal {
    Number(f32),
}

impl ItemKind for Token {
    fn kind(&self) -> u8 {
        match self {
            Self::Identifier(_) => 0,
            Self::Literal(_) => 1,
            Self::ParenClose => 2,
            Self::ParenOpen => 3,
        }
    }
}

pub struct Lexer {
    cursor: Cursor<char>,
    tokens: Vec<Token>,
}

impl<'lexer> Lexer {
    fn identifier(&mut self) {
        let mut buffer = String::from(self.cursor.eat().unwrap());

        while self
            .cursor
            .peek_iff(None, |char| char.is_alphabetic())
            .is_some()
        {
            buffer.push(self.cursor.eat().unwrap());
        }

        self.tokens.push(Token::Identifier(buffer));
    }

    fn number(&mut self) {
        let mut buffer = String::from(self.cursor.eat().unwrap());

        while self
            .cursor
            .peek_iff(None, |char| char.is_numeric() || char == '.')
            .is_some()
        {
            buffer.push(self.cursor.eat().unwrap());
        }

        let float: f32 = buffer.parse().unwrap();

        if float.to_string() != buffer {
            println!("Warning: '{}' turns into '{}'", buffer, float);
        }

        self.tokens.push(Token::Literal(Literal::Number(float)));
    }

    fn next(&mut self) -> Option<&Token> {
        let char = self.cursor.peek(None);
        let len = self.tokens.len();

        if let Some(char) = char {
            match char {
                'a'..='z' | 'A'..='Z' => {
                    self.identifier();
                }
                '0'..='9' => {
                    self.number();
                }
                '(' => {
                    self.tokens.push(Token::ParenOpen);
                    self.cursor.eat();
                }
                ')' => {
                    self.tokens.push(Token::ParenClose);
                    self.cursor.eat();
                }
                '"' => {
                    unimplemented!()
                }
                '\'' => {
                    unimplemented!()
                }
                _ => {
                    if !char.is_whitespace() {
                        panic!("Unexpected char: '{}'", char)
                    }

                    self.cursor.eat();
                }
            }
        }

        if len != self.tokens.len() {
            self.tokens.last()
        } else {
            None
        }
    }

    pub fn load(&'lexer mut self) -> &'lexer Vec<Token> {
        while self.cursor.peek(None).is_some() {
            self.next();
        }

        &self.tokens
    }

    pub fn new(src: &'lexer str) -> Self {
        let cursor = Cursor::new(src.chars().collect());
        let tokens = vec![];

        Self { cursor, tokens }
    }
}
