mod lexer;

use crate::lexer::{Lexer, Token};

fn main() {
    let src = "exit(0)";

    let mut lexer = Lexer::new(src);
    let tokens = lexer.load();

    println!("{:?}", tokens);

    std::process::exit(match tokens[2] {
        Token::Number(num) => num as i32,
        _ => panic!(),
    });
}
