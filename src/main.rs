use lexer::Lexer;

pub mod cursor;
mod lexer;

fn main() {
    let src = r#"
    
    print("Character ooo!!", 'c')
    exit(0)

    "#;

    let mut lexer = Lexer::new(src);
    let tokens = lexer.load();

    println!("{:?}", tokens);
}
