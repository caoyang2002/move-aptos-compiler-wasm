use wasm_bindgen::prelude::*;
use move_compiler::*;

#[wasm_bindgen]
pub fn compile_move(source: &str) -> String {
    let a = "some string"; // 假设这是某个字符串切片变量
    return a.to_string(); // 转换为 String 类型并返回
}
