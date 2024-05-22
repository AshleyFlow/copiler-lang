use crate::frontend::parser::{Expression, Statement};

pub enum GenType {
    VariableDeclaration {
        ident: String,
        value: String,
        value_type: Option<String>,
    },
    FunctionCall {
        ident: String,
        values: Vec<String>,
    },
    LScope,
    RScope,
}

pub struct CodeGen {
    pub src: String,
    root_stmt: Statement,
    nest: usize,
}

impl CodeGen {
    pub fn new(stmt: Statement) -> Self {
        Self {
            src: String::new(),
            root_stmt: stmt,
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
            GenType::FunctionCall { ident, values } => {
                let mut values_str = values[0].clone();

                for value in values.iter().skip(1) {
                    values_str += &(", ".to_owned() + value)
                }

                format!("{ident}({values_str})")
            }
        };

        let spaces = "    ".repeat(self.nest);
        self.src += &format!("{spaces}{code}\n");
    }

    fn expr_to_value(expr: Expression) -> (Option<String>, String) {
        let type_str: Option<String> = match expr {
            Expression::Identifier(_) => None,
            Expression::Char(_) | Expression::String(_) => Some("string".into()),
            Expression::Number(_) => Some("number".into()),
        };

        let value_str = match expr {
            Expression::Identifier(ident) => ident,
            Expression::Char(char) => format!("\"{char}\""),
            Expression::String(string) => format!("\"{string}\""),
            Expression::Number(number) => number.to_string(),
        };

        (type_str, value_str)
    }

    #[allow(clippy::only_used_in_recursion)]
    fn gen_statement(&mut self, stmt: Statement) {
        match stmt {
            Statement::Scope(statements) => {
                self.write(GenType::LScope);
                self.nest += 1;

                for stmt in statements {
                    self.gen_statement(stmt.clone());
                }

                self.nest -= 1;
                self.write(GenType::RScope);
            }
            Statement::VariableDeclaration { ident, value } => {
                if let Expression::Identifier(value_ident) = value.clone() {
                    self.write(GenType::VariableDeclaration {
                        ident: match ident {
                            Expression::Identifier(ident) => ident,
                            _ => panic!("{ident:?} can't be converted to identifier"),
                        },
                        value: value_ident.to_owned(),
                        value_type: None,
                    });
                } else {
                    let (type_str, value_str) = Self::expr_to_value(value);

                    self.write(GenType::VariableDeclaration {
                        ident: match ident {
                            Expression::Identifier(ident) => ident,
                            _ => panic!("{ident:?} can't be converted to identifier"),
                        },
                        value: value_str,
                        value_type: type_str,
                    });
                }
            }
            Statement::FunctionCall { ident, args } => self.write(GenType::FunctionCall {
                ident: match ident {
                    Expression::Identifier(ident) => ident,
                    _ => panic!("{ident:?} can't be converted to identifier"),
                },
                values: args
                    .iter()
                    .map(|expr| {
                        let (_, value_str) = Self::expr_to_value(expr.clone());
                        value_str
                    })
                    .collect(),
            }),
        }
    }

    pub fn run(&mut self) {
        if let Statement::Scope(scope) = self.root_stmt.clone() {
            for stmt in scope {
                self.gen_statement(stmt.clone());
            }
        } else {
            panic!("Root stmt must be a scope");
        }
    }
}
