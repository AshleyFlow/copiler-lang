use crate::{
    backend::codegen::CodeGen,
    frontend::{lexer::Lexer, parser::Parser},
};

pub mod codegen;

pub fn gen(scr: &str) {
    let mut lexer = Lexer::new(scr);
    let tokens = lexer.load();

    let mut parser = Parser::new(tokens);
    let expression = parser.load();

    #[cfg(debug_assertions)]
    println!("{expression:#?}");

    let mut codegen = CodeGen::new(expression);
    codegen.run();

    println!("{}", codegen.src);
}
