pub mod backend;
pub mod frontend;
pub mod util;

const SRC: &str = r#"

let a = "This is a string"
let a_copy = a

{
    let c = 'c'

    {
        let _long_str = 5000
    }
}

let b = 25.5234

print(a, a_copy, b)
exit(0)

"#;

fn main() {
    backend::gen(SRC);
}
