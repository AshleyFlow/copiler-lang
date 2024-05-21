use backend::codegen;

pub mod backend;
pub mod frontend;
pub mod util;

const SRC: &str = r#"

let a = "This is a string"

{
    let c = 'c'

    {
        let _long_str = 5000
    }
}

let b = 25.5234

"#;

fn main() {
    codegen::gen(SRC);
}
