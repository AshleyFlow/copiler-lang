use std::{iter::Peekable, str::Chars};

use crate::cursor::ItemKind;

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Sof,
    Eof,

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
            Self::Eof => 0,
            Self::Identifier(_) => 1,
            Self::Literal(_) => 2,
            Self::ParenClose => 3,
            Self::ParenOpen => 4,
            Self::Sof => 5,
        }
    }
}

pub struct Lexer<'a> {
    iter: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
}

impl<'lexer> Lexer<'lexer> {
    fn identifier(&mut self) {
        let mut buffer = String::from(self.iter.next().unwrap());

        while let Some(char) = self.iter.peek() {
            if char.is_alphanumeric() {
                buffer.push(self.iter.next().unwrap());
            } else {
                break;
            }
        }

        self.tokens.push(Token::Identifier(buffer));
    }

    fn number(&mut self) {
        let mut buffer = String::from(self.iter.next().unwrap());

        while let Some(char) = self.iter.peek() {
            if char.is_numeric() || *char == '.' {
                buffer.push(self.iter.next().unwrap());
            } else {
                break;
            }
        }

        let float: f32 = buffer.parse().unwrap();

        if float.to_string() != buffer {
            println!("Warning: '{}' turns into '{}'", buffer, float);
        }

        self.tokens.push(Token::Literal(Literal::Number(float)));
    }

    fn next(&mut self) -> Option<&Token> {
        let char = self.iter.peek();
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
                    self.iter.next().unwrap();
                }
                ')' => {
                    self.tokens.push(Token::ParenClose);
                    self.iter.next().unwrap();
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

                    self.iter.next().unwrap();
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
        self.tokens.push(Token::Sof);

        while self.iter.peek().is_some() {
            self.next();
        }

        self.tokens.push(Token::Eof);

        &self.tokens
    }

    pub fn new(src: &'lexer str) -> Self {
        let iter = src.chars().peekable();
        let tokens = vec![];

        Self { iter, tokens }
    }
}
