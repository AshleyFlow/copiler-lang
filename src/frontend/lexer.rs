use crate::util::cursor::Cursor;

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Identifier(String),
    Literal(Literal),

    Colon,
    LScope,
    RScope,
    Equal,
    Comma,
    LParen,
    RParen,
    Dot,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Literal {
    Identifier(String),
    String(String),
    Number(f32),
    Char(char),
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
            .peek_iff(None, |char| char.is_alphabetic() || char == '_')
            .is_some()
        {
            buffer.push(self.cursor.eat().unwrap());
        }

        self.tokens.push(Token::Identifier(buffer));
    }

    fn number(&mut self) {
        let mut buffer = String::from(self.cursor.eat().unwrap());

        while let Some(char) = self.cursor.eat_iff(|char| char.is_numeric() || char == '.') {
            buffer.push(char);
        }

        let float: f32 = buffer.parse().unwrap();

        if float.to_string() != buffer {
            println!("Warning: '{}' turns into '{}'", buffer, float);
        }

        self.tokens.push(Token::Literal(Literal::Number(float)));
    }

    fn string(&mut self) {
        let mut buffer = String::new();
        self.cursor.eat(); // "

        while let Some(char) = self.cursor.eat() {
            if char != '"' {
                buffer.push(char);
            } else {
                break;
            }
        }

        self.tokens.push(Token::Literal(Literal::String(buffer)));
    }

    fn char(&mut self) {
        self.cursor.eat(); // '

        let char = self.cursor.eat().unwrap();

        if self.cursor.eat_iff(|char| char == '\'').is_none() {
            panic!("Expected ' after one character")
        }

        self.tokens.push(Token::Literal(Literal::Char(char)));
    }

    fn next(&mut self) -> Option<&Token> {
        let char = self.cursor.peek(None);
        let len = self.tokens.len();

        if let Some(char) = char {
            match char {
                '_' | 'a'..='z' | 'A'..='Z' => {
                    self.identifier();
                }
                '0'..='9' => {
                    self.number();
                }
                '"' => self.string(),
                '\'' => self.char(),
                '{' => {
                    self.tokens.push(Token::LScope);
                    self.cursor.eat();
                }
                '}' => {
                    self.tokens.push(Token::RScope);
                    self.cursor.eat();
                }
                '(' => {
                    self.tokens.push(Token::LParen);
                    self.cursor.eat();
                }
                ')' => {
                    self.tokens.push(Token::RParen);
                    self.cursor.eat();
                }
                ',' => {
                    self.tokens.push(Token::Comma);
                    self.cursor.eat();
                }
                '=' => {
                    self.tokens.push(Token::Equal);
                    self.cursor.eat();
                }
                ':' => {
                    self.tokens.push(Token::Colon);
                    self.cursor.eat();
                }
                '.' => {
                    self.tokens.push(Token::Dot);
                    self.cursor.eat();
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
