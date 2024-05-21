use crate::frontend::{
    lexer::{Lexer, Literal},
    parser::{Expression, Parser},
};

enum GenType {
    VariableDeclaration {
        ident: String,
        value: String,
        value_type: Option<String>,
    },
    LScope,
    RScope,
}

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

    fn write(&mut self, code: GenType) {
        let code: String = match code {
            GenType::LScope => "do".into(),
            GenType::RScope => "end".into(),
            GenType::VariableDeclaration {
                ident,
                value,
                value_type,
            } => {
                if let Some(value_type) = value_type {
                    format!("local {ident}: {value_type} = {value}")
                } else {
                    format!("local {ident} = {value}")
                }
            }
        };

        let spaces = "    ".repeat(self.nest);
        self.src += &format!("{spaces}{code}\n");
    }

    #[allow(clippy::only_used_in_recursion)]
    fn gen_expression(&mut self, expr: Expression) {
        match expr {
            Expression::Scope(expressions) => {
                self.write(GenType::LScope);
                self.nest += 1;

                for expr in expressions {
                    self.gen_expression(expr.clone());
                }

                self.nest -= 1;
                self.write(GenType::RScope);
            }
            Expression::Value(_) => panic!("Found standalone value expression"),
            Expression::Variable(key, value) => {
                let type_str = match value {
                    Literal::Identifier(_) => None,
                    Literal::Char(_) | Literal::String(_) => Some("string".into()),
                    Literal::Number(_) => Some("number".into()),
                };

                let value_str = match value {
                    Literal::Identifier(ident) => ident,
                    Literal::Char(char) => format!("\"{char}\""),
                    Literal::String(string) => format!("\"{string}\""),
                    Literal::Number(number) => number.to_string(),
                };

                self.write(GenType::VariableDeclaration {
                    ident: key,
                    value: value_str,
                    value_type: type_str,
                });
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
