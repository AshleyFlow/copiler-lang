use std::{env::args, fs, path::PathBuf};

pub mod backend;
pub mod frontend;
pub mod util;

fn main() {
    let args = args().collect::<Vec<_>>();
    let src = fs::read(&args[1]).expect("Failed to read file");

    let out = backend::gen(&String::from_utf8(src).unwrap());
    let dist_dir = PathBuf::from("dist/");

    if !dist_dir.exists() {
        fs::create_dir_all(&dist_dir).expect("Failed to create dist directory");
    }

    fs::write(dist_dir.join("out.luau"), out).expect("Failed to write");
}
