mod lexer;

use crate::lexer::{Lexer, Token};
use std::iter::Peekable;

struct Parser<'a> {
    iter: Peekable<core::slice::Iter<'a, Token>>,
}

impl<'parser> Parser<'parser> {
    fn next(&mut self) {
        let token = self.iter.peek();

        if let Some(_token) = token {
            todo!()
        }
    }

    pub fn load(&mut self) {
        while self.iter.peek().is_some() {
            self.next();
        }
    }

    pub fn new(tokens: &'parser [Token]) -> Self {
        let iter = tokens.iter().peekable();

        Self { iter }
    }
}

fn main() {
    let src = "print(10.25) exit(0)";

    let mut lexer = Lexer::new(src);
    let tokens = lexer.load();

    let mut parser = Parser::new(tokens);
    parser.load();

    std::process::exit(0);
}
