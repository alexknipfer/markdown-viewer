use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod markdown_parser;

#[wasm_bindgen]
pub fn parse_input(input: &str) -> String {
    let result = markdown_parser::parse_input(input.to_string());
    result
}
