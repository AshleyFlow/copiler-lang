use lexer::Lexer;

pub mod cursor;
mod lexer;

fn main() {
    let src = "exit(0)";

    let mut lexer = Lexer::new(src);
    let _tokens = lexer.load();
}
