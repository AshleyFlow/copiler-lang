use crate::frontend::{
    lexer::{Lexer, Literal},
    parser::{Expression, Parser},
};

struct CodeGen {
    pub src: String,
    root_expr: Expression,
    nest: usize,
}

impl CodeGen {
    fn new(expression: Expression) -> Self {
        Self {
            src: String::new(),
            root_expr: expression,
            nest: 0,
        }
    }

    fn write(&mut self, code: impl Into<String>) {
        let code = code.into();
        let spaces = "    ".repeat(self.nest);
        self.src += &format!("{spaces}{code}\n");
    }

    #[allow(clippy::only_used_in_recursion)]
    fn gen_expression(&mut self, expr: Expression) {
        match expr {
            Expression::Scope(expressions) => {
                self.write("do");
                self.nest += 1;

                for expr in expressions {
                    self.gen_expression(expr.clone());
                }

                self.nest -= 1;
                self.write("end");
            }
            Expression::Value(_) => panic!("Found standalone value expression"),
            Expression::Variable(key, value) => {
                let type_str = match value {
                    Literal::Identifier(_) => None,
                    Literal::Char(_) | Literal::String(_) => Some("string"),
                    Literal::Number(_) => Some("number"),
                };

                let value_str = match value {
                    Literal::Identifier(ident) => ident,
                    Literal::Char(char) => format!("\"{char}\""),
                    Literal::String(string) => format!("\"{string}\""),
                    Literal::Number(number) => number.to_string(),
                };

                if let Some(type_str) = type_str {
                    self.write(format!("local {key}: {type_str} = {value_str}"));
                } else {
                    self.write(format!("local {key} = {value_str}"));
                }
            }
        }
    }

    fn run(&mut self) {
        if let Expression::Scope(scope) = self.root_expr.clone() {
            for expr in scope {
                self.gen_expression(expr.clone());
            }
        } else {
            panic!("Root expression must be a scope");
        }
    }
}

pub fn gen(scr: &str) {
    let mut lexer = Lexer::new(scr);
    let tokens = lexer.load();

    let mut parser = Parser::new(tokens);
    let expression = parser.load();

    let mut codegen = CodeGen::new(expression);
    codegen.run();

    println!("{}", codegen.src);
}
