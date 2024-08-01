use wasm_bindgen::prelude::wasm_bindgen;

pub mod backend;
pub mod frontend;
pub mod util;

#[wasm_bindgen]
pub fn compile(src: &str) -> String {
    backend::gen(src)
}
