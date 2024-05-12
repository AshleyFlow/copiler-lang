use cursor::Cursor;
use lexer::{Lexer, Literal, Token};

pub mod cursor;
mod lexer;

const SRC: &str = r#"

let a = "This is a string"

{
    let c = 'c'
}

let b = 25.5234

"#;

#[derive(Debug)]
#[allow(dead_code)]
enum Expression {
    Variable(String, Literal),
    Value(Literal),
    Scope(Vec<Expression>),
}

struct Parser {
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

    pub fn scope(&mut self) -> Expression {
        let mut stack: Vec<Expression> = Vec::new();

        while let Some(token) = self.cursor.eat() {
            match token {
                Token::Literal(literal) => {
                    let expr = Expression::Value(literal);
                    stack.push(expr);
                }
                Token::Identifier(identifier) => {
                    if identifier == "let" {
                        stack.push(self.variable());
                    } else {
                        panic!("Unexpected identifier '{}'", identifier)
                    }
                }
                Token::ScopeOpen => stack.push(self.scope()),
                Token::ScopeClose => {
                    break;
                }
                _ => todo!(),
            }
        }

        Expression::Scope(stack)
    }

    /**
    Loads the scope and returns its last expression

    ### Example

    ```rs
    let mut lexer = Lexer::new("let a = 200 50.26");
    let tokens = lexer.load();
    let mut parser = Parser::new(tokens);
    let expression = parser.load_last_expr();

    println!("{:#?}", expression);

    /*
    --- OUTPUT ---

    Scope(
        [
            Variable(
                "a",
                Number(
                    200.0
                ),
            ),
            Value(
                Number(
                    50.26
                ),
            ),
        ]
    )
    */
    ```
    **/
    pub fn load(&mut self) -> Expression {
        self.scope()
    }

    pub fn new(tokens: &[Token]) -> Self {
        Self {
            cursor: Cursor::new(tokens.to_owned()),
        }
    }
}

fn main() {
    let mut lexer = Lexer::new(SRC);
    let tokens = lexer.load();
    let mut parser = Parser::new(tokens);
    let expression = parser.load();

    println!("{:#?}", expression);
}
